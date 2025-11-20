use crate::util::serializer::datetime_serializer;
use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use tiberius::{IntoRow, numeric::Decimal};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Conditions {
    pub id: String,
    #[serde(with = "datetime_serializer")]
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
    pub fn from_create_request_dummy(request: ConditionsRequest, c: i32) -> Self {
        let yaer_in_range: i32 = 2020 + (c % 6);
        let date_string = format!("{}-01-01 01:00:00", yaer_in_range);
        let date_now = NaiveDateTime::parse_from_str(&date_string, "%Y-%m-%d %H:%M:%S").unwrap();

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
        let created_on_datetime: DateTime<FixedOffset> = row.get("created_on").unwrap();
        Conditions {
            id: row.get::<&str, _>("id").unwrap_or_default().to_owned(),
            created_on: created_on_datetime.with_timezone(&Utc).naive_utc(),
            location: row.get::<&str, _>("location").unwrap_or_default().to_owned(),
            temperature: row.get::<f64, _>("temperature"),
            humidity: row.get::<f64, _>("humidity"),
        }
    }
    pub fn to_tiberius_row(data: &Self) -> tiberius::TokenRow<'static> {
		return (
			Some(data.id.clone()),
			Some(data.created_on),
			Some(data.location.clone()),
			data.temperature,
			data.humidity
		).into_row();
	}
}

pub struct CountResult {
    pub count: i64,
}
