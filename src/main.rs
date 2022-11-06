use clap::Parser;
use std::io::{self, BufRead};
use std::process::exit;
use regex::Regex;

/// Convert durations to seconds (e.g. 01:00:05 => 3605)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Text containing one or more durations
    /// (read from stdin if not provided here)
    input: Vec<String>
}

fn find_durations(text: &str) -> Vec<&str> {
    if text.is_empty() {
        return Vec::new();
    }

    let re = Regex::new(r"(\b\d+:\d\d:\d\d(?:\.\d*)?)").unwrap();

    let durations: Vec<&str> = re.find_iter(text)
        .map(|match_| match_.as_str())
        .collect();

    return durations;
}

fn duration_as_secs(duration: &str) -> f32 {
    if duration.is_empty() {
        return 0.0
    }

    let re = Regex::new(r":").unwrap();

    let parts: Vec<&str> = re.split(duration).collect();

    assert_eq!(parts.len(), 3);

    let hours: f32 = parts[0].parse().unwrap();
    let minutes: f32 = parts[1].parse().unwrap();
    let seconds: f32 = parts[2].parse().unwrap();

    return (hours * 3600.0) + (minutes * 60.0) + seconds;
}

fn process_input(text: &str) -> Vec<String> {
    let mut output_lines = Vec::new();

    for duration in find_durations(text) {
        let secs = duration_as_secs(duration);
        output_lines.push(format!("{duration} => {secs}"));
    }

    output_lines
}

fn display_and_exit(output_lines: Vec<String>) {
    if output_lines.is_empty() {
        eprintln!("No durations found in input");
        exit(1);
    }

    for out_line in output_lines {
        println!("{out_line}");
    }
}

fn read_stdin() -> String {
    eprintln!("Reading from stdin...");

    let mut lines = io::stdin().lock().lines();
    let mut user_input = String::new();

    while let Some(line) = lines.next() {
        let last_input = line.unwrap();

        // stop reading
        if last_input.len() == 0 {
            break;
        }

        // add a new line once user_input starts storing user input
        if user_input.len() > 0 {
            user_input.push_str("\n");
        }

        // store user input
        user_input.push_str(&last_input);
    }

    user_input
}

fn main() {
    let cli = Cli::parse();

    let input = if cli.input.is_empty() { read_stdin() } else { cli.input.join(" ") };

    let output_lines = process_input(&input);

    display_and_exit(output_lines);
}
