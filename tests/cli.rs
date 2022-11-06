use assert_cmd::Command;
use assert_cmd::crate_name;
use predicates::prelude::*;

#[test]
fn fails_when_no_args() {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No durations"));
}

#[test]
fn fails_when_input_has_no_valid_durations() {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.arg("no durations here")
        .assert()
        .failure()
        .stderr(predicate::str::contains("No durations"));
}

#[test]
fn success_when_input_has_one_duration() {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.arg("01:00:00")
        .assert()
        .success()
        .stdout(predicate::str::contains("3600"));
}

#[test]
fn success_when_input_has_multiple_durations() {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.arg("01:00:00 01:00:01")
        .assert()
        .success()
        .stdout(predicate::str::contains("3600"))
        .stdout(predicate::str::contains("3601"));
}
