use crate::err::{Error, ErrorKind};
use crate::policy::condition::flags::Flags;
use crate::policy::condition::Condition;
use base64::{CharacterSet, Config};
use serde_json::{Map, Value};
use std::cmp::Ordering;

lazy_static! {
    static ref BASE64_CONFIG: Config = Config::new(CharacterSet::Standard, false);
}

#[inline]
fn base64_to_vec_u8(s: &str) -> Result<Vec<u8>, Error> {
    let len = (s.len() + 3) / 4 * 3;
    let mut vector: Vec<u8> = vec![0; len];
    let bytes = vector.as_mut_slice();

    base64::decode_config_slice(s, *BASE64_CONFIG, bytes)
        .map(|size| {
            vector.truncate(size);
            vector
        })
        .map_err(|e| Error::new(ErrorKind::UnknownError, e.to_string()))
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
    key: &str,
    other: &[u8],
) -> bool {
    value
        .get(key)
        .map(|v| eval_value_binary_equals(v, other))
        .or(Some(false))
        .unwrap()
}

#[inline]
pub(super) fn eval_value_binary_equals(value: &Value, other: &[u8]) -> bool {
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

#[cfg(test)]
mod tests {
    use crate::policy::condition::binary_compare::{eval_value_binary_equals, evaluate_binary_equals};
    use serde_json::Value;

    #[test]
    fn should_correctly_evaluate_binary_comparison() {
        let bytes_one: [u8; 12] = [ 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33 ];
        assert_eq!(eval_value_binary_equals(&Value::from("SGVsbG8gd29ybGQh"), &bytes_one), true);

        let bytes_two: [u8; 11] = [ 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100 ];
        assert_eq!(eval_value_binary_equals(&Value::from("SGVsbG8gd29ybGQh"), &bytes_two), false);

        let map = serde_json::json!({
            "FieldOne": "SGVsbG8gd29ybGQh",
            "FieldTwo": "SGVsbG8gd29ybGQ=",
        });

        assert_eq!(evaluate_binary_equals(map.as_object().unwrap(), "FieldOne", &bytes_one), true);
        assert_eq!(evaluate_binary_equals(map.as_object().unwrap(), "FieldOne", &bytes_two), false);

        assert_eq!(evaluate_binary_equals(map.as_object().unwrap(), "FieldTwo", &bytes_one), false);
        assert_eq!(evaluate_binary_equals(map.as_object().unwrap(), "FieldTwo", &bytes_two), true);
    }

    #[test]
    fn should_return_false_if_value_is_not_a_string() {
        let bytes: [u8; 12] = [ 72, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100, 33 ];
        let map = serde_json::json!({
            "FieldOne": [ "AnArray" ],
        });

        assert_eq!(evaluate_binary_equals(map.as_object().unwrap(), "FieldOne", &bytes), false);
    }
}
