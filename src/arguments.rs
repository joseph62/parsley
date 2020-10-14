extern crate clap;
use crate::field::{process_raw_fields, Field};
use crate::serializers::OutputFormat;
use clap::{crate_authors, crate_name, crate_version, App, Arg, ArgGroup, ArgMatches};
use std::ffi::OsString;

pub fn get_arguments<I, T>(args: I) -> ParsleyArguments
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches: ArgMatches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!("\n"))
        .about("Parse input lines using specified fields")
        .arg(
            Arg::with_name("fields")
                .value_name("FIELD")
                .help("Define the fields to parse")
                .long_help(
                    "Define the fields to parse.
Field format is as follows 
Named:     '<name>:<regular expression>'
Anonymous: '<regular expression>'
                    -or-
           '_:<regular expression>'
Named groups will show up in the structured output.",
                )
                .multiple(true),
        )
        .arg(
            Arg::with_name("json")
                .long("json")
                .help("Format output as JSON"),
        )
        .arg(
            Arg::with_name("csv")
                .long("csv")
                .help("Format output as CSV"),
        )
        .group(ArgGroup::with_name("output_format").arg("json").arg("csv"))
        .get_matches_from(args);

    let raw_fields: Vec<&str> = matches.values_of("fields").unwrap().collect();

    let fields: Vec<Field> = process_raw_fields(raw_fields);

    let format = if matches.is_present("csv") {
        OutputFormat::CSV
    } else {
        OutputFormat::JSON
    };

    ParsleyArguments::new(fields, format)
}

#[derive(Debug)]
pub struct ParsleyArguments {
    pub fields: Vec<Field>,
    pub format: OutputFormat,
}

impl ParsleyArguments {
    fn new(fields: Vec<Field>, format: OutputFormat) -> ParsleyArguments {
        ParsleyArguments {
            fields: fields,
            format: format,
        }
    }
}
