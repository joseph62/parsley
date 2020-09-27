mod csv;
mod json;

use self::csv::CsvSerializer;
use self::json::JsonSerializer;
use std::collections::HashMap;

#[derive(Debug)]
pub enum OutputFormat {
    JSON,
    CSV,
}

pub trait ParsleySerializer {
    fn start(&mut self);
    fn serialize(&mut self, map: HashMap<String, String>);
    fn end(&mut self);
}

pub fn get_serializer(
    format: OutputFormat,
    output_callback: fn(String),
    keys: Vec<String>,
) -> Box<dyn ParsleySerializer> {
    match format {
        OutputFormat::JSON => Box::new(JsonSerializer::new(output_callback)),
        OutputFormat::CSV => Box::new(CsvSerializer::new(output_callback, keys)),
    }
}
