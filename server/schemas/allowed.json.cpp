namespace zephir::json_schema::internal {
    const char *allowedSchema = R"({
  "$schema": "http://json-schema.org/draft-07/schema",
  "$id": "http://example.com/example.json",
  "type": "object",
  "title": "The root schema",
  "description": "The root schema comprises the entire JSON document.",
  "default": {},
  "examples": [
    {
      "subject": "urn:example:identity:test-id",
      "action": "identity:list",
      "resource": "urn:example:class:this-resource"
    }
  ],
  "required": [
    "subject",
    "action"
  ],
  "additionalProperties": true,
  "properties": {
    "subject": {
      "$id": "#/properties/subject",
      "type": "string",
      "title": "The subject of the action",
      "description": "The identity/subject on which the action has to be computed",
      "default": "",
      "examples": [
        "urn:example:identity:test-id"
      ]
    },
    "action": {
      "$id": "#/properties/action",
      "type": "string",
      "title": "The action",
      "description": "Represents the action on the policies",
      "default": "",
      "examples": [
        "identity:list"
      ]
    },
    "resource": {
      "$id": "#/properties/resource",
      "type": "string",
      "title": "The resource",
      "description": "Represents the optional resource in the policies",
      "default": "",
      "examples": [
        "urn:example:class:this-resource"
      ]
    }
  }
})";
}