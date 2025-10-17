use std::sync::Arc;

use axum::{
    extract::Path, http::StatusCode, routing::{delete, get, post, put}, Extension, Json, Router
};
use validator::Validate;

use crate::{
    dto::{app_error::AppError, app_response::AppResponse},
    modules::conditions::{
        repository,
        schema::{Conditions, ConditionsRequest},
    },
    state::AppState
};

pub fn new() -> Router {
    Router::new()
        .route("/list", get(find_all))
        .route("/", post(create))
        .route("/", put(update))
        .route("/{id}", delete(delete_by_id))
        .route("/{id}", get(find_by_id))
}

pub async fn find_all(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    // get db connection
    let client = _state.tokio_postgres_client.lock().await;

    let _result: Vec<Conditions> = repository::find_all(&client).await?;

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse::ok(
            format!("success"),
            Some(_result),
        )),
    ));
}

pub async fn find_by_id(
    Path(id): Path<String>,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Conditions>>), AppError> {
    let client = _state.tokio_postgres_client.lock().await;

    let _result: Conditions = repository::find_by_id(&client, id).await?;

    let status_code = StatusCode::OK;
    return Ok((
        status_code,
        Json(AppResponse::ok(
            format!("success"),
            Some(_result),
        )),
    ));
}


pub async fn delete_by_id(
    Path(id): Path<String>,
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    let client = _state.tokio_postgres_client.lock().await;

    let _ = repository::delete_by_id(&client, id).await?;

    let status_code = StatusCode::OK;
    return Ok((status_code, Json(AppResponse::ok(format!("success"), None))));
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

    let mut client = _state.tokio_postgres_client.lock().await;

    let new_conditions = Conditions::from_create_request(conditions_request);

    let _result = repository::find_by_id(&client, new_conditions.id.to_owned()).await;
    if _result.is_ok() {
        return Err(AppError::DataExist);
    }

    let _ = repository::insert_one(&mut client, new_conditions).await?;

    let status_code = StatusCode::OK;
    return Ok((status_code, Json(AppResponse::ok("success", None))));
}


pub async fn update(
    Extension(_state): Extension<Arc<AppState>>,
    Json(conditions_request): Json<ConditionsRequest>,
) -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    let _is_valid = match conditions_request.validate() {
        Ok(value) => value,
        Err(error) => {
            return Err(AppError::InvalidRequest(error).into());
        }
    };
    if conditions_request.id.is_none() {
        return Err(AppError::Other(format!("invalid id")));
    }

    let mut client = _state.tokio_postgres_client.lock().await;

    let _result = repository::find_by_id(&client, conditions_request.id.clone().unwrap()).await;
    if _result.is_err() {
        return Err(_result.err().unwrap());
    }

    let new_conditions = Conditions::from_update_request(conditions_request, _result.unwrap());

    let _ = repository::update_one(&mut client, new_conditions).await?;

    let status_code = StatusCode::OK;
    return Ok((status_code, Json(AppResponse::ok("success", None))));
}