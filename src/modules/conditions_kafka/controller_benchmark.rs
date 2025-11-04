use std::{sync::Arc, time::Duration};

use axum::{Extension, Json, Router, extract::Path, http::StatusCode, routing::get};
use futures_util::StreamExt;
use rdkafka::{
    ClientConfig, Message,
    consumer::{Consumer, StreamConsumer},
    producer::{FutureProducer, FutureRecord},
};
use tokio::time::Instant;

use crate::{
    dto::{app_error::AppError, app_response::AppResponse},
    modules::conditions_kafka::{
        repository,
        schema::{Conditions, ConditionsRequest},
    },
    state::AppState,
    util,
};

pub fn new() -> Router {
    Router::new()
        .route("/producer/{size}", get(producer))
        .route("/consumer", get(consumer))
}

pub async fn producer(
    Path(size): Path<i32>,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092")
        .set("message.timeout.ms", "5000")
        .set("message.max.bytes", "10000000")
        .set("message.copy.max.bytes", "65535")
        .set("receive.message.max.bytes", "100000000")
        .create()
        .expect("Producer creation error");

    let topic = "my-topic";
    let key = "conditions";

    let mut durations = String::new();

    for i in 0..10 {
        let start = Instant::now();
        let mut conditions_list: Vec<Conditions> = Vec::new();
        for c in 0..size {
            let location =
                util::generator::generate_word(util::generator::generate_numbers_usize(10, 20));
            let temperature = util::generator::generate_numbers_f64(27.0, 60.0);
            let humidity = util::generator::generate_numbers_f64(0.0, 100.0);

            let _conditions_request = ConditionsRequest {
                id: None,
                location: location,
                temperature: Some(temperature),
                humidity: Some(humidity),
            };
            let new_conditions = Conditions::from_create_request(_conditions_request);
            conditions_list.push(new_conditions);

            if conditions_list.len() == 5000 {
                let payload_bytes: Vec<u8> = match serde_json::to_vec(&conditions_list) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("❌ Failed to serialize struct to bytes: {}", e);
                        continue;
                    }
                };
                let key_final = format!("{}_{}", key, c);
                // println!("size: {}", payload_bytes.len());
                let record = FutureRecord::to(topic)
                    .key(&key_final)
                    .payload(&payload_bytes);

                let _result = producer.send(record, Duration::from_secs(0)).await;
                match _result {
                    Err((e, _)) => eprintln!("Failed to deliver message: {:?}", e),
                    Ok(_) => {}
                }
                conditions_list.clear();
                continue;
            }
            if c < size - 1 {
                continue;
            }

            let payload_bytes: Vec<u8> = match serde_json::to_vec(&conditions_list) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("❌ Failed to serialize struct to bytes: {}", e);
                    continue;
                }
            };
            let key_final = format!("{}_{}", key, -1);
            let record = FutureRecord::to(topic)
                .key(&key_final)
                .payload(&payload_bytes);

            let _result = producer.send(record, Duration::from_secs(0)).await;
            match _result {
                Err((e, _)) => eprintln!("Failed to deliver message: {:?}", e),
                Ok(_) => {}
            }
            conditions_list.clear();
        }

        let duration = start.elapsed();
        durations = format!("{},{}", durations, duration.as_millis());
    }

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse::ok(
            format!("Time in milliseconds: {} ms", durations),
            None,
        )),
    ));
}
pub async fn consumer(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    let mut client = _state.tokio_postgres_client.lock().await;

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092")
        .set("group.id", "my-consumer-group")
        // Start consuming from the beginning of the topic if no offset is stored
        .set("auto.offset.reset", "earliest")
        .set("fetch.message.max.bytes", "10485760")
        .set("fetch.max.bytes", "52428800")
        .set("message.max.bytes", "1000000")
        .set("message.copy.max.bytes", "65535")
        .set("receive.message.max.bytes", "100000000")
        .create()
        .expect("Consumer creation error");

    consumer
        .subscribe(&["my-topic"])
        .expect("Can't subscribe to topic");

    println!(
        "Consumer started. Waiting for messages on topic '{}'...",
        "my-topic"
    );

    // Message stream: use .next() to wait for the next message
    // If you prefer a loop with explicit polling, use `consumer.recv().await`
    let mut message_stream = consumer.stream();

    while let Some(message_result) = message_stream.next().await {
        match message_result {
            Ok(message) => {
                let key = match message.key_view::<str>() {
                    Some(Ok(s)) => s,
                    _ => "N/A",
                };

                if !String::from(key).contains("conditions") {
                    continue;
                }

                let payload = message.payload().unwrap_or(&[]);
                match serde_json::from_slice::<Vec<Conditions>>(payload) {
                    Ok(conditions) => {
                        let _ = repository::insert_batch(&mut client, conditions).await;
                    }
                    Err(e) => {
                        eprintln!("❌ Failed to deserialize payload: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ Kafka error: {}", e);
            }
        }
    }

    let status_code = StatusCode::OK;
    return Ok((status_code, Json(AppResponse::ok(format!("success"), None))));
}
