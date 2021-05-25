use crate::err::{Error, ErrorKind};
use crate::policy::condition::flags::Flags;
use crate::policy::condition::CompareFn;
use crate::policy::condition::Condition;
use serde_json::{Map, Value};
use std::cmp::Ordering;

macro_rules! impl_make_numeric {
    ($suffix: ident, $key: literal, $fn: ident) => {
        #[inline]
        pub(super) fn $suffix(value: &Value, flags: Flags) -> Result<Vec<Condition>, Error> {
            let mut result = vec![];
            for (field, comp) in value.as_object().ok_or_else(|| {
                Error::new(
                    ErrorKind::UnwrapNoneValueError,
                    std::format!("Conditions.{} is not an object", $key),
                )
            })? {
                let comp = comp.as_i64().ok_or_else(|| {
                    Error::new(
                        ErrorKind::UnwrapNoneValueError,
                        std::format!("Conditions.{} value is not a string", $key),
                    )
                })?;

                result.push(Condition::NumericCompare(
                    field.clone(),
                    comp,
                    CompareFn::$fn,
                    flags,
                ));
            }

            Ok(result)
        }
    };
}

impl_make_numeric!(make_numeric_equals, "NumericEquals", Eq);
impl_make_numeric!(make_numeric_not_equals, "NumericNotEquals", NEq);
impl_make_numeric!(make_numeric_less_than, "NumericLessThan", Lt);
impl_make_numeric!(
    make_numeric_less_than_or_equal,
    "NumericLessThanEquals",
    Lte
);
impl_make_numeric!(make_numeric_greater_than, "NumericGreaterThan", Gt);
impl_make_numeric!(
    make_numeric_greater_than_or_equal,
    "NumericGreaterThanEquals",
    Gte
);

#[inline]
pub(super) fn evaluate_numeric_compare(
    value: &Map<String, Value>,
    key: &str,
    other: &i64,
    operator: &CompareFn,
) -> bool {
    value
        .get(key)
        .map(|v| eval_value_numeric_compare(v, other, operator))
        .or(Some(false))
        .unwrap()
}

#[inline]
pub(super) fn eval_value_numeric_compare(value: &Value, other: &i64, op: &CompareFn) -> bool {
    value
        .as_i64()
        .map(|v| {
            let cmp = v.cmp(other);
            match *op {
                CompareFn::Eq => Ordering::is_eq(cmp),
                CompareFn::NEq => Ordering::is_ne(cmp),
                CompareFn::Lt => Ordering::is_lt(cmp),
                CompareFn::Lte => Ordering::is_le(cmp),
                CompareFn::Gt => Ordering::is_gt(cmp),
                CompareFn::Gte => Ordering::is_ge(cmp),
            }
        })
        .unwrap()
}
