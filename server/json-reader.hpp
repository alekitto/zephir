#ifndef ZEPHIR_JSON_READER_HPP
#define ZEPHIR_JSON_READER_HPP

#include <string>
#include <nlohmann/json.hpp>

namespace zephir::server {
    using namespace nlohmann;

    auto json_reader(const httplib::ContentReader &content_reader) {
        std::string body;
        content_reader([&](const char *data, size_t data_length) {
            body.append(data, data_length);
            return true;
        });

        return json::parse(body);
    }
}

#endif //ZEPHIR_JSON_READER_HPP
