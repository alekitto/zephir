use crate::identity::role::{allowed, Role};
use crate::identity::subject::{Subject, SubjectIterator};
use crate::policy::allowed_result::AllowedResult;
use crate::policy::policy::{CompletePolicy, ToJson};
use crate::policy::policy_set::{PolicySet, PolicySetHelper, PolicySetTrait};
use serde_json::{Map, Value};
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Identity {
    pub(crate) id: String,
    pub(crate) inline_policy: Option<CompletePolicy>,
    pub(crate) linked_policies: PolicySet<CompletePolicy>,
}

impl Identity {
    pub fn new<T: ToString>(id: T, policy: Option<CompletePolicy>) -> Self {
        Identity {
            id: id.to_string(),
            inline_policy: policy,
            linked_policies: PolicySet::new(),
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn clear_inline_policy(mut self) -> Self {
        self.inline_policy = Option::None;
        self
    }

    pub fn set_inline_policy(mut self, policy: CompletePolicy) -> Self {
        let mut policy = policy;
        policy.id = "__embedded_policy_identity_".to_owned() + self.id.as_str() + "__";

        self.inline_policy = Option::Some(policy);
        self
    }
}

pub trait ToIdentityId {
    fn to_identity_id(&self) -> &String;
}

impl ToIdentityId for Identity {
    fn to_identity_id(&self) -> &String {
        &self.id
    }
}

impl ToIdentityId for String {
    fn to_identity_id(&self) -> &String {
        self
    }
}

impl ToIdentityId for &String {
    fn to_identity_id(&self) -> &String {
        self
    }
}

impl Subject for Identity {
    fn get_inline_policy(&self) -> Option<&CompletePolicy> {
        self.inline_policy.as_ref()
    }
}

impl ToJson for Identity {
    fn to_json(&self) -> Map<String, Value> {
        let linked_policies = &self.linked_policies;
        let mut map = Map::new();
        map.insert(String::from("id"), Value::from(self.id.as_str()));
        map.insert(
            String::from("inline_policy"),
            if self.inline_policy.is_none() {
                Value::Null
            } else {
                Value::from(self.inline_policy.as_ref().unwrap().to_json())
            },
        );
        map.insert(
            String::from("linked_policies"),
            Value::from(
                linked_policies
                    .into_iter()
                    .map(|ref p| p.id.as_str())
                    .collect::<Vec<&str>>(),
            ),
        );

        map
    }
}

impl Into<Value> for Identity {
    fn into(self) -> Value {
        Value::Object(self.to_json())
    }
}

impl PolicySetTrait<CompletePolicy> for Identity {
    fn add_policy(mut self, policy: CompletePolicy) -> Self {
        self.linked_policies = PolicySetHelper::link_policy(self.linked_policies, policy);
        self
    }

    fn remove_policy<S: ToString>(mut self, id: S) -> Self {
        self.linked_policies = PolicySetHelper::unlink_policy(self.linked_policies, id);
        self
    }
}

impl Role for Identity {
    fn linked_policies(&self) -> &PolicySet<CompletePolicy> {
        &self.linked_policies
    }

    fn allowed<T, S>(&self, action: Option<T>, resource: Option<S>) -> AllowedResult
    where
        T: ToString + Display,
        S: ToString + Display + Debug {
        allowed(SubjectIterator::new(self), action, resource)
    }
}

#[cfg(test)]
mod tests {
    use crate::identity::identity::Identity;
    use crate::identity::role::Role;
    use crate::policy::allowed_result::AllowedOutcome;
    use crate::policy::policy_set::PolicySetTrait;
    use crate::policy::{PolicyEffect, PolicyVersion};
    use crate::zephir_policy;

    #[test]
    fn can_be_created() {
        let i = Identity::new("Identity", Option::None);
        assert_eq!(i.linked_policies().len(), 0);

        let i = Identity::new(
            "IdentityTest2",
            Option::Some(
                zephir_policy!(
                    "TestPolicyGroup",
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["Action"]
                )
                .unwrap(),
            ),
        );
        assert_eq!(i.linked_policies().len(), 0);
    }

    #[test]
    fn allow_should_check_inline_policy() {
        let i = Identity::new(
            "IdentityTestAllowShouldCheckInlinePolicy",
            Option::Some(
                zephir_policy!(
                    "TestInlinePolicyOnIdentity",
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["*"],
                    vec!["urn:test-resource:id"]
                )
                .unwrap(),
            ),
        );

        let result = i.allowed(
            Option::Some("test:identity"),
            Option::Some("urn:test-resource:id"),
        );
        assert_eq!(result.outcome(), AllowedOutcome::Allowed);
        assert_eq!(result.get_partials().len(), 0);

        let result = i.allowed::<&str, String>(Option::Some("test:identity"), Option::None);
        assert_eq!(result.outcome(), AllowedOutcome::Abstain);
        assert_eq!(result.get_partials().len(), 1);
    }

    #[test]
    fn should_check_inline_and_linked_policies() {
        let i = Identity::new(
            "IdentityTestShouldCheckInlineAndLinkedPolicies",
            Option::Some(
                zephir_policy!(
                    "TestInlinePolicyOnIdentity",
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["test:not-identity"],
                    vec!["urn:test-resource:id"]
                )
                .unwrap(),
            ),
        );

        let i = i.add_policy(
            zephir_policy!(
                "TestLinkedPolicyOnIdentity",
                PolicyVersion::Version1,
                PolicyEffect::Allow,
                vec!["test:identity"],
                vec!["*"]
            )
            .unwrap(),
        );

        let result = i.allowed(
            Option::Some("test:identity"),
            Option::Some("urn:test:zephir:identity"),
        );
        assert_eq!(result.outcome(), AllowedOutcome::Allowed);
    }
}
