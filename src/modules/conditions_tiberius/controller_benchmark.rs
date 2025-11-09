use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get},
};
use tokio::time::Instant;

use crate::{
    dto::{app_error::AppError, app_response::AppResponse},
    modules::conditions_tiberius::{
        repository,
        schema::{Conditions, ConditionsRequest},
    },
    state::AppState,
    util,
};

pub fn new() -> Router {
    Router::new()
        .route("/list", get(find_all))
        .route("/delete", delete(delete_all))
        .route("/generate/{size}", get(generate))
}

pub async fn find_all(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    // get db connection
    let mut client: tokio::sync::MutexGuard<
        '_,
        tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
    > = _state.tiberius_client.lock().await;

    let _result: Vec<Conditions> = repository::find_all(&mut client).await?;

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse::ok(format!("success"), Some(_result))),
    ));
}

pub async fn delete_all(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    let mut client: tokio::sync::MutexGuard<
        '_,
        tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
    > = _state.tiberius_client.lock().await;

    let _ = repository::delete_all(&mut client).await?;

    let status_code = StatusCode::OK;
    return Ok((status_code, Json(AppResponse::ok(format!("success"), None))));
}

pub async fn generate(
    Path(size): Path<i32>,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    let mut client: tokio::sync::MutexGuard<
        '_,
        tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
    > = _state.tiberius_client.lock().await;

    let mut durations = String::new();
    for _ in 0..1 {
        let mut conditions_list: Vec<Conditions> = Vec::new();
        let start = Instant::now();
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

            if conditions_list.len() == 400 {
                let _result = repository::insert_batch(&mut client, conditions_list.clone()).await;
                conditions_list.clear();
                continue;
            }
            if c < size - 1 {
                continue;
            }

            let _result = repository::insert_batch(&mut client, conditions_list.clone()).await;
            conditions_list.clear();
        }
        let duration = start.elapsed();
        if durations.len() == 0 {
            durations = format!("{}", duration.as_millis());
            continue;
        }
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

pub async fn generate2(
    Path(size): Path<i32>,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    let mut client: tokio::sync::MutexGuard<
        '_,
        tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>,
    > = _state.tiberius_client.lock().await;

    let mut conditions_list: Vec<Conditions> = Vec::new();
    for _c in 0..size {
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
    }
    let _result = repository::insert_batch_2(&mut client, conditions_list.clone()).await;

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse::ok(
            format!("success"),
            None,
        )),
    ));
}
