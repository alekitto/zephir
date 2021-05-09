use pcre2::bytes::{RegexBuilder, Regex};

pub fn from_str(glob: &str) -> Regex {
    from_string(glob.to_string())
}

pub fn from_string(glob: String) -> Regex {
    if glob == "*" {
        return RegexBuilder::new()
            .jit_if_available(true)
            .build(r".+")
            .unwrap();
    }

    let mut escaping = false;
    let mut in_curlies = 0;
    let mut regex = String::new();

    let chars = glob.chars();
    let glob_size = glob.len();
    for (i, car) in chars.enumerate() {
        let first_byte = car == ':';
        if first_byte && i + 2 < glob_size {
            let next = glob.chars().nth(i + 1).unwrap();
            let next_next = glob.chars().nth(i + 2).unwrap();
            if next == '*' && next_next == '*' {
                regex += ".*";
                continue;
            }
        }

        if car == '.'
            || car == '('
            || car == ')'
            || car == '|'
            || car == '+'
            || car == '^'
            || car == '$'
        {
            regex += "\\";
            regex.push(car);
        } else if car == '*' {
            regex += if escaping { "\\*" } else { "[^:]*" };
        } else if car == '?' {
            regex += if escaping { "\\?" } else { "[^:]" };
        } else if car == '{' {
            regex += if escaping { "\\{" } else { "(" };
            if !escaping {
                in_curlies += 1;
            }
        } else if car == '}' && in_curlies > 0 {
            regex += if escaping { "}" } else { ")" };
            if !escaping {
                in_curlies -= 1;
            }
        } else if car == ',' && in_curlies > 0 {
            regex += if escaping { "," } else { "|" };
        } else if car == '\\' {
            if escaping {
                regex += "\\\\";
            }

            escaping = !escaping;
            continue;
        } else {
            regex.push(car);
        }

        escaping = false;
    }

    RegexBuilder::new()
        .jit_if_available(true)
        .build(regex.as_str())
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::utils::glob_to_regex::{from_str, from_string};

    #[test]
    fn from_string_should_return_match_all_regex() {
        assert_eq!(from_string("*".to_string()).as_str(), ".+");
    }

    #[test]
    fn from_string_should_work_correctly() {
        assert_eq!(
            from_string("foo_{bar,foo}.*".to_string()).as_str(),
            "foo_(bar|foo)\\.[^:]*"
        );
        assert_eq!(
            from_string("foo_ba?.\\*".to_string()).as_str(),
            "foo_ba[^:]\\.\\*"
        );
    }

    #[test]
    fn from_str_should_work_correctly() {
        assert_eq!(
            from_str("foo_{bar,foo}.*").as_str(),
            "foo_(bar|foo)\\.[^:]*"
        );
    }
}
