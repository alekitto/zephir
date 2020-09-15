#include "../../Server.hpp"

namespace zephir::server {
    void Server::registerGroupsActions(httplib::Server &srv) {
        srv.Post("/groups", [&](const Request &req, Response &res, const ContentReader &content_reader) {
            this->upsertGroup(req, res, content_reader);
        });

        srv.Post("/group/([^/]+)/members", [&](const Request &req, Response &res, const ContentReader &content_reader) {
            auto groupId = req.matches[1];
            auto g = this->m_manager.getGroup(groupId.str());
            if (nullptr == g) {
                Server::createNotFoundResponse(res);
                return;
            }

            this->addGroupMember(g, res, content_reader);
        });

        srv.Delete("/group/([^/]+)/member/([^/]+)", [&](const Request &req, Response &res, const ContentReader &content_reader) {
            auto groupId = req.matches[1];
            auto g = this->m_manager.getGroup(groupId.str());
            if (nullptr == g) {
                Server::createNotFoundResponse(res);
                return;
            }

            this->removeGroupMember(g, res, req.matches[2].str());
        });
    }
}
