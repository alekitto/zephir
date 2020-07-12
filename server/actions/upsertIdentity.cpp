#include "../Server.hpp"
#include "util.hpp"
#include "../../libzephir/EmptyPolicy.hpp"

namespace zephir::server {
    std::shared_ptr<libzephir::Policy>
    decode_policy(const nlohmann::json & j) {
        libzephir::PolicyEffect e;
        std::vector<std::string> actions, resources;

        actions = j["actions"].get<std::vector<std::string>>();
        resources = j["resources"].get<std::vector<std::string>>();
        e = j["effect"].get<std::string>() == "ALLOW" ? libzephir::ALLOW : libzephir::DENY;

        return std::make_shared<libzephir::Policy>(libzephir::VERSION_1, "", e, actions, resources);
    }

    void Server::upsertIdentity(const Request &req, Response &res, const ContentReader &content_reader) {
        using namespace nlohmann;
        DECODE_AND_VALIDATE_JSON(j, zephir::json_schema::sIdentityUpsert, res)

        const nlohmann::json & embed = j["inline_policy"].get<nlohmann::json>();

        std::shared_ptr<libzephir::Policy> embeddedPolicy = nullptr;
        std::string id;
        std::vector<std::string> policies;

        try {
            id = j["id"].get<std::string>();
            policies = j["linked_policies"].get<std::vector<std::string>>();

            if (! embed.is_null()) {
                embeddedPolicy = decode_policy(embed);
            }
        } catch (json::type_error & ex) {
            invalid_request_handler("Invalid data", res);
            return;
        }

        libzephir::Identity i(id, embeddedPolicy != nullptr ? embeddedPolicy : std::make_shared<libzephir::EmptyPolicy>());
        for (auto & policyId : policies) {
            auto p = this->m_manager.getPolicy(policyId);
            if (nullptr != p) {
                i.addPolicy(p);
            }
        }

        this->m_manager.save(i);

        res.set_content(i.toJsonString(), "application/json");
    }
}