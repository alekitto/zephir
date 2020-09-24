namespace libzephir::storage::sql::postgres {
    namespace sql = ::sqlpp::postgresql;

    class PostgresManager : public Manager {
        sql::connection db;

        GroupTable group;
        GroupIdentityTable groupIdentity;
        GroupPolicyTable groupPolicy;
        IdentityTable identity;
        IdentityPolicyTable identityPolicy;
        PolicyTable policy;

    public:
        explicit PostgresManager(const std::string & dsn);

        std::vector<std::shared_ptr<Group>> getGroupsFor(const Identity &target) override;

        void save(const Group &g) override;
        void save(const Identity &i) override;
        void save(const Policy &p) override;

    protected:
        std::shared_ptr<Group> _findGroup(const std::string &id) override;
        std::shared_ptr<Identity> _findIdentity(const std::string &id) override;
        std::shared_ptr<Policy> _findPolicy(const std::string &id) override;
    };
}
