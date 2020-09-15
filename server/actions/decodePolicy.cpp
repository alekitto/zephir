#include "../Server.hpp"
#include "../../libzephir/EmptyPolicy.hpp"

namespace zephir::server {
    std::shared_ptr<libzephir::Policy>
    Server::decodePolicy(const nlohmann::json &j) {
        libzephir::PolicyEffect e;
        std::vector<std::string> actions, resources;

        actions = j["actions"].get<std::vector<std::string>>();
        resources = j["resources"].get<std::vector<std::string>>();
        e = j["effect"].get<std::string>() == "ALLOW" ? libzephir::ALLOW : libzephir::DENY;

        return std::make_shared<libzephir::Policy>(libzephir::VERSION_1, "", e, actions, resources);
    }
}