use crate::shared::error::Error;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct Id {
    value: Uuid,
}

impl TryFrom<&str> for Id {
    type Error = Error;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
            value: Uuid::from_str(value).map_err(|e| {
                log::error!("{e}");
                Error::InvalidFormat("Id".to_string())
            })?,
        })
    }
}

impl TryFrom<String> for Id {
    type Error = Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.as_str().try_into()
    }
}

impl From<Id> for String {
    fn from(value: Id) -> Self {
        value.value.to_string()
    }
}

impl Default for Id {
    fn default() -> Self {
        Self {
            value: Uuid::new_v4(),
        }
    }
}

impl Id {
    pub fn value(&self) -> Uuid {
        self.value
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::shared::error::Result;
    use crate::shared::test::*;

    #[test]
    fn from_string() -> Result<()> {
        let table = vec![
            TestStruct {
                title: "uuidではないstring",
                input: "not_uuid".to_string(),
                output: None,
                has_error: true,
            },
            TestStruct {
                title: "uuid形式のstring",
                input: "18e89209-9c2d-4081-bc2a-6d8508e714e4".to_string(),
                output: Some("18e89209-9c2d-4081-bc2a-6d8508e714e4".to_string()),
                has_error: false,
            },
        ];
        let func = |value: String| {
            let id = Id::try_from(value)?;
            Ok::<String, Error>(id.into())
        };
        test_runner(table, func)?;
        Ok(())
    }

    #[test]
    fn from_str() -> Result<()> {
        let table = vec![
            TestStruct {
                title: "uuidではないstr",
                input: "not_uuid",
                output: None,
                has_error: true,
            },
            TestStruct {
                title: "uuid形式のstr",
                input: "18e89209-9c2d-4081-bc2a-6d8508e714e4",
                output: Some("18e89209-9c2d-4081-bc2a-6d8508e714e4"),
                has_error: false,
            },
        ];
        let func = |value: &str| {
            let id = Id::try_from(value)?;
            Ok::<&'static str, Error>(Box::leak(String::from(id).into_boxed_str()))
        };
        test_runner(table, func)?;
        Ok(())
    }
}
