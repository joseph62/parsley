

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_capture_group() {
        let field = Field::new(String::from("name"), String::from("[A-Z][A-Za-z]*"));

        let capture_group = field.to_capture_group();
        
        assert_eq!(capture_group, "(?P<name>[A-Z][A-Za-z]*)");
    }
}