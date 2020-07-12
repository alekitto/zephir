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

        static void invalid_request_handler(const char * msg, httplib::Response &res) {
            json j = json({
                {"status", "Bad Request"},
                {"code",   400},
                {"detail", { msg }}
            });

            res.set_content(j.dump(), "application/json");
            res.status = 400;
        }

        template<class T>
        static void invalid_request_handler(std::vector<T> detail, httplib::Response &res) {
            json j = json({
                {"status", "Bad Request"},
                {"code",   400},
                {"detail", detail}
            });

            res.set_content(j.dump(), "application/json");
            res.status = 400;
        }

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

            spdlog::info("Listening on port 8091");
            srv.listen("0.0.0.0", 8091);
        }
    };
}
