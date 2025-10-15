use std::sync::Arc;

use axum::{
    extract::Path, http::StatusCode, routing::{delete, get, post, put}, Extension, Json, Router
};
use validator::Validate;

use crate::{
    dto::{app_error::AppError, app_response::AppResponse},
    modules::conditions_diesel::{
        repository,
        schema::{Conditions, ConditionsRequest},
    },
    state::AppState,
};

pub fn new() -> Router {
    Router::new()
        .route("/list", get(find_all))
        .route("/", post(create))
        .route("/", put(update))
        .route("/{id}", delete(delete_by_id))
        .route("/{id}", get(find_by_id))
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

    let result =
        repository::find_all(&mut db_conn).map_err(|error| AppError::Other(format!("get data failed, {:?}", error)))?;

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse::ok(format!("success"), Some(result))),
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