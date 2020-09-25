#include "../stdafx.hpp"

namespace zephir::server {
    void Server::allowedAction(const Request &req, Response &res, const ContentReader &content_reader) {
        using namespace nlohmann;
        DECODE_AND_VALIDATE_JSON(j, zephir::json_schema::sAllowed, res, content_reader);

        std::optional<std::string> resource(std::nullopt);
        try {
            resource = j["resource"].get<std::string>();
        } catch (json::type_error & ex) {
            // Do nothing.
        }

        std::string identity, action;
        try {
            identity = j["subject"].get<std::string>();
            action = j["action"].get<std::string>();
        } catch (json::type_error & ex) {
            invalidRequestHandler("Invalid data", res);
            return;
        }

        auto i = this->m_manager.getIdentity(identity);
        libzephir::AllowedResult result(libzephir::AllowedOutcome::ABSTAIN, {});
        if (i == nullptr) {
            result.merge(libzephir::AllowedResult(libzephir::AllowedOutcome::DENIED, {}));
        } else {
            result.merge(*i->allowed(action, resource));
            auto groups = this->m_manager.getGroupsFor(*i);

            for (auto &g : groups) {
                result.merge(*g->allowed(action, resource));
            }
        }

        res.status = result.outcome == libzephir::DENIED ? 403 : 200;
        res.set_content(result.toJsonString(), "application/json");
    }
}
