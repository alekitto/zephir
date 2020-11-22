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

    auto result1 = i.allowed("test:identity", "urn:test-resource:id");
    ASSERT_EQ(AllowedOutcome::ALLOWED, result1->outcome);
    ASSERT_EQ(0, result1->partials.size());

    auto result2 = i.allowed("test:identity", std::nullopt);
    ASSERT_EQ(AllowedOutcome::ABSTAIN, result2->outcome);
    ASSERT_EQ(1, result2->partials.size());
}

TEST(IdentityTest, ShouldCheckInlineAndLinkedPolicies) {
    Identity i = Identity("IdentityTestShouldCheckInlineAndLinkedPolicies", std::make_shared<Policy>(
        VERSION_1,
        "TestInlinePolicyOnIdentity",
        ALLOW,
        string_vector{"test:not-identity"},
        string_vector{"urn:test-resource:id"}
    ));

    i.addPolicy(std::make_shared<Policy>(
        VERSION_1,
        "TestLinkedPolicyOnIdentity",
        ALLOW,
        string_vector{"test:identity"},
        string_vector{"*"}
    ));

    auto result = i.allowed("test:identity", "urn:test:zephir:identity");
    ASSERT_EQ(AllowedOutcome::ALLOWED, result->outcome);
}
