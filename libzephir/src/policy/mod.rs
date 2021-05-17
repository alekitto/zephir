use crate::err::{Error, ErrorKind};
use crate::policy::policy::CompletePolicy;
use serde_json::{Number, Value};
use std::convert::TryFrom;

pub mod allowed_result;
pub(crate) mod condition;
pub mod match_result;
pub mod policy;
pub mod policy_set;

/// Get a new policy object
pub fn policy_new<A, R>(
    id: String,
    version: PolicyVersion,
    effect: PolicyEffect,
    actions: Vec<A>,
    resources: Vec<R>,
    conditions: Value,
) -> Result<CompletePolicy, Error>
where
    A: ToString,
    R: ToString,
{
    CompletePolicy::new(id, version, effect, actions, resources, conditions)
}

#[derive(Clone, Debug, PartialEq)]
pub enum PolicyVersion {
    Version1 = 1,
}

impl TryFrom<i32> for PolicyVersion {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(PolicyVersion::Version1),
            _ => Err(Error::unknown_policy_version(value)),
        }
    }
}

impl From<&PolicyVersion> for i32 {
    fn from(value: &PolicyVersion) -> Self {
        match value {
            PolicyVersion::Version1 => 1,
        }
    }
}

impl From<&PolicyVersion> for Value {
    fn from(value: &PolicyVersion) -> Self {
        let value: i32 = value.into();
        Value::Number(Number::from(value))
    }
}

impl PartialEq<i32> for PolicyVersion {
    fn eq(&self, other: &i32) -> bool {
        let value: i32 = self.into();
        value == *other
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PolicyEffect {
    Deny = 0,
    Allow = 1,
}

impl From<&PolicyEffect> for Value {
    fn from(value: &PolicyEffect) -> Self {
        Value::from(match value {
            PolicyEffect::Deny => "DENY",
            PolicyEffect::Allow => "ALLOW",
        })
    }
}

impl From<&PolicyEffect> for bool {
    fn from(value: &PolicyEffect) -> Self {
        match value {
            PolicyEffect::Allow => true,
            PolicyEffect::Deny => false,
        }
    }
}

impl TryFrom<&String> for PolicyEffect {
    type Error = Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.eq_ignore_ascii_case("ALLOW") {
            Ok(Self::Allow)
        } else if value.eq_ignore_ascii_case("DENY") {
            Ok(Self::Deny)
        } else {
            Err(Error::new(ErrorKind::UnknownError, "Invalid"))
        }
    }
}

impl PartialEq<i32> for PolicyEffect {
    fn eq(&self, other: &i32) -> bool {
        let value: bool = self.into();
        value == (*other != 0)
    }
}

#[cfg(test)]
mod tests {
    use crate::policy::{PolicyEffect, PolicyVersion};
    use serde_json::Value;

    #[test]
    fn policy_version_could_be_converted_to_json() {
        assert_eq!(Value::from(&PolicyVersion::Version1), Value::from(1));
    }

    #[test]
    fn number_can_be_converted_to_policy_version() {
        use std::convert::TryFrom;
        assert_eq!(PolicyVersion::try_from(1).unwrap(), PolicyVersion::Version1);
    }

    #[test]
    fn number_should_raise_error_if_version_number_is_unknown() {
        use std::convert::TryFrom;
        assert_eq!(
            PolicyVersion::try_from(-1).unwrap_err().to_string(),
            "Unknown policy version"
        );
    }

    #[test]
    fn policy_version_should_be_equatable() {
        assert_eq!(PolicyVersion::Version1 == 1, true);
    }

    #[test]
    fn policy_effect_should_be_equatable() {
        assert_eq!(PolicyEffect::Deny == 0, true);
        assert_eq!(PolicyEffect::Allow == 1, true);

        assert_eq!(PolicyEffect::Deny == 1, false);
        assert_eq!(PolicyEffect::Allow == 0, false);

        assert_eq!(PolicyEffect::Allow != 0, true);
        assert_eq!(PolicyEffect::Deny != 1, true);
    }
}
