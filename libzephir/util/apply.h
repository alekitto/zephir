#ifndef ZEPHIR_APPLY_H
#define ZEPHIR_APPLY_H

#include <vector>

namespace libzephir::util {
    template<typename C, typename F>
    auto apply(C &&container, F &&func) {
        using std::begin;
        using std::end;

        using E = std::decay_t<decltype(std::forward<F>(func)(
                *begin(std::forward<C>(container))))>;

        std::vector<E> result;
        auto first = begin(std::forward<C>(container));
        auto last = end(std::forward<C>(container));

        result.reserve(std::distance(first, last));
        for (; first != last; ++first) {
            result.push_back(std::forward<F>(func)(*first));
        }
        return result;
    }
}

#endif //ZEPHIR_APPLY_H
