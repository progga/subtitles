//! Prepare subtitle file (i.e. .srt file) from given text.
extern crate clap;
use clap::{App, Arg};
use std::fs;

/// Converts input file content into an SRT file.
///
/// Prints SRT file content to stdout.
fn main() {
    let cli_args = App::new("subtitles")
        .version("0.1.4")
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
        .get_matches();

    let input_filename = cli_args.value_of("input-filename").unwrap();
    let length_in_seconds = cli_args
        .value_of("length-in-seconds")
        .unwrap()
        .parse::<u32>()
        .expect("Expecting a number.");

    let input_file_content: String = fs::read_to_string(input_filename).expect("Cannot find file.");

    let srt_output: String =
        subtitles::srt::prepare_srt_content(&input_file_content, length_in_seconds);
    print!("{}", srt_output);
}
