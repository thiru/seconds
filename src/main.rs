use clap::Parser;
use std::process::exit;
use regex::Regex;

/// Convert durations to seconds (e.g. 01:00:05 => 3605)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Text containing one or more durations
    input: String
}

fn find_durations(text: &str) -> Vec<&str> {
    if text.is_empty() {
        return Vec::new();
    }

    let re = Regex::new(r"(\d\d:\d\d:\d\d)").unwrap();

    let durations: Vec<&str> = re.find_iter(text)
        .map(|match_| match_.as_str())
        .collect();

    return durations;
}

fn duration_as_secs(duration: &str) -> u32 {
    if duration.is_empty() {
        return 0
    }

    let re = Regex::new(r":").unwrap();

    let parts: Vec<&str> = re.split(duration).collect();

    assert_eq!(parts.len(), 3);

    let hours: u32 = parts[0].parse().unwrap();
    let minutes: u32 = parts[1].parse().unwrap();
    let seconds: u32 = parts[2].parse().unwrap();

    return (hours * 3600) + (minutes * 60) + seconds;
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

fn main() {
    let cli = Cli::parse();

    let output_lines = process_input(&cli.input);

    display_and_exit(output_lines);
}
