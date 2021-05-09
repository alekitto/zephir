use crate::identity::identity::{Identity, ToIdentityId};
use crate::identity::role::{Role, allowed};
use crate::identity::subject::{Subject, SubjectIterator};
use crate::policy::policy::{CompletePolicy, ToJson};
use crate::policy::policy_set::{PolicySet, PolicySetHelper, PolicySetTrait};
use serde_json::{Map, Value};
use std::cmp::Ordering;
use std::slice::Iter;
use crate::policy::allowed_result::AllowedResult;
use std::fmt::{Display, Debug};

pub struct IdentitySet {
    identities: Vec<Identity>,
}

impl<'a> IntoIterator for &'a IdentitySet {
    type Item = &'a Identity;
    type IntoIter = Iter<'a, Identity>;

    fn into_iter(self) -> Self::IntoIter {
        self.identities.iter()
    }
}

impl IdentitySet {
    fn new() -> Self {
        IdentitySet { identities: vec![] }
    }

    pub fn len(&self) -> usize {
        self.identities.len()
    }

    pub fn is_empty(&self) -> bool {
        self.identities.is_empty()
    }

    fn insert_if_missing(identities: &mut Vec<Identity>, identity: Identity) {
        let found = identities.iter_mut().find(|ref i| i.id == identity.id);
        if found.is_none() {
            identities.push(identity);
        }
    }

    pub fn add_identity(mut self, identity: Identity) -> Self {
        Self::insert_if_missing(self.identities.as_mut(), identity);

        self
    }

    pub fn remove_identity<T: ToIdentityId>(mut self, identity: T) -> Self {
        let identity_id = identity.to_identity_id();
        self.identities = self
            .identities
            .into_iter()
            .filter(|i| i.id.cmp(identity_id) != Ordering::Equal)
            .collect();

        self
    }
}

pub struct Group {
    pub(crate) name: String,
    pub(crate) identities: IdentitySet,

    pub(crate) inline_policy: Option<CompletePolicy>,
    pub(crate) linked_policies: PolicySet<CompletePolicy>,
}

impl Group {
    pub fn new<T>(name: T, policy: Option<CompletePolicy>) -> Self
    where
        T: ToString,
    {
        Group {
            name: name.to_string(),
            identities: IdentitySet::new(),
            inline_policy: policy,
            linked_policies: PolicySet::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn clear_inline_policy(mut self) -> Self {
        self.inline_policy = Option::None;
        self
    }

    pub fn set_inline_policy(mut self, policy: CompletePolicy) -> Self {
        let mut policy = policy;
        policy.id = "__embedded_policy_group_".to_owned() + self.name.as_str() + "__";

        self.inline_policy = Option::Some(policy);
        self
    }

    pub fn get_identities(&self) -> &Vec<Identity> {
        self.identities.identities.as_ref()
    }

    pub fn add_identity(mut self, identity: Identity) -> Self {
        self.identities = self.identities.add_identity(identity);
        self
    }

    pub fn remove_identity<T: ToIdentityId>(mut self, identity: T) -> Self {
        self.identities = self.identities.remove_identity(identity);
        self
    }
}

impl PolicySetTrait<CompletePolicy> for Group {
    fn add_policy(mut self, policy: CompletePolicy) -> Self {
        self.linked_policies = PolicySetHelper::link_policy(self.linked_policies, policy);
        self
    }

    fn remove_policy<S: ToString>(mut self, id: S) -> Self {
        self.linked_policies = PolicySetHelper::unlink_policy(self.linked_policies, id);
        self
    }
}

impl Into<Value> for Group {
    fn into(self) -> Value {
        Value::Object(self.to_json())
    }
}

impl ToJson for Group {
    fn to_json(&self) -> Map<String, Value> {
        let linked_policies = &self.linked_policies;
        let mut map = Map::new();
        map.insert(String::from("id"), Value::from(self.name.as_str()));
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

impl Subject for Group {
    fn get_inline_policy(&self) -> Option<&CompletePolicy> {
        self.inline_policy.as_ref()
    }
}

impl Role for Group {
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
    use crate::identity::group::Group;
    use crate::identity::identity::Identity;
    use crate::policy::{PolicyEffect, PolicyVersion};
    use crate::zephir_policy;

    #[test]
    fn group_could_be_created() {
        let g = Group::new("Group", Option::None);
        assert_eq!(g.identities.len(), 0);

        let g = Group::new(
            "Group2",
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
        assert_eq!(g.identities.len(), 0);
    }

    #[test]
    fn identites_can_be_added_to_a_group() {
        let mut g = Group::new("Group", Option::None);
        assert_eq!(g.identities.len(), 0);

        let i = Identity::new("TestIdentity", Option::None);
        g = g.add_identity(i);

        let i = Identity::new("TestIdentity", Option::None);
        g = g.add_identity(i);

        assert_eq!(g.identities.len(), 1);
    }

    #[test]
    fn identites_can_be_removed_from_a_group() {
        let mut g = Group::new("Group", Option::None);
        assert_eq!(g.identities.len(), 0);

        let i = Identity::new("TestIdentity", Option::None);
        let i2 = Identity::new("TestIdentity2", Option::None);
        g = g.add_identity(i);
        g = g.add_identity(i2);
        assert_eq!(g.identities.len(), 2);

        let i = Identity::new("TestIdentity", Option::None);
        g = g.remove_identity(i);
        assert_eq!(g.identities.len(), 1);

        g = g.remove_identity(String::from("TestIdentity"));
        assert_eq!(g.identities.len(), 1);

        let i2 = Identity::new("TestIdentity2", Option::None);
        g = g.remove_identity(i2.id);

        assert_eq!(g.identities.len(), 0);
    }
}
