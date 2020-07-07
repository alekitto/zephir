#ifndef ZEPHIR_UNSUPPORTEDSTORAGEDSN_HPP
#define ZEPHIR_UNSUPPORTEDSTORAGEDSN_HPP

#include <exception>

namespace libzephir::exception {
    class UnsupportedStorageDsn : public std::exception {
    public:
        [[nodiscard]] const char *what() const noexcept override {
            return "Given storage DSN is not supported.";
        }
    };
}

#endif //ZEPHIR_UNSUPPORTEDSTORAGEDSN_HPP
