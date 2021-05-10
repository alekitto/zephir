use crate::policy::policy::{CompletePolicy, Policy};
use std::borrow::BorrowMut;
use std::collections::hash_set::Iter;
use std::collections::HashSet;

pub(crate) struct PolicySetHelper {}

impl PolicySetHelper {
    pub(crate) fn link_policy(
        policy_set: PolicySet<CompletePolicy>,
        policy: CompletePolicy,
    ) -> PolicySet<CompletePolicy> {
        policy_set.add_policy(policy)
    }

    pub(crate) fn unlink_policy<S: ToString>(
        policy_set: PolicySet<CompletePolicy>,
        policy: S,
    ) -> PolicySet<CompletePolicy> {
        policy_set.remove_policy(policy.to_string())
    }
}

#[derive(Debug)]
pub struct PolicySet<T: Policy> {
    policies: HashSet<T>,
}

impl<T: Policy> Default for PolicySet<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Policy> PolicySet<T> {
    pub fn new() -> Self {
        PolicySet {
            policies: HashSet::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.policies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.policies.is_empty()
    }

    fn insert_if_missing(policies: &mut HashSet<T>, policy: T) {
        match policies.iter().find(|p| *p.id() == *policy.id()) {
            Some(_) => {}
            None => {
                policies.insert(policy);
            }
        }
    }
}

/// Represents a PolicySet implementation.
/// The set does not guarantee to retain the insertion order.
pub trait PolicySetTrait<T: Policy> {
    /// Adds a policy to the set.
    ///
    /// # Returns
    /// The current object, to allow fluid interface.
    fn add_policy(self, policy: T) -> Self;

    /// Removes a policy from the set, identified by id.
    ///
    /// # Returns
    /// The current object, to allow fluid interface.
    fn remove_policy<S: ToString>(self, id: S) -> Self;
}

impl<T: Policy> PolicySetTrait<T> for PolicySet<T> {
    fn add_policy(mut self, policy: T) -> Self {
        Self::insert_if_missing(self.policies.borrow_mut(), policy);
        self
    }

    fn remove_policy<S: ToString>(mut self, id: S) -> Self {
        let policy_id = id.to_string();
        self.policies = self
            .policies
            .drain_filter(|p| policy_id != *p.id())
            .collect();

        self
    }
}

impl<'a, T: Policy> IntoIterator for &'a PolicySet<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        let policy_set = &self.policies;
        policy_set.iter()
    }
}

#[cfg(test)]
mod tests {
    use crate::policy::policy::CompletePolicy;
    use crate::policy::policy_set::{PolicySet, PolicySetTrait};
    use crate::policy::{PolicyEffect, PolicyVersion};
    use crate::zephir_policy;

    #[test]
    fn should_be_created_empty() {
        let ps: PolicySet<CompletePolicy> = PolicySet::new();
        assert_eq!(ps.len(), 0);
    }

    #[test]
    fn policies_can_be_added() {
        let mut ps: PolicySet<CompletePolicy> = PolicySet::new();
        ps = ps.add_policy(
            zephir_policy!(
                "p1",
                PolicyVersion::Version1,
                PolicyEffect::Allow,
                vec!["action"]
            )
            .unwrap(),
        );

        let ps_ref = &ps;
        let policies: Vec<&CompletePolicy> = ps_ref.into_iter().collect();

        assert_eq!(ps.len(), 1);
        assert_eq!(policies.len(), 1);
    }

    #[test]
    fn policies_can_be_removed_by_id() {
        let mut ps: PolicySet<CompletePolicy> = PolicySet::new();
        ps = ps.add_policy(
            zephir_policy!(
                "p1",
                PolicyVersion::Version1,
                PolicyEffect::Allow,
                vec!["action"]
            )
            .unwrap(),
        );
        ps = ps.add_policy(
            zephir_policy!(
                "p2",
                PolicyVersion::Version1,
                PolicyEffect::Allow,
                vec!["action"]
            )
            .unwrap(),
        );
        ps = ps.add_policy(
            zephir_policy!(
                "p3",
                PolicyVersion::Version1,
                PolicyEffect::Allow,
                vec!["action"]
            )
            .unwrap(),
        );

        ps = ps.remove_policy("p2");

        let ps_ref = &ps;
        let mut policies: Vec<&str> = ps_ref
            .into_iter()
            .map(|p: &CompletePolicy| p.id.as_str())
            .collect();

        policies.sort();

        assert_eq!(policies.len(), 2);
        assert_eq!(policies, vec!["p1", "p3"]);
    }
}
