use assert_cmd::Command;    // uses the assert_cmd included in Cargo.toml

#[test] // attribute which tells rust to run this when testing
fn runs() {
    let mut cmd = Command::cargo_bin("echo").unwrap();  // unwrap will panic if binary not found
    cmd.assert().success();
}