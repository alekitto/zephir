#include <gtest/gtest.h>

using namespace libzephir;
using namespace libzephir::identity;

TEST(IdentityTest, CanBeCreated) {
    Identity i = Identity("Identity", std::make_shared<EmptyPolicy>());
    ASSERT_EQ(0, i.linkedPolicies().size());

    Identity i2 = Identity("IdentityTest2", std::make_shared<Policy>(VERSION_1, "TestPolicyGroup", ALLOW, string_vector{ "Action" }));
    ASSERT_EQ(0, i2.linkedPolicies().size());
}

TEST(IdentityTest, AllowShouldCheckInlinePolicy) {
    Identity i = Identity("IdentityTestAllowShouldCheckInlinePolicy", std::make_shared<Policy>(
        VERSION_1,
        "TestInlinePolicyOnIdentity",
        ALLOW,
        string_vector{"*"},
        string_vector{"urn:test-resource:id"}
    ));

    auto result = i.allowed("test:identity", std::nullopt);
    ASSERT_EQ(AllowedOutcome::ABSTAIN, result->outcome);
}
