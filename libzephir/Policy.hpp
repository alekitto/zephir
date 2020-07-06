#ifndef ZEPHIR_POLICY_HPP
#define ZEPHIR_POLICY_HPP

#include <memory>

#include "compiler/Compiler.h"
#include "MatchResult.hpp"
#include "PartialPolicy.hpp"

namespace libzephir {
    using namespace compiler;

    class Policy : public PartialPolicy {
        friend class compiler::Compiler;

        std::string _id;
        std::shared_ptr<CompiledPolicy> _compiled;

        void compile();

        void compile(Compiler &compiler);

    public:
        const PolicyVersion &version;
        const std::string &id;

        Policy &operator=(Policy p) {
            using namespace std;

            PartialPolicy::operator= (p);
            swap(_id, p._id);
            _compiled = nullptr;

            return *this;
        }

        Policy(const Policy &p) :
            PartialPolicy(p._version, p._effect, std::make_optional(p._actions.value()), std::make_optional(p._resources.value())),
            _id(p._id),
            _compiled(nullptr),
            id(_id),
            version(_version) { }

        Policy(PolicyVersion pVersion, std::string pId, PolicyEffect effect, string_vector actions, string_vector resources = {}) :
            PartialPolicy(pVersion, effect, std::make_optional(std::move(actions)), std::make_optional(std::move(resources))),
            _id(std::move(pId)),
            _compiled(nullptr),
            version(_version),
            id(_id)
        {
            if (this->_actions.value().empty()) {
                throw exception::ActionsCannotBeEmptyException();
            }

            if (this->_resources.value().empty()) {
                this->_resources.value().push_back("*");
            }
        }

        virtual std::unique_ptr<MatchResult> match(std::string &action, std::string &resource);

        virtual std::unique_ptr<MatchResult>
        match(const std::optional<std::string> &action, const std::optional<std::string> &resource);

        bool complete() override { return true; }

        std::string toJson() override {
            using namespace nlohmann;

            json j = {
                {"version",   (int) this->_version},
                {"id",        this->_id},
                {"effect",    this->_effect == ALLOW ? "Allow" : "Deny"},
                {"actions",   this->_actions.value()},
                {"resources", this->_resources.value()}
            };

            return j.dump();
        }
    };
}

#endif //ZEPHIR_POLICY_HPP
