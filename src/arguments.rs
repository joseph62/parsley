extern crate clap;
use clap::{App, Arg, ArgGroup, ArgMatches};

pub fn get_arguments<'a>() -> ParsleyArguments<'a> {
    let matches: ArgMatches<'a> = App::new("parsley")
        .about("Parse input lines using specified fields")
        .arg(
            Arg::with_name("fields")
                .value_name("FIELD")
                .help("Define the fields to parse")
                .long_help(
                    "Define the fields to parse.
Field format is as follows '<name>:<regular expression>'.",
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
        .get_matches();

    let raw_fields: Vec<&str> = matches.values_of("fields").unwrap().collect();
    let output_csv: bool = matches.value_of("csv").is_some();

    let fields: Vec<Field<'a>> = process_raw_fields(raw_fields);
    let format = if output_csv {
        OutputFormat::CSV
    } else {
        OutputFormat::JSON
    };

    ParsleyArguments::new(fields, format)
}

fn process_raw_fields<'a>(raw_fields: Vec<&str>) -> Vec<Field<'a>> {
    let mut fields: Vec<Field> = vec![];
    for raw_field in raw_fields {
        let raw_field = String::from(raw_field);
        let raw_field: &'a str = raw_field.as_str();
        let split_field: Vec<&'a str> = raw_field.splitn(2, ":").collect();
        fields.push(Field::new(split_field[0], split_field[1]));
    }
    fields
}

#[derive(Debug)]
pub struct ParsleyArguments<'a> {
    fields: Vec<Field<'a>>,
    format: OutputFormat,
}

impl ParsleyArguments<'_> {
    fn new<'a>(fields: Vec<Field<'a>>, format: OutputFormat) -> ParsleyArguments {
        ParsleyArguments {
            fields: fields,
            format: format,
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Field<'a> {
    name: &'a str,
    expression: &'a str,
}

impl Field<'_> {
    fn new<'a>(name: &'a str, expression: &'a str) -> Field<'a> {
        Field {
            name: name,
            expression: expression,
        }
    }
}

#[derive(Debug)]
pub enum OutputFormat {
    JSON,
    CSV,
}
