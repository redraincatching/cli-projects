use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;
// this is a type of result that is either Ok with the unit type, or anything that implements Error
// this can be used as it is much safer than just calling unwrap, which will panic

#[test]
fn fails_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("echo_r")?;    // the ? operator unpacks an Ok or propagates an Err
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    // assert that 1) fails, 2) prints to stderr a message containing the string USAGE
    // will fail before the Ok if about to fail, this just allows the return type
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("echo_r")?;
    cmd.arg("hello world").assert().success();
    Ok(())
}

// if you have a consistent naming structure, like fails/dies and succeeds/runs or whatever, you can run all tests of that type with cargo test <type>, e.g. cargo test fails

// tests/expected/
#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["Hello  there", "-n"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}


// helper function
fn run(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echo_r")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())

    // so args are a slice of references to string literals, and expected file is also &str
    // then, try to read to a string, and attempt to run echo_r
}