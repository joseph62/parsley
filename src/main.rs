mod arguments;
mod field;
mod matcher;
mod serializers;

use self::arguments::get_arguments;
use self::field::all_names;
use self::matcher::Matcher;
use self::serializers::{get_serializer, ParsleySerializer};
use std::env;
use std::io;

fn main() {
    let args = get_arguments(env::args());
    let mut serializer = get_serializer(
        args.format,
        |line| println!("{}", line),
        all_names(args.fields.as_slice()),
    );
    let matcher = Matcher::new(args.fields);
    process_input(matcher, &mut serializer);
}

fn process_input(matcher: Matcher, serializer: &mut Box<dyn ParsleySerializer>) {
    serializer.start();
    process_next_line(matcher, serializer);
    serializer.end();
}
fn process_next_line(matcher: Matcher, serializer: &mut Box<dyn ParsleySerializer>) {
    if let Some(line) = read_line() {
        match matcher.match_line(line.as_str()) {
            Some(bindings) => serializer.serialize(bindings),
            None => eprintln!("Failed to match: {}", line),
        }
        process_next_line(matcher, serializer);
    }
}

fn read_line() -> Option<String> {
    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .ok()
        .map(|bytes| {
            if bytes > 0 {
                Option::from(line)
            } else {
                Option::None
            }
        })
        .flatten()
}
