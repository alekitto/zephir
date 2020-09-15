#include "../../Server.hpp"
#include "../util.hpp"

namespace zephir::server {
    void Server::removeGroupMember(const std::shared_ptr <libzephir::Group>& group, Response &res, const std::string & identityId) {
        auto identity = this->m_manager.getIdentity(identityId);
        if (identity == nullptr) {
            Server::createNotFoundResponse(res);
            return;
        }

        group->removeIdentity(identity);
        this->m_manager.save(group);
    }
}