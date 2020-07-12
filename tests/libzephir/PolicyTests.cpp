#include <gtest/gtest.h>
#include "../../libzephir/Policy.hpp"

using namespace libzephir;
using namespace libzephir::exception;

TEST(PolicyTest, CanBeCreated) {
    Policy p = Policy(VERSION_1, "TestPolicy", ALLOW, { "*" });
    ASSERT_EQ(ALLOW, p.effect);
}

TEST(PolicyTest, ShouldThrowIfActionsAreEmpty) {
    ASSERT_THROW(Policy(VERSION_1, "TestPolicy", DENY, { }), ActionsCannotBeEmptyException);
}

TEST(PolicyTest, ShouldThrowOnUnknownPolicyVersion) {
    ASSERT_THROW(Policy((PolicyVersion) -1, "TestPolicy", DENY, { }), UnknownPolicyVersionException);
}

TEST(PolicyTest, MatchShouldWork) {
    Policy p1 = Policy(VERSION_1, "TestPolicy1", ALLOW, { "*" });
    ASSERT_EQ(MATCH, p1.match("TestAction", "urn::resource:test")->outcome);
    ASSERT_EQ(MATCH, p1.match("FooAction", "urn::resource:test")->outcome);

    Policy p2 = Policy(VERSION_1, "TestPolicy2", ALLOW, { "*Action" });
    ASSERT_EQ(MATCH, p2.match("TestAction", "urn::resource:test")->outcome);
    ASSERT_EQ(NOT_MATCH, p2.match("FooBar", "urn::resource:test")->outcome);

    Policy p3 = Policy(VERSION_1, "TestPolicy3", ALLOW, { "Foo?ar" });
    ASSERT_EQ(NOT_MATCH, p3.match("TestAction", "urn::resource:test")->outcome);
    ASSERT_EQ(MATCH, p3.match("FooBar", "urn::resource:test")->outcome);
    ASSERT_EQ(MATCH, p3.match("FooFar", "urn::resource:test")->outcome);
    ASSERT_EQ(MATCH, p3.match("FooDar", "urn::resource:test")->outcome);

    Policy p4 = Policy(VERSION_1, "TestPolicy3", ALLOW, { "Foo?ar" });
    ASSERT_EQ(NOT_MATCH, p4.match("TestAction", "urn::resource:test")->outcome);
    ASSERT_EQ(MATCH, p4.match("FooBar", "urn::resource:test")->outcome);
    ASSERT_EQ(MATCH, p4.match("FooFar", "urn::resource:test")->outcome);
    ASSERT_EQ(MATCH, p4.match("FooDar", "urn::resource:test")->outcome);

    // Policy name is the same, should use the same compiled policy object
    Policy p5 = Policy(VERSION_1, "TestPolicy5", ALLOW, { "Test" }, { "urn::resource:test" });
    ASSERT_EQ(MATCH, p5.match("Test", "urn::resource:test")->outcome);
    ASSERT_EQ(FULL, p5.match("Test", "urn::resource:test")->type);
}

TEST(PolicyTest, ShouldMatchPartialPolicy) {
    Policy p1 = Policy(VERSION_1, "PartialPolicy1", ALLOW, { "*" });
    ASSERT_EQ(FULL, p1.match("TestAction", std::nullopt)->type);

    Policy p2 = Policy(VERSION_1, "PartialPolicy2", ALLOW, { "TestAction" }, { "urn:resource:test" });
    ASSERT_EQ(FULL, p2.match("NoAction", std::nullopt)->type);

    auto p2Result = p2.match("TestAction", std::nullopt);
    ASSERT_EQ(PARTIAL, p2Result->type);
    ASSERT_EQ("{\"effect\":\"ALLOW\",\"resources\":[\"urn:resource:test\"],\"version\":1}", p2Result->getPartial()->toJsonString());
}
