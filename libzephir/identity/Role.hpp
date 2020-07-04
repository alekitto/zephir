#ifndef ZEPHIR_ROLE_HPP
#define ZEPHIR_ROLE_HPP

#include <memory>
#include <vector>
#include "../AllowedResult.hpp"
#include "../Policy.hpp"

class Role {
    std::vector<Policy> _linkedPolicies;

protected:
    Role(): _linkedPolicies(std::vector<Policy>()) { }
    explicit Role(std::vector<Policy> policies) :
        _linkedPolicies(std::move(policies)) { }

    std::unique_ptr<AllowedResult>
    _allowed(const std::vector<std::reference_wrapper<Policy>> & policies, const std::optional<std::string>& action, const std::optional<std::string>& resource)
    {
        AllowedOutcome outcome = ABSTAIN;
        std::vector<PartialPolicy> partials;

        for (Policy & p : policies) {
            auto result = p.match(action, resource);
            if (result->outcome == NOT_MATCH) {
                continue;
            }

            if (result->type == FULL) {
                if (p.effect == DENY) {
                    return std::make_unique<AllowedResult>(DENIED);
                } else {
                    outcome = ALLOWED;
                }

                continue;
            }

            partials.push_back(*result->getPartial());
        }

        return std::make_unique<AllowedResult>(outcome, partials);
    }

public:
    std::vector<Policy>& linkedPolicies() { return this->_linkedPolicies; };

    void addPolicy(Policy policy)
    {
        using namespace std;
        if (any_of(begin(_linkedPolicies), end(_linkedPolicies), [&policy](Policy & p) { return p.id == policy.id; })) {
            return;
        }

        _linkedPolicies.push_back(move(policy));
    }

    void removePolicy(Policy& policy) { this->removePolicy(policy.id); }
    void removePolicy(const std::string & id)
    {
        using namespace std;
        this->_linkedPolicies.erase(
            remove_if(begin(_linkedPolicies), end(_linkedPolicies), [id](Policy & p) { return p.id == id; }),
            end(_linkedPolicies)
        );
    }

    virtual std::unique_ptr<AllowedResult>
    allowed(const std::optional<std::string>& action, const std::optional<std::string>& resource)
    {
        std::vector<std::reference_wrapper<Policy>> policies = {};
        for (Policy & policy : this->_linkedPolicies) {
            policies.push_back(std::ref(policy));
        }

        return this->_allowed(policies, action, resource);
    }
};

#endif //ZEPHIR_ROLE_HPP
