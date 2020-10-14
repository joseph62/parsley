extern crate clap;
use crate::field::{process_raw_fields, Field};
use clap::{arg_enum, crate_authors, crate_name, crate_version, value_t, App, Arg, ArgMatches};
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
            Arg::with_name("format")
                .long("format")
                .short("f")
                .possible_values(&OutputFormat::variants())
                .default_value("json")
                .case_insensitive(true)
                .help("Specify the format of the output data"),
        )
        .get_matches_from(args);

    let raw_fields: Vec<&str> = matches.values_of("fields").unwrap().collect();

    let fields: Vec<Field> = process_raw_fields(raw_fields);

    let format = value_t!(matches, "format", OutputFormat).unwrap_or_else(|e| e.exit());

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

arg_enum! {
    #[derive(PartialEq, Debug)]
    pub enum OutputFormat {
        Json,
        Csv,
    }
}
