#include "Manager.hpp"
#include "sql/postgres/PostgresManager.hpp"

namespace libzephir::storage {
    std::shared_ptr<Manager>
    Manager::createManager(const std::string & dsn) {
        if (std::string::npos != dsn.rfind("postgres", 0)) {
            return std::make_shared<sql::postgres::PostgresManager>(dsn);
        }

        throw ::libzephir::exception::UnsupportedStorageDsn();
    }
}
