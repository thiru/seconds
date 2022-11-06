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

    assert!(parts.len() == 3);

    let hours: u32 = parts[0].parse().unwrap();
    let minutes: u32 = parts[1].parse().unwrap();
    let seconds: u32 = parts[2].parse().unwrap();

    return (hours * 3600) + (minutes * 60) + seconds;
}

fn main() {
    let cli = Cli::parse();

    let mut out_lines = Vec::new();

    for duration in find_durations(&cli.input) {
        let secs = duration_as_secs(duration);
        out_lines.push(format!("{duration} = {secs}"));
    }

    if out_lines.is_empty() {
        eprintln!("No durations found in input");
        exit(1);
    }

    for out_line in out_lines {
        println!("{out_line}");
    }
}
