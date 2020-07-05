#include <gtest/gtest.h>
#include "../../../libzephir/identity/Subject.hpp"
#include "../../../libzephir/EmptyPolicy.hpp"

using namespace libzephir;
using namespace libzephir::identity;

class ConcreteSubject : public Subject {
public:
    explicit ConcreteSubject(const Policy & policy): Subject(policy) {}
    ConcreteSubject(const Policy & policy, std::vector<Policy> policies): Subject(policy, std::move(policies)) { }
};

TEST(SubjectTest, CanBeCreated) {
    Subject subject = ConcreteSubject(EmptyPolicy());
    ASSERT_EQ(0, subject.linkedPolicies().size());
}

TEST(SubjectTest, PoliciesCanBeAdded) {
    Subject subject = ConcreteSubject(EmptyPolicy());
    subject.addPolicy(Policy(VERSION_1, "RoleTestPolicy", ALLOW, {"*" }));

    Policy rtp2 = Policy(VERSION_1, "RoleTestPolicy2", ALLOW, { "*" });
    subject.addPolicy(rtp2);

    ASSERT_EQ(2, subject.linkedPolicies().size());

    subject.addPolicy(rtp2);
    ASSERT_EQ(2, subject.linkedPolicies().size());

    Subject subjectWithPolicies = ConcreteSubject(EmptyPolicy(), {
        Policy(VERSION_1, "RoleTestPolicy3", ALLOW, { "*" })
    });

    ASSERT_EQ(1, subjectWithPolicies.linkedPolicies().size());

    Subject subjectWithInlinePolicy = ConcreteSubject(
        Policy(VERSION_1, "RoleTestPolicy3", ALLOW, { "*" })
    );
    ASSERT_EQ(0, subjectWithInlinePolicy.linkedPolicies().size());
}

TEST(SubjectTest, PoliciesCanBeRemoved) {
    Subject subject = ConcreteSubject(EmptyPolicy());
    subject.addPolicy(Policy(VERSION_1, "RoleTestPolicy", ALLOW, {"*" }));

    Policy rtp2 = Policy(VERSION_1, "RoleTestPolicy2", ALLOW, { "*" });
    subject.addPolicy(rtp2);

    ASSERT_EQ(2, subject.linkedPolicies().size());

    subject.removePolicy(rtp2);
    ASSERT_EQ(1, subject.linkedPolicies().size());

    subject.removePolicy("RoleTestPolicy");
    ASSERT_EQ(0, subject.linkedPolicies().size());
}

TEST(SubjectTest, AllowedShouldWork) {
    Subject subject = ConcreteSubject(EmptyPolicy(), {
        Policy(VERSION_1, "RoleTestPolicy", ALLOW, { "TestAction" }),
        Policy(VERSION_1, "RoleTestPolicy2", DENY, { "TestAction" }, { "urn:resource:test-class-deny:*" })
    });

    auto result = subject.allowed("TestAction", "urn:resource:test-class-allow:test-id");
    ASSERT_EQ(ALLOWED, result->outcome);
    ASSERT_EQ(0, result->partials.size());

    result = subject.allowed("TestAction", "urn:resource:test-class-deny:test-id");
    ASSERT_EQ(DENIED, result->outcome);
    ASSERT_EQ(0, result->partials.size());

    result = subject.allowed("FooAction", "urn:resource:test-class-deny:test-id");
    ASSERT_EQ(ABSTAIN, result->outcome);
    ASSERT_EQ(0, result->partials.size());

    result = subject.allowed("TestAction", std::nullopt);
    ASSERT_EQ(ALLOWED, result->outcome);
    ASSERT_EQ(1, result->partials.size());
}
