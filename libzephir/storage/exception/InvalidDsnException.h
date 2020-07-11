#pragma once

#include <exception>

namespace libzephir::storage::exception {
    class InvalidDsnException : public std::exception {
    public:
        const char *what() const noexcept override {
            return "Invalid dsn.";
        }
    };
}
