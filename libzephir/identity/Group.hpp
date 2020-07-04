#ifndef ZEPHIR_GROUP_HPP
#define ZEPHIR_GROUP_HPP

#include <string>
#include <utility>
#include "Identity.hpp"

class Group : public Subject {
    std::string m_name;
    std::vector<std::shared_ptr<Identity>> m_identities;

public:
    Group(std::string name, const Policy & policy) :
        Subject(policy),
        m_name(std::move(name)),
        m_identities({}) { }

    const std::vector<std::shared_ptr<Identity>> &
    getIdentities() {
        return this->m_identities;
    }

    void addIdentity(const std::shared_ptr<Identity>& identity)
    {
        using namespace std;
        auto l = [&identity](const std::shared_ptr<Identity>& i) { return i->id == identity->id; };
        if (any_of(begin(this->m_identities), end(this->m_identities), l)) {
            return;
        }

        this->m_identities.push_back(identity);
    }

    void removeIdentity(Identity& identity) { this->removeIdentity(identity.id); }
    void removeIdentity(const std::string& id)
    {
        using namespace std;

        auto l = [id](const shared_ptr<Identity> & i) { return i->id == id; };
        this->m_identities.erase(
            remove_if(begin(m_identities), end(m_identities), l),
            end(m_identities)
        );
    }
};

#endif //ZEPHIR_GROUP_HPP
