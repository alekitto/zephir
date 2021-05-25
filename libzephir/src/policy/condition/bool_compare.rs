use crate::err::{Error, ErrorKind};
use crate::policy::condition::flags::Flags;
use crate::policy::condition::Condition;
use serde_json::{Map, Value};

#[inline]
pub(super) fn make_bool_equals(value: &Value, flags: Flags) -> Result<Vec<Condition>, Error> {
    let mut result = vec![];
    for (field, comp) in value.as_object().ok_or_else(|| {
        Error::new(
            ErrorKind::UnwrapNoneValueError,
            "Conditions.Bool is not an object",
        )
    })? {
        let comp = comp.as_bool().ok_or_else(|| {
            Error::new(
                ErrorKind::UnwrapNoneValueError,
                "Conditions.Bool value is not a string",
            )
        })?;

        result.push(Condition::BoolEquals(field.clone(), comp, flags));
    }

    Ok(result)
}

#[inline]
pub(super) fn evaluate_bool_equals(value: &Map<String, Value>, key: &str, other: &bool) -> bool {
    value
        .get(key)
        .map(|v| eval_value_bool_equals(v, other))
        .or(Some(false))
        .unwrap()
}

#[inline]
pub(super) fn eval_value_bool_equals(value: &Value, other: &bool) -> bool {
    value.as_bool().map(|v| v == *other).unwrap()
}
