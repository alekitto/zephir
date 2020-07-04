#ifndef ZEPHIR_IDENTITY_H
#define ZEPHIR_IDENTITY_H

#include <string>
#include "Subject.hpp"

class Identity : public Subject {
    std::string _id;

public:
    const std::string & id;
    Identity(std::string pId, const Policy& policy) : Subject(policy), _id(std::move(pId)), id(_id) { }
};

#endif //ZEPHIR_IDENTITY_H
