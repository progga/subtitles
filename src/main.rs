//! Prepare subtitle file (i.e. .srt file) from given text.
extern crate clap;
use clap::{App, Arg};
use csv;
use std::collections::HashMap;
use std::fs;
use std::process;

/// Converts input file content into an SRT file.
///
/// Prints SRT file content to stdout.
fn main() {
    let cli_args = App::new("subtitles")
        .version("0.2.0")
        .arg(
            Arg::with_name("input-filename")
                .long("transcript")
                .help("Input filename")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("length-in-seconds")
                .long("length")
                .help("Video length in seconds")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("abbreviations")
                .long("abbr")
                .help("Path of CSV file containing abbreviations")
                .takes_value(true),
        )
        .get_matches();

    let input_filename = cli_args.value_of("input-filename").unwrap();
    let length_in_seconds = cli_args
        .value_of("length-in-seconds")
        .unwrap()
        .parse::<u32>()
        .expect("Expecting a number.");

    let input_file_content: String = fs::read_to_string(input_filename).expect("Cannot find file.");

    let mut abbr_map = HashMap::new();
    if let Some(abbr_file) = cli_args.value_of("abbreviations") {
        let result = fetch_abbreviations(abbr_file);
        if let Ok(extracted_abbr_map) = result {
            abbr_map = extracted_abbr_map;
        } else if let Err(err) = result {
            eprintln!("Abbreviation error: {}", err);
            process::exit(1);
        }
    }

    let srt_output: String =
        subtitles::srt::prepare_srt_content(&input_file_content, length_in_seconds, &abbr_map);
    print!("{}", srt_output);
}

/// Load hashmap of abbreviations.
///
/// Keys are abbreviations, values are their full form.
/// Example ["foo" => "Foo bar", "bar" => "Bar baz"]
fn fetch_abbreviations(csv_file: &str) -> Result<HashMap<String, String>, &str> {
    let mut map = HashMap::new();

    let mut csv_reader = csv::ReaderBuilder::new();
    csv_reader.has_headers(false);

    if let Ok(mut csv_stream) = csv_reader.from_path(csv_file) {
        for record in csv_stream.records() {
            if let Ok(row) = record {
                map.insert(row[0].to_string(), row[1].to_string());
            } else {
                return Err("Trouble reading CSV.");
            }
        }
    } else {
        return Err("Trouble opening CSV.");
    }

    return Ok(map);
}
