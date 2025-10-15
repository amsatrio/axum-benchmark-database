use diesel::prelude::Insertable;
use diesel::QueryableByName;
use uuid::Uuid;
use validator::Validate;
use crate::util::serializer::{date_serializer};
use crate::schema::{conditions};
use chrono::NaiveDateTime;
use diesel::{prelude::Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Clone,
    Queryable, Selectable, PartialEq,
    QueryableByName,
    Insertable
)]
#[diesel(table_name = conditions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
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

        Conditions { id: new_uuid.to_string(), created_on: date_now, location: request.location, temperature: request.temperature, humidity: request.humidity }
    }

    pub fn from_update_request(request: ConditionsRequest, mut existing: Conditions) -> Self {
        existing.humidity = request.humidity;
        existing.location = request.location;
        existing.temperature = request.temperature;

        existing
    }
}


#[derive(QueryableByName)]
pub struct CountResult {
    #[diesel(sql_type = diesel::sql_types::BigInt)]
    pub count: i64,
}
