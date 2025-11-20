pub mod datetime_serializer {
    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    fn time_to_json(t: NaiveDateTime) -> String {
        let datetime: DateTime<Local> = Local.from_utc_datetime(&t);
        let datetime_string = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        datetime_string
    }

    pub fn serialize<S: Serializer>(
        time: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        time_to_json(time.clone()).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        Ok(NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S").map_err(D::Error::custom)?)
    }
}

pub mod date_serializer {
    use chrono::{DateTime, Local, NaiveDate, NaiveTime, TimeZone};
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    fn time_to_json(t: NaiveDate) -> String {
        let naive_datetime = t.and_time(NaiveTime::from_hms_opt(0, 0, 0).unwrap());
        let datetime: DateTime<Local> = Local.from_utc_datetime(&naive_datetime);
        let datetime_string = datetime.format("%Y-%m-%d").to_string();
        datetime_string
    }

    pub fn serialize<S: Serializer>(
        time: &NaiveDate,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        time_to_json(time.clone()).serialize(serializer)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<NaiveDate, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        Ok(NaiveDate::parse_from_str(&time, "%Y-%m-%d").map_err(D::Error::custom)?)
    }
}