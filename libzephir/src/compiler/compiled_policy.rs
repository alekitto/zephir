use crate::err::{Error, ErrorKind, NoneError};
use crate::policy::condition::Condition;
use log::{log_enabled, trace, warn, Level};
use mouscache::{CacheError, Cacheable};
use pcre2::bytes::{Regex, RegexBuilder};
use serde_json::Value;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct CompiledPolicy {
    actions: Vec<Regex>,
    resources: Vec<Regex>,
    conditions: Vec<Condition>,

    pub no_conditions: bool,
    pub all_resources: bool,
}

fn redis_obj_to_regex(obj: &HashMap<String, String>, key: &str) -> Result<Vec<Regex>, Error> {
    let value = obj[key].parse::<Value>()?;
    let value = value.as_array();
    if value.is_none() {
        return Err(Error::new(ErrorKind::UnwrapNoneValueError, NoneError {}));
    }

    let mut result = vec![];
    for r in value.unwrap() {
        let r = r.as_str();
        if r.is_none() {
            return Err(Error::new(ErrorKind::UnwrapNoneValueError, NoneError {}));
        }

        result.push(
            RegexBuilder::new()
                .jit_if_available(true)
                .build(r.unwrap())?,
        );
    }

    Ok(result)
}

impl CompiledPolicy {
    /// Creates a new compiled policy
    ///
    /// This should *only* by called by the compiler after it has analysed
    /// the actions and resources arrays and converted them to vectors of
    /// Regex objects, ready to match the incoming requests.
    ///
    /// An empty set of resources means that all the resources are valid
    /// for a matching operation, while an empty actions vector means
    /// that no actions will be valid. This however should be prevented
    /// by the CompletePolicy::new that should not allow an empty actions array.
    pub fn new(
        actions: Vec<Regex>,
        resources: Vec<Regex>,
        conditions: Vec<Condition>,
    ) -> CompiledPolicy {
        if log_enabled!(Level::Trace) {
            trace!(
                "Compiled policy: actions: {:#?}, resources: {:#?}",
                actions,
                resources
            );
        }

        let no_conditions = conditions.is_empty();
        let all_resources = resources.is_empty();

        CompiledPolicy {
            actions,
            resources,
            conditions,
            no_conditions,
            all_resources,
        }
    }

    /// Try to match an action against this compiled policy
    ///
    /// This function will simply iterate the vector of actions regexes
    /// to check if *AT LEAST ONE* matches the given action.
    /// The iteration will stop at the first valid match found.
    ///
    /// # Returns
    ///
    /// True if at least one match is found, false otherwise
    pub fn match_action<T: ToString>(&self, action: &T) -> bool {
        let action = action.to_string();
        let action_str = action.as_bytes();

        trace!("Requesting match for action {}...", action);
        for regex in &self.actions {
            let is_match = regex.is_match(action_str);
            match is_match {
                Err(e) => warn!(
                    "Regex {} caused an error: {}",
                    regex.as_str(),
                    e.to_string()
                ),
                Ok(result) => {
                    if result {
                        trace!("Regex {} matches the action {}", regex.as_str(), action);
                        return true;
                    }
                }
            }
        }

        trace!("No match");
        false
    }

    /// Try to match a resource string against this compiled policy.
    ///
    /// This function is a little bit more complex than match_action:
    /// in fact, to allow partial matching, resource could be None.
    /// Additionally an empty resources vector could be passed to the policy
    /// meaning that this should return true whatever the resource argument is.
    ///
    /// As per match_action function, if resource is given and this policy
    /// does not represent a match all, the first match will stop the iteration.
    ///
    /// # Returns
    ///
    /// An optional boolean value:
    /// - true if this policy is a match-all or *at least one* resources regexes matches
    /// - false if this policy is *NOT* a match-all an no regex matches
    /// - Option::None if this policy is *NOT* a match-all and the passed resource is None
    pub fn match_resource<T: ToString + Debug>(&self, resource: Option<T>) -> Option<bool> {
        trace!("Requesting match for resource {:#?}...", resource);
        if self.all_resources {
            trace!("Policy is resource-match-all");
            return Option::Some(true);
        }

        match resource {
            Option::None => {
                trace!("Returning None");
                Option::None
            }
            Option::Some(resource) => Option::Some({
                let string = resource.to_string();
                let res = string.as_bytes();
                let mut result = false;

                for regex in &self.resources {
                    let is_match = regex.is_match(res);
                    match is_match {
                        Err(e) => warn!(
                            "Regex {} caused an error: {}",
                            regex.as_str(),
                            e.to_string()
                        ),
                        Ok(regex_matching) => {
                            if regex_matching {
                                trace!("Regex {} matches the resource {}", regex.as_str(), string);
                                result = true;
                                break;
                            }
                        }
                    }
                }

                if !result {
                    trace!("No match");
                }

                result
            }),
        }
    }

