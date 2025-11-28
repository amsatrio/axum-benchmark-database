use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get},
};
use chrono::{NaiveDate, NaiveDateTime};
use futures_util::future::join_all;
use tokio::{task::JoinHandle, time::{Instant, sleep, Duration}};

use crate::{
    dto::{app_error::AppError, app_response::AppResponse},
    modules::conditions_tiberius_columns::{
        repository,
        schema::{Conditions, ConditionsRequest},
    },
    state::AppState,
    util,
};

pub fn new() -> Router {
    Router::new()
        .route("/list_page", get(find_all_by_page))
        .route("/list", get(find_all))
        .route("/delete", delete(delete_all))
        .route("/generate/{size}", get(generate))
}

pub async fn find_all_by_page(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    let mut durations = String::new();
    for c in 0..10 {
        let start = Instant::now();

        // get total data
        let client = _state.pool_tiberius.clone();
        let mut client_total_data = client.get().await.unwrap();
        let total_data = repository::count_data(&mut client_total_data).await?;

        let limit = 10000;
        let mut total_page = total_data / limit;
        if total_data < limit {
            total_page = 0;
        } else if total_data % limit > 0 {
            total_page = total_page + 1;
        }

        // get the data
        let mut handles = Vec::new();
        let mut conditions: Vec<Conditions> = Vec::new();
        for p in 0..total_page {
            let client = _state.pool_tiberius.clone();

            let mut client_thread_result = client.get().await.map_err(|err| AppError::Other(format!("{:#?}", err)));
            loop {
                match client_thread_result {
                    Ok(d) => {
                        let mut client_thread: deadpool_tiberius::deadpool::managed::Object<deadpool_tiberius::Manager> = d;
                        let handle = tokio::spawn(async move {
                            repository::find_all_stream_pagination(&mut client_thread, p * limit, limit).await
                        });
                        handles.push(handle);
                        break;
                    },
                    Err(_) => {
                        client_thread_result = client.get().await.map_err(|err| AppError::Other(format!("{:#?}", err)));
                        let _ = sleep(Duration::from_millis(500)).await;
                    },
                }
            };
        }

        let result = join_all(handles).await;
        for r in result {
            let mut data = r.map_err(|err| AppError::Other(format!("{:#?}", err)))??;
            conditions.append(&mut data);
        }
        

        let duration = start.elapsed();
        durations = format!("{},{}", durations, duration.as_millis());
        // println!("{:?}", _result);
        println!("total data: {}", conditions.len());
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
            let mut new_conditions =
                Conditions::from_create_request(ConditionsRequest::generate_request());
            let datetime_string = format!("202{}-01-01 00:00:00", c % 6);
            new_conditions.created_on =
                NaiveDateTime::parse_from_str(&datetime_string, "%Y-%m-%d %H:%M:%S")
                    .map_err(|err| AppError::Other(format!("{:?}", err)))?;
            let datetime_string = format!("202{}-01-01 00:00:00", c % 6);
            new_conditions.modified_on =
                NaiveDateTime::parse_from_str(&datetime_string, "%Y-%m-%d %H:%M:%S")
                    .map_err(|err| AppError::Other(format!("{:?}", err)))?;
            conditions_list.push(new_conditions.clone());
            // println!("{:?}", new_conditions);

            if conditions_list.len() == 10000 {
                let condition_thread = conditions_list.clone();
                let mut client_thread_result = client.get().await.map_err(|err| AppError::Other(format!("{:#?}", err)));
                loop {
                    match client_thread_result {
                        Ok(d) => {
                            let mut client_thread: deadpool_tiberius::deadpool::managed::Object<deadpool_tiberius::Manager> = d;
                            join_handlers.push(tokio::spawn(async move {
                                return repository::insert_batch(&mut client_thread, condition_thread).await;
                            }));
                            conditions_list.clear();
                            break;
                        },
                        Err(_) => {
                            client_thread_result = client.get().await.map_err(|err| AppError::Other(format!("{:#?}", err)));
                            let _ = sleep(Duration::from_millis(500)).await;
                        },
                    }
                };
            }
        }

        let condition_thread = conditions_list.clone();
        let mut client_thread_result = client.get().await.map_err(|err| AppError::Other(format!("{:#?}", err)));
        loop {
            match client_thread_result {
                Ok(d) => {
                    let mut client_thread: deadpool_tiberius::deadpool::managed::Object<deadpool_tiberius::Manager> = d;
                    join_handlers.push(tokio::spawn(async move {
                        return repository::insert_batch(&mut client_thread, condition_thread).await;
                    }));
                    conditions_list.clear();
                    break;
                },
                Err(_) => {
                    client_thread_result = client.get().await.map_err(|err| AppError::Other(format!("{:#?}", err)));
                    let _ = sleep(Duration::from_millis(500)).await;
                },
            }
        };

        let result = join_all(join_handlers).await;

        for r in result {
            match r {
                Err(err) => {
                    println!("{:?}", err)
                }
                Ok(_) => {}
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
