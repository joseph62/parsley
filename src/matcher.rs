extern crate regex;

use crate::field::{Field, combine_fields};
use std::collections::HashMap;
use std::iter::FromIterator;
use regex::Regex;

pub struct Matcher {
    keys: Vec<String>,
    pattern: Regex,
}

impl Matcher {
    pub fn new(fields: Vec<Field>) -> Matcher {
        let pattern = combine_fields("", fields.as_slice());
        let pattern = Regex::new(combine_fields("", fields.as_slice()).as_str())
            .expect(format!("Failed to build regex using the provided patterns: '{}'", pattern).as_str());
        let mut keys = Vec::new();
        for field in fields {
            if let Field::Named(name, _) = field {
                keys.push(name);
            }
        }
        Matcher{keys:keys, pattern:pattern}
    }

    pub fn match_line(&self, line: &str) -> Option<HashMap<String, String>> {
        self.pattern.captures(line)
        .map(|capture| {
            HashMap::from_iter(self.keys.iter().map(|key| (String::from(key), String::from(&capture[key.as_str()]))))
        })
    }
}