    /// Try to match request parameters to the policy conditions
    /// This function will not support partial matching.
    ///
    /// # Returns
    ///
    /// True if all conditions matches, false otherwise
    pub fn match_conditions(&self, params: &Value) -> bool {
        for c in &self.conditions {
            if !c.matching(params) {
                return false;
            }
        }

        true
    }

    /// INTERNAL: Hydrate from redis cache object.
    ///
    /// Compiled policies can be cached in memory or on redis.
    /// This function will hydrate a new CompiledPolicy from a cached structure
    /// or return an error if the operation fails.
    ///
    /// # Returns
    ///
    /// Result with CompiledPolicy object or an Error
    fn from_redis_obj(obj: HashMap<String, String>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let all_resources: bool = obj["all_res"].parse()?;
        let no_conditions: bool = obj["no_cond"].parse()?;
        let actions = redis_obj_to_regex(&obj, "actions")?;
        let resources = redis_obj_to_regex(&obj, "resources")?;
        let conditions: Vec<Condition> = serde_json::from_str(obj["conditions"].as_str())?;

        Ok(CompiledPolicy {
            actions,
            resources,
            conditions,
            no_conditions,
            all_resources,
        })
    }
}

impl Cacheable for CompiledPolicy {
    /// Will be used as cache prefix in redis
    fn model_name() -> &'static str
    where
        Self: Sized,
    {
        "zephir_cpolicy"
    }

    /// Serializes a CompiledPolicy into a redis-storable structure
    /// The returned vector will be converted to an HashMap
    /// on cache retrieval which will be used to hydrate a new object.
    fn to_redis_obj(&self) -> Vec<(String, String)> {
        let mut v = Vec::new();

        let mut actions = Vec::new();
        for r in &self.actions {
            actions.push(r.as_str().to_string());
        }

        let mut resources = Vec::new();
        for r in &self.resources {
            resources.push(r.as_str().to_string());
        }

        v.push((String::from("actions"), Value::from(actions).to_string()));
        v.push((
            String::from("resources"),
            Value::from(resources).to_string(),
        ));
        v.push((String::from("all_res"), self.all_resources.to_string()));
        v.push((
            String::from("conditions"),
            serde_json::to_string(&self.conditions).unwrap(),
        ));

        v
    }

    /// Hydrate an object from a redis-stored hashmap.
    ///
    /// Please note: this function will be called even if no cache is present
    /// on Redis. In this case the hashmap will contain no elements.
    /// An error should be returned to signal the caller that no object
    /// could be constructed from an empty hashmap and should then follow a
    /// possibly expensive path to re-compute the CompiledPolicy object.
    fn from_redis_obj(obj: HashMap<String, String>) -> mouscache::Result<Self>
    where
        Self: Sized,
    {
        if obj.is_empty() {
            return Err(CacheError::Other(String::new()));
        }

        match CompiledPolicy::from_redis_obj(obj) {
            Err(err) => Err(CacheError::Other(err.to_string())),
            Ok(res) => Ok(res),
        }
    }

    /// A default TTL for this element.
    /// None means that no TTL is defined.
    fn expires_after(&self) -> Option<usize> {
        Option::None
    }

    /// Used in memory cache.
    fn as_any(&self) -> &dyn Any {
        self
    }
}
