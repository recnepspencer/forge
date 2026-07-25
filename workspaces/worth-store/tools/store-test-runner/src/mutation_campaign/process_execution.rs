use std::io::{Read, Write};
use std::process::{Command, Output, Stdio};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(15);
const POLL_INTERVAL: Duration = Duration::from_millis(25);

pub(super) fn run(command: &mut Command, mutant: u8) -> Result<Output, String> {
    run_with_progress(command, HEARTBEAT_INTERVAL, |elapsed| {
        println!(
            "mutate: {mutant} still running after {}s",
            elapsed.as_secs()
        );
        let _ = std::io::stdout().flush();
    })
    .map_err(|error| format!("cannot execute mutant {mutant}: {error}"))
}

fn run_with_progress(
    command: &mut Command,
    heartbeat_interval: Duration,
    mut heartbeat: impl FnMut(Duration),
) -> Result<Output, String> {
    assert!(!heartbeat_interval.is_zero());
    command.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = command
        .spawn()
        .map_err(|error| format!("cannot spawn child: {error}"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "child omitted piped stdout".to_owned())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "child omitted piped stderr".to_owned())?;
    let stdout = capture(stdout, "stdout");
    let stderr = capture(stderr, "stderr");
    let started = Instant::now();
    let mut next_heartbeat = heartbeat_interval;
    let status = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("cannot inspect child: {error}"))?
        {
            break status;
        }
        let elapsed = started.elapsed();
        if elapsed >= next_heartbeat {
            heartbeat(elapsed);
            next_heartbeat = next_heartbeat.saturating_add(heartbeat_interval);
        }
        std::thread::sleep(POLL_INTERVAL.min(next_heartbeat.saturating_sub(started.elapsed())));
    };
    Ok(Output {
        status,
        stdout: join_capture(stdout, "stdout")?,
        stderr: join_capture(stderr, "stderr")?,
    })
}

fn capture(
    mut stream: impl Read + Send + 'static,
    label: &'static str,
) -> JoinHandle<Result<Vec<u8>, String>> {
    std::thread::spawn(move || {
        let mut captured = Vec::new();
        stream
            .read_to_end(&mut captured)
            .map_err(|error| format!("cannot read child {label}: {error}"))?;
        Ok(captured)
    })
}

fn join_capture(
    capture: JoinHandle<Result<Vec<u8>, String>>,
    label: &str,
) -> Result<Vec<u8>, String> {
    capture
        .join()
        .map_err(|_| format!("child {label} reader panicked"))?
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::process::Command;
    use std::time::Duration;

    use super::run_with_progress;

    #[test]
    fn concurrent_capture_emits_progress_without_losing_streams() {
        let mut command = fixture();
        let mut heartbeats = Vec::new();
        let output = run_with_progress(&mut command, Duration::from_millis(10), |elapsed| {
            heartbeats.push(elapsed)
        })
        .unwrap();
        assert!(output.status.success());
        assert!(!heartbeats.is_empty());
        assert!(String::from_utf8(output.stdout)
            .unwrap()
            .contains("mutation-child-stdout"));
        assert!(String::from_utf8(output.stderr)
            .unwrap()
            .contains("mutation-child-stderr"));
    }

    #[test]
    fn child_process_fixture() {
        if std::env::var_os("WORTH_C5_MUTATION_PROCESS_FIXTURE").is_none() {
            return;
        }
        println!("mutation-child-stdout");
        eprintln!("mutation-child-stderr");
        std::io::stdout().flush().unwrap();
        std::io::stderr().flush().unwrap();
        std::thread::sleep(Duration::from_millis(80));
    }

    fn fixture() -> Command {
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--exact",
                "mutation_campaign::process_execution::tests::child_process_fixture",
                "--nocapture",
            ])
            .env("WORTH_C5_MUTATION_PROCESS_FIXTURE", "1");
        command
    }
}
