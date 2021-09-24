extern crate regex;

use crate::field::{combine_fields, Field};
use regex::Regex;
use std::collections::HashMap;

pub struct Matcher {
    keys: Vec<String>,
    pattern: Regex,
}

impl Matcher {
    pub fn new(fields: Vec<Field>) -> Matcher {
        let pattern = combine_fields("", fields.as_slice());
        let pattern =
            Regex::new(combine_fields("", fields.as_slice()).as_str()).unwrap_or_else(|_| {
                panic!(
                    "Failed to build regex using the provided patterns: '{}'",
                    pattern
                )
            });
        let mut keys = Vec::new();
        for field in fields {
            if let Field::Named(name, _) = field {
                keys.push(name);
            }
        }
        Matcher { keys, pattern }
    }

    pub fn match_line(&self, line: &str) -> Option<HashMap<String, String>> {
        self.pattern.captures(line).map(|capture| {
            self.keys
                .iter()
                .map(|key| (String::from(key), String::from(&capture[key.as_str()])))
                .collect::<HashMap<String, String>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_match_on_success() {
        let matcher = Matcher::new(vec![
            Field::Named(String::from("name"), String::from("[^ ]+")),
            Field::Anonymous(String::from(" ")),
            Field::Named(String::from("age"), String::from(".*")),
        ]);

        let result = matcher.match_line("Sean 100");
        let mut expected: HashMap<String, String> = HashMap::new();
        expected.insert(String::from("name"), String::from("Sean"));
        expected.insert(String::from("age"), String::from("100"));

        assert_eq!(result, Option::Some(expected));
    }

    #[test]
    fn returns_none_if_line_does_not_match() {
        let matcher = Matcher::new(vec![
            Field::Named(String::from("name"), String::from("[^ ]+")),
            Field::Anonymous(String::from(" ")),
            Field::Named(String::from("age"), String::from(".*")),
        ]);

        let result = matcher.match_line("Sean:100");

        assert_eq!(result, Option::None);
    }
}
