mod arguments;
mod driver;
mod field;
mod matcher;
mod serializers;

use self::arguments::get_arguments;
use self::driver::process_input;
use self::field::all_names;
use self::matcher::Matcher;
use self::serializers::get_serializer;
use std::env;

fn main() {
    let args = get_arguments(env::args());
    let serializer = get_serializer(
        &args.format,
        Box::new(|line| println!("{}", line)),
        all_names(args.fields.as_slice()),
    );
    let matcher = Matcher::new(args.fields);
    process_input(args.silence_errors, matcher, serializer);
}
