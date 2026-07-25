use std::io::{BufRead, BufReader, Read};
use std::num::NonZeroU32;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::mpsc;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use worth_store::physical_runtime::PhysicalWorkProcessEvidence;

const DIAGNOSTIC_BYTE_LIMIT: usize = 8 * 1024;

pub(super) struct CapturedProcess {
    process: NonZeroU32,
    stdout: Box<[String]>,
    stderr: Box<str>,
    elapsed: Duration,
    fate: CapturedProcessFate,
}

enum CapturedProcessFate {
    ExitedSuccess,
    KilledAtYieldpoint(Box<str>),
    DiagnosticOnly,
}

impl CapturedProcess {
    #[cfg(test)]
    pub(super) fn for_test(process: NonZeroU32) -> Self {
        Self {
            process,
            stdout: Box::new([]),
            stderr: "".into(),
            elapsed: Duration::ZERO,
            fate: CapturedProcessFate::ExitedSuccess,
        }
    }

    pub(super) const fn process(&self) -> NonZeroU32 {
        self.process
    }

    pub(super) fn stdout(&self) -> &[String] {
        &self.stdout
    }

    pub(super) fn stderr(&self) -> &str {
        &self.stderr
    }

    pub(super) const fn elapsed(&self) -> Duration {
        self.elapsed
    }

    pub(super) fn evidence(&self, role: &str) -> Result<PhysicalWorkProcessEvidence, String> {
        let evidence = match &self.fate {
            CapturedProcessFate::ExitedSuccess => {
                PhysicalWorkProcessEvidence::exited_success(role, self.process)
            }
            CapturedProcessFate::KilledAtYieldpoint(yieldpoint) => {
                PhysicalWorkProcessEvidence::killed_at_yieldpoint(
                    role,
                    self.process,
                    yieldpoint.clone(),
                )
            }
            CapturedProcessFate::DiagnosticOnly => {
                return Err(format!(
                    "diagnostic-only process {} cannot enter courtroom evidence",
                    self.process
                ))
            }
        };
        evidence.map_err(|denial| format!("process evidence denied: {denial:?}"))
    }
}

pub(super) fn run_success(
    command: &mut Command,
    timeout: Duration,
    label: &str,
) -> Result<CapturedProcess, String> {
    run_to_success(command, timeout, label, true)
}

pub(super) fn run_success_allowing_stderr(
    command: &mut Command,
    timeout: Duration,
    label: &str,
) -> Result<CapturedProcess, String> {
    run_to_success(command, timeout, label, false)
}

fn run_to_success(
    command: &mut Command,
    timeout: Duration,
    label: &str,
    reject_stderr: bool,
) -> Result<CapturedProcess, String> {
    let started = Instant::now();
    let mut running = RunningProcess::spawn(command, label)?;
    let status = match wait_until_exit(&mut running.child, timeout, label) {
        Ok(status) => status,
        Err(failure) => {
            let captured =
                running.finish(started.elapsed(), CapturedProcessFate::DiagnosticOnly)?;
            return Err(format!("{failure}\n{}", captured_streams(&captured)));
        }
    };
    let fate = if status.success() {
        CapturedProcessFate::ExitedSuccess
    } else {
        CapturedProcessFate::DiagnosticOnly
    };
    let captured = running.finish(started.elapsed(), fate)?;
    if !status.success() {
        Err(failed_process(label, status, &captured))
    } else if reject_stderr && !captured.stderr().is_empty() {
        Err(format!(
            "{label} exited successfully but emitted stderr\n{}",
            captured_streams(&captured),
        ))
    } else {
        Ok(captured)
    }
}

pub(super) fn kill_at_stdout_marker(
    command: &mut Command,
    timeout: Duration,
    marker: &str,
    label: &str,
) -> Result<(CapturedProcess, String), String> {
    let started = Instant::now();
    let mut running = RunningProcess::spawn(command, label)?;
    let deadline = started + timeout;
    let reached = loop {
        let now = Instant::now();
        if now >= deadline {
            terminate(&mut running.child, label)?;
            let captured =
                running.finish(started.elapsed(), CapturedProcessFate::DiagnosticOnly)?;
            return Err(format!(
                "{label} timed out waiting for `{marker}`\n{}",
                captured_streams(&captured)
            ));
        }
        match running
            .lines
            .recv_timeout((deadline - now).min(Duration::from_millis(20)))
        {
            Ok(line) if line.starts_with(marker) => break line,
            Ok(_) | Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                if let Some(status) = running
                    .child
                    .try_wait()
                    .map_err(|error| format!("cannot inspect {label}: {error}"))?
                {
                    let captured =
                        running.finish(started.elapsed(), CapturedProcessFate::DiagnosticOnly)?;
                    return Err(failed_process(label, status, &captured));
                }
            }
        }
    };
    running
        .child
        .kill()
        .map_err(|error| format!("cannot kill {label} at `{marker}`: {error}"))?;
    let status = running
        .child
        .wait()
        .map_err(|error| format!("cannot reap killed {label}: {error}"))?;
    if status.success() {
        let captured = running.finish(started.elapsed(), CapturedProcessFate::DiagnosticOnly)?;
        return Err(format!(
            "{label} reported successful exit after kill at `{marker}`\n{}",
            captured_streams(&captured)
        ));
    }
    let captured = running.finish(
        started.elapsed(),
        CapturedProcessFate::KilledAtYieldpoint(marker.into()),
    )?;
    if captured.stderr().is_empty() {
        Ok((captured, reached))
    } else {
        Err(format!(
            "{label} reached `{marker}` but emitted stderr\n{}",
            captured_streams(&captured),
        ))
    }
}

