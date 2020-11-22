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
            spdlog::trace(std::string("Identity \"") + identity + std::string("\" not found. Denying access..."));
        } else {
            spdlog::trace(std::string("Loaded identity \"") + identity + std::string("\": ") + i->toJsonString());

            auto identityResult = i->allowed(action, resource);
            result.merge(*identityResult);
            spdlog::trace(
                std::string("Identity policies ") +
                std::string(result.outcome == libzephir::DENIED ? "denied" : (result.outcome == libzephir::ALLOWED ? "allow" : "conditional allow")) +
                std::string(" access. Now evaluating groups..")
            );

            auto groups = this->m_manager.getGroupsFor(*i);
            for (auto &g : groups) {
                result.merge(*g->allowed(action, resource));
            }
        }

        spdlog::debug(
            std::string(result.outcome == libzephir::DENIED ? "Denied" : (result.outcome == libzephir::ALLOWED ? "Allowed" : "Conditional allowed")) +
            std::string(" access for action \"") + action + std::string("\" to \"") + identity + std::string("\"") +
            std::string(resource.has_value() ? " on resource \"" + resource.value() + "\"" : "")
        );

        res.status = result.outcome == libzephir::DENIED ? 403 : 200;
        res.set_content(result.toJsonString(), "application/json");
    }
}
