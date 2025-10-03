pub mod date_serializer {
    use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    fn time_to_json(t: NaiveDateTime) -> String {
        let datetime: DateTime<Local> = Local.from_utc_datetime(&t);
        let datetime_string = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        // log::info!("datetime: {}", &datetime_string);
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