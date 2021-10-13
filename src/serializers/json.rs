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
    use super::super::test::make_buffer_output_callback;
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn json_serializer_serialize_writes_json_object() {
        let (buffer, output_callback) = make_buffer_output_callback();
        let mut serializer = JsonSerializer::new(output_callback);

        serializer.serialize(HashMap::from_iter(vec![
            (String::from("one"), String::from("1")),
            (String::from("two"), String::from("2")),
        ]));

        serializer.serialize(HashMap::from_iter(vec![
            (String::from("one"), String::from("3")),
            (String::from("two"), String::from("4")),
        ]));

        assert_eq!(
            buffer.borrow().as_str(),
            "{\"one\":\"1\",\"two\":\"2\"}{\"two\":\"4\",\"one\":\"3\"}"
        );
    }

    #[test]
    fn json_serializer_start_does_nothing() {
        let (buffer, output_callback) = make_buffer_output_callback();
        let mut serializer = JsonSerializer::new(output_callback);

        serializer.start();

        assert_eq!(buffer.borrow().as_str(), "");
    }

    #[test]
    fn json_serializer_end_does_nothing() {
        let (buffer, output_callback) = make_buffer_output_callback();
        let mut serializer = JsonSerializer::new(output_callback);

        serializer.end();

        assert_eq!(buffer.borrow().as_str(), "");
    }
}
