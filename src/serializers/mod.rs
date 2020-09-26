mod json;

use self::json::JsonSerializer;
use std::collections::HashMap;

#[derive(Debug)]
pub enum OutputFormat {
    JSON,
    CSV,
}

pub trait ParsleySerializer {
    fn start(&self);
    fn serialize(&self, map: HashMap<String, String>);
    fn end(&self);
}

pub fn get_serializer(
    format: OutputFormat,
    output_callback: fn(String),
) -> Box<dyn ParsleySerializer> {
    match format {
        _ => Box::new(JsonSerializer::new(output_callback)),
    }
}
