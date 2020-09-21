

#[derive(Debug)]
pub struct Field {
    name: String,
    expression: String,
}

impl Field {
    pub fn new(name: String, expression: String) -> Field {
        Field {
            name: name,
            expression: expression,
        }
    }

    pub fn to_capture_group(&self) -> String {
        format!("(?P<{}>{})", self.name, self.expression)
    }
}

pub fn combine_fields(separator: &str, fields: Vec<Field>) -> String {
     fields
        .into_iter()
        .map(|field| field.to_capture_group())
        .collect::<Vec<String>>()
        .join(separator)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_capture_group() {
        let field = Field::new(String::from("name"), String::from("[A-Z][A-Za-z]*"));

        let capture_group = field.to_capture_group();
        
        assert_eq!(capture_group, "(?P<name>[A-Z][A-Za-z]*)");
    }

    #[test]
    fn combines_fields() {
        let fields = vec!(
            Field::new(String::from("name"), String::from("[A-Z][A-Za-z]*")),
            Field::new(String::from("age"), String::from("[0-9]+")),
        );

        let pattern = combine_fields("", fields);

        assert_eq!(pattern, "(?P<name>[A-Z][A-Za-z]*)(?P<age>[0-9]+)");
    }
}