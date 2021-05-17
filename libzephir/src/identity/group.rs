use crate::identity::identity::{Identity, ToIdentityId};
use crate::identity::role::{allowed, Role};
use crate::identity::subject::{Subject, SubjectIterator};
use crate::policy::allowed_result::AllowedResult;
use crate::policy::policy::{CompletePolicy, ToJson};
use crate::policy::policy_set::{PolicySet, PolicySetHelper, PolicySetTrait};
use serde_json::{Map, Value};
use std::borrow::BorrowMut;
use std::collections::hash_set::Iter;
use std::collections::HashSet;
use std::fmt::{Debug, Display};

/// Helper function to insert an identity to the set
/// if no identity with the same id is found.
fn insert_if_missing(identities: &mut HashSet<Identity>, identity: Identity) {
    let found = identities.iter().find(|i| i.id == identity.id);
    if found.is_none() {
        identities.insert(identity);
    }
}

pub struct IdentitySet {
    identities: HashSet<Identity>,
}

/// Represents a set of unique identities
impl IdentitySet {
    /// Creates a new identity set.
    ///
    /// # Returns
    /// A new IdentitySet object. The identity set passed as argument
    /// will be owned by the newly created object.
    fn new(identities: HashSet<Identity>) -> Self {
        IdentitySet { identities }
    }

    /// Returns the length of the identity set.
    ///
    /// # Returns
    /// The size of the set as usize.
    pub fn len(&self) -> usize {
        self.identities.len()
    }

    /// Returns whether the identity set is empty or not.
    ///
    /// # Returns
    /// TRUE if the set length is equal to 0, FALSE otherwise.
    pub fn is_empty(&self) -> bool {
        self.identities.is_empty()
    }

    /// Inserts an Identity object into the Set.
    ///
    /// # Returns
    /// This function moves the self object returning it after the
    /// operation is completed.
    pub fn insert(mut self, identity: Identity) -> Self {
        insert_if_missing(self.identities.borrow_mut(), identity);

        self
    }

    /// Removes an identity from the Set.
    /// The identity could be an Identity object or an id as String.
    ///
    /// # Returns
    /// Similarly to the insert function, this method returns the
    /// self object after the operation is completed.
    pub fn remove<T: ToIdentityId>(mut self, identity: T) -> Self {
        let identity_id = identity.to_identity_id();
        self.identities = self
            .identities
            .drain_filter(|i| i.id != *identity_id)
            .collect();

        self
    }
}

impl Default for IdentitySet {
    /// Creates a default, empty set.
    fn default() -> Self {
        Self::new(HashSet::new())
    }
}

impl<'a> IntoIterator for &'a IdentitySet {
    type Item = &'a Identity;
    type IntoIter = Iter<'a, Identity>;

    fn into_iter(self) -> Self::IntoIter {
        let set = &self.identities;
        set.iter()
    }
}

/// Represents a Group
pub struct Group {
    /// The name of the group. Must be unique.
    pub(crate) name: String,

    /// The identity set.
    pub(crate) identities: IdentitySet,

    /// An optional inline policy linked to this group.
    pub(crate) inline_policy: Option<CompletePolicy>,

    /// All the linked policies to this group.
    pub(crate) linked_policies: PolicySet<CompletePolicy>,
}

impl Group {
    /// Creates a new Group object.
    ///
    /// # Returns
    /// A new Group object, with empty identity set and no linked policies.
    pub fn new<T>(name: T, policy: Option<CompletePolicy>) -> Self
    where
        T: ToString,
    {
        Group {
            name: name.to_string(),
            identities: IdentitySet::default(),
            inline_policy: policy,
            linked_policies: PolicySet::new(),
        }
    }

    /// Gets the name (unique) of this group.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Clears the inline policy associated to this group.
    /// The inline policy will be effectively deleted on next saving.
    ///
    /// # Returns
    /// The current object.
    pub fn clear_inline_policy(mut self) -> Self {
        self.inline_policy = Option::None;
        self
    }

    /// Sets the inline policy for the current group.
    /// Policy id will be discarded and a new id will be generated for it.
    ///
    /// # Returns
    /// The current object.
    pub fn set_inline_policy(mut self, policy: CompletePolicy) -> Self {
        let mut policy = policy;
        policy.id = "__embedded_policy_group_".to_owned() + self.name.as_str() + "__";

        self.inline_policy = Option::Some(policy);
        self
    }

    /// Creates an iterator upon the identities set.
    /// The iterator will not consume the set and yields elements
    /// of type is &'a Identity, where 'a is the lifetime of this group.
    pub async fn get_identities(&self) -> Iter<'_, Identity> {
        let identity_set = &self.identities;
        identity_set.into_iter()
    }

    /// Adds an identity to the Group.
    ///
    /// # Returns
    /// This function moves the self object returning it after the
    /// operation is completed.
    pub fn add_identity(mut self, identity: Identity) -> Self {
        self.identities = self.identities.insert(identity);
        self
    }

    /// Removes an identity from the Group.
    /// The identity could be an Identity object or an id as String.
    ///
    /// # Returns
    /// This function moves the self object returning it after the
    /// operation is completed.
    pub fn remove_identity<T: ToIdentityId>(mut self, identity: T) -> Self {
        self.identities = self.identities.remove(identity);
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

    fn allowed<T, S>(&self, action: Option<T>, resource: Option<S>, params: &Value) -> AllowedResult
    where
        T: ToString + Display,
        S: ToString + Display + Debug,
    {
        allowed(SubjectIterator::new(self), action, resource, params)
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
    fn identities_can_be_added_to_a_group() {
        let mut g = Group::new("Group", Option::None);
        assert_eq!(g.identities.len(), 0);

        let i = Identity::new("TestIdentity", Option::None);
        g = g.add_identity(i);

        let i = Identity::new("TestIdentity", Option::None);
        g = g.add_identity(i);

        assert_eq!(g.identities.len(), 1);
    }

    #[test]
    fn identities_can_be_removed_from_a_group() {
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
