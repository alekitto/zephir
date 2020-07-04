#include <string>
#include <regex>

namespace util {
    // Returns a RegExp which is the equivalent of the glob pattern.
    std::regex glob_to_regex(std::string glob) {
        if ("*" == glob) {
            // Short-circuit common case.
            // This is the only case where the "*" does not stop at first colon character.
            return std::regex(".*");
        }

        bool escaping = false;
        int inCurlies = 0;
        std::string regex;

        auto globSize = glob.size();
        for (int i = 0; i < globSize; ++i) {
            auto car = glob[i];
            bool firstByte = car == ':';

            if (firstByte && i + 2 < globSize && glob[i + 1] == '*' && glob[i + 2] == '*') {
                regex += ".*";
                continue;
            }

            if (car == '.' || car == '(' || car == ')' || car == '|' || car == '+' || car == '^' || car == '$') {
                regex += "\\";
                regex += car;
            } else if (car == '*') {
                regex += escaping ? "\\*" : "[^:]*";
            } else if (car == '?') {
                regex += escaping ? "\\?" : "[^:]";
            } else if (car == '{') {
                regex += escaping ? "\\{" : "(";
                if (!escaping) { inCurlies++; }
            } else if (car == '}' && inCurlies) {
                regex += escaping ? "}" : ")";
                if (!escaping) { inCurlies--; }
            } else if (car == ',' && inCurlies) {
                regex += escaping ? "," : "|";
            } else if (car == '\\') {
                if (escaping) {
                    regex += "\\\\";
                }

                escaping = !escaping;
                continue;
            } else {
                regex += car;
            }

            escaping = false;
        }

        return std::regex(regex);
    }
}
