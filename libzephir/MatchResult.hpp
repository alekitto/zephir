namespace libzephir {
    class Policy;

    enum ResultType {
        PARTIAL,
        FULL,
    };

    enum ResultOutcome {
        MATCH = 0,
        NOT_MATCH = 1,
    };

    class MatchResult {
        ResultType _type = PARTIAL;
        ResultOutcome _outcome = NOT_MATCH;

        Policy &m_policy;
        std::unique_ptr<PartialPolicy> m_partial = nullptr;

        std::optional<bool> m_action;
        std::optional<bool> m_resource;

    public:
        const ResultOutcome &outcome;
        const ResultType &type;

        explicit MatchResult(Policy &p) : outcome(this->_outcome), type(this->_type), m_policy(p) {}

        void action(bool r) { m_action = r; }
        void resource(bool r) { m_resource = r; }
        std::unique_ptr<PartialPolicy> &getPartial() { return this->m_partial; }

        void _update();
    };
}
