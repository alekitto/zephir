#ifndef ZEPHIR_UNKNOWNPOLICYVERSIONEXCEPTION_H
#define ZEPHIR_UNKNOWNPOLICYVERSIONEXCEPTION_H

#include <exception>

namespace libzephir::exception {
    class UnknownPolicyVersionException : public std::exception {
        int _version;

    public:
        const int &version;

        explicit UnknownPolicyVersionException(int version) : std::exception(), version(_version) {
            this->_version = version;
        }

        const char *what() const noexcept override {
            return "Unknown policy version";
        }
    };
}

#endif //ZEPHIR_UNKNOWNPOLICYVERSIONEXCEPTION_H
