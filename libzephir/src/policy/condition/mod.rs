use crate::err::{Error, ErrorKind};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::cmp::Ordering;
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Condition {
    StringEquals(String, String),
}

lazy_static! {
    static ref EMPTY_MAP: Map<String, Value> = Map::new();
}

impl Condition {
    pub fn from_value(conditions: &Value) -> Result<Vec<Self>, Error> {
        let mut result = vec![];
        if conditions.is_null() {
            return Ok(result);
        }

        let map = conditions.as_object().ok_or_else(|| {
            Error::new(
                ErrorKind::UnwrapNoneValueError,
                "Conditions is not an object",
            )
        })?;

        for (key, value) in map {
            match key.as_str() {
                "StringEquals" => {
                    for (field, comp) in value.as_object().ok_or_else(|| {
                        Error::new(
                            ErrorKind::UnwrapNoneValueError,
                            "Conditions.StringEquals is not an object",
                        )
                    })? {
                        let comp = comp.as_str().ok_or_else(|| {
                            Error::new(
                                ErrorKind::UnwrapNoneValueError,
                                "Conditions.StringEquals value is not a string",
                            )
                        })?;
                        result.push(Condition::StringEquals(field.clone(), comp.to_string()));
                    }
                }

                _ => return Err(Error::from("Unknown condition key")),
            }
        }

        Ok(result)
    }

    pub fn matching(&self, params: &Value) -> bool {
        let extra = params.as_object().unwrap_or(&EMPTY_MAP);
        match self {
            Self::StringEquals(key, other) => extra
                .get(key)
                .and_then(|v| v.as_str())
                .map(|v| v.cmp(other) == Ordering::Equal)
                .or(Some(false))
                .unwrap(),
        }
    }
}
