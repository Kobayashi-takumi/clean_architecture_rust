use crate::domain::primitive::date::Date as DatePrimitive;
use crate::shared::error::Error;
use async_graphql::*;
use std::convert::From;

pub mod task;

#[derive(Debug, PartialEq, Clone)]
pub struct Date(DatePrimitive);

#[Scalar]
impl ScalarType for Date {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::Number(value) = &value {
            let value = value
                .as_i64()
                .ok_or(InputValueError::custom(Error::InvalidFormat(
                    "日付".to_string(),
                )))?;
            // ミリ秒単位
            let dt = DatePrimitive::try_from(value).map_err(|e| InputValueError::custom(e))?;
            Ok(Date(dt))
        } else {
            // If the type does not match
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        // Javascript/FlutterのTimestampをベースにしている
        Value::Number(Number::from(self.0.timestamp()))
    }
}

impl Date {
    pub fn value(&self) -> DatePrimitive {
        self.0.clone()
    }
    pub fn new(value: &DatePrimitive) -> Self {
        Date(value.clone())
    }
}

impl From<Date> for DatePrimitive {
    fn from(value: Date) -> Self {
        value.0.into()
    }
}

impl From<DatePrimitive> for Date {
    fn from(value: DatePrimitive) -> Self {
        Self(value)
    }
}

impl TryFrom<i64> for Date {
    type Error = Error;
    fn try_from(value: i64) -> std::result::Result<Self, Self::Error> {
        Ok(Date(value.try_into()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn date() {
        assert!(
            <Date as async_graphql::ScalarType>::parse(Value::Number(Number::from(
                1670385398272 as i64
            )))
            .is_ok()
        );
    }
}
