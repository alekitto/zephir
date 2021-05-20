use crate::err::{Error, ErrorKind};
use crate::policy::condition::flags::Flags;
use crate::policy::condition::Condition;
use serde_json::{Map, Value};
use std::cmp::Ordering;

#[inline]
pub(super) fn make_string_equals(
    value: &Value,
    case_sensitive: bool,
    flags: Flags,
) -> Result<Vec<Condition>, Error> {
    let mut result = vec![];
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

        let comp = if case_sensitive {
            comp.to_string()
        } else {
            comp.to_string().to_lowercase()
        };
        result.push(Condition::StringEquals(
            field.clone(),
            comp,
            case_sensitive,
            flags,
        ));
    }

    Ok(result)
}

#[inline]
pub(super) fn evaluate_string_equals(
    value: &Map<String, Value>,
    key: &String,
    other: &str,
    case_sensitive: &bool,
) -> bool {
    value
        .get(key)
        .map(|v| eval_value_str_equals(v, other, case_sensitive))
        .unwrap()
}

#[inline]
pub(super) fn eval_value_str_equals(value: &Value, other: &str, case_sensitive: &bool) -> bool {
    value
        .as_str()
        .map(|v| {
            let cmp = if *case_sensitive {
                v.cmp(other)
            } else {
                v.to_lowercase().as_str().cmp(other)
            };

            cmp == Ordering::Equal
        })
        .or(Some(false))
        .unwrap()
}
