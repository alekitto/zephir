#ifndef ZEPHIR_SERVER_HPP
#define ZEPHIR_SERVER_HPP

#include <httplib.h>
#include "../libzephir/storage/Manager.hpp"
#include "json-reader.hpp"

namespace zephir::server {
    using namespace libzephir::storage;

    class Server {
        Manager & m_manager;

    public:
        explicit Server(Manager & manager): m_manager(manager) {}
        void listen() {
            using namespace httplib;
            using HttpServer = httplib::Server;

            HttpServer srv;
            srv.Get("/_status", [&](const Request &req, Response &res) {
                res.set_content(R"({"status":"OK"})", "application/json");
                res.status = 200;
            });

            srv.Post("/allowed", [&](const Request &req, Response &res, const ContentReader &content_reader) {
                using namespace nlohmann;

                auto invalid_request_handler = [&](const char * msg, Response &res) {
                    json j = json({
                        {"status", "Bad Request"},
                        {"code", 400},
                        {"detail", msg}
                    });

                    res.set_content(j.dump(), "application/json");
                    res.status = 400;
                };

                json j;
                try {
                    j = json_reader(content_reader);
                } catch (json::parse_error& ex) {
                    invalid_request_handler("Invalid body", res);
                    return;
                }

                try {
                    j.at("subject");
                } catch (json::out_of_range & ex) {
                    invalid_request_handler("Identity must be specified", res);
                    return;
                }

                try {
                    j.at("action");
                } catch (json::out_of_range & ex) {
                    invalid_request_handler("Action must be specified", res);
                    return;
                }

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
                    invalid_request_handler("Invalid data", res);
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

                res.set_content(result.toJson(), "application/json");
                res.status = result.outcome == libzephir::DENIED ? 403 : 200;
            });

            std::cout << "Listening on port 8091" << std::endl;
            srv.listen("0.0.0.0", 8091);
        }
    };
}

#endif //ZEPHIR_SERVER_HPP
