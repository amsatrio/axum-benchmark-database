
use crate::util::serializer::{date_serializer};
use crate::schema::{conditions_default};
use chrono::NaiveDateTime;
use diesel::{prelude::Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Deserialize,
    Serialize,
    Clone,
    Queryable, Selectable, PartialEq
)]
#[diesel(table_name = conditions_default)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ConditionsDefault {
    pub id: String,
    #[serde(with = "date_serializer")]
    pub created_on: NaiveDateTime,
    pub location: String,
    pub temperature: Option<f64>,
    pub humidity: Option<f64>,
}
