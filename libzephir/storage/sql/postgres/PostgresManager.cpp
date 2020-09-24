#include <stdafx.h>

using namespace libzephir::storage::sql::postgres;
using namespace libzephir;

std::shared_ptr<::sqlpp::postgresql::connection_config>
parseDsn(const std::string & dsn) {
    UriUriA uri;
    const char * errorPos;
    if (URI_SUCCESS != uriParseSingleUriA(&uri, dsn.c_str(), &errorPos)) {
        throw storage::exception::InvalidDsnException();
    }

    auto config = std::make_shared<::sqlpp::postgresql::connection_config>();
    if (uri.hostText.first != NULL) {
        config->host = std::string(uri.hostText.first)
            .substr(0,uri.hostText.afterLast - uri.hostText.first);
    } else {
        throw storage::exception::InvalidDsnException();
    }

    char *pEnd;
    config->port = uri.portText.first != NULL ? strtol(uri.portText.first, &pEnd, 10) : 5432;

    if (uri.userInfo.first != NULL) {
        std::string userInfo = std::string(uri.userInfo.first).substr(0, uri.userInfo.afterLast - uri.userInfo.first);
        std::string::size_type delimiterPos = userInfo.find(':', 0);

        std::string username = std::string::npos != delimiterPos ? userInfo.substr(0, delimiterPos) : userInfo;
        std::string password = std::string::npos == delimiterPos ? "" : userInfo.substr(delimiterPos + 1, userInfo.length() - delimiterPos - 1);

        config->user = username;
        config->password = password;
    }

    auto dbname = uri.pathHead->text;
    config->dbname = std::string(dbname.first).substr(0, dbname.afterLast - dbname.first);

    return config;
}

PostgresManager::PostgresManager(const std::string &dsn) :
    db(sql::connection(parseDsn(dsn))),
    group(),
    groupIdentity(),
    groupPolicy(),
    identity(),
    identityPolicy(),
    policy()
{ }

std::vector<std::shared_ptr<Group>> PostgresManager::getGroupsFor(const Identity &target) {
    std::vector<std::shared_ptr<Group>> result;
    lock::Guard g(this->m_lock);

    auto cacheItem = this->m_cache.groupsPerIdentity.get(target.id);
    if (cacheItem.has_value()) {
        for (const auto &gId : cacheItem.value()) {
            result.push_back(this->getGroup(gId));
        }
    } else {
        auto rows = this->db(
            ::sqlpp::select(groupIdentity.group_id)
                .flags(::sqlpp::distinct)
                .from(groupIdentity)
                .where(groupIdentity.identity_id == target.id)
        );

        for (const auto &row : rows) {
            result.push_back(this->getGroup(row.group_id));
        }
    }

    return std::move(result);
}

std::shared_ptr<Group> PostgresManager::_findGroup(const std::string &id) {
    auto rows = this->db(
        ::sqlpp::select(::sqlpp::all_of(group))
            .from(group)
            .where(group.id == id)
    );

    if (rows.empty()) {
        return nullptr;
    }

    auto & row = rows.front();
    auto embeddedPolicy = row.policy_id.is_null() ? std::make_shared<EmptyPolicy>() : this->getPolicy(row.policy_id.value());

    auto g = std::make_shared<Group>(row.id.value(), embeddedPolicy);
    for (auto & pRow : this->db(
        ::sqlpp::select(groupPolicy.policy_id)
            .from(groupPolicy)
            .where(groupPolicy.group_id == g->name)
    )) {
        auto p = this->getPolicy(pRow.policy_id);
        if (p != nullptr) {
            g->addPolicy(p);
        }
    }

    for (auto & iRow : this->db(
        ::sqlpp::select(groupIdentity.identity_id)
            .from(groupIdentity)
            .where(groupIdentity.group_id == g->name)
    )) {
        auto pIdentity = this->getIdentity(iRow.identity_id);
        if (pIdentity != nullptr) {
            g->addIdentity(pIdentity);
        }
    }

    this->m_cache.groups.insert(id, g);
    return g;
}

std::shared_ptr<Identity> PostgresManager::_findIdentity(const std::string &id) {
    auto rows = this->db(
        ::sqlpp::select(::sqlpp::all_of(identity))
            .from(identity)
            .where(identity.id == id)
    );

    if (rows.empty()) {
        return nullptr;
    }

    auto & row = rows.front();
    auto embeddedPolicy = row.policy_id.is_null() ? std::make_shared<EmptyPolicy>() : this->getPolicy(row.policy_id.value());

    auto i = std::make_shared<Identity>(row.id.value(), embeddedPolicy);
    for (auto & pRow : this->db(
        ::sqlpp::select(identityPolicy.policy_id)
            .from(identityPolicy)
            .where(identityPolicy.identity_id == i->id)
    )) {
        auto p = this->getPolicy(pRow.policy_id);
        if (p != nullptr) {
            i->addPolicy(p);
        }
    }

    this->m_cache.identities.insert(id, i);
    return i;
}

