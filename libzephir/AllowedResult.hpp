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
    };
}

#endif //ZEPHIR_ALLOWEDRESULT_HPP
