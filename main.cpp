#include <iostream>
#include "libzephir/EmptyPolicy.hpp"
#include "libzephir/Policy.hpp"
#include "libzephir/identity/Identity.hpp"
#include "libzephir/identity/Group.hpp"
#include "libzephir/storage/Manager.hpp"
#include "server/Server.hpp"

int main() {
    using namespace libzephir;
    using namespace libzephir::storage;
    using namespace zephir::server;

    auto manager = Manager::createManager("postgres://postgres:postgres@127.0.0.1/zephir");
//    auto id = manager->getIdentity("urn:giocaresport::::identity:test-id");
//    auto idResult = id->allowed("identity:List", "*");
//
//    auto groups = manager->getGroupsFor(*id);
//    for (auto & g : groups) {
//        idResult->merge(*g->allowed("identity:List", "*"));
//    }

//
//    Policy p = Policy(VERSION_1, "AllowAll", ALLOW, { "*" });
//    auto result = p.match("identity:ListIdentities", std::nullopt);
//
//    auto i = identity::Identity("urn:giocare::::identity:test-id", EmptyPolicy());
//    i.addPolicy(p);
//
//    auto iResult = i.allowed("identity:ListIdentities", std::nullopt);

//    std::cout << (idResult->outcome ? "ALLOWED" : "DENINED") << std::endl;

    Server s(*manager);
    s.listen();

    return 0;
}
