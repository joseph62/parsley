const ANONYMOUS_NAME: &str = "_";
const FIELD_SEPARATION: &str = ":";

#[derive(Debug)]
pub enum Field {
    Named(String, String),
    Anonymous(String),
}

impl Field {
    pub fn to_capture_group(&self) -> String {
        match self {
            Field::Named(name, expression) => format!("(?P<{}>{})", name, expression),
            Field::Anonymous(expression) => format!("{}", expression),
        }
    }
}

pub fn combine_fields(separator: &str, fields: &[Field]) -> String {
    fields
        .into_iter()
        .map(|field| field.to_capture_group())
        .collect::<Vec<String>>()
        .join(separator)
}

pub fn process_raw_fields(raw_fields: Vec<&str>) -> Vec<Field> {
    let mut fields: Vec<Field> = vec![];
    for raw_field in raw_fields {
        fields.push(process_raw_field(raw_field));
    }
    fields
}

fn process_raw_field(raw_field: &str) -> Field {
    let split_field: Vec<&str> = raw_field.splitn(2, FIELD_SEPARATION).collect();
    match split_field.len() {
        1 => Field::Anonymous(String::from(split_field[0])),
        2 | _ => {
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
    fn process_multiple_fields() {
        let fields = combine_fields(
            "",
            process_raw_fields(vec!["name:[A-Z][a-z]+", "_::", "age:[0-9]+"]).as_slice(),
        );

        let expected = combine_fields(
            "",
            vec![
                Field::Named(String::from("name"), String::from("[A-Z][a-z]+")),
                Field::Anonymous(String::from(":")),
                Field::Named(String::from("age"), String::from("[0-9]+")),
            ]
            .as_slice(),
        );

        assert_eq!(fields, expected);
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
