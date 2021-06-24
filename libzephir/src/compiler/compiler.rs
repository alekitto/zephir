use crate::cache::create_cache;
use crate::compiler::compiled_policy::CompiledPolicy;
use crate::policy::condition::Condition;
use crate::utils::glob_to_regex;
use log::{debug, log_enabled, trace, warn, Level};
use mouscache::{Cache, CacheError};
use std::lazy::SyncLazy;
use std::ops::Deref;

static COMPILER: SyncLazy<Compiler> = SyncLazy::new(|| Compiler::new(create_cache()));

pub(crate) mod cache {
    use crate::compiler::compiled_policy::CompiledPolicy;
    use crate::compiler::compiler::COMPILER;

    /// Removes a compiled policy from the cache.
    /// Should be called from the storage manager, when a policy is updated or removed.
    pub fn flush_policy(id: &str) {
        let _ = COMPILER.cache.remove::<_, CompiledPolicy>(id);
    }
}

pub struct Compiler {
    cache: Cache,
}

impl Default for Compiler {
    /// Creates a new compiler with the default *in-memory* cache.
    /// Should not be used if more than one instance of zephir is in execution
    /// and a redis cache should be preferred in case.
    ///
    /// However, is small deployments, in memory cache is more than enough
    /// and avoids an expensive redis (or redis-cluster) deployment.
    fn default() -> Self {
        Self::new(mouscache::memory())
    }
}

impl Compiler {
    /// Creates a new compiler object using the given cache implementation
    ///
    /// The cache will be used to store copies of CompiledPolicy objects:
    /// glob to regex operation is in fact very expensive, while "allowed"
    /// operation should be very fast in order to be usable.
    fn new(cache: Cache) -> Self {
        Compiler { cache }
    }

    /// Gets a reference to the compiler singleton.
    /// Compiler should be used as a singleton. One compiler will live for the entire
    /// life of the application: could be lazily initialized, but should only be
    /// unloaded when the application stops.
    pub fn get_instance() -> &'static Self {
        COMPILER.deref()
    }

    /// Compiles a policy
    ///
    /// This function will convert policy components (actions, resources) into
    /// regexes that could be easily matched against the strings present in
    /// an "allowed" request.
    ///
    /// The id field must be unique and represents the identifier of the policy.
    /// The field will be used as a cache key to avoid glob-to-regex recalculation.
    ///
    /// # Returns
    ///
    /// A CompiledPolicy object
    pub fn compile(
        &self,
        id: &str,
        actions: &[String],
        resources: &[String],
        conditions: Vec<Condition>,
    ) -> CompiledPolicy {
        let item = if id.is_empty() {
            Err(CacheError::Other("".to_string()))
        } else {
            self.cache.get(id)
        };
        if (&item).is_ok() && (&item).as_ref().unwrap().is_some() {
            debug!("Compiled policy {} found in cache.", id);
            return item.unwrap().unwrap();
        }

        let compiled_actions = actions
            .iter()
            .map(|a| glob_to_regex::from_string(a.to_string()))
            .collect();

        let any_resource = resources.iter().any(|v| v == r"*");
        let compiled_resources = if any_resource {
            vec![]
        } else {
            resources
                .iter()
                .map(|a| glob_to_regex::from_string(a.to_string()))
                .collect()
        };

        let cp = CompiledPolicy::new(compiled_actions, compiled_resources, conditions);
        if !id.is_empty() {
            self.cache
                .insert(id, cp.clone())
                .map(|_| {
                    trace!(r#"Compiled policy "{}" successfully stored in cache"#, id);
                })
                .map_err(|err| {
                    warn!(
                        r#"Compiled policy "{}" failed to be stored in cache: {}"#,
                        id,
                        err.to_string()
                    );
                    err
                })
                .ok();
        }

        debug!("Compiled policy with id {}", id);
        if log_enabled!(Level::Trace) {
            trace!(r#"Compiled policy "{}": {:#?}"#, id, cp);
        }

        cp
    }
}
