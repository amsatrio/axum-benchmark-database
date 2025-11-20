use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::util::serializer::datetime_serializer;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct AppResponse<T> {
    pub status: u16,
    pub message: String,
    #[serde(with = "datetime_serializer")]
    pub timestamp: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<T>,
}


impl<T> AppResponse<T> {
    pub fn ok(message: impl Into<String>, data: Option<T>) -> Self {
        AppResponse { status: 200, message: message.into(), timestamp: Utc::now().naive_utc(), data: data, error: None }
    }
    pub fn err(status: u16, message: impl Into<String>, error_data: T) -> Self {
        AppResponse { status: status, message: message.into(), timestamp: Utc::now().naive_utc(), data: None, error: Some(error_data) }
    }
}