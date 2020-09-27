extern crate csv;

use super::ParsleySerializer;
use csv::Writer;
use std::collections::HashMap;
use std::io::Write;

struct CallbackWriter {
    callback: fn(String),
}

impl CallbackWriter {
    fn new(callback: fn(String)) -> CallbackWriter {
        CallbackWriter { callback }
    }
}

impl Write for CallbackWriter {
    fn write(&mut self, bytes: &[u8]) -> std::result::Result<usize, std::io::Error> {
        let callback = self.callback;
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
    pub fn new(output_callback: fn(String), columns: Vec<String>) -> CsvSerializer {
        CsvSerializer {
            columns,
            writer: Writer::from_writer(CallbackWriter::new(output_callback)),
        }
    }
}

impl ParsleySerializer for CsvSerializer {
    fn start(&mut self) {
        self.writer.write_record(self.columns.as_slice()).ok();
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
