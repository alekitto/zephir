#include <gtest/gtest.h>

using namespace libzephir;
using namespace libzephir::identity;

class ConcreteRole : public Role {
public:
    ConcreteRole(): Role() {}
    explicit ConcreteRole(std::vector<std::shared_ptr<Policy>> policies): Role(std::move(policies)) { }
};

TEST(RoleTest, CanBeCreated) {
    Role role = ConcreteRole();
    ASSERT_EQ(0, role.linkedPolicies().size());
}

TEST(RoleTest, PoliciesCanBeAdded) {
    Role role = ConcreteRole();
    role.addPolicy(std::make_shared<Policy>(VERSION_1, "RoleTestPolicy", ALLOW, string_vector{ "*" }));

    auto rtp2 = std::make_shared<Policy>(VERSION_1, "RoleTestPolicy2", ALLOW, string_vector{ "*" });
    role.addPolicy(rtp2);

    ASSERT_EQ(2, role.linkedPolicies().size());

    role.addPolicy(rtp2);
    ASSERT_EQ(2, role.linkedPolicies().size());

    ConcreteRole roleWithPolicies = ConcreteRole({
         std::make_shared<Policy>(VERSION_1, "RoleTestPolicy3", ALLOW, string_vector{ "*" })
    });

    ASSERT_EQ(1, roleWithPolicies.linkedPolicies().size());
}

TEST(RoleTest, PoliciesCanBeRemoved) {
    Role role = ConcreteRole();
    role.addPolicy(std::make_shared<Policy>(VERSION_1, "RoleTestPolicy", ALLOW, string_vector{ "*" }));

    auto rtp2 = std::make_shared<Policy>(VERSION_1, "RoleTestPolicy2", ALLOW, string_vector{ "*" });
    role.addPolicy(rtp2);

    ASSERT_EQ(2, role.linkedPolicies().size());

    role.removePolicy(rtp2);
    ASSERT_EQ(1, role.linkedPolicies().size());

    role.removePolicy("RoleTestPolicy");
    ASSERT_EQ(0, role.linkedPolicies().size());
}

TEST(RoleTest, AllowedShouldWork) {
    Role role = ConcreteRole({
        std::make_shared<Policy>(VERSION_1, "RoleTestPolicy", ALLOW, string_vector{ "TestAction" }),
        std::make_shared<Policy>(VERSION_1, "RoleTestPolicy2", DENY, string_vector{ "TestAction" }, string_vector{ "urn:resource:test-class-deny:*" })
    });

    auto result = role.allowed("TestAction", "urn:resource:test-class-allow:test-id");
    ASSERT_EQ(ALLOWED, result->outcome);
    ASSERT_EQ(0, result->partials.size());

    result = role.allowed("TestAction", "urn:resource:test-class-deny:test-id");
    ASSERT_EQ(DENIED, result->outcome);
    ASSERT_EQ(0, result->partials.size());

    result = role.allowed("FooAction", "urn:resource:test-class-deny:test-id");
    ASSERT_EQ(ABSTAIN, result->outcome);
    ASSERT_EQ(0, result->partials.size());

    result = role.allowed("TestAction", std::nullopt);
    ASSERT_EQ(ALLOWED, result->outcome);
    ASSERT_EQ(1, result->partials.size());
}
