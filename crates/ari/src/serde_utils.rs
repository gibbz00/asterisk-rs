pub mod ari_date_format {
    //! Adapted from: <https://github.com/jabber-tools/asterisk-ari-client-rs/blob/6befed33f64d464ef941918f8c7faee4f76ee848/src/models/channels.rs>

    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.f%:z";

    // pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    // where
    //     S: Serializer,
    // {
    //     serializer.serialize_str(&date.format(FORMAT).to_string())
    // }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let str = std::borrow::Cow::<'de, str>::deserialize(deserializer)?;
        DateTime::parse_from_str(&str, FORMAT)
            .map(|date_time| date_time.to_utc())
            .map_err(serde::de::Error::custom)
    }
}
