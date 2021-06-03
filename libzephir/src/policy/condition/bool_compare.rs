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
    value
        .as_bool()
        .map(|v| v == *other)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use crate::policy::condition::bool_compare::{eval_value_bool_equals, evaluate_bool_equals, make_bool_equals};
    use serde_json::Value;
    use crate::policy::condition::flags::Flags;

    #[test]
    fn should_build_boolean_condition() {
        let obj = serde_json::json!({
            "FieldOne": true
        });

        let mut condition = make_bool_equals(&obj, Flags::None).unwrap();
        assert_eq!(condition.len(), 1);

        let cond = condition.pop().unwrap();
        assert_eq!(cond.matching(&serde_json::json!({
            "FieldOne": true,
            "FieldTwo": false,
        })), true);
    }

    #[test]
    fn should_raise_err_if_malformed_object() {
        let obj = serde_json::json!("");
        make_bool_equals(&obj, Flags::None).expect_err("Should raise error");

        let obj = serde_json::json!({
            "FieldOne": [ "" ]
        });
        make_bool_equals(&obj, Flags::None).expect_err("Should raise error");
    }

    #[test]
    fn should_correctly_evaluate_boolean_comparison() {
        assert_eq!(eval_value_bool_equals(&Value::from(true), &true), true);
        assert_eq!(eval_value_bool_equals(&Value::from(true), &false), false);
        assert_eq!(eval_value_bool_equals(&Value::from(false), &false), true);
        assert_eq!(eval_value_bool_equals(&Value::from(false), &true), false);
    }

    #[test]
    fn should_return_false_if_value_is_not_a_boolean() {
        let map = serde_json::json!({
            "FieldOne": [ "AnArray" ],
        });

        assert_eq!(evaluate_bool_equals(map.as_object().unwrap(), "FieldOne", &true), false);
    }
}
