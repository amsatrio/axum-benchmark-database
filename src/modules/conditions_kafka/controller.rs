use std::{sync::Arc, time::Duration};

use axum::{http::StatusCode, routing::get, Extension, Json, Router};
use futures_util::StreamExt;
use rdkafka::{consumer::{Consumer, StreamConsumer}, producer::{FutureProducer, FutureRecord}, ClientConfig, Message};

use crate::{dto::{app_error::AppError, app_response::AppResponse}, modules::conditions_kafka::schema::Conditions, state::AppState};

pub fn new() -> Router {
    Router::new()
        .route("/producer", get(producer))
        .route("/consumer", get(consumer))
}

pub async fn producer(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let topic = "my-topic";
    let key = "my_key";
    let payload = "Hello from Rust Kafka!";

    let record = FutureRecord::to(topic).key(key).payload(payload);

    match producer.send(record, Duration::from_secs(0)).await {
        Ok(delivery) => println!("Delivered message to Kafka: {:?}", delivery),
        Err((e, _)) => eprintln!("Failed to deliver message: {:?}", e),
    }

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse::ok(
            format!("success"),
            None,
        )),
    ));
}
pub async fn consumer(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:29092")
        .set("group.id", "my-consumer-group")
        // Start consuming from the beginning of the topic if no offset is stored
        .set("auto.offset.reset", "earliest") 
        .create()
        .expect("Consumer creation error");

    consumer
        .subscribe(&["my-topic"])
        .expect("Can't subscribe to topic");

    println!("Consumer started. Waiting for messages on topic '{}'...", "my-topic");

    // Message stream: use .next() to wait for the next message
    // If you prefer a loop with explicit polling, use `consumer.recv().await`
    let mut message_stream = consumer.stream(); 

    while let Some(message_result) = message_stream.next().await {
        match message_result {
            Ok(message) => {
                let payload = match message.payload_view::<str>() {
                    Some(Ok(s)) => s,
                    _ => "N/A",
                };
                let key = match message.key_view::<str>() {
                    Some(Ok(s)) => s,
                    _ => "N/A",
                };

                println!("📦 Received message | Key: {} | Payload: {} | Partition: {} | Offset: {}",
                    key, payload, message.partition(), message.offset());

                // Manually commit offset for at-least-once processing (optional)
                // consumer.commit_message(&message, CommitMode::Async).unwrap();
            }
            Err(e) => {
                eprintln!("❌ Kafka error: {}", e);
            }
        }
    }

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse::ok(
            format!("success"),
            None,
        )),
    ));
}
