namespace zephir::json_schema::internal {
    const char * upsertIdentitySchema = R";-]({
    "$schema": "http://json-schema.org/draft-07/schema",
    "$id": "http://example.com/example.json",
    "type": "object",
    "title": "The schema for identity insertion",
    "default": {},
    "examples": [
        {
            "id": "urn:identity:example:id",
            "linked_policies": [
                "urn:policy::test"
            ],
            "inline_policy": {
                "effect": "ALLOW",
                "actions": [
                    "action1"
                ],
                "resources": [
                    "resource1"
                ]
            }
        }
    ],
    "required": [
        "id",
        "linked_policies",
        "inline_policy"
    ],
    "properties": {
        "id": {
            "$id": "#/properties/id",
            "type": "string",
            "title": "The identity identifier",
            "description": "The URN of the current identity",
            "default": "",
            "examples": [
                "urn:identity:example:id"
            ]
        },
        "linked_policies": {
            "$id": "#/properties/linked_policies",
            "type": "array",
            "title": "The policies linked to this identity",
            "default": [],
            "examples": [
                [
                    "urn:policy::test"
                ]
            ],
            "additionalItems": true,
            "items": {
                "$id": "#/properties/linked_policies/items",
                "title": "The linked policy identifier",
                "description": "The policy must be already persisted",
                "type": "string"
            }
        },
        "inline_policy": {
            "$id": "#/properties/inline_policy",
            "type": [
                "object",
                "null"
            ],
            "title": "An inline policy linked to this identity",
            "default": {},
            "examples": [
                {
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
                "effect",
                "actions",
                "resources"
            ],
            "properties": {
                "effect": {
                    "$id": "#/properties/inline_policy/properties/effect",
                    "type": "string",
                    "title": "The policy effect (allow/deny)",
                    "description": "Must be \"ALLOW\" or \"DENY\"",
                    "default": "",
                    "examples": [
                        "ALLOW"
                    ]
                },
                "actions": {
                    "$id": "#/properties/inline_policy/properties/actions",
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
                        "$id": "#/properties/inline_policy/properties/actions/items",
                        "type": "string"
                    }
                },
                "resources": {
                    "$id": "#/properties/inline_policy/properties/resources",
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
                        "$id": "#/properties/inline_policy/properties/resources/items",
                        "type": "string"
                    }
                }
            }
        }
    }
});-]";
}

