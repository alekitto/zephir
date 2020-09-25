namespace libzephir {
    enum AllowedOutcome {
        DENIED = -1,
        ABSTAIN = 0,
        ALLOWED = 1,
    };

    class AllowedResult {
        friend class outcome;
        AllowedOutcome _outcome;
        std::vector<PartialPolicy> _partials;

    public:
        const class outcome {
            const AllowedOutcome & value;
            const AllowedResult & result;

            [[nodiscard]] libzephir::AllowedOutcome computeOutcome() const {
                auto rOutcome = this->result._outcome;
                if (ABSTAIN == rOutcome && this->result._partials.empty()) {
                    rOutcome = DENIED;
                }

                return rOutcome;
            }

            public:
                outcome(const libzephir::AllowedOutcome & v, const libzephir::AllowedResult & r) : value(v), result(r) {}

                operator libzephir::AllowedOutcome () const { return this->computeOutcome(); }
                operator int () const { return (int) this->computeOutcome(); }
                operator long long int () const { return (long long int) this->computeOutcome(); }

                bool operator == (libzephir::AllowedOutcome other) const {
                    return this->computeOutcome() == other;
                }
        } outcome;
        const std::vector<PartialPolicy> &partials;

        AllowedResult(AllowedOutcome pOutcome) : AllowedResult(pOutcome, {}) {}
        AllowedResult(AllowedOutcome pOutcome, std::vector<PartialPolicy> partials) :
            _outcome(pOutcome),
            _partials(partials),
            outcome(_outcome, *this),
            partials(_partials) {}

        std::string toJsonString() {
            using namespace nlohmann;

            std::vector<nlohmann::json> partialsJson;
            for (auto & partial : this->_partials) {
                partialsJson.push_back(partial.toJson());
            }

            auto rOutcome = (const AllowedOutcome) this->outcome;
            json j = {
                {"outcome",  rOutcome == ALLOWED ? "ALLOWED" : rOutcome == ABSTAIN ? "ABSTAIN" : "DENIED"},
                {"partials", partialsJson}
            };

            return j.dump();
        }

        void merge(const AllowedResult & other) {
            if (other._outcome == DENIED) {
                this->_outcome = DENIED;
                this->_partials = {};
            }

            if (this->_outcome == DENIED) {
                return;
            }

            if (other._outcome == ALLOWED) {
                this->_outcome = ALLOWED;
            }

            this->_partials.insert(this->_partials.end(), std::begin(other._partials), std::end(other._partials));
            if (this->_outcome == ALLOWED) {
                this->_partials.erase(
                    std::remove_if(std::begin(this->_partials), std::end(this->_partials), [](const PartialPolicy & policy) {
                        return policy.effect != DENY;
                    }),
                    std::end(this->_partials)
                );
            }
        }
    };
}
