#pragma once

#include <memory>
#include <regex>
#include <vector>

#include "CompiledPolicy.h"
#include "../cache/LruCache.hpp"

namespace libzephir {
    class Policy;

    namespace compiler {
        class Compiler {
            typedef cache::LruCache<std::string, std::shared_ptr<CompiledPolicy>> CompilerPolicyCache;
            CompilerPolicyCache _cache = CompilerPolicyCache(1000);

        public:
            std::shared_ptr<CompiledPolicy> compile(Policy &policy);
            void clearCache() { this->_cache.clear(); }

            static Compiler &getInstance() {
                static Compiler instance;
                return instance;
            }
        };
    }
}
