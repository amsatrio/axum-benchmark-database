use std::sync::Arc;

use axum::{Extension, Json, Router, http::StatusCode, routing::get};
use tokio::time::Instant;

use crate::{dto::{app_error::AppError, app_response::AppResponse}, modules::conditions_tiberius_columns::{repository, schema::Conditions}, state::AppState};



pub fn new() -> Router {
    Router::new()
        .route("/list", get(find_all))
        // .route("/delete", delete(delete_all))
        // .route("/generate/{size}", get(generate))
}

pub async fn find_all(
    Extension(_state): Extension<Arc<AppState>>,
) -> Result<(StatusCode, Json<AppResponse<Vec<Conditions>>>), AppError> {
    let client = _state.pool_tiberius.clone();
    let mut client_thread: deadpool_tiberius::deadpool::managed::Object<deadpool_tiberius::Manager> = client.get().await.unwrap();

    let mut durations = String::new();
    for _ in 0..10 {
        let start = Instant::now();
        let _result: Vec<Conditions> = repository::find_all_stream(&mut client_thread).await?;
        let duration = start.elapsed();
        durations = format!("{},{}", durations, duration.as_millis());
        println!("{:?}", _result);
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