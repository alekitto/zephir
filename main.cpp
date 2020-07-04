#include <iostream>
#include "libzephir/EmptyPolicy.hpp"
#include "libzephir/Policy.hpp"
#include "libzephir/identity/Identity.hpp"
#include "libzephir/identity/Group.hpp"

int main() {
    Policy p = Policy(VERSION_1, "AllowAll", ALLOW, { "*" });
    auto result = p.match("identity:ListIdentities", std::nullopt);

    auto i = Identity("urn:giocare::::identity:test-id", EmptyPolicy());
    i.addPolicy(p);

    auto iResult = i.allowed("identity:ListIdentities", std::nullopt);

    std::cout << p.toJson() << std::endl;
    return 0;
}
