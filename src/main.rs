use std::sync::Arc;

use axum::{Extension, Json, Router, http::StatusCode, routing::get};
use axum_benchmark_database::{
    config::{self, environment::CONFIG},
    dto::{app_error::AppError, app_response::AppResponse},
    modules::{conditions, conditions_diesel, conditions_kafka},
    state::AppState,
};
use tokio::{net::TcpListener, sync::Mutex};

#[tokio::main]
async fn main() {
    let diesel_pool = config::database::get_diesel_postgres_db_pool();
    let deadpool_postgres_pool = config::database::get_tokio_postgres_db_pool();
    let tokio_postgres_client = config::database::get_tokio_postgresql().await.unwrap();

    let state = AppState {
        diesel_pool_pg: Arc::new(diesel_pool),
        pool_pg: deadpool_postgres_pool,
        tokio_postgres_client: Mutex::new(tokio_postgres_client),
        status: "up".to_string(),
    };
    let shared_state = Arc::new(state);

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/conditions/benchmark", conditions::controller_benchmark::new())
        .nest("/conditions/crud", conditions::controller_crud::new())
        .nest("/conditions_diesel/benchmark", conditions_diesel::controller_benchmark::new())
        .nest("/conditions_diesel/crud", conditions_diesel::controller_crud::new())
        .nest("/conditions_kafka", conditions_kafka::controller::new())
        .nest("/conditions_kafka/benchmark", conditions_kafka::controller_benchmark::new())
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
