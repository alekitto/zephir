#ifndef ZEPHIR_GUARD_HPP
#define ZEPHIR_GUARD_HPP

#include <mutex>

namespace libzephir::lock {
    template <class T>
    class Guard {
        T & lock;

    public:
        explicit Guard(T & mutex) : lock(mutex) { this->lock.lock(); }
        ~Guard() { this->lock.unlock(); }
    };
}

#endif //ZEPHIR_GUARD_HPP
