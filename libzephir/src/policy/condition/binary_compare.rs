use crate::err::{Error, ErrorKind};
use crate::policy::condition::flags::Flags;
use crate::policy::condition::Condition;
use base64::{CharacterSet, Config};
use serde_json::{Map, Value};
use std::cmp::Ordering;

lazy_static! {
    static ref BASE64_CONFIG: Config = Config::new(CharacterSet::Standard, true);
}

#[inline]
fn base64_to_vec_u8(s: &str) -> Result<Vec<u8>, Error> {
    let len = (s.len() + 3) / 4 * 3;
    let mut vector: Vec<u8> = vec![0; len];
    let bytes = vector.as_mut_slice();

    base64::decode_config_slice(s, BASE64_CONFIG.clone(), bytes)
        .and_then(|size| {
            vector.shrink_to(size);
            Ok(vector)
        })
        .or_else(|e| Err(Error::new(ErrorKind::UnknownError, e.to_string())))
}

#[inline]
pub(super) fn make_binary_equals(value: &Value, flags: Flags) -> Result<Vec<Condition>, Error> {
    let mut result = vec![];
    for (field, comp) in value.as_object().ok_or_else(|| {
        Error::new(
            ErrorKind::UnwrapNoneValueError,
            "Conditions.Binary is not an object",
        )
    })? {
        let comp = comp
            .as_str()
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::UnwrapNoneValueError,
                    "Conditions.Binary value is not a string",
                )
            })
            .and_then(base64_to_vec_u8)?;

        result.push(Condition::BinaryEquals(field.clone(), comp, flags));
    }

    Ok(result)
}

#[inline]
pub(super) fn evaluate_binary_equals(
    value: &Map<String, Value>,
    key: &String,
    other: &Vec<u8>,
) -> bool {
    value
        .get(key)
        .map(|v| eval_value_binary_equals(v, other))
        .or(Some(false))
        .unwrap()
}

#[inline]
pub(super) fn eval_value_binary_equals(value: &Value, other: &Vec<u8>) -> bool {
    value
        .as_str()
        .ok_or_else(|| Error::new(ErrorKind::UnwrapNoneValueError, "Value is not a string"))
        .and_then(base64_to_vec_u8)
        .map(|v| {
            let other_len = other.len();
            if v.len() != other_len {
                return false;
            }

            for (ai, bi) in v.iter().zip(other.iter()) {
                match ai.cmp(bi) {
                    Ordering::Equal => continue,
                    _ => return false,
                }
            }

            true
        })
        .unwrap_or(false)
}
