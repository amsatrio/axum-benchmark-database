use std::sync::Arc;

use axum::{Extension, Json, Router, extract::Path, http::StatusCode, routing::{delete, get}};
use chrono::{NaiveDate, NaiveDateTime};
use tokio::{task::JoinHandle, time::Instant};

use crate::{dto::{app_error::AppError, app_response::AppResponse}, modules::conditions_tiberius_columns::{repository, schema::{Conditions, ConditionsRequest}}, state::AppState, util};


pub fn new() -> Router {
    Router::new()
        .route("/list", get(find_all))
        .route("/delete", delete(delete_all))
        .route("/generate/{size}", get(generate))
}

pub async fn find_all(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    let client = _state.pool_tiberius.clone();
    let mut client_thread = client.get().await.unwrap();

    let mut durations = String::new();
    for c in 0..10 {
        let start = Instant::now();
        let _result: Vec<Conditions> = repository::find_all_stream(&mut client_thread).await?;
        let duration = start.elapsed();
        durations = format!("{},{}", durations, duration.as_millis());
        // println!("{:?}", _result);
        println!("{:?}", c)
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

pub async fn delete_all(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    let client = _state.pool_tiberius.clone();
    let mut client_thread = client.get().await.unwrap();

    let _ = repository::delete_all(&mut client_thread).await?;

    let status_code = StatusCode::OK;
    return Ok((status_code, Json(AppResponse::ok(format!("success"), None))));
}


pub async fn generate(
    Path(size): Path<i32>,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    let client = _state.pool_tiberius.clone();

    let mut durations = String::new();
    for _ in 0..10 {
        let mut conditions_list: Vec<Conditions> = Vec::new();
        let start = Instant::now();

        let mut join_handlers: Vec<JoinHandle<Result<(), AppError>>> = Vec::new();

        for c in 0..size {
            let mut new_conditions = Conditions::from_create_request(ConditionsRequest::generate_request());
            let datetime_string = format!("202{}-01-01 00:00:00", c%6);
            new_conditions.created_on = NaiveDateTime::parse_from_str(&datetime_string, "%Y-%m-%d %H:%M:%S").map_err(|err| {AppError::Other(format!("{:?}", err))})?;
            let datetime_string = format!("202{}-01-01 00:00:00", c%6);
            new_conditions.modified_on = NaiveDateTime::parse_from_str(&datetime_string, "%Y-%m-%d %H:%M:%S").map_err(|err| {AppError::Other(format!("{:?}", err))})?;
            conditions_list.push(new_conditions.clone());
            // println!("{:?}", new_conditions);

            if conditions_list.len() == 10000 {
                let condition_thread = conditions_list.clone();
                let mut client_thread = client.get().await.map_err(|err| {AppError::Other(format!("{:?}", err))})?;
                join_handlers.push(tokio::spawn(async move {
                    return repository::insert_batch(&mut client_thread, condition_thread).await;
                }));
                conditions_list.clear();
            }
        }
        let mut client_thread = client.get().await.map_err(|err| {AppError::Other(format!("{:?}", err))})?;
        join_handlers.push(tokio::spawn(async move {
            return repository::insert_batch(&mut client_thread, conditions_list).await;
        }));

        for t in join_handlers {
            let result = t.await.map_err(|err| {AppError::Other(format!("{:?}", err))})?;
            match result {
                Err(err) => {
                    println!("{:?}", err)
                },
                Ok(_) => {},
            }
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