#include "schemas.hpp"

#define POPULATE_SCHEMA(cSchema, schema)  { \
        nlohmann::json __document__; \
        valijson::SchemaParser __parser; \
        \
        __document__ = nlohmann::json::parse(cSchema); \
        valijson::adapters::NlohmannJsonAdapter __document_adapter(__document__); \
        __parser.populateSchema(__document_adapter, schema); \
    }

namespace zephir::json_schema {
    valijson::Schema sPolicyUpsert;
    valijson::Schema sIdentityUpsert;
    valijson::Schema sAllowed;

    init__schemas::init__schemas() {
        POPULATE_SCHEMA(internal::allowedSchema, sAllowed)
        POPULATE_SCHEMA(internal::upsertPolicySchema, sPolicyUpsert)
        POPULATE_SCHEMA(internal::upsertIdentitySchema, sIdentityUpsert)
    }

    init__schemas s = init__schemas();
}
