#pragma once

#include <httplib.h>
#include <spdlog/spdlog.h>
#include "../libzephir/storage/Manager.hpp"
#include "schemas/schemas.hpp"

namespace zephir::server {
    using namespace nlohmann;
    using namespace libzephir::storage;
    using namespace httplib;

    extern json json_reader(const httplib::ContentReader &content_reader);

    class Server {
        Manager & m_manager;

        static std::shared_ptr<libzephir::Policy> decodePolicy(const nlohmann::json &j);

        static void invalidRequestHandler(const char * msg, httplib::Response &res) {
            json j = json({
                {"status", "Bad Request"},
                {"code",   400},
                {"detail", { msg }}
            });

            res.set_content(j.dump(), "application/json");
            res.status = 400;
        }

        template<class T>
        static void invalidRequestHandler(std::vector<T> detail, httplib::Response &res) {
            json j = json({
                {"status", "Bad Request"},
                {"code",   400},
                {"detail", detail}
            });

            res.set_content(j.dump(), "application/json");
            res.status = 400;
        }

        inline static void createNotFoundResponse(httplib::Response &res) { Server::createNotFoundResponse("Not found.", res); }
        static void createNotFoundResponse(const char * msg, httplib::Response &res) {
            json j = json({
                {"status", msg},
                {"code",   404},
            });

            res.set_content(j.dump(), "application/json");
            res.status = 404;
        }

        void registerGroupsActions(httplib::Server &srv);
        void addGroupMember(const std::shared_ptr<libzephir::Group>& group, Response &res, const ContentReader &content_reader);
        void removeGroupMember(const std::shared_ptr <libzephir::Group>& group, Response &res, const std::string & identityId);

        void upsertGroup(const Request &req, Response &res, const ContentReader &content_reader);
        void upsertIdentity(const Request &req, Response &res, const ContentReader &content_reader);
        void upsertPolicy(const Request &req, Response &res, const ContentReader &content_reader);

        void allowedAction(const Request &req, Response &res, const ContentReader &content_reader);

    public:
        explicit Server(Manager & manager): m_manager(manager) {}
        void listen() {
            using HttpServer = httplib::Server;

            HttpServer srv;
            srv.set_logger([&](const Request& req, const Response &res) {
                spdlog::debug(std::string("Handled ") + req.path + " [from " + req.remote_addr + "]. Result: " + std::to_string(res.status));
            });

            srv.Get("/_status", [&](const Request &req, Response &res) {
                res.set_content(R"({"status":"OK"})", "application/json");
                res.status = 200;
            });

            srv.Post("/allowed", [&](const Request &req, Response &res, const ContentReader &content_reader) {
                this->allowedAction(req, res, content_reader);
            });

            srv.Post("/policies", [&](const Request &req, Response &res, const ContentReader &content_reader) {
                this->upsertPolicy(req, res, content_reader);
            });
            srv.Post("/identities", [&](const Request &req, Response &res, const ContentReader &content_reader) {
                this->upsertIdentity(req, res, content_reader);
            });

            this->registerGroupsActions(srv);

            auto server_port = 8091;
            auto server_port_str = std::getenv("SERVE_PORT");
            if (nullptr != server_port_str) {
                server_port = atoi(server_port_str);
            }

            spdlog::info(std::string("Listening on port ") + std::to_string(server_port));
            srv.listen("0.0.0.0", server_port);
        }
    };
}
