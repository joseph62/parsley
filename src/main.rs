mod arguments;
mod field;
mod matcher;

use self::arguments::get_arguments;
use self::matcher::Matcher;
use std::env;
use std::io;

fn main() {
    let args = get_arguments(env::args());
    let matcher = Matcher::new(args.fields);
    process_input(matcher);
}

fn process_input(matcher: Matcher) {
    if let Some(line) = read_line() {
        match matcher.match_line(line.as_str()) {
            Some(bindings) => println!("Found a match! {:?}", bindings),
            None => println!("Did not match line '{}'", line),
        }
        process_input(matcher);
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
