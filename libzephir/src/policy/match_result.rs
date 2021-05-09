use crate::policy::policy::{MatchablePolicy, PartialPolicy};
use crate::policy::PolicyVersion;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResultType {
    Partial,
    Full,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ResultOutcome {
    Match = 0,
    NotMatch = 1,
}

#[derive(Clone)]
pub struct MatchResult {
    result_type: ResultType,
    outcome: ResultOutcome,

    partial: PartialPolicy,

    action_matches: Option<bool>,
    resource_matches: Option<bool>,
    conditions_match: Option<bool>,
}

impl Default for MatchResult {
    fn default() -> Self {
        Self::new()
    }
}

impl AsRef<MatchResult> for MatchResult {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl MatchResult {
    /// Creates a new MatchResult structure
    pub fn new() -> MatchResult {
        MatchResult {
            result_type: ResultType::Partial,
            outcome: ResultOutcome::NotMatch,
            partial: PartialPolicy::default(),
            action_matches: None,
            resource_matches: None,
            conditions_match: None,
        }
    }

    /// Updates the match flag for action
    pub(super) fn update_action(&mut self, result: bool) {
        self.action_matches = Option::Some(result);
    }

    /// Updates the match flag for resource
    pub(super) fn update_resource(&mut self, result: bool) {
        self.resource_matches = Option::Some(result);
    }

    /// Updates the match flag for conditions
    pub(super) fn update_conditions(&mut self, result: bool) {
        self.conditions_match = Option::Some(result);
    }

    /// Gets the partial policy.
    /// Has meaning only if result type is not full and outcome is "match"
    pub fn get_partial(self) -> PartialPolicy {
        self.partial
    }

    /// Whether the outcome is "match" or not
    #[inline]
    pub fn is_match(&self) -> bool {
        self.outcome == ResultOutcome::Match
    }

    /// Whether the result type is full or partial
    pub fn is_full(&self) -> bool {
        self.result_type == ResultType::Full
    }

    /// Internal: updates the result
    pub(super) fn _update(&mut self, policy: &impl MatchablePolicy) {
        self.partial.reset();
        self.partial.effect = policy.get_effect();

        if (self.action_matches.is_some() && !self.action_matches.unwrap())
            || (self.resource_matches.is_some() && !self.resource_matches.unwrap())
            || (self.conditions_match.is_some() && !self.conditions_match.unwrap())
        {
            self.result_type = ResultType::Full;
            self.outcome = ResultOutcome::NotMatch;

            return;
        }

        if self.action_matches.unwrap_or(false) || self.resource_matches.unwrap_or(false) {
            self.outcome = ResultOutcome::Match;
        }

        if self.action_matches.is_some() && self.resource_matches.is_some() {
            self.result_type = ResultType::Full;
        } else {
            self.partial = PartialPolicy {
                version: PolicyVersion::Version1,
                effect: self.partial.effect,
                actions: if self.action_matches.is_some() {
                    None
                } else {
                    Option::Some(policy.get_actions().to_vec())
                },
                resources: if self.resource_matches.is_some() {
                    None
                } else {
                    Option::Some(policy.get_resources().to_vec())
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::policy::match_result::{MatchResult, ResultOutcome, ResultType};
    use crate::policy::{PolicyEffect, PolicyVersion};
    use crate::zephir_policy;

    #[test]
    fn match_result_could_be_created() {
        let mr = MatchResult::new();

        assert_eq!(mr.outcome, ResultOutcome::NotMatch);
        assert_eq!(mr.result_type, ResultType::Partial);
    }

    #[test]
    fn match_result_is_full_if_something_does_not_match() {
        let policy = zephir_policy!(
            "TestPolicy",
            PolicyVersion::Version1,
            PolicyEffect::Allow,
            vec!["get_action"]
        )
        .unwrap();

        let mut mr = MatchResult::new();
        mr.update_action(false);
        mr._update(&policy);
        assert_eq!(mr.is_full(), true);
        assert_eq!(mr.is_match(), false);

        let mut mr = MatchResult::new();
        mr.update_resource(false);
        mr._update(&policy);
        assert_eq!(mr.is_full(), true);
        assert_eq!(mr.is_match(), false);

        let mut mr = MatchResult::new();
        mr.update_conditions(false);
        mr._update(&policy);
        assert_eq!(mr.is_full(), true);
        assert_eq!(mr.is_match(), false);
    }

    #[test]
    fn actions_should_be_included_in_partial_if_resource_match() {
        let policy = zephir_policy!(
            "TestPolicy",
            PolicyVersion::Version1,
            PolicyEffect::Allow,
            vec!["get_action"]
        )
        .unwrap();

        let mut mr = MatchResult::new();
        mr.update_resource(true);
        mr._update(&policy);

        assert_eq!(mr.is_full(), false);
        assert_eq!(mr.is_match(), true);

        let partial = mr.get_partial();
        assert_eq!(partial.effect, PolicyEffect::Allow);
        assert_eq!(
            partial.actions,
            Option::Some(vec!["get_action".to_string()])
        );
        assert_eq!(partial.resources, Option::None);
    }

    #[test]
    fn resources_should_be_included_in_partial_if_resource_match() {
        let policy = zephir_policy!(
            "TestPolicy",
            PolicyVersion::Version1,
            PolicyEffect::Allow,
            vec!["get_action"],
            vec!["resource1", "resource2"]
        )
        .unwrap();

        let mut mr = MatchResult::new();
        mr.update_action(true);
        mr._update(&policy);

        assert_eq!(mr.is_full(), false);
        assert_eq!(mr.is_match(), true);

        let partial = mr.get_partial();
        assert_eq!(partial.effect, PolicyEffect::Allow);
        assert_eq!(
            partial.resources,
            Option::Some(vec!["resource1".to_string(), "resource2".to_string(),])
        );
        assert_eq!(partial.actions, Option::None);
    }
}
