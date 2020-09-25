#include <gtest/gtest.h>

using namespace libzephir;
using namespace libzephir::exception;

TEST(AllowedResultTest, CanBeCreated) {
    AllowedResult r(ALLOWED, {});
    ASSERT_EQ(ALLOWED, r.outcome);
}

TEST(AllowedResultTest, AbstainOutcomeWithNoPartialsShouldOutputDenied) {
    AllowedResult r(ABSTAIN, {});
    ASSERT_EQ(DENIED, r.outcome);
}

TEST(AllowedResultTest, ShouldMergeResultsCorrectly) {
    AllowedResult r(ABSTAIN, {});

    AllowedResult r1(ABSTAIN, { PartialPolicy(VERSION_1, DENY, {"*"}, {"urn::resource1"}) });
    AllowedResult r2(ALLOWED, {});
    AllowedResult r3(ABSTAIN, { PartialPolicy(VERSION_1, DENY, {"*"}, {"urn::resource2"}) });
    AllowedResult r4(ABSTAIN, { PartialPolicy(VERSION_1, ALLOW, {"*"}, {"urn::resource4"}) });

    r.merge(r1);
    r.merge(r2);
    r.merge(r3);
    r.merge(r4);

    ASSERT_EQ(ALLOWED, r.outcome);
    ASSERT_EQ(2, r.partials.size());
}
