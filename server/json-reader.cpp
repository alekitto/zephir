#include "stdafx.hpp"

namespace zephir::server {
    using namespace nlohmann;

    json json_reader(const httplib::ContentReader &content_reader) {
        std::string body;
        content_reader([&](const char *data, size_t data_length) {
            body.append(data, data_length);
            return true;
        });

        return json::parse(body);
    }
}
