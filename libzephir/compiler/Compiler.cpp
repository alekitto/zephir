#include "CompiledPolicy.h"
#include "Compiler.h"
#include "../Policy.hpp"
#include "../util/util.h"

std::shared_ptr<CompiledPolicy> Compiler::compile(Policy &policy)
{
    using namespace std;
    using namespace util;

    auto item = _cache.get(policy.id);
    if ((bool) item) {
        return item.value();
    }

    regex_vec compiledActions = util::apply(policy._actions.value(), &glob_to_regex);

    auto resources = policy._resources.value();
    bool anyResources = any_of(begin(resources), end(resources), [](const string &v) { return "*" == v; });

    regex_vec compiledResources = anyResources ? vector<regex>() : util::apply(policy._resources.value(), &glob_to_regex);

    // TODO: conditions

    auto compiled = std::make_shared<CompiledPolicy>(compiledActions, compiledResources);
    _cache.insert(policy.id, compiled);

    return compiled;
}