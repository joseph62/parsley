extern crate serde_json;
use super::ParsleySerializer;
use serde_json::to_string;
use std::collections::HashMap;

pub struct JsonSerializer {
    output_callback: fn(String),
}

impl JsonSerializer {
    pub fn new(output_callback: fn(String)) -> JsonSerializer {
        JsonSerializer { output_callback }
    }
}

impl ParsleySerializer for JsonSerializer {
    fn start(&mut self) {}

    fn serialize(&mut self, map: HashMap<String, String>) {
        let callback = self.output_callback;
        if let Ok(line) = to_string(&map) {
            callback(line)
        }
    }

    fn end(&mut self) {}
}

#[cfg(tests)]
mod tests {
    #[test]
    fn json_serializer_serialize_writes_json_object() {
        let mut result = String::new();
        let serializer = JsonSerializer::new(|line| result.add(line));

        serializer.serialize(HashMap::from_iter(vec![("test", "value")]));

        assert_eq!(result, "{\"test\":\"value\"}");
    }

    #[test]
    fn json_serializer_start_does_nothing() {
        let mut result = String::new();
        let serializer = JsonSerializer::new(|line| result.add(line));

        serializer.start();

        assert_eq!(result, "".to_string())
    }

    #[test]
    fn json_serializer_end_does_nothing() {
        let mut result = String::new();
        let serializer = JsonSerializer::new(|line| result.add(line));

        serializer.end();

        assert_eq!(result, "".to_string())
    }
}
