use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn fails_no_args() {
    let mut cmd = Command::cargo_bin("echo_r").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    // assert that 1) fails, 2) prints to stderr a message containing the string USAGE
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echo_r").unwrap();
    cmd.arg("hello world").assert().success();
}

// if you have a consistent naming structure, like fails/dies and succeeds/runs or whatever, you can run all tests of that type with cargo test <type>, e.g. cargo test fails