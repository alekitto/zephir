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

    auto dsn = std::getenv("DSN");
    if (dsn == nullptr) {
        std::cerr << "Database DSN is not defined" << std::endl;
        abort();
    }

    auto manager = Manager::createManager(dsn);

    zephir::server::Server s(*manager);
    s.listen();

    return 0;
}
