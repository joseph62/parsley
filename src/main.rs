mod arguments;
mod field;

use self::arguments::get_arguments;
use std::env;
use std::io;

fn main() {
    let args = get_arguments(env::args());
    let capture_expression = args
        .fields
        .into_iter()
        .map(|field| field.to_capture_group())
        .collect::<Vec<String>>()
        .join(" ");
    println!("Using capture expression: '{}'", capture_expression);
    process_input();
}

fn process_input() {
    if let Some(line) = read_line() {
        println!("{}", line);
        process_input();
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
