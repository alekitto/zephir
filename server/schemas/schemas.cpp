#define POPULATE_SCHEMA(cSchema, schema)  { \
        nlohmann::json __document__; \
        valijson::SchemaParser __parser; \
        \
        __document__ = nlohmann::json::parse(cSchema); \
        valijson::adapters::NlohmannJsonAdapter __document_adapter(__document__); \
        __parser.populateSchema(__document_adapter, schema); \
    }

namespace zephir::json_schema {
    valijson::Schema sAddGroupMember;
    valijson::Schema sAllowed;
    valijson::Schema sGroupUpsert;
    valijson::Schema sIdentityUpsert;
    valijson::Schema sPolicyUpsert;

    init__schemas::init__schemas() {
        POPULATE_SCHEMA(internal::addGroupMember, sAddGroupMember)
        POPULATE_SCHEMA(internal::allowedSchema, sAllowed)
        POPULATE_SCHEMA(internal::upsertGroupSchema, sGroupUpsert)
        POPULATE_SCHEMA(internal::upsertPolicySchema, sPolicyUpsert)
        POPULATE_SCHEMA(internal::upsertIdentitySchema, sIdentityUpsert)
    }

    init__schemas s = init__schemas();
}
