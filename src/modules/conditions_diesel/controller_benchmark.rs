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
    modules::conditions_diesel::{
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
        .route("/update", get(update_all))
}

pub async fn find_all(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    // get db connection
    let db_conn_result = _state.diesel_pool_pg.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}")).into());
        }
    };

    let mut durations = String::new();
    for _ in 0..10 {
        let start = Instant::now();
        let result: Result<Vec<Conditions>, AppError> = repository::find_all(&mut db_conn);
        match result {
            Ok(_) => {
                let duration = start.elapsed();
                if durations.len() == 0 {
                    durations = format!("{}", duration.as_millis());
                    continue;
                }
                durations = format!("{},{}", durations, duration.as_millis());
            }
            Err(err) => {
                return Err(err);
            }
        }
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
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    // get db connection
    let db_conn_result = _state.diesel_pool_pg.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}")).into());
        }
    };

    let start = Instant::now();

    let result = repository::delete_all(&mut db_conn);
    match result {
        Ok(Some(_)) => {
            let duration = start.elapsed();

            let status_code = StatusCode::OK;
            return Ok((
                status_code,
                Json(AppResponse::ok(
                    format!("Time in milliseconds: {} ms", duration.as_millis()),
                    None,
                )),
            ));
        }
        Ok(None) => {
            return Err(AppError::NotFound);
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub async fn generate(
    Path(size): Path<i32>,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    // get db connection
    let db_conn_result = _state.diesel_pool_pg.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}")).into());
        }
    };

    let mut durations = String::new();
    for _ in 0..10 {

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
    
            if conditions_list.len() == 1000 {
                let _result = repository::create_bacth(&mut db_conn, conditions_list.clone());
                conditions_list.clear();
                continue;
            }
            if c < size-1 {
                continue;
            }
    
            // println!("conditions_list size: {}", conditions_list.len());
            let _result = repository::create_bacth(&mut db_conn, conditions_list.clone());
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

pub async fn update_all(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    // get db connection
    let db_conn_result = _state.diesel_pool_pg.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}")).into());
        }
    };

    let mut vec: Vec<Conditions>;
    let result = repository::find_all(&mut db_conn);
    match result {
        Ok(value) => {
            vec = value;
        }
        Err(err) => {
            return Err(err);
        }
    };

    let start = Instant::now();
    for v in vec.iter_mut() {
        let location =
            util::generator::generate_word(util::generator::generate_numbers_usize(10, 20));
        let temperature = util::generator::generate_numbers_f64(27.0, 60.0);
        let humidity = util::generator::generate_numbers_f64(0.0, 100.0);
        v.location = location;
        v.humidity = Some(humidity);
        v.temperature = Some(temperature);

        let _ = repository::update_data(&mut db_conn, v.clone());
    }

    let duration = start.elapsed();

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse::ok(
            format!("Time in milliseconds: {} ms", duration.as_millis()),
            None,
        )),
    ));
}
