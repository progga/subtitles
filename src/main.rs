//! Prepare subtitle file (i.e. .srt file) from given text.
use std::env;
use std::fs;
use std::process;

/// Converts input file content into an SRT file.
///
/// Prints SRT file content to stdout.
fn main() {
    let cli_args: Vec<String> = env::args().collect();
    if cli_args.len() != 3 {
        eprintln!(
            "Usage: {} INPUT-FILENAME LENGTH-IN-SECONDS > OUTPUT-FILENAME.srt",
            cli_args[0]
        );
        process::exit(1);
    }

    let input_filename: &str = cli_args[1].as_str();
    let length_in_seconds: u32 = cli_args[2].parse().unwrap();

    let input_file_content: String = fs::read_to_string(input_filename).expect("Cannot find file.");

    let srt_output: String =
        subtitles::srt::prepare_srt_content(&input_file_content, length_in_seconds);
    print!("{}", srt_output);
}
