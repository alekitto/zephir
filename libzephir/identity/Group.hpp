#pragma once

#include <string>
#include <utility>
#include "Identity.hpp"

namespace libzephir::identity {
    class Group : public Subject {
        std::string m_name;
        std::vector<std::shared_ptr<Identity>> m_identities;

    public:
        const std::string &name;

        Group(std::string name, std::shared_ptr<Policy> policy) :
            Subject(policy),
            m_name(std::move(name)),
            name(m_name),
            m_identities({}) {}

        const std::vector<std::shared_ptr<Identity>> &
        getIdentities() {
            return this->m_identities;
        }

        void addIdentity(const std::shared_ptr<Identity> &identity) {
            using namespace std;
            auto l = [&identity](const std::shared_ptr<Identity> &i) { return i->id == identity->id; };
            if (any_of(begin(this->m_identities), end(this->m_identities), l)) {
                return;
            }

            this->m_identities.push_back(identity);
        }

        void removeIdentity(const std::shared_ptr<Identity> &identity) { this->removeIdentity(identity->id); }
        void removeIdentity(Identity &identity) { this->removeIdentity(identity.id); }
        void removeIdentity(const std::string &id) {
            using namespace std;

            auto l = [id](const shared_ptr<Identity> &i) { return i->id == id; };
            this->m_identities.erase(
                    remove_if(begin(m_identities), end(m_identities), l),
                    end(m_identities)
            );
        }

        nlohmann::json toJson() override {
            std::vector<std::string> identities;
            for (auto & i : this->m_identities) {
                identities.push_back(i->id);
            }

            std::vector<std::string> policies;
            for (auto & p : this->linkedPolicies()) {
                policies.push_back(p->id);
            }

            return nlohmann::json{
                {"id",              this->m_name},
                {"members",         identities},
                {"inline_policy",   this->inlinePolicy->toJson()},
                {"linked_policies", policies},
            };
        }
    };
}
