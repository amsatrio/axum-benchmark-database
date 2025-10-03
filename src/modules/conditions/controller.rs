use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
};
use tokio::time::Instant;
use validator::Validate;

use crate::{
    dto::{app_error::AppError, app_response::AppResponse},
    modules::conditions::{
        repository,
        schema::{Conditions, ConditionsRequest},
    },
    state::AppState,
    util,
};

pub fn new() -> Router {
    Router::new()
        .route("/list", get(find_all))
        .route("/", post(create))
        .route("/", put(update))
        .route("/delete/{id}", delete(delete_by_id))
        .route("/delete", delete(delete_all))
        .route("/{id}", get(find_by_id))
        .route("/generate/{size}", get(generate))
        .route("/update", get(update_all))
}

pub async fn find_by_id(
    Path(id): Path<String>,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Conditions>>), AppError> {
    // get db connection
    let db_conn_result = _state.diesel_pool_pg.get();
    let mut db_conn;
    match db_conn_result {
        Ok(value) => {
            db_conn = value;
        }
        Err(error) => {
            return Err(AppError::Other(format!("get connection failed {error}, id: {id}")).into());
        }
    };

    let result = repository::find_by_id(&mut db_conn, id);
    match result {
        Ok(Some(value)) => {
            let status_code = StatusCode::OK;
            return Ok((status_code, Json(AppResponse::ok("success", Some(value)))));
        }
        Ok(None) => {
            return Err(AppError::NotFound);
        }
        Err(err) => {
            return Err(err);
        }
    }
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
        let result = repository::find_all(&mut db_conn);
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

pub async fn delete_by_id(
    Path(id): Path<String>,
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
            return Err(AppError::Other(format!("get connection failed {error}, id: {id}")).into());
        }
    };

    let result = repository::delete_by_id(&mut db_conn, id);
    match result {
        Ok(Some(_)) => {
            let status_code = StatusCode::OK;
            return Ok((
                status_code,
                Json(AppResponse {
                    status: status_code.as_u16(),
                    message: "success".to_owned(),
                    timestamp: chrono::Utc::now().naive_utc(),
                    data: None,
                    error: None,
                }),
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

pub async fn create(
    Extension(_state): Extension<Arc<AppState>>,
    Json(conditions_request): Json<ConditionsRequest>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    let _is_valid = match conditions_request.validate() {
        Ok(value) => value,
        Err(error) => {
            return Err(AppError::InvalidRequest(error).into());
        }
    };

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

    let new_conditions = Conditions::from_create_request(conditions_request);
    let existing_biodata_result =
        repository::find_by_id(&mut db_conn, new_conditions.id.to_owned());
    match existing_biodata_result {
        Ok(Some(_)) => {
            return Err(AppError::DataExist);
        }
        Ok(None) => {}
        Err(err) => {
            return Err(err);
        }
    };

    let result = repository::create(&mut db_conn, new_conditions);

    match result {
        Ok(Some(_)) => {
            let status_code = StatusCode::OK;
            return Ok((status_code, Json(AppResponse::ok("success", None))));
        }
        Ok(None) => {
            return Err(AppError::Other(format!("save data failed")).into());
        }
        Err(err) => {
            return Err(err);
        }
    }
}

pub async fn update(
    Extension(_state): Extension<Arc<AppState>>,
    Json(payload): Json<ConditionsRequest>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    let _is_valid = match payload.validate() {
        Ok(value) => value,
        Err(error) => {
            return Err(AppError::InvalidRequest(error).into());
        }
    };

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

    let mut _new_data: Conditions;
    let existing_biodata_result =
        repository::find_by_id(&mut db_conn, payload.id.to_owned().unwrap());
    match existing_biodata_result {
        Ok(None) => {
            return Err(AppError::NotFound);
        }
        Ok(Some(value)) => {
            _new_data = <Conditions>::from_update_request(payload, value);
        }
        Err(err) => {
            return Err(err);
        }
    };

    let result = repository::update_data(&mut db_conn, _new_data);

    match result {
        Ok(Some(_)) => {
            let status_code = StatusCode::OK;
            return Ok((status_code, Json(AppResponse::ok("success", None))));
        }
        Ok(None) => {
            return Err(AppError::Other(format!("save data failed")).into());
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
