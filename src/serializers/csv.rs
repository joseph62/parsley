extern crate csv;

use super::ParsleySerializer;
use csv::Writer;
use std::collections::HashMap;
use std::io::Write;

struct CallbackWriter {
    callback: Box<dyn Fn(String)>,
}

impl CallbackWriter {
    fn new(callback: Box<dyn Fn(String)>) -> CallbackWriter {
        CallbackWriter { callback }
    }
}

impl Write for CallbackWriter {
    fn write(&mut self, bytes: &[u8]) -> std::result::Result<usize, std::io::Error> {
        let callback = &self.callback;
        if let Ok(line) = String::from_utf8(bytes.to_vec()) {
            callback(line);
        }
        Ok(bytes.len())
    }

    fn flush(&mut self) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
}

pub struct CsvSerializer {
    columns: Vec<String>,
    writer: Writer<CallbackWriter>,
}

impl CsvSerializer {
    pub fn new(output_callback: Box<dyn Fn(String)>, columns: Vec<String>) -> CsvSerializer {
        CsvSerializer {
            columns,
            writer: Writer::from_writer(CallbackWriter::new(output_callback)),
        }
    }
}

impl ParsleySerializer for CsvSerializer {
    fn start(&mut self) {
        self.writer.write_record(&self.columns).ok();
    }

    fn serialize(&mut self, map: HashMap<String, String>) {
        self.writer
            .write_record(
                self.columns
                    .iter()
                    .map(|key| map.get(key).unwrap())
                    .collect::<Vec<&String>>(),
            )
            .ok();
    }

    fn end(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::super::test::make_buffer_output_callback;
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn csv_serializer_serialize_writes_csv_row() {
        let (buffer, output_callback) = make_buffer_output_callback();
        let mut serializer = CsvSerializer::new(output_callback, vec!["test".to_string()]);

        serializer.serialize(HashMap::from_iter(vec![(
            String::from("test"),
            String::from("value"),
        )]));

        assert_eq!(buffer.borrow().as_str(), "value\n");
    }

    #[test]
    fn csv_serializer_start_writes_header() {
        let (buffer, output_callback) = make_buffer_output_callback();
        let mut serializer =
            CsvSerializer::new(output_callback, vec!["one".to_string(), "two".to_string()]);

        serializer.start();

        assert_eq!(buffer.borrow().as_str(), "one,two\n");
    }

    #[test]
    fn csv_serializer_end_does_nothing() {
        let (buffer, output_callback) = make_buffer_output_callback();
        let mut serializer = CsvSerializer::new(output_callback, vec!["test".to_string()]);

        serializer.end();

        assert_eq!(buffer.borrow().as_str(), "");
    }
}