std::shared_ptr<Policy> PostgresManager::_findPolicy(const std::string &id) {
    using namespace nlohmann;

    auto rows = this->db(
        ::sqlpp::select(::sqlpp::all_of(policy))
            .from(policy)
            .where(policy.id == id)
    );

    if (rows.empty()) {
        return nullptr;
    }

    auto & row = rows.front();
    auto actions = json::parse((std::string) row.actions).get<string_vector>();
    auto resources = json::parse((std::string) row.resources).get<string_vector>();

    auto p = std::make_shared<Policy>((PolicyVersion) ((long long) row.version), (std::string) row.id, (PolicyEffect) ((long long) row.effect), actions, resources);
    this->m_cache.policies.insert(p->id, p);

    return p;
}

void PostgresManager::save(const Group &g) {
    std::string embedded_policy_id = "__embedded_policy_group_" + g.name + "__";
    const Policy & embeddedPolicy = g.getInlinePolicy();

    this->db.start_transaction();
    if (embeddedPolicy.complete()) {
        Policy persistingPolicy(embeddedPolicy.version, embedded_policy_id, embeddedPolicy.effect,
            embeddedPolicy.actions(), embeddedPolicy.resources());
        this->save(persistingPolicy);
    } else {
        this->db(::sqlpp::remove_from(policy).where(policy.id == embedded_policy_id));
    }

    auto row = this->db(::sqlpp::select(group.id).from(group).where(group.id == g.name));
    if (! row.empty()) {
        auto update = ::sqlpp::update(group)
            .where(group.id == g.name);

        if (embeddedPolicy.complete()) {
            this->db(update.set(group.policy_id = embedded_policy_id));
        } else {
            this->db(update.set(group.policy_id = ::sqlpp::null));
        }
    } else {
        auto insert = ::sqlpp::insert_into(group).columns(group.id, group.policy_id);
        if (embeddedPolicy.complete()) {
            insert.values.add(group.id = g.name, group.policy_id = embedded_policy_id);
        } else {
            insert.values.add(group.id = g.name, group.policy_id = ::sqlpp::null);
        }

        this->db(insert);
    }

    this->db(::sqlpp::remove_from(groupPolicy).where(groupPolicy.group_id == g.name));
    for (auto & p : g.linkedPolicies()) {
        this->db(::sqlpp::insert_into(groupPolicy)
            .set(groupPolicy.group_id = g.name, groupPolicy.policy_id = p->id)
        );
    }

    this->db(::sqlpp::remove_from(groupIdentity).where(groupIdentity.group_id == g.name));
    for (auto & m : g.getIdentities()) {
        this->db(::sqlpp::insert_into(groupIdentity)
             .set(groupIdentity.group_id = g.name, groupIdentity.identity_id = m->id)
        );
    }

    this->db.commit_transaction();

    this->m_cache.groups.clear();
    this->m_cache.groupsPerIdentity.clear();
    Compiler::getInstance().clearCache();
}

void PostgresManager::save(const Identity &i) {
    std::string embedded_policy_id = "__embedded_policy_identity_" + i.id + "__";
    const Policy & embeddedPolicy = i.getInlinePolicy();

    this->db.start_transaction();
    if (embeddedPolicy.complete()) {
        Policy persistingPolicy(embeddedPolicy.version, embedded_policy_id, embeddedPolicy.effect, embeddedPolicy.actions(), embeddedPolicy.resources());
        this->save(persistingPolicy);
    } else {
        this->db(::sqlpp::update(identity).set(identity.policy_id = ::sqlpp::null).where(identity.id == i.id));
        this->db(::sqlpp::remove_from(policy).where(policy.id == embedded_policy_id));
    }

    auto row = this->db(::sqlpp::select(identity.id).from(identity).where(identity.id == i.id));
    if (! row.empty()) {
        auto update = ::sqlpp::update(identity)
            .where(identity.id == i.id);

        if (embeddedPolicy.complete()) {
            this->db(update.set(identity.policy_id = embedded_policy_id));
        } else {
            this->db(update.set(identity.policy_id = ::sqlpp::null));
        }
    } else {
        auto insert = ::sqlpp::insert_into(identity).columns(identity.id, identity.policy_id);
        if (embeddedPolicy.complete()) {
            insert.values.add(identity.id = i.id, identity.policy_id = embedded_policy_id);
        } else {
            insert.values.add(identity.id = i.id, identity.policy_id = ::sqlpp::null);
        }

        this->db(insert);
    }

    this->db(::sqlpp::remove_from(identityPolicy).where(identityPolicy.identity_id == i.id));
    for (auto & p : i.linkedPolicies()) {
        this->db(::sqlpp::insert_into(identityPolicy)
            .set(identityPolicy.identity_id = i.id, identityPolicy.policy_id = p->id)
        );
    }

    this->db.commit_transaction();

    this->m_cache.identities.clear();
    Compiler::getInstance().clearCache();
}

void PostgresManager::save(const Policy &p) {
    nlohmann::json jActions(p.actions());
    nlohmann::json jResources(p.resources());

    auto row = this->db(::sqlpp::select(policy.id).from(policy).where(policy.id == p.id));
    if (! row.empty()) {
        this->db(::sqlpp::update(policy)
            .set(policy.effect = p.effect == ALLOW, policy.id = p.id,
                policy.actions = jActions.dump(), policy.resources = jResources.dump())
            .where(policy.id == p.id));
    } else {
        this->db(::sqlpp::insert_into(policy)
            .set(policy.effect = p.effect == ALLOW, policy.id = p.id,
                policy.actions = jActions.dump(), policy.resources = jResources.dump()));
    }

    this->m_cache.policies.clear();
    Compiler::getInstance().clearCache();
}
