#include "../../Server.hpp"
#include "../util.hpp"

namespace zephir::server {
    void Server::addGroupMember(const std::shared_ptr <libzephir::Group>& group, Response &res, const ContentReader &content_reader) {
        using namespace nlohmann;
        DECODE_AND_VALIDATE_JSON(j, zephir::json_schema::sAddGroupMember, res, content_reader)

        std::shared_ptr<libzephir::Identity> identity;
        try {
            identity = this->m_manager.getIdentity(j["id"].get<std::string>());
        } catch (json::type_error & ex) {
            invalidRequestHandler("Invalid data", res);
            return;
        }

        if (identity == nullptr) {
            Server::createNotFoundResponse(res);
            return;
        }

        group->addIdentity(identity);
        this->m_manager.save(group);

        res.set_content(j.dump(), "application/json");
    }
}