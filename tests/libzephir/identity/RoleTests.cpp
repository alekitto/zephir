#include <gtest/gtest.h>
#include "../../../libzephir/identity/Role.hpp"

class ConcreteRole : public Role {
public:
    ConcreteRole(): Role() {}
    explicit ConcreteRole(std::vector<Policy> policies): Role(std::move(policies)) { }
};

TEST(RoleTest, CanBeCreated) {
    Role role = ConcreteRole();
    ASSERT_EQ(0, role.linkedPolicies().size());
}

TEST(RoleTest, PoliciesCanBeAdded) {
    Role role = ConcreteRole();
    role.addPolicy(Policy(VERSION_1, "RoleTestPolicy", ALLOW, { "*" }));

    Policy rtp2 = Policy(VERSION_1, "RoleTestPolicy2", ALLOW, { "*" });
    role.addPolicy(rtp2);

    ASSERT_EQ(2, role.linkedPolicies().size());

    role.addPolicy(rtp2);
    ASSERT_EQ(2, role.linkedPolicies().size());

    Role roleWithPolicies = ConcreteRole({
        Policy(VERSION_1, "RoleTestPolicy3", ALLOW, { "*" })
    });

    ASSERT_EQ(1, roleWithPolicies.linkedPolicies().size());
}

TEST(RoleTest, PoliciesCanBeRemoved) {
    Role role = ConcreteRole();
    role.addPolicy(Policy(VERSION_1, "RoleTestPolicy", ALLOW, { "*" }));

    Policy rtp2 = Policy(VERSION_1, "RoleTestPolicy2", ALLOW, { "*" });
    role.addPolicy(rtp2);

    ASSERT_EQ(2, role.linkedPolicies().size());

    role.removePolicy(rtp2);
    ASSERT_EQ(1, role.linkedPolicies().size());

    role.removePolicy("RoleTestPolicy");
    ASSERT_EQ(0, role.linkedPolicies().size());
}

TEST(RoleTest, AllowedShouldWork) {
    Role role = ConcreteRole({
        Policy(VERSION_1, "RoleTestPolicy", ALLOW, { "TestAction" }),
        Policy(VERSION_1, "RoleTestPolicy2", DENY, { "TestAction" }, { "urn:resource:test-class-deny:*" })
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
