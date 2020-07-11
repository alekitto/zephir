#pragma once

#include <string>
#include "Subject.hpp"

namespace libzephir::identity {
    class Identity : public Subject {
        std::string _id;

    public:
        const std::string &id;

        Identity(std::string pId, const Policy &policy) : Subject(policy), _id(std::move(pId)), id(_id) {}
    };
}
