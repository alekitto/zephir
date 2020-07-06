#ifndef ZEPHIR_INVALIDDSNEXCEPTION_H
#define ZEPHIR_INVALIDDSNEXCEPTION_H

#include <exception>

namespace libzephir::storage::exception {
    class InvalidDsnException : public std::exception {
    public:
        const char *what() const noexcept override {
            return "Invalid dsn.";
        }
    };
}

#endif //ZEPHIR_INVALIDDSNEXCEPTION_H
