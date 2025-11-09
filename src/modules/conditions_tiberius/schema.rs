use crate::util::serializer::date_serializer;
use chrono::{DateTime, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Conditions {
    pub id: String,
    #[serde(with = "date_serializer")]
    pub created_on: NaiveDateTime,
    pub location: String,
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct ConditionsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub location: String,
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
}

impl Conditions {
    pub fn from_create_request(request: ConditionsRequest) -> Self {
        let date_now = chrono::Utc::now().naive_utc();

        let new_uuid = Uuid::new_v4();
        let id = request.id.unwrap_or(new_uuid.to_string());

        Conditions {
            id: id,
            created_on: date_now,
            location: request.location,
            temperature: request.temperature,
            humidity: request.humidity,
        }
    }

    pub fn from_update_request(request: ConditionsRequest, mut existing: Conditions) -> Self {
        existing.humidity = request.humidity;
        existing.location = request.location;
        existing.temperature = request.temperature;

        existing
    }

    pub fn from_row_tiberius(row: &tiberius::Row) -> Self {
        let created_on = row
            .get::<&str, _>("created_on")
            .and_then(|s| NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").ok())
            .unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap().naive_utc());

        Conditions {
            id: row.get::<&str, _>("id").unwrap_or_default().to_owned(),
            created_on: created_on,
            location: row
                .get::<&str, _>("location")
                .unwrap_or_default()
                .to_owned(),
            temperature: row.get::<f64, _>("temperature"),
            humidity: row.get::<f64, _>("id"),
        }
    }
}

pub struct CountResult {
    pub count: i64,
}
