use std::str::FromStr;

const ANONYMOUS_NAME: &str = "_";
const FIELD_SEPARATION: &str = ":";

pub struct ParseFieldError;

#[derive(Debug)]
pub enum Field {
    Named(String, String),
    Anonymous(String),
}

impl Field {
    pub fn to_capture_group(&self) -> String {
        match self {
            Field::Named(name, expression) => format!("(?P<{}>{})", name, expression),
            Field::Anonymous(expression) => expression.to_string(),
        }
    }

    pub fn get_name(&self) -> Option<String> {
        match self {
            Field::Named(name, _) => Option::from(String::from(name)),
            Field::Anonymous(_) => Option::None,
        }
    }
}

impl FromStr for Field {
    type Err = ParseFieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(process_raw_field(s))
    }
}

pub fn all_names(fields: &[Field]) -> Vec<String> {
    fields
        .iter()
        .map(|field| field.get_name())
        .flatten()
        .collect()
}

pub fn combine_fields(separator: &str, fields: &[Field]) -> String {
    fields
        .iter()
        .map(|field| field.to_capture_group())
        .collect::<Vec<String>>()
        .join(separator)
}

fn process_raw_field(raw_field: &str) -> Field {
    let split_field: Vec<&str> = raw_field.splitn(2, FIELD_SEPARATION).collect();
    match split_field.len() {
        1 => Field::Anonymous(String::from(split_field[0])),
        _ => {
            if split_field[0] == ANONYMOUS_NAME {
                Field::Anonymous(String::from(split_field[1]))
            } else {
                Field::Named(String::from(split_field[0]), String::from(split_field[1]))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_anonymous_field() {
        let field = process_raw_field(".*");
        let expected = Field::Anonymous(String::from(".*"));
        assert_eq!(field.to_capture_group(), expected.to_capture_group());
    }

    #[test]
    fn process_field_as_anonymous_with_underscore_name() {
        let field = process_raw_field("_::");
        let expected = Field::Anonymous(String::from(":"));
        assert_eq!(field.to_capture_group(), expected.to_capture_group());
    }

    #[test]
    fn process_named_field() {
        let field = process_raw_field("name:[A-Z][a-z]+");
        let expected = Field::Named(String::from("name"), String::from("[A-Z][a-z]+"));
        assert_eq!(field.to_capture_group(), expected.to_capture_group());
    }

    #[test]
    fn makes_capture_group() {
        let field = Field::Named(String::from("name"), String::from("[A-Z][A-Za-z]*"));

        let capture_group = field.to_capture_group();

        assert_eq!(capture_group, "(?P<name>[A-Z][A-Za-z]*)");
    }

    #[test]
    fn combines_fields() {
        let fields = vec![
            Field::Named(String::from("name"), String::from("[A-Z][A-Za-z]*")),
            Field::Named(String::from("age"), String::from("[0-9]+")),
        ];

        let pattern = combine_fields("", fields.as_slice());

        assert_eq!(pattern, "(?P<name>[A-Z][A-Za-z]*)(?P<age>[0-9]+)");
    }
}
