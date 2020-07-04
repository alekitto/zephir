#ifndef ZEPHIR_ACTIONSCANNOTBEEMPTYEXCEPTION_H
#define ZEPHIR_ACTIONSCANNOTBEEMPTYEXCEPTION_H

#include <exception>

class ActionsCannotBeEmptyException : public std::exception {
public:
    const char * what() const noexcept override
    {
        return "Policy actions cannot be empty.";
    }
};

#endif //ZEPHIR_ACTIONSCANNOTBEEMPTYEXCEPTION_H
