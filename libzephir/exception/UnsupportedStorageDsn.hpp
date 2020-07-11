#pragma once

#include <exception>

namespace libzephir::exception {
    class UnsupportedStorageDsn : public std::exception {
    public:
        [[nodiscard]] const char *what() const noexcept override {
            return "Given storage DSN is not supported.";
        }
    };
}
