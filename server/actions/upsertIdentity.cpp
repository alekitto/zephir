#include "../Server.hpp"
#include "util.hpp"
#include "../../libzephir/EmptyPolicy.hpp"

namespace zephir::server {
    void Server::upsertIdentity(const Request &req, Response &res, const ContentReader &content_reader) {
        using namespace nlohmann;
        DECODE_AND_VALIDATE_JSON(j, zephir::json_schema::sIdentityUpsert, res, content_reader)

        const nlohmann::json & embed = j["inline_policy"].get<nlohmann::json>();
        const nlohmann::json & jPolicies = j["linked_policies"].get<nlohmann::json>();

        std::shared_ptr<libzephir::Policy> embeddedPolicy = nullptr;
        std::string id;
        std::vector<std::string> policies;

        try {
            id = j["id"].get<std::string>();

            if (! embed.is_null()) {
                embeddedPolicy = Server::decodePolicy(embed);
            }

            if (! jPolicies.is_null()) {
                policies = jPolicies.get<std::vector<std::string>>();
            }
        } catch (json::type_error & ex) {
            invalidRequestHandler("Invalid data", res);
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