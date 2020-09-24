#include <stdafx.h>

namespace libzephir {
    void Policy::compile() {
        this->compile(Compiler::getInstance());
    }

    void Policy::compile(Compiler &compiler) {
        if (nullptr != this->_compiled) {
            return;
        }

        this->_compiled = compiler.compile(*this);
    }

    std::unique_ptr<MatchResult> Policy::match(std::string &action, std::string &resource) {
        return this->match(std::make_optional(action), std::make_optional(resource));
    }

    std::unique_ptr<MatchResult>
    Policy::match(const std::optional<std::string> &action, const std::optional<std::string> &resource) {
        auto result = std::make_unique<MatchResult>(*this);
        this->compile();

        if (action.has_value()) {
            auto r = this->_compiled->matchAction(action.value());
            result->action(r);

            if (!r) goto fail;
        }

        {
            auto m = this->_compiled->matchResource(resource);
            if ((bool) m) {
                result->resource(m.value());
                if (!m.value()) goto fail;
            }
        }

    fail:
        result->_update();

        return result;
    }
}
