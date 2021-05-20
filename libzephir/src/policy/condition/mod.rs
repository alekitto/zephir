mod flags;
mod string_equals;
mod string_not_equals;

use crate::err::{Error, ErrorKind};
use crate::policy::condition::string_equals::{
    eval_value_str_equals, evaluate_string_equals, make_string_equals,
};
use crate::policy::condition::string_not_equals::{
    eval_value_str_not_equals, evaluate_string_not_equals, make_string_not_equals,
};
use crate::utils::string_utils::StringUtils;
use flags::Flags;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Condition {
    StringEquals(String, String, bool, Flags),
    StringNotEquals(String, String, bool, Flags),
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
            let mut key = key.as_str();
            let mut flags = Flags::None;

            if key.starts_with("ForAnyValue") {
                flags.set(Flags::ForAnyValue, true);
                key = key.slice(11..)
            } else if key.starts_with("ForAllValues") {
                flags.set(Flags::ForAllValues, true);
                key = key.slice(12..)
            }

            if key.ends_with("IfExists") {
                flags.set(Flags::IfExists, true);
                key = key.slice(0..(key.len() - 8));
            }

            match key {
                "StringEquals" => result.append(make_string_equals(value, false, flags)?.as_mut()),
                "StringNotEquals" => {
                    result.append(make_string_not_equals(value, false, flags)?.as_mut())
                }
                "StringEqualsIgnoreCase" => {
                    result.append(make_string_equals(value, true, flags)?.as_mut())
                }
                "StringNotEqualsIgnoreCase" => {
                    result.append(make_string_not_equals(value, true, flags)?.as_mut())
                }

                _ => return Err(Error::from("Unknown condition key")),
            }
        }

        Ok(result)
    }

    pub fn matching(&self, params: &Value) -> bool {
        let extra = params.as_object().unwrap_or(&EMPTY_MAP);
        match self {
            Self::StringEquals(key, other, case_sensitive, flags) => {
                let flags = *flags;
                if flags.intersects(Flags::IfExists) && extra.get(key).is_none() {
                    return true;
                }

                if flags.intersects(Flags::ForAnyValue | Flags::ForAllValues) {
                    if let Some(value) = extra.get(key).and_then(|v| v.as_array()) {
                        if flags.intersects(Flags::ForAnyValue) {
                            value
                                .iter()
                                .any(|v| eval_value_str_equals(v, other, case_sensitive))
                        } else {
                            value
                                .iter()
                                .all(|v| eval_value_str_equals(v, other, case_sensitive))
                        }
                    } else {
                        false
                    }
                } else {
                    evaluate_string_equals(extra, key, other, case_sensitive)
                }
            }
            Self::StringNotEquals(key, other, case_sensitive, flags) => {
                let flags = *flags;
                if flags.intersects(Flags::IfExists) && extra.get(key).is_none() {
                    return true;
                }

                if flags.intersects(Flags::ForAnyValue | Flags::ForAllValues) {
                    if let Some(value) = extra.get(key).and_then(|v| v.as_array()) {
                        if flags.intersects(Flags::ForAnyValue) {
                            value
                                .iter()
                                .any(|v| eval_value_str_not_equals(v, other, case_sensitive))
                        } else {
                            value
                                .iter()
                                .all(|v| eval_value_str_not_equals(v, other, case_sensitive))
                        }
                    } else {
                        false
                    }
                } else {
                    evaluate_string_not_equals(extra, key, other, case_sensitive)
                }
            }
        }
    }
}
