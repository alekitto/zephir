#include <iostream>
#include <sqlpp11/exception.h>
#include <spdlog/spdlog.h>

#include "libzephir/storage/Manager.hpp"
#include "server/Server.hpp"

int main() {
    using namespace libzephir;
    using namespace libzephir::storage;

    const char* logLevel = std::getenv("LOG_LEVEL");
    if (logLevel == nullptr) {
        logLevel = "debug";
    }

    spdlog::set_level(spdlog::level::from_str(logLevel));

    auto dsn = std::getenv("DSN");
    if (dsn == nullptr) {
        spdlog::critical("Database DSN is not defined");
        abort();
    }

    std::shared_ptr<Manager> manager;
    for(;;) {
        try {
            spdlog::debug(std::string("Trying to connect to ") + dsn + "...");
            manager = Manager::createManager(dsn);
            spdlog::debug("Connected.");

            break;
        } catch (sqlpp::exception &ex) {
            spdlog::debug(std::string("Connection failed: ") + ex.what());
            sleep(5);
        }
    }

    zephir::server::Server s(*manager);
    s.listen();

    return 0;
}
