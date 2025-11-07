use std::sync::Arc;

use axum::{Extension, Json, Router, extract::Path, http::StatusCode, routing::{delete, get, post, put}};

use crate::{dto::{app_error::AppError, app_response::AppResponse}, modules::conditions_tiberius::{repository, schema::Conditions}, state::AppState};



pub fn new() -> Router {
    Router::new()
        .route("/list", get(find_all))
        // .route("/", post(create))
        // .route("/", put(update))
        .route("/{id}", delete(delete_by_id))
        // .route("/{id}", get(find_by_id))
}


pub async fn find_all(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    // get db connection
    let mut client: tokio::sync::MutexGuard<'_, tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>> = _state.tiberius_client.lock().await;

    let _result: Vec<Conditions> = repository::find_all(&mut client).await?;

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
    let mut client: tokio::sync::MutexGuard<'_, tiberius::Client<tokio_util::compat::Compat<tokio::net::TcpStream>>> = _state.tiberius_client.lock().await;

    let _ = repository::delete_by_id(&mut client, id).await?;

    let status_code = StatusCode::OK;
    return Ok((status_code, Json(AppResponse::ok(format!("success"), None))));
}