use std::collections::HashMap;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use validator::ValidationErrors;

use crate::dto::app_response::AppResponse;

#[derive(Debug, PartialEq)]
pub enum AppError {
    InvalidRequest(ValidationErrors),
    DataExist,
    NotFound,
    InternalServerError,
    Other(String),
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DataExist => {
                let status_code = StatusCode::BAD_REQUEST;
                (
                    status_code,
                    Json(AppResponse {
                        status: status_code.as_u16(),
                        message: "error".to_owned(),
                        timestamp: chrono::Utc::now().naive_utc(),
                        error: Some("resource exist".to_string()),
                        data: None,
                    }),
                )
                    .into_response()
            }
            AppError::NotFound => {
                let status_code = StatusCode::NOT_FOUND;
                (
                    status_code,
                    Json(AppResponse {
                        status: status_code.as_u16(),
                        message: "error".to_owned(),
                        timestamp: chrono::Utc::now().naive_utc(),
                        error: Some("resource not found".to_string()),
                        data: None,
                    }),
                )
                    .into_response()
            }
            AppError::InternalServerError => {
                let status_code = StatusCode::INTERNAL_SERVER_ERROR;
                (
                    status_code,
                    Json(AppResponse {
                        status: status_code.as_u16(),
                        message: "error".to_owned(),
                        timestamp: chrono::Utc::now().naive_utc(),
                        error: Some("internal server error".to_string()),
                        data: None
                    }),
                )
                    .into_response()
            }
            AppError::Other(message) => {
                let status_code = StatusCode::INTERNAL_SERVER_ERROR;
                (
                    status_code,
                    Json(AppResponse {
                        status: status_code.as_u16(),
                        message: "error".to_owned(),
                        timestamp: chrono::Utc::now().naive_utc(),
                        error: Some(message),
                        data: None
                    }),
                )
                    .into_response()
            }
            AppError::InvalidRequest(validation_errors) => {
                let status_code = StatusCode::BAD_REQUEST;
                (
                    status_code,
                    Json(AppResponse {
                        status: status_code.as_u16(),
                        message: "error".to_owned(),
                        timestamp: chrono::Utc::now().naive_utc(),
                        error: Some(parse_validation_error_message(&format!("{validation_errors}"))),
                        data: None
                    }),
                )
                    .into_response()
            }
        }
    }
}

fn parse_validation_error_message(error_message: &str) -> HashMap<String, String> {
    let mut error_map = HashMap::new();

    for line in error_message.lines() {
        let mut parts = line.split(": ");
        let field = parts.next().unwrap().to_string();
        let message = parts.next().unwrap().to_string();
        error_map.insert(field, message);
    }

    error_map
}