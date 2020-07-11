#pragma once

#include <memory>
#include "Policy.hpp"

namespace libzephir {
    class EmptyPolicy : public Policy {
    public:
        EmptyPolicy() : Policy(VERSION_1, "", DENY, {""}) {}

        std::unique_ptr<MatchResult>
        match(const std::optional<std::string> &action, const std::optional<std::string> &resource) override {
            auto result = std::make_unique<MatchResult>(*this);
            result->resource(false);
            result->resource(false);
            result->_update();

            return result;
        }

        bool complete() override { return false; }

        std::string toJson() override {
            return "null";
        }
    };
}
