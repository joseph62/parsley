extern crate serde_json;
use super::ParsleySerializer;
use serde_json::to_string;
use std::collections::HashMap;

pub struct JsonSerializer {
    output_callback: Box<dyn Fn(String)>,
}

impl JsonSerializer {
    pub fn new(output_callback: Box<dyn Fn(String)>) -> JsonSerializer {
        JsonSerializer { output_callback }
    }
}

impl ParsleySerializer for JsonSerializer {
    fn start(&mut self) {}

    fn serialize(&mut self, map: HashMap<String, String>) {
        let callback = &self.output_callback;
        if let Ok(line) = to_string(&map) {
            callback(line)
        }
    }

    fn end(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn json_serializer_serialize_writes_json_object() {
        let mut serializer =
            JsonSerializer::new(Box::new(|line| assert_eq!(line, "{\"test\":\"value\"}")));

        serializer.serialize(HashMap::from_iter(vec![(
            String::from("test"),
            String::from("value"),
        )]));
    }

    #[test]
    fn json_serializer_start_does_nothing() {
        let mut serializer = JsonSerializer::new(Box::new(|_| assert!(false)));

        serializer.start();
    }

    #[test]
    fn json_serializer_end_does_nothing() {
        let mut serializer = JsonSerializer::new(Box::new(|_| assert!(false)));

        serializer.end();
    }
}
