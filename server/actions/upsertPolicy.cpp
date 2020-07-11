#include "../Server.hpp"
#include "util.hpp"

namespace zephir::server {
    void Server::upsertPolicy(const Request &req, Response &res, const ContentReader &content_reader) {
        using namespace nlohmann;
        DECODE_AND_VALIDATE_JSON(j, zephir::json_schema::sPolicyUpsert, res)

        std::string id;
        libzephir::PolicyEffect e;
        std::vector<std::string> actions, resources;
        try {
            id = j["id"].get<std::string>();
            actions = j["actions"].get<std::vector<std::string>>();
            resources = j["resources"].get<std::vector<std::string>>();
            e = j["effect"].get<std::string>() == "ALLOW" ? libzephir::ALLOW : libzephir::DENY;
        } catch (json::type_error & ex) {
            invalid_request_handler("Invalid data", res);
            return;
        }

        libzephir::Policy p(libzephir::VERSION_1, id, e, actions, resources);
        this->m_manager.save(p);

        res.set_content(p.toJsonString(), "application/json");
    }
}