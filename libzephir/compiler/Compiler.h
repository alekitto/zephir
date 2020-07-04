#ifndef ZEPHIR_COMPILER_H
#define ZEPHIR_COMPILER_H

#include <memory>
#include <regex>
#include <vector>

#include "CompiledPolicy.h"
#include "../cache/LruCache.hpp"

class Policy;

class Compiler {
    LruCache<std::string, std::shared_ptr<CompiledPolicy>> _cache = LruCache<std::string, std::shared_ptr<CompiledPolicy>>(1000);

public:
    std::shared_ptr<CompiledPolicy> compile(Policy &policy);

    static Compiler & getInstance() {
        static Compiler instance;
        return instance;
    }
};

#endif //ZEPHIR_COMPILER_H
