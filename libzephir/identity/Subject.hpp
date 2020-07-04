#ifndef ZEPHIR_SUBJECT_HPP
#define ZEPHIR_SUBJECT_HPP

#include <functional>

#include "Role.hpp"
#include "../Policy.hpp"

class Subject : public Role {
    Policy inlinePolicy;

protected:
    explicit Subject(const Policy& policy): Role(), inlinePolicy(policy) {}
    Subject(const Policy& policy, std::vector<Policy> policies): Role(std::move(policies)), inlinePolicy(policy) {}

public:
    std::unique_ptr<AllowedResult>
    allowed(const std::optional<std::string>& action, const std::optional<std::string>& resource) override
    {
        std::vector<std::reference_wrapper<Policy>> policies = { std::ref(this->inlinePolicy) };
        for (Policy & policy : this->linkedPolicies()) {
            policies.push_back(std::ref(policy));
        }

        return this->_allowed(policies, action, resource);
    }
};

#endif //ZEPHIR_SUBJECT_HPP
