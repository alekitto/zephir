use crate::err::{Error, ErrorKind};
use crate::policy::condition::flags::Flags;
use crate::policy::condition::CompareFn;
use crate::policy::condition::Condition;
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use std::cmp::Ordering;

macro_rules! impl_make_date {
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
                let comp = comp
                    .as_str()
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|d| d.with_timezone(&chrono::Utc))
                    .ok_or_else(|| {
                        Error::new(
                            ErrorKind::UnwrapNoneValueError,
                            std::format!("Conditions.{} value is not a string", $key),
                        )
                    })?;

                result.push(Condition::DateCompare(
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

impl_make_date!(make_date_equals, "DateEquals", Eq);
impl_make_date!(make_date_not_equals, "DateNotEquals", NEq);
impl_make_date!(make_date_less_than, "DateLessThan", Lt);
impl_make_date!(make_date_less_than_or_equal, "DateLessThanEquals", Lte);
impl_make_date!(make_date_greater_than, "DateGreaterThan", Gt);
impl_make_date!(
    make_date_greater_than_or_equal,
    "DateGreaterThanEquals",
    Gte
);

#[inline]
pub(super) fn evaluate_date_compare(
    value: &Map<String, Value>,
    key: &String,
    other: &DateTime<Utc>,
    operator: &CompareFn,
) -> bool {
    value
        .get(key)
        .map(|v| eval_value_date_compare(v, other, operator))
        .or(Some(false))
        .unwrap()
}

#[inline]
pub(super) fn eval_value_date_compare(
    value: &Value,
    other: &DateTime<Utc>,
    op: &CompareFn,
) -> bool {
    value
        .as_str()
        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
        .map(|d| d.with_timezone(&Utc))
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
