use chrono::{DateTime, FixedOffset, NaiveDateTime};
use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    let offset = FixedOffset::east_opt(3 * 3600)
        .ok_or_else(|| SerdeError::custom("Invalid timezone offset"))?;
    // Парсинг даты без временной зоны
    match NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S") {
        Ok(naive) => Ok(DateTime::from_naive_utc_and_offset(naive, offset)),
        Err(err) => Err(SerdeError::custom(format!(
            "Failed to parse DateTime: {err}"
        ))),
    }
}
