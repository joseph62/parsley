mod csv;
mod json;

use self::csv::CsvSerializer;
use self::json::JsonSerializer;
use crate::arguments::OutputFormat;
use std::collections::HashMap;

pub trait ParsleySerializer {
    fn start(&mut self);
    fn serialize(&mut self, map: HashMap<String, String>);
    fn end(&mut self);
}

pub fn get_serializer(
    format: &OutputFormat,
    output_callback: Box<dyn Fn(String)>,
    keys: Vec<String>,
) -> Box<dyn ParsleySerializer> {
    match format {
        OutputFormat::Json => Box::new(JsonSerializer::new(output_callback)),
        OutputFormat::Csv => Box::new(CsvSerializer::new(output_callback, keys)),
    }
}

#[cfg(test)]
mod test {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub fn make_buffer_output_callback() -> (Rc<RefCell<String>>, Box<dyn Fn(String)>) {
        let buffer = Rc::new(RefCell::new(String::new()));
        let callback_buffer = Rc::clone(&buffer);
        let output_callback =
            Box::new(move |line: String| callback_buffer.borrow_mut().push_str(line.as_str()));
        (buffer, output_callback)
    }
}
