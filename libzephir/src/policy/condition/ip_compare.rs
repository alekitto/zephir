use crate::err::{Error, ErrorKind};
use crate::policy::condition::flags::Flags;
use crate::policy::condition::Condition;
use cidr::AnyIpCidr;
use serde_json::{Map, Value};
use std::net::IpAddr;

#[inline]
fn str_to_any_cidr(s: &str) -> Result<AnyIpCidr, Error> {
    s.parse::<AnyIpCidr>()
        .map_err(|e| Error::new(ErrorKind::UnknownError, e.to_string()))
}

#[inline]
fn str_to_any_ip(s: &str) -> Result<IpAddr, Error> {
    s.parse::<IpAddr>()
        .map_err(|e| Error::new(ErrorKind::UnknownError, e.to_string()))
}

#[inline]
pub(super) fn make_ip_address(value: &Value, flags: Flags) -> Result<Vec<Condition>, Error> {
    let mut result = vec![];
    for (field, comp) in value.as_object().ok_or_else(|| {
        Error::new(
            ErrorKind::UnwrapNoneValueError,
            "Conditions.IpAddress is not an object",
        )
    })? {
        let comp = comp
            .as_str()
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::UnwrapNoneValueError,
                    "Conditions.IpAddress value is not a string",
                )
            })
            .and_then(str_to_any_cidr)?;

        result.push(Condition::IpAddress(field.clone(), comp, flags));
    }

    Ok(result)
}

#[inline]
pub(super) fn make_not_ip_address(value: &Value, flags: Flags) -> Result<Vec<Condition>, Error> {
    let mut result = vec![];
    for (field, comp) in value.as_object().ok_or_else(|| {
        Error::new(
            ErrorKind::UnwrapNoneValueError,
            "Conditions.NotIpAddress is not an object",
        )
    })? {
        let comp = comp
            .as_str()
            .ok_or_else(|| {
                Error::new(
                    ErrorKind::UnwrapNoneValueError,
                    "Conditions.NotIpAddress value is not a string",
                )
            })
            .and_then(str_to_any_cidr)?;

        result.push(Condition::NotIpAddress(field.clone(), comp, flags));
    }

    Ok(result)
}

#[inline]
pub(super) fn evaluate_ip_address(
    value: &Map<String, Value>,
    key: &String,
    other: &AnyIpCidr,
) -> bool {
    value
        .get(key)
        .map(|v| eval_value_ip_address(v, other))
        .or(Some(false))
        .unwrap()
}
#[inline]
pub(super) fn evaluate_not_ip_address(
    value: &Map<String, Value>,
    key: &String,
    other: &AnyIpCidr,
) -> bool {
    value
        .get(key)
        .map(|v| eval_value_ip_address(v, other))
        .or(Some(false))
        .unwrap()
}

#[inline]
pub(super) fn eval_value_ip_address(value: &Value, other: &AnyIpCidr) -> bool {
    value
        .as_str()
        .ok_or_else(|| Error::new(ErrorKind::UnwrapNoneValueError, "Value is not a string"))
        .and_then(str_to_any_ip)
        .map(|v| other.contains(&v))
        .unwrap_or(false)
}

#[inline]
pub(super) fn eval_value_not_ip_address(value: &Value, other: &AnyIpCidr) -> bool {
    value
        .as_str()
        .ok_or_else(|| Error::new(ErrorKind::UnwrapNoneValueError, "Value is not a string"))
        .and_then(str_to_any_ip)
        .map(|v| !other.contains(&v))
        .unwrap_or(false)
}
