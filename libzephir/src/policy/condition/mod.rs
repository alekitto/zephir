mod flags;

mod binary_compare;
mod bool_compare;
mod date_compare;
mod ip_compare;
mod numeric_compare;
mod script;
mod string_equals;
mod string_not_equals;

pub(crate) use script::initialize_v8_platform;

use crate::err::{Error, ErrorKind};
use crate::policy::condition::binary_compare::{
    eval_value_binary_equals, evaluate_binary_equals, make_binary_equals,
};
use crate::policy::condition::bool_compare::{
    eval_value_bool_equals, evaluate_bool_equals, make_bool_equals,
};
use crate::policy::condition::date_compare::{
    eval_value_date_compare, evaluate_date_compare, make_date_equals, make_date_greater_than,
    make_date_greater_than_or_equal, make_date_less_than, make_date_less_than_or_equal,
    make_date_not_equals,
};
use crate::policy::condition::ip_compare::{
    eval_value_ip_address, eval_value_not_ip_address, evaluate_ip_address, evaluate_not_ip_address,
    make_ip_address, make_not_ip_address,
};
use crate::policy::condition::numeric_compare::{
    eval_value_numeric_compare, evaluate_numeric_compare, make_numeric_equals,
    make_numeric_greater_than, make_numeric_greater_than_or_equal, make_numeric_less_than,
    make_numeric_less_than_or_equal, make_numeric_not_equals,
};
use crate::policy::condition::script::{evaluate_script, make_script};
use crate::policy::condition::string_equals::{
    eval_value_str_equals, evaluate_string_equals, make_string_equals,
};
use crate::policy::condition::string_not_equals::{
    eval_value_str_not_equals, evaluate_string_not_equals, make_string_not_equals,
};
use crate::utils::string_utils::StringUtils;
use chrono::{DateTime, Utc};
use cidr::AnyIpCidr;
use flags::Flags;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CompareFn {
    Eq,
    NEq,
    Lt,
    Lte,
    Gt,
    Gte,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Condition {
    StringEquals(String, String, bool, Flags),
    StringNotEquals(String, String, bool, Flags),
    NumericCompare(String, i64, CompareFn, Flags),
    DateCompare(
        String,
        #[serde(with = "chrono::serde::ts_milliseconds")] DateTime<Utc>,
        CompareFn,
        Flags,
    ),
    BoolEquals(String, bool, Flags),
    BinaryEquals(String, Vec<u8>, Flags),
    IpAddress(String, AnyIpCidr, Flags),
    NotIpAddress(String, AnyIpCidr, Flags),
    Script(String),
}

lazy_static! {
    static ref EMPTY_MAP: Map<String, Value> = Map::new();
}

fn internal_matching<E>(
    params: &Map<String, Value>,
    key: &str,
    flags: &Flags,
    eval_value: E,
) -> Option<bool>
where
    E: FnMut(&Value) -> bool,
{
    let flags = *flags;
    if flags.intersects(Flags::IfExists) && params.get(key).is_none() {
        return Some(true);
    }

    if flags.intersects(Flags::ForAnyValue | Flags::ForAllValues) {
        Some(
            if let Some(value) = params.get(key).and_then(|v| v.as_array()) {
                if flags.intersects(Flags::ForAnyValue) {
                    value.iter().any(eval_value)
                } else {
                    value.iter().all(eval_value)
                }
            } else {
                false
            },
        )
    } else {
        None
    }
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

            if key == "Script" {
                result.push(make_script(value)?);
                continue;
            }

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
                "NumericEquals" => result.append(make_numeric_equals(value, flags)?.as_mut()),
                "NumericNotEquals" => {
                    result.append(make_numeric_not_equals(value, flags)?.as_mut())
                }
                "NumericLessThan" => result.append(make_numeric_less_than(value, flags)?.as_mut()),
                "NumericLessThanEquals" => {
                    result.append(make_numeric_less_than_or_equal(value, flags)?.as_mut())
                }
                "NumericGreaterThan" => {
                    result.append(make_numeric_greater_than(value, flags)?.as_mut())
                }
                "NumericGreaterThanEquals" => {
                    result.append(make_numeric_greater_than_or_equal(value, flags)?.as_mut())
                }
                "DateEquals" => result.append(make_date_equals(value, flags)?.as_mut()),
                "DateNotEquals" => result.append(make_date_not_equals(value, flags)?.as_mut()),
                "DateLessThan" => result.append(make_date_less_than(value, flags)?.as_mut()),
                "DateLessThanEquals" => {
                    result.append(make_date_less_than_or_equal(value, flags)?.as_mut())
                }
                "DateGreaterThan" => result.append(make_date_greater_than(value, flags)?.as_mut()),
                "DateGreaterThanEquals" => {
                    result.append(make_date_greater_than_or_equal(value, flags)?.as_mut())
                }
                "Bool" => result.append(make_bool_equals(value, flags)?.as_mut()),
                "Binary" => result.append(make_binary_equals(value, flags)?.as_mut()),
                "IpAddress" => result.append(make_ip_address(value, flags)?.as_mut()),
                "NotIpAddress" => result.append(make_not_ip_address(value, flags)?.as_mut()),

                _ => return Err(Error::from("Unknown condition key")),
            }
        }

        Ok(result)
    }

    pub fn matching(&self, params: &Value) -> bool {
        let extra = params.as_object().unwrap_or(&EMPTY_MAP);
        match self {
            Self::StringEquals(key, other, case_sensitive, flags) => {
                internal_matching(extra, key, flags, |v| {
                    eval_value_str_equals(v, other, case_sensitive)
                })
                .unwrap_or_else(|| evaluate_string_equals(extra, key, other, case_sensitive))
            }
            Self::StringNotEquals(key, other, case_sensitive, flags) => {
                internal_matching(extra, key, flags, |v| {
                    eval_value_str_not_equals(v, other, case_sensitive)
                })
                .unwrap_or_else(|| evaluate_string_not_equals(extra, key, other, case_sensitive))
            }
            Self::NumericCompare(key, other, operator, flags) => {
                internal_matching(extra, key, flags, |v| {
                    eval_value_numeric_compare(v, other, operator)
                })
                .unwrap_or_else(|| evaluate_numeric_compare(extra, key, other, operator))
            }
            Self::DateCompare(key, other, operator, flags) => {
                internal_matching(extra, key, flags, |v| {
                    eval_value_date_compare(v, other, operator)
                })
                .unwrap_or_else(|| evaluate_date_compare(extra, key, other, operator))
            }
            Self::BoolEquals(key, other, flags) => {
                internal_matching(extra, key, flags, |v| eval_value_bool_equals(v, other))
                    .unwrap_or_else(|| evaluate_bool_equals(extra, key, other))
            }
            Self::BinaryEquals(key, other, flags) => {
                internal_matching(extra, key, flags, |v| eval_value_binary_equals(v, other))
                    .unwrap_or_else(|| evaluate_binary_equals(extra, key, other))
            }
            Self::IpAddress(key, other, flags) => {
                internal_matching(extra, key, flags, |v| eval_value_ip_address(v, other))
                    .unwrap_or_else(|| evaluate_ip_address(extra, key, other))
            }
            Self::NotIpAddress(key, other, flags) => {
                internal_matching(extra, key, flags, |v| eval_value_not_ip_address(v, other))
                    .unwrap_or_else(|| evaluate_not_ip_address(extra, key, other))
            }
            Self::Script(script) => evaluate_script(script.as_str(), params),
        }
    }
}
