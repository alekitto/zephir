#pragma once

#include <memory>
#include <optional>
#include <string>
#include <utility>
#include <vector>

#include <nlohmann/json.hpp>

#include "exception/exceptions.h"
#include "Effect.hpp"

namespace libzephir {
    typedef std::vector<std::string> string_vector;

    enum PolicyVersion {
        VERSION_1 = 1,
    };

    class PartialPolicy {
        friend class Compiler;
        friend class MatchResult;

    protected:
        PolicyVersion _version = VERSION_1;
        PolicyEffect _effect;
        std::optional<string_vector> _actions;
        std::optional<string_vector> _resources;

        // TODO: conditions

    public:
        const PolicyEffect& effect;

        PartialPolicy &operator=(PartialPolicy p) {
            using namespace std;

            swap(_version, p._version);
            swap(_effect, p._effect);
            swap(_actions, p._actions);
            swap(_resources, p._resources);

            return *this;
        }

        PartialPolicy(const PartialPolicy &p) :
            _version(p._version),
            _effect(p._effect),
            effect(_effect),
            _actions(p._actions.has_value() ? std::make_optional(p._actions.value()) : std::nullopt),
            _resources(p._resources.has_value() ? std::make_optional(p._resources.value()) : std::nullopt) {}

        PartialPolicy(PolicyVersion pVersion, PolicyEffect pEffect) :
            PartialPolicy(pVersion, pEffect, std::nullopt, std::nullopt) {}

        PartialPolicy(PolicyVersion pVersion, PolicyEffect pEffect, std::optional<string_vector> actions)
            : PartialPolicy(pVersion, pEffect, actions, std::nullopt) {}

        PartialPolicy(PolicyVersion pVersion, PolicyEffect pEffect, std::optional<string_vector> actions, std::optional<string_vector> resources) :
            _effect(pEffect),
            effect(_effect),
            _actions(std::move(actions)),
            _resources(std::move(resources))
        {
            if (pVersion != VERSION_1) {
                throw exception::UnknownPolicyVersionException((int) pVersion);
            }
        }

        virtual bool complete() { return false; }

        virtual std::string toJson() {
            using namespace nlohmann;

            json j = {
                    {"version", (int) this->_version},
                    {"effect",  this->_effect == ALLOW ? "Allow" : "Deny"},
            };

            if (this->_actions.has_value()) {
                j["actions"] = this->_actions.value();
            }

            if (this->_resources.has_value()) {
                j["actions"] = this->_resources.value();
            }

            return j.dump();
        }
    };
}
