namespace zephir::json_schema::internal {
    const char *addGroupMember = R"({
    "default": {},
    "required": [
        "id"
    ],
    "title": "Add members to group",
    "properties": {
        "id": {
            "default": "",
            "title": "A valid identity id"
        }
    }
})";
}