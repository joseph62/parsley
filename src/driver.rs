use super::matcher::Matcher;
use super::serializers::ParsleySerializer;
use std::io;

pub fn process_input(
    silence_errors: bool,
    matcher: Matcher,
    mut serializer: Box<dyn ParsleySerializer>,
) {
    serializer.start();
    process_next_line(silence_errors, matcher, &mut serializer);
    serializer.end();
}

fn process_next_line(
    silence_errors: bool,
    matcher: Matcher,
    serializer: &mut Box<dyn ParsleySerializer>,
) {
    if let Some(line) = read_line() {
        match matcher.match_line(line.as_str()) {
            Some(bindings) => serializer.serialize(bindings),
            None => {
                if !silence_errors {
                    eprintln!("Failed to match: {}", line)
                }
            }
        }
        process_next_line(silence_errors, matcher, serializer);
    }
}

fn read_line() -> Option<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok().and_then(|bytes| {
        if bytes > 0 {
            Option::from(line)
        } else {
            Option::None
        }
    })
}
