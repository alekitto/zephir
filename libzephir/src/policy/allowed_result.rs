use crate::policy::policy::{PartialPolicy, ToJson};
use crate::policy::PolicyEffect;
use serde_json::{Map, Value};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum AllowedOutcome {
    Denied = -1,
    Abstain = 0,
    Allowed = 1,
}

#[derive(Debug)]
pub struct AllowedResult {
    outcome: AllowedOutcome,
    partials: Vec<PartialPolicy>,
}

impl AllowedResult {
    pub(crate) fn new(outcome: AllowedOutcome, partials: Vec<PartialPolicy>) -> Self {
        AllowedResult {
            outcome,
            partials: match outcome {
                AllowedOutcome::Denied => vec![],
                AllowedOutcome::Allowed => partials
                    .into_iter()
                    .filter(|p| p.effect == PolicyEffect::Deny)
                    .collect(),
                _ => partials,
            },
        }
    }

    pub fn denied() -> Self {
        Self {
            outcome: AllowedOutcome::Denied,
            partials: vec![],
        }
    }

    pub fn get_partials(&self) -> Vec<&PartialPolicy> {
        self.partials.iter().collect()
    }

    pub fn outcome(&self) -> AllowedOutcome {
        let outcome = self.outcome;

        if outcome == AllowedOutcome::Abstain && self.partials.is_empty() {
            AllowedOutcome::Denied
        } else {
            outcome
        }
    }

    pub fn merge(&mut self, other: Self) {
        if other.outcome == AllowedOutcome::Denied {
            self.outcome = AllowedOutcome::Denied;
            self.partials = vec![];
        }

        if self.outcome == AllowedOutcome::Denied {
            return;
        }

        if other.outcome == AllowedOutcome::Allowed {
            self.outcome = AllowedOutcome::Allowed;
        }

        for p in other.partials {
            self.partials.push(p);
        }

        if self.outcome == AllowedOutcome::Allowed {
            self.partials = self
                .partials
                .drain(..)
                .filter(|p| p.effect == PolicyEffect::Deny)
                .collect();
        }
    }
}

impl ToJson for AllowedResult {
    fn to_json(&self) -> Map<String, Value> {
        let mut result = Map::new();
        result.insert(
            String::from("outcome"),
            Value::from(match self.outcome() {
                AllowedOutcome::Denied => "DENIED",
                AllowedOutcome::Abstain => "ABSTAIN",
                AllowedOutcome::Allowed => "ALLOWED",
            }),
        );

        result.insert(String::from("partials"), Value::from(self.partials.as_slice()));

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::policy::allowed_result::{AllowedOutcome, AllowedResult};
    use crate::policy::policy::{PartialPolicy, ToJson};
    use crate::policy::PolicyEffect;
    use serde_json::{Map, Value};

    #[test]
    fn new_with_denied_should_reset_partials() {
        let ar = AllowedResult::new(AllowedOutcome::Denied, vec![PartialPolicy::default()]);

        assert_eq!(ar.outcome(), AllowedOutcome::Denied);
        assert_eq!(ar.partials.len(), 0);
    }

    #[test]
    fn new_with_allowed_should_retain_only_deny_partials() {
        let p1 = PartialPolicy::default();
        let mut p2 = PartialPolicy::default();
        p2.effect = PolicyEffect::Deny;

        let partials = vec![p1, p2];
        let ar = AllowedResult::new(AllowedOutcome::Allowed, partials);

        assert_eq!(ar.outcome(), AllowedOutcome::Allowed);
        assert_eq!(ar.partials.len(), 1);
    }

    #[test]
    fn outcome_should_be_denied_if_abstain_with_no_partials() {
        let ar = AllowedResult {
            outcome: AllowedOutcome::Abstain,
            partials: vec![],
        };

        let mut json = Map::new();
        json.insert(String::from("outcome"), Value::from("DENIED"));
        json.insert(
            String::from("partials"),
            Value::from(Vec::<PartialPolicy>::new()),
        );

        assert_eq!(ar.outcome(), AllowedOutcome::Denied);
        assert_eq!(ar.to_json(), json);
    }

    #[test]
    fn outcome_abstain_should_be_returned() {
        let ar = AllowedResult {
            outcome: AllowedOutcome::Abstain,
            partials: vec![PartialPolicy::default()],
        };

        let mut json = Map::new();
        json.insert(String::from("outcome"), Value::from("ABSTAIN"));
        json.insert(String::from("partials"), Value::from(ar.partials.as_slice()));

        assert_eq!(ar.outcome(), AllowedOutcome::Abstain);
        assert_eq!(ar.to_json(), json);
    }

    #[test]
    fn merge_with_denied_result_should_reset_partials() {
        let mut ar = AllowedResult::new(AllowedOutcome::Abstain, vec![PartialPolicy::default()]);

        ar.merge(AllowedResult::new(AllowedOutcome::Denied, vec![]));

        let mut json = Map::new();
        json.insert(String::from("outcome"), Value::from("DENIED"));
        json.insert(
            String::from("partials"),
            Value::from(Vec::<PartialPolicy>::new()),
        );

        assert_eq!(ar.outcome(), AllowedOutcome::Denied);
        assert_eq!(ar.to_json(), json);
    }

    #[test]
    fn merge_with_abstain_should_copy_partials() {
        let mut ar = AllowedResult::new(AllowedOutcome::Abstain, vec![]);

        ar.merge(AllowedResult::new(
            AllowedOutcome::Abstain,
            vec![PartialPolicy::default()],
        ));

        let mut json = Map::new();
        json.insert(String::from("outcome"), Value::from("ABSTAIN"));
        json.insert(
            String::from("partials"),
            Value::from(vec![PartialPolicy::default()]),
        );

        assert_eq!(ar.outcome(), AllowedOutcome::Abstain);
        assert_eq!(ar.to_json(), json);
    }

    #[test]
    fn merge_with_allow_should_copy_deny_partials() {
        let mut ar = AllowedResult::new(AllowedOutcome::Abstain, vec![]);

        let mut partial = PartialPolicy::default();
        partial.effect = PolicyEffect::Deny;
        ar.merge(AllowedResult::new(
            AllowedOutcome::Allowed,
            vec![partial.clone()],
        ));

        let mut json = Map::new();
        json.insert(String::from("outcome"), Value::from("ALLOWED"));
        json.insert(String::from("partials"), Value::from(vec![partial.clone()]));

        assert_eq!(ar.outcome(), AllowedOutcome::Allowed);
        assert_eq!(ar.to_json(), json);
    }
}
