#pragma once

#include <valijson/adapters/nlohmann_json_adapter.hpp>
#include <valijson/utils/nlohmann_json_utils.hpp>
#include <valijson/schema.hpp>
#include <valijson/schema_parser.hpp>
#include <valijson/validator.hpp>

namespace zephir::json_schema {
    namespace internal {
        extern const char *addGroupMember;
        extern const char *allowedSchema;
        extern const char *upsertIdentitySchema;
        extern const char *upsertPolicySchema;
    }

    class init__schemas {
    public:
        init__schemas();
    };

    extern valijson::Schema sAddGroupMember;
    extern valijson::Schema sAllowed;
    extern valijson::Schema sIdentityUpsert;
    extern valijson::Schema sPolicyUpsert;
    extern init__schemas s;
}
