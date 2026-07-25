use std::process::Command;

#[test]
fn global_help_succeeds_on_stdout() {
    let output = Command::new(env!("CARGO_BIN_EXE_store-test-runner"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8(output.stdout)
        .unwrap()
        .starts_with("usage: store-test-runner"));
    assert!(output.stderr.is_empty());
}

#[test]
fn unknown_command_fails_on_stderr() {
    let output = Command::new(env!("CARGO_BIN_EXE_store-test-runner"))
        .arg("definitely-not-a-command")
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("unknown command `definitely-not-a-command`"));
    assert!(stderr.contains("usage: store-test-runner"));
}
