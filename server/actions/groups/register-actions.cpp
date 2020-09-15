#include "../../Server.hpp"

namespace zephir::server {
    void Server::register_groups_actions(httplib::Server &srv) {
        srv.Post("/group/([^/]+)/members", [&](const Request &req, Response &res, const ContentReader &content_reader) {
            auto groupId = req.matches[1];
            auto g = this->m_manager.getGroup(groupId.str());
            if (nullptr == g) {
                Server::createNotFoundResponse(res);
                return;
            }

            this->addGroupMember(g, res, content_reader);
        });
    }
}
