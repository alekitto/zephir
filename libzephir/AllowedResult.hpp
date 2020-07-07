#ifndef ZEPHIR_ALLOWEDRESULT_HPP
#define ZEPHIR_ALLOWEDRESULT_HPP

#include <vector>

#include "Policy.hpp"

namespace libzephir {
    enum AllowedOutcome {
        DENIED = -1,
        ABSTAIN = 0,
        ALLOWED = 1,
    };

    class AllowedResult {
        AllowedOutcome _outcome;
        std::vector<PartialPolicy> _partials;

    public:
        const AllowedOutcome &outcome;
        const std::vector<PartialPolicy> &partials;

        AllowedResult(AllowedOutcome pOutcome) : AllowedResult(pOutcome, {}) {}
        AllowedResult(AllowedOutcome pOutcome, std::vector<PartialPolicy> partials) :
            _outcome(pOutcome),
            _partials(partials),
            outcome(_outcome),
            partials(_partials) {}

        std::string toJson() {
            using namespace nlohmann;

            std::vector<std::string> partialsJson;
            for (auto & partial : this->_partials) {
                partialsJson.push_back(partial.toJson());
            }

            auto rOutcome = this->_outcome;
            if (ABSTAIN == rOutcome && this->partials.size() == 0) {
                rOutcome = DENIED;
            }

            json j = {
                {"outcome", rOutcome == ALLOWED ? "Allowed" : rOutcome == ABSTAIN ? "Abstain" : "Denied"},
                {"partials", partialsJson}
            };

            return j.dump();
        }

        void merge(const AllowedResult & other) {
            if (other._outcome == DENIED) {
                this->_outcome = DENIED;
                this->_partials = {};
                return;
            }

            if (this->_outcome != ABSTAIN) {
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

#endif //ZEPHIR_ALLOWEDRESULT_HPP
