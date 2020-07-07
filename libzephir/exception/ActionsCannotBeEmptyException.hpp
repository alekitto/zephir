#ifndef ZEPHIR_ACTIONSCANNOTBEEMPTYEXCEPTION_HPP
#define ZEPHIR_ACTIONSCANNOTBEEMPTYEXCEPTION_HPP

#include <exception>

namespace libzephir::exception {
    class ActionsCannotBeEmptyException : public std::exception {
    public:
        [[nodiscard]] const char *what() const noexcept override {
            return "Policy actions cannot be empty.";
        }
    };
}

#endif //ZEPHIR_ACTIONSCANNOTBEEMPTYEXCEPTION_HPP
