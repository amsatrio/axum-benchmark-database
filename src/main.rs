use std::sync::Arc;

use axum::{Extension, Json, Router, http::StatusCode, routing::get};
use axum_benchmark_database::{
    config::{self, environment::CONFIG},
    dto::{app_error::AppError, app_response::AppResponse},
    modules::{conditions},
    state::AppState,
};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let diesel_pool = config::database::get_diesel_postgres_db_pool();
    let deadpool_postgres_pool = config::database::get_tokio_postgres_db_pool();

    let state = AppState {
        diesel_pool_pg: Arc::new(diesel_pool),
        pool_pg: deadpool_postgres_pool,
        status: "up".to_string(),
    };
    let shared_state = Arc::new(state);

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/conditions", conditions::controller::new())
        .layer(Extension(shared_state));

    let config_env = &CONFIG;
    let listener = TcpListener::bind(config_env.get_server_url()).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn root() -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    Ok((
        StatusCode::OK,
        Json(AppResponse::ok("success", Some("root".to_string()))),
    ))
}

async fn health_check() -> Result<(StatusCode, Json<AppResponse<String>>), AppError> {
    Ok((
        StatusCode::OK,
        Json(AppResponse::ok("success", Some("ok".to_string()))),
    ))
}
