namespace libzephir::identity {
    class Identity : public Subject {
        std::string _id;

    public:
        const std::string &id;
        Identity(std::string pId, std::shared_ptr<Policy> policy) : Subject(policy), _id(std::move(pId)), id(_id) {}

        nlohmann::json toJson() override {
            std::vector<std::string> policies;
            for (auto & p : this->linkedPolicies()) {
                policies.push_back(p->id);
            }

            return nlohmann::json{
                {"id",              this->_id},
                {"inline_policy",   this->inlinePolicy->toJson()},
                {"linked_policies", policies},
            };
        }
    };
}
