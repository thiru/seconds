use std::env;
use regex::Regex;

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
    for arg in env::args().skip(1) {
        for duration in find_durations(&arg) {
            let secs = duration_as_secs(duration);
            println!("{duration} = {secs}")
        }
    }
}
