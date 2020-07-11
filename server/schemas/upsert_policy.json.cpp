namespace zephir::json_schema::internal {
    const char * upsertPolicySchema = R";-]({
"$schema": "http://json-schema.org/draft-07/schema",
"$id": "http://example.com/example.json",
"type": "object",
"title": "The upset policy schema",
"default": {},
"examples": [
    {
        "id": "PolicyId",
        "effect": "ALLOW",
        "actions": [
            "action1"
        ],
        "resources": [
            "resource1"
        ]
    }
],
"required": [
    "id",
    "effect",
    "actions",
    "resources"
],
"additionalProperties": false,
"properties": {
    "id": {
        "$id": "#/properties/id",
        "type": "string",
        "title": "The policy ID",
        "description": "Must be unique",
        "default": "",
        "examples": [
            "PolicyId"
        ],
        "minLength": 1,
        "pattern": "[A-Za-z][A-Za-z0-9_\\-.]*"
    },
    "effect": {
        "$id": "#/properties/effect",
        "type": "string",
        "title": "The policy effect (allow/deny)",
        "default": "",
        "enum": [ "ALLOW", "DENY" ],
        "examples": [
            "ALLOW"
        ],
        "description": "Must be \"ALLOW\" or \"DENY\""
        },
        "actions": {
            "$id": "#/properties/actions",
            "type": "array",
            "title": "The actions allowed/denied by this policy",
            "description": "An array of globs of the allowed/denied actions. Use * to represent all actions.",
            "default": [],
            "examples": [
                [
                    "action1"
                ]
            ],
            "additionalItems": true,
            "items": {
                "$id": "#/properties/actions/items",
                "type": "string"
            },
            "uniqueItems": "true",
            "minItems": 1
        },
        "resources": {
            "$id": "#/properties/resources",
            "type": "array",
            "title": "The resources subject of this policy",
            "description": "An array of globs representing the resources affected by this policy. Use * to represent all resources.",
            "default": [],
            "examples": [
                [
                    "resource1"
                ]
            ],
            "additionalItems": true,
            "items": {
                "$id": "#/properties/resources/items",
                "type": "string"
            },
            "minItems": 1,
            "uniqueItems": "true"
        }
    }
});-]";
}

