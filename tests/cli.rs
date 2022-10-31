use assert_cmd::Command;
use assert_cmd::crate_name;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    cmd.assert().success();
}
