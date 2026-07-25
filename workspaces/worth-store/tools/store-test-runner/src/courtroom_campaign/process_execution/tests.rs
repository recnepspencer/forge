use std::io::Write;
use std::process::Command;
use std::time::Duration;

use worth_store::physical_runtime::PhysicalWorkProcessFateEvidence;

use super::{
    kill_at_stdout_marker, run_success, run_success_allowing_stderr, DIAGNOSTIC_BYTE_LIMIT,
};

#[test]
fn completion_and_marker_kill_are_bounded() {
    let mut success = shell("echo ready");
    let output = run_success(&mut success, Duration::from_secs(2), "success").unwrap();
    assert_eq!(output.stdout(), ["ready"]);
    assert!(matches!(
        output.evidence("successful-child").unwrap().fate(),
        PhysicalWorkProcessFateEvidence::ExitedSuccess
    ));

    let mut parked = fixture("checkpoint-clean");
    let (killed, marker) =
        kill_at_stdout_marker(&mut parked, Duration::from_secs(2), "checkpoint", "parked").unwrap();
    assert_eq!(marker.trim(), "checkpoint");
    let killed = killed.evidence("faulting-child").unwrap();
    assert_eq!(killed.fate().yieldpoint(), Some("checkpoint"));
}

#[test]
fn successful_process_with_stderr_is_rejected_by_strict_protocols() {
    let mut command = fixture("success-stderr");
    let failure = require_failure(run_success(&mut command, Duration::from_secs(2), "warning"));
    assert!(failure.contains("exited successfully but emitted stderr"));
    assert!(failure.contains("warning"));
}

#[test]
fn successful_process_with_stderr_can_be_explicitly_admitted() {
    let mut command = fixture("success-stderr");
    let output =
        run_success_allowing_stderr(&mut command, Duration::from_secs(2), "metadata").unwrap();
    assert!(output.stderr().contains("warning"));
}

#[test]
fn failed_process_diagnostics_are_bounded() {
    let mut command = fixture("failure-large");
    let failure = require_failure(run_success_allowing_stderr(
        &mut command,
        Duration::from_secs(2),
        "large failure",
    ));
    assert!(failure.contains("bytes omitted"));
    assert!(failure.len() < (DIAGNOSTIC_BYTE_LIMIT * 2) + 1_024);
}

#[test]
fn timeout_retains_both_captured_streams() {
    let mut command = fixture("timeout");
    let failure = require_failure(run_success(
        &mut command,
        Duration::from_millis(50),
        "timeout",
    ));
    assert!(failure.contains("exceeded"));
    assert!(failure.contains("before"));
    assert!(failure.contains("problem"));
}

#[test]
fn checkpoint_process_with_stderr_is_rejected() {
    let mut command = fixture("checkpoint-stderr");
    let failure = require_failure(kill_at_stdout_marker(
        &mut command,
        Duration::from_secs(2),
        "checkpoint",
        "checkpoint-warning",
    ));
    assert!(failure.contains("reached `checkpoint` but emitted stderr"));
    assert!(failure.contains("warning"));
}

#[test]
fn child_process_fixture() {
    let Ok(mode) = std::env::var("WORTH_C5_1_PROCESS_FIXTURE") else {
        return;
    };
    match mode.as_str() {
        "success-stderr" => eprintln!("warning"),
        "failure-large" => {
            println!("{}", "stdout".repeat(DIAGNOSTIC_BYTE_LIMIT));
            eprintln!("{}", "stderr".repeat(DIAGNOSTIC_BYTE_LIMIT));
            flush_streams();
            std::process::exit(7);
        }
        "timeout" => {
            println!("before");
            eprintln!("problem");
            flush_streams();
            park_forever();
        }
        "checkpoint-clean" => {
            println!("checkpoint");
            flush_streams();
            park_forever();
        }
        "checkpoint-stderr" => {
            eprintln!("warning");
            println!("checkpoint");
            flush_streams();
            park_forever();
        }
        _ => panic!("unknown process fixture mode"),
    }
}

fn flush_streams() {
    std::io::stdout().flush().unwrap();
    std::io::stderr().flush().unwrap();
}

fn park_forever() -> ! {
    loop {
        std::thread::park();
    }
}

fn fixture(mode: &str) -> Command {
    let mut command = Command::new(std::env::current_exe().unwrap());
    command
        .args([
            "--exact",
            "courtroom_campaign::process_execution::tests::child_process_fixture",
            "--nocapture",
        ])
        .env("WORTH_C5_1_PROCESS_FIXTURE", mode);
    command
}

fn require_failure<T>(result: Result<T, String>) -> String {
    match result {
        Ok(_) => panic!("expected process evidence rejection"),
        Err(failure) => failure,
    }
}

fn shell(script: &str) -> Command {
    if cfg!(windows) {
        let mut command = Command::new("cmd");
        command.args(["/d", "/s", "/c", script]);
        command
    } else {
        let mut command = Command::new("sh");
        command.args(["-c", script]);
        command
    }
}
