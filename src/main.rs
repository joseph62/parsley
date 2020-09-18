mod arguments;

use self::arguments::get_arguments;
use std::env;

fn main() {
    let args = get_arguments(env::args());
    println!("Arguments: {:?}", args);
}
