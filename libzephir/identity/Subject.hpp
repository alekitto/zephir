namespace libzephir::identity {
    class Subject : public Role {
    protected:
        std::shared_ptr<Policy> inlinePolicy;

        explicit Subject(std::shared_ptr<Policy> policy) : Role(), inlinePolicy(policy) {}
        Subject(std::shared_ptr<Policy> policy, std::vector<std::shared_ptr<Policy>> policies) : Role(std::move(policies)), inlinePolicy(policy) {}

    public:
        const Policy & getInlinePolicy() const { return *this->inlinePolicy; }

        std::unique_ptr<AllowedResult>
        allowed(const std::optional<std::string> &action, const std::optional<std::string> &resource) override {
            std::vector<std::reference_wrapper<Policy>> policies = { std::ref(*this->inlinePolicy) };
            for (std::shared_ptr<Policy> &policy : this->linkedPolicies()) {
                policies.push_back(std::ref(*policy));
            }

            return this->_allowed(policies, action, resource);
        }

        virtual nlohmann::json toJson() override = 0;
        virtual std::string toJsonString() {
            return this->toJson().dump();
        }
    };
}
