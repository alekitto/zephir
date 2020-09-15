#pragma once

#include <nlohmann/json.hpp>

#define DECODE_AND_VALIDATE_JSON(json_var, schema, res, content_reader) \
        nlohmann::json json_var; \
        try { \
            json_var = json_reader(content_reader); \
        } catch (json::parse_error& ex) { \
            this->invalidRequestHandler("Invalid body", res); \
            return; \
        } \
        \
        valijson::adapters::NlohmannJsonAdapter __json_adapter(json_var); \
        valijson::Validator __json_validator; \
        valijson::ValidationResults __json_validation_results; \
        \
        if (! __json_validator.validate(schema, __json_adapter, &__json_validation_results)) { \
            std::vector<std::string> errors; \
            for (auto & error : __json_validation_results) { \
                errors.push_back(error.description); \
            } \
            \
            this->invalidRequestHandler(errors, res); \
            return; \
        }
