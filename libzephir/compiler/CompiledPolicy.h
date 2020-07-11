#pragma once

#include <optional>
#include <regex>
#include <vector>

namespace libzephir::compiler {
    typedef const std::vector<std::regex> regex_vec;

    class CompiledPolicy {
        const regex_vec actions;
        const regex_vec resources;

        bool _allResources;

    public:
        CompiledPolicy(const regex_vec &actions, const regex_vec &resources) :
                actions(actions),
                resources(resources),
                _allResources(resources.empty()) {
        }

        bool matchAction(const std::string &action) {
            for (const std::regex &regex : this->actions) {
                if (std::regex_match(action, regex)) {
                    return true;
                }
            }

            return false;
        }

        std::optional<bool> matchResource(const std::optional<std::string> &resource) {
            if (this->_allResources) {
                return true;
            }

            if (!resource.has_value()) {
                return std::nullopt;
            }

            auto r = resource.value();
            for (const std::regex &regex : this->resources) {
                if (std::regex_match(r, regex)) {
                    return true;
                }
            }

            return false;
        }
    };
}
