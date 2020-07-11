#pragma once

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
