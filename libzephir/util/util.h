#ifndef ZEPHIR_UTIL_H
#define ZEPHIR_UTIL_H

#include "apply.h"

namespace libzephir::util {
    std::regex glob_to_regex(std::string glob);
}

#endif //ZEPHIR_UTIL_H
