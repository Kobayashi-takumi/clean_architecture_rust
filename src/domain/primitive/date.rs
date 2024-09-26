use crate::shared::error::Error;
use chrono::{DateTime, Duration, Utc};

#[derive(Debug, Clone, PartialEq)]
pub struct Date {
    value: DateTime<Utc>,
}

impl Default for Date {
    fn default() -> Self {
        Self { value: Utc::now() }
    }
}

impl TryFrom<i64> for Date {
    type Error = Error;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(Self {
            value: DateTime::<Utc>::from_timestamp_millis(value)
                .ok_or(Error::InvalidFormat("日付".to_string()))?,
        })
    }
}

impl From<DateTime<Utc>> for Date {
    fn from(value: DateTime<Utc>) -> Self {
        Self { value }
    }
}

impl Date {
    pub fn add_days(mut self, days: i64) -> Self {
        self.value += Duration::days(days);
        self
    }
    pub fn add_minutes(mut self, minutes: i64) -> Self {
        self.value += Duration::minutes(minutes);
        self
    }
    pub fn add_hours(mut self, hours: i64) -> Self {
        self.value += Duration::hours(hours);
        self
    }
    pub fn utc(&self) -> DateTime<Utc> {
        self.value
    }
    pub fn timestamp(&self) -> i64 {
        self.value.timestamp_millis()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::shared::error::Result;
    use crate::shared::test::*;
    use chrono::NaiveDateTime;

    #[test]
    fn from_i64() -> Result<()> {
        let table = vec![
            TestStruct {
                title: "1970-01-01 00:00:00",
                input: 0,
                output: Some(DateTime::<Utc>::from_naive_utc_and_offset(
                    NaiveDateTime::parse_from_str("1970-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                        .map_err(|e| {
                            log::error!("{e}");
                            Error::Unknown
                        })?,
                    Utc,
                )),
                has_error: false,
            },
            TestStruct {
                title: "unixtimestamp形式のi64",
                input: 1727222400000,
                output: Some(DateTime::<Utc>::from_naive_utc_and_offset(
                    NaiveDateTime::parse_from_str("2024-09-25 00:00:00", "%Y-%m-%d %H:%M:%S")
                        .map_err(|e| {
                            log::error!("{e}");
                            Error::Unknown
                        })?,
                    Utc,
                )),
                has_error: false,
            },
        ];
        let func = |value: i64| {
            let date = Date::try_from(value)?;
            Ok::<DateTime<Utc>, Error>(date.utc())
        };
        test_runner(table, func)?;
        Ok(())
    }
    #[test]
    fn from_datetime() -> Result<()> {
        let table = vec![
            TestStruct {
                title: "1970-01-01 00:00:00",
                input: DateTime::<Utc>::from_naive_utc_and_offset(
                    NaiveDateTime::parse_from_str("1970-01-01 00:00:00", "%Y-%m-%d %H:%M:%S")
                        .map_err(|e| {
                            log::error!("{e}");
                            Error::Unknown
                        })?,
                    Utc,
                ),
                output: Some(0),
                has_error: false,
            },
            TestStruct {
                title: "unixtimestamp形式のi64",
                input: DateTime::<Utc>::from_naive_utc_and_offset(
                    NaiveDateTime::parse_from_str("2024-09-25 00:00:00", "%Y-%m-%d %H:%M:%S")
                        .map_err(|e| {
                            log::error!("{e}");
                            Error::Unknown
                        })?,
                    Utc,
                ),
                output: Some(1727222400000),
                has_error: false,
            },
        ];
        let func = |value: DateTime<Utc>| {
            let date = Date::from(value);
            Ok::<i64, Error>(date.timestamp())
        };
        test_runner(table, func)?;
        Ok(())
    }
}
