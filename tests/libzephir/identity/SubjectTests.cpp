#include <gtest/gtest.h>

using namespace libzephir;
using namespace libzephir::identity;

class ConcreteSubject : public Subject {
public:
    explicit ConcreteSubject(std::shared_ptr<Policy> policy): Subject(policy) {}
    ConcreteSubject(std::shared_ptr<Policy> policy, std::vector<std::shared_ptr<Policy>> policies): Subject(policy, std::move(policies)) { }

    nlohmann::json toJson() override {
        return nlohmann::json();
    }
};

TEST(SubjectTest, CanBeCreated) {
    ConcreteSubject subject = ConcreteSubject(std::make_shared<EmptyPolicy>());
    ASSERT_EQ(0, subject.linkedPolicies().size());
}

TEST(SubjectTest, PoliciesCanBeAdded) {
    ConcreteSubject subject = ConcreteSubject(std::make_shared<EmptyPolicy>());
    subject.addPolicy(std::make_shared<Policy>(VERSION_1, "RoleTestPolicy", ALLOW, string_vector{"*"}));

    auto rtp2 = std::make_shared<Policy>(VERSION_1, "RoleTestPolicy2", ALLOW, string_vector{ "*" });
    subject.addPolicy(rtp2);

    ASSERT_EQ(2, subject.linkedPolicies().size());

    subject.addPolicy(rtp2);
    ASSERT_EQ(2, subject.linkedPolicies().size());

    ConcreteSubject subjectWithPolicies = ConcreteSubject(std::make_shared<EmptyPolicy>(), {
        std::make_shared<Policy>(VERSION_1, "RoleTestPolicy3", ALLOW, string_vector{ "*" })
    });

    ASSERT_EQ(1, subjectWithPolicies.linkedPolicies().size());

    ConcreteSubject subjectWithInlinePolicy = ConcreteSubject(
        std::make_shared<Policy>(VERSION_1, "RoleTestPolicy3", ALLOW, string_vector{ "*" })
    );
    ASSERT_EQ(0, subjectWithInlinePolicy.linkedPolicies().size());
}

TEST(SubjectTest, PoliciesCanBeRemoved) {
    ConcreteSubject subject = ConcreteSubject(std::make_shared<EmptyPolicy>());
    subject.addPolicy(std::make_shared<Policy>(VERSION_1, "RoleTestPolicy", ALLOW, string_vector{"*" }));

    auto rtp2 = std::make_shared<Policy>(VERSION_1, "RoleTestPolicy2", ALLOW, string_vector{ "*" });
    subject.addPolicy(rtp2);

    ASSERT_EQ(2, subject.linkedPolicies().size());

    subject.removePolicy(rtp2);
    ASSERT_EQ(1, subject.linkedPolicies().size());

    subject.removePolicy("RoleTestPolicy");
    ASSERT_EQ(0, subject.linkedPolicies().size());
}

TEST(SubjectTest, AllowedShouldWork) {
    ConcreteSubject subject = ConcreteSubject(std::make_shared<EmptyPolicy>(), {
        std::make_shared<Policy>(VERSION_1, "RoleTestPolicy", ALLOW, string_vector{ "TestAction" }),
        std::make_shared<Policy>(VERSION_1, "RoleTestPolicy2", DENY, string_vector{ "TestAction" }, string_vector{ "urn:resource:test-class-deny:*" })
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
