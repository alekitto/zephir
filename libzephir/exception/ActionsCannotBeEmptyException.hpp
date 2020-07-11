#pragma once

#include <exception>

namespace libzephir::exception {
    class ActionsCannotBeEmptyException : public std::exception {
    public:
        [[nodiscard]] const char *what() const noexcept override {
            return "Policy actions cannot be empty.";
        }
    };
}
