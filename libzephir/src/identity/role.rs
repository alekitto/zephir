use crate::policy::allowed_result::{AllowedOutcome, AllowedResult};
use crate::policy::policy::{CompletePolicy, MatchablePolicy};
use crate::policy::policy_set::PolicySet;
use crate::policy::PolicyEffect;
use serde_json::Value;
use std::fmt::{Debug, Display};

pub(super) fn allowed<'a, T, S, I>(
    policies: I,
    action: Option<T>,
    resource: Option<S>,
) -> AllowedResult
where
    T: ToString + Display,
    S: ToString + Display + Debug,
    I: Iterator<Item = &'a CompletePolicy>,
{
    let mut outcome: AllowedOutcome = AllowedOutcome::Abstain;
    let mut partials = vec![];

    for p in policies {
        let result = p.matching(action.as_ref(), resource.as_ref());
        if !result.is_match() {
            continue;
        }

        if result.is_full() {
            if p.effect == PolicyEffect::Deny {
                return AllowedResult::new(AllowedOutcome::Denied, vec![]);
            }

            outcome = AllowedOutcome::Allowed;
            continue;
        }

        partials.push(result.get_partial());
    }

    AllowedResult::new(outcome, partials)
}

pub trait Role: Into<Value> {
    fn linked_policies(&self) -> &PolicySet<CompletePolicy>;

    fn allowed<T, S>(&self, action: Option<T>, resource: Option<S>) -> AllowedResult
    where
        T: ToString + Display,
        S: ToString + Display + Debug,
    {
        let mut policies = vec![];
        let linked_policies = self.linked_policies();
        for policy in linked_policies {
            policies.push(policy);
        }

        allowed(policies.into_iter(), action, resource)
    }

    fn into(self) -> Value {
        Value::Null
    }
}

#[cfg(test)]
mod tests {
    use crate::identity::role::{allowed, Role};
    use crate::policy::allowed_result::AllowedOutcome;
    use crate::policy::policy::{CompletePolicy, PartialPolicy, ToJson};
    use crate::policy::policy_set::{PolicySet, PolicySetTrait};
    use crate::policy::{PolicyEffect, PolicyVersion};
    use crate::zephir_policy;
    use serde_json::{Map, Value};

    struct ConcreteRole {
        policy_set: PolicySet<CompletePolicy>,
    }

    impl Role for ConcreteRole {
        fn linked_policies(&self) -> &PolicySet<CompletePolicy> {
            &self.policy_set
        }
    }

    impl Into<Value> for ConcreteRole {
        fn into(self) -> Value {
            todo!()
        }
    }

    #[test]
    fn allowed_should_return_denied_on_no_policy() {
        let res = allowed::<String, String, _>(vec![].into_iter(), Option::None, Option::None);
        assert_eq!(res.outcome(), AllowedOutcome::Denied);
    }

    #[test]
    fn allowed_should_check_matching_on_all_passed_policies() {
        let res = allowed::<&str, String, _>(
            vec![
                &zephir_policy!(
                    "p1",
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["get_first"]
                )
                .unwrap(),
                &zephir_policy!(
                    "p2",
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["get_second"]
                )
                .unwrap(),
            ].into_iter(),
            Option::Some("get_first"),
            Option::None,
        );

        assert_eq!(res.outcome(), AllowedOutcome::Allowed);
    }

    #[test]
    fn allowed_should_check_matching_with_resources() {
        let res = allowed::<&str, String, _>(
            vec![
                &zephir_policy!(
                    "p12",
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["get_first"],
                    vec!["resource_one"]
                )
                .unwrap(),
                &zephir_policy!(
                    "p22",
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["get_second"],
                    vec!["resource_one"]
                )
                .unwrap(),
            ].into_iter(),
            Option::Some("get_first"),
            Option::None,
        );

        assert_eq!(res.outcome(), AllowedOutcome::Abstain);

        let mut partial = PartialPolicy::default();
        partial.effect = PolicyEffect::Allow;
        partial.resources = Option::Some(vec![String::from("resource_one")]);

        let mut json = Map::new();
        json.insert(String::from("outcome"), Value::from("ABSTAIN"));
        json.insert(String::from("partials"), Value::from(vec![partial]));
        assert_eq!(res.to_json(), json);
    }

    #[test]
    fn should_return_full_deny() {
        let res = allowed(
            vec![
                &zephir_policy!(
                    String::from("p13"),
                    PolicyVersion::Version1,
                    PolicyEffect::Deny,
                    vec!["get_first"],
                    vec!["resource_one"]
                )
                .unwrap(),
                &zephir_policy!(
                    String::from("p23"),
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec!["get_second"],
                    vec!["resource_one"]
                )
                .unwrap(),
            ].into_iter(),
            Option::Some(String::from("get_first")),
            Option::Some(String::from("resource_onw")),
        );

        assert_eq!(res.outcome(), AllowedOutcome::Denied);

        let mut json = Map::new();
        json.insert(String::from("outcome"), Value::from("DENIED"));
        json.insert(
            String::from("partials"),
            Value::from(Vec::<PartialPolicy>::new()),
        );
        assert_eq!(res.to_json(), json);
    }

    #[test]
    fn allowed_should_work_correctly() {
        let ps = PolicySet::new()
            .add_policy(
                zephir_policy!(
                    String::from("RoleTestPolicy"),
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec![String::from("TestAction")]
                )
                .unwrap(),
            )
            .add_policy(
                zephir_policy!(
                    String::from("RoleTestPolicy2"),
                    PolicyVersion::Version1,
                    PolicyEffect::Deny,
                    vec![String::from("TestAction")],
                    vec![String::from("urn:resource:test-class-deny:*")]
                )
                .unwrap(),
            )
            .add_policy(
                zephir_policy!(
                    String::from("RoleTestPolicy3"),
                    PolicyVersion::Version1,
                    PolicyEffect::Allow,
                    vec![String::from("FooAction")],
                    vec![String::from("urn:resource:test-class:*")]
                )
                .unwrap(),
            );

        let role = ConcreteRole { policy_set: ps };

        let result = role.allowed(
            Option::Some("TestAction"),
            Option::Some("urn:resource:test-class-allow:test-id"),
        );
        assert_eq!(result.outcome(), AllowedOutcome::Allowed);
        assert_eq!(result.get_partials().len(), 0);

        let result = role.allowed(
            Option::Some("TestAction"),
            Option::Some("urn:resource:test-class-deny:test-id"),
        );
        assert_eq!(result.outcome(), AllowedOutcome::Denied);
        assert_eq!(result.get_partials().len(), 0);

        let result = role.allowed(
            Option::Some("FooAction"),
            Option::Some("urn:resource:test-class-deny:test-id"),
        );
        assert_eq!(result.outcome(), AllowedOutcome::Denied);
        assert_eq!(result.get_partials().len(), 0);

        let result = role.allowed::<&str, String>(Option::Some("FooAction"), Option::None);
        assert_eq!(result.outcome(), AllowedOutcome::Abstain);
        assert_eq!(result.get_partials().len(), 1);

        let result = role.allowed::<&str, String>(Option::Some("TestAction"), Option::None);
        assert_eq!(result.outcome(), AllowedOutcome::Allowed);
        assert_eq!(result.get_partials().len(), 1);
    }
}