struct RunningProcess {
    child: Child,
    lines: mpsc::Receiver<String>,
    stdout: JoinHandle<Result<Vec<String>, String>>,
    stderr: JoinHandle<Result<String, String>>,
}

impl RunningProcess {
    fn spawn(command: &mut Command, label: &str) -> Result<Self, String> {
        command.stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = command
            .spawn()
            .map_err(|error| format!("cannot spawn {label}: {error}"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| format!("{label} omitted piped stdout"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| format!("{label} omitted piped stderr"))?;
        let (sender, lines) = mpsc::channel();
        let stdout = std::thread::spawn(move || {
            let mut captured = Vec::new();
            for line in BufReader::new(stdout).lines() {
                let line = line.map_err(|error| format!("cannot read process stdout: {error}"))?;
                let _ = sender.send(line.clone());
                captured.push(line);
            }
            Ok(captured)
        });
        let stderr = std::thread::spawn(move || {
            let mut captured = String::new();
            BufReader::new(stderr)
                .read_to_string(&mut captured)
                .map_err(|error| format!("cannot read process stderr: {error}"))?;
            Ok(captured)
        });
        Ok(Self {
            child,
            lines,
            stdout,
            stderr,
        })
    }

    fn finish(
        self,
        elapsed: Duration,
        fate: CapturedProcessFate,
    ) -> Result<CapturedProcess, String> {
        let process = NonZeroU32::new(self.child.id())
            .ok_or_else(|| "spawned process had a zero identity".to_owned())?;
        let stdout = self
            .stdout
            .join()
            .map_err(|_| "stdout reader panicked".to_owned())??;
        let stderr = self
            .stderr
            .join()
            .map_err(|_| "stderr reader panicked".to_owned())??;
        Ok(CapturedProcess {
            process,
            stdout: stdout.into_boxed_slice(),
            stderr: stderr.into_boxed_str(),
            elapsed,
            fate,
        })
    }
}

fn wait_until_exit(
    child: &mut Child,
    timeout: Duration,
    label: &str,
) -> Result<ExitStatus, String> {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("cannot inspect {label}: {error}"))?
        {
            return Ok(status);
        }
        if Instant::now() >= deadline {
            terminate(child, label)?;
            return Err(format!("{label} exceeded {}ms", timeout.as_millis()));
        }
        std::thread::sleep(Duration::from_millis(5));
    }
}

fn terminate(child: &mut Child, label: &str) -> Result<(), String> {
    if child
        .try_wait()
        .map_err(|error| format!("cannot inspect timed-out {label}: {error}"))?
        .is_some()
    {
        return Ok(());
    }
    if let Err(kill_error) = child.kill() {
        if child
            .try_wait()
            .map_err(|error| format!("cannot inspect {label} after failed kill: {error}"))?
            .is_none()
        {
            return Err(format!("cannot kill timed-out {label}: {kill_error}"));
        }
        return Ok(());
    }
    child
        .wait()
        .map_err(|error| format!("cannot reap timed-out {label}: {error}"))?;
    Ok(())
}

fn failed_process(label: &str, status: ExitStatus, captured: &CapturedProcess) -> String {
    format!(
        "{label} exited with {status}\n{}",
        captured_streams(captured),
    )
}

fn captured_streams(captured: &CapturedProcess) -> String {
    let stdout = captured.stdout().join("\n");
    format!(
        "stdout:\n{}\nstderr:\n{}",
        bounded_diagnostic(&stdout),
        bounded_diagnostic(captured.stderr()),
    )
}

fn bounded_diagnostic(stream: &str) -> String {
    if stream.len() <= DIAGNOSTIC_BYTE_LIMIT {
        return stream.to_owned();
    }
    let mut end = DIAGNOSTIC_BYTE_LIMIT;
    while !stream.is_char_boundary(end) {
        end -= 1;
    }
    format!(
        "{}\n...[{} bytes omitted]",
        &stream[..end],
        stream.len() - end
    )
}

#[cfg(test)]
mod tests;
