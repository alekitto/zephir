#include <gtest/gtest.h>
#include "../../../libzephir/identity/Group.hpp"
#include "../../../libzephir/EmptyPolicy.hpp"

TEST(GroupTest, CanBeCreated) {
    Group g = Group("Group", EmptyPolicy());
    ASSERT_EQ(0, g.getIdentities().size());

    Group g2 = Group("Group", Policy(VERSION_1, "TestPolicyGroup", ALLOW, { "Action" }));
    ASSERT_EQ(0, g.getIdentities().size());
}

TEST(GroupTest, IdentitiesCanBeAdded) {
    Group g = Group("Group", EmptyPolicy());
    auto i = std::make_shared<Identity>("TestIdentity", EmptyPolicy());
    g.addIdentity(i);
    g.addIdentity(i);

    ASSERT_EQ(1, g.getIdentities().size());
}

TEST(GroupTest, IdentitiesCanBeRemoved) {
    Group g = Group("Group", EmptyPolicy());
    auto i = std::make_shared<Identity>("TestIdentity", EmptyPolicy());
    auto i2 = std::make_shared<Identity>("TestIdentity2", EmptyPolicy());

    g.addIdentity(i);
    g.addIdentity(i2);
    ASSERT_EQ(2, g.getIdentities().size());

    g.removeIdentity(*i);
    ASSERT_EQ(1, g.getIdentities().size());

    g.removeIdentity(i->id);
    ASSERT_EQ(1, g.getIdentities().size());

    g.removeIdentity(i2->id);
    ASSERT_EQ(0, g.getIdentities().size());
}
