//! Bounded process-tree ownership for compiler and witness commands.

use command_group::{CommandGroup, GroupChild};
use std::io::{Read, Write};
use std::process::{Command, ExitStatus, Output, Stdio};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::time::{Duration, Instant};

const POLL_INTERVAL: Duration = Duration::from_millis(10);
const TERMINATION_GRACE: Duration = Duration::from_secs(2);
const READ_CHUNK_BYTES: usize = 8192;
const CHANNEL_MESSAGES: usize = 8;
const STREAM_POLL_MESSAGES: usize = 4;

#[derive(Clone, Copy)]
pub(super) struct Limits {
    timeout: Duration,
    max_output_bytes: usize,
}

impl Limits {
    pub(super) fn new(timeout: Duration, max_output_bytes: usize) -> Self {
        Self {
            timeout,
            max_output_bytes,
        }
    }

    pub(super) fn max_output_bytes(self) -> usize {
        self.max_output_bytes
    }
}

pub(super) fn run(
    command: &mut Command,
    stdin: Option<&[u8]>,
    limits: Limits,
    label: &str,
) -> Result<Output, String> {
    let deadline = Instant::now() + limits.timeout;
    let mut process = ProcessTree::spawn(command, stdin, limits.max_output_bytes, label)?;
    process.wait(deadline, limits.timeout, label)
}

struct ProcessTree {
    child: GroupChild,
    status: Option<ExitStatus>,
    stdout: CapturedStream,
    stderr: CapturedStream,
    max_output_bytes: usize,
}

impl ProcessTree {
    fn spawn(
        command: &mut Command,
        stdin: Option<&[u8]>,
        max_output_bytes: usize,
        label: &str,
    ) -> Result<Self, String> {
        let mut child = command
            .stdin(if stdin.is_some() {
                Stdio::piped()
            } else {
                Stdio::null()
            })
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .group_spawn()
            .map_err(|error| format!("start {label} process tree: {error}"))?;
        let stdout = child
            .inner()
            .stdout
            .take()
            .ok_or_else(|| format!("{label} stdout is unavailable"))?;
        let stderr = child
            .inner()
            .stderr
            .take()
            .ok_or_else(|| format!("{label} stderr is unavailable"))?;
        let mut process = Self {
            child,
            status: None,
            stdout: CapturedStream::start(stdout),
            stderr: CapturedStream::start(stderr),
            max_output_bytes,
        };
        if let Some(input) = stdin {
            let Some(mut child_stdin) = process.child.inner().stdin.take() else {
                let _ = process.child.kill();
                return Err(format!("{label} stdin is unavailable"));
            };
            let write_result = child_stdin.write_all(input);
            if let Err(error) = write_result {
                let _ = process.child.kill();
                return Err(format!("write {label} stdin: {error}"));
            }
        }
        Ok(process)
    }

    fn wait(
        &mut self,
        deadline: Instant,
        timeout: Duration,
        label: &str,
    ) -> Result<Output, String> {
        loop {
            if let Err(reason) = self.poll(label) {
                return Err(self.terminate(reason, label));
            }
            if self.is_complete() {
                let output = self.output();
                let _ = self.child.kill();
                return Ok(output);
            }
            if Instant::now() >= deadline {
                return Err(self.terminate(TerminationReason::Timeout(timeout), label));
            }
            std::thread::sleep(POLL_INTERVAL);
        }
    }

    fn poll(&mut self, label: &str) -> Result<(), TerminationReason> {
        if self.status.is_none() {
            self.status = self.child.try_wait().map_err(|error| {
                TerminationReason::Collection(format!("poll {label} process tree: {error}"))
            })?;
        }
        let stdout_remaining = self.max_output_bytes.saturating_sub(self.captured_bytes());
        self.stdout
            .poll("stdout", label, stdout_remaining)
            .map_err(|error| self.capture_failure(error))?;
        let stderr_remaining = self.max_output_bytes.saturating_sub(self.captured_bytes());
        self.stderr
            .poll("stderr", label, stderr_remaining)
            .map_err(|error| self.capture_failure(error))?;
        Ok(())
    }

    fn capture_failure(&self, failure: StreamPollFailure) -> TerminationReason {
        match failure {
            StreamPollFailure::OutputLimit => TerminationReason::OutputLimit(self.max_output_bytes),
            StreamPollFailure::Collection(error) => TerminationReason::Collection(error),
        }
    }

    fn captured_bytes(&self) -> usize {
        self.stdout.bytes.len() + self.stderr.bytes.len()
    }

    fn terminate(&mut self, reason: TerminationReason, label: &str) -> String {
        let kill_error = self.child.kill().err();
        let cleanup_deadline = Instant::now() + TERMINATION_GRACE;
        let mut cleanup_error = None;
        while Instant::now() < cleanup_deadline {
            match self.poll_cleanup(label) {
                Ok(()) if self.is_complete() => break,
                Ok(()) => std::thread::sleep(POLL_INTERVAL),
                Err(error) => {
                    cleanup_error = Some(error);
                    break;
                }
            }
        }
        let mut diagnostic = reason.message(label);
        if let Some(error) = kill_error {
            diagnostic.push_str(&format!("; process-tree kill reported: {error}"));
        }
        if !self.is_complete() {
            diagnostic.push_str("; process-tree reap/output drain exceeded 2000 ms");
        }
        if let Some(error) = cleanup_error {
            diagnostic.push_str(&format!("; cleanup reported: {error}"));
        }
        diagnostic.push_str(&format!(
            "\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&self.stdout.bytes),
            String::from_utf8_lossy(&self.stderr.bytes)
        ));
        diagnostic
    }

    fn poll_cleanup(&mut self, label: &str) -> Result<(), String> {
        if self.status.is_none() {
            self.status = self
                .child
                .try_wait()
                .map_err(|error| format!("poll {label} process tree: {error}"))?;
        }
        self.stdout.discard_poll("stdout", label)?;
        self.stderr.discard_poll("stderr", label)?;
        Ok(())
    }

    fn is_complete(&self) -> bool {
        self.status.is_some() && self.stdout.done && self.stderr.done
    }

    fn output(&mut self) -> Output {
        Output {
            status: self.status.expect("complete process has exit status"),
            stdout: std::mem::take(&mut self.stdout.bytes),
            stderr: std::mem::take(&mut self.stderr.bytes),
        }
    }
}

enum TerminationReason {
    Timeout(Duration),
    OutputLimit(usize),
    Collection(String),
}

impl TerminationReason {
    fn message(self, label: &str) -> String {
        match self {
            Self::Timeout(timeout) => {
                format!("{label} timed out after {} ms", timeout.as_millis())
            }
            Self::OutputLimit(limit) => format!(
                "{label} exceeded configured output limit of {limit} bytes; captured output was truncated"
            ),
            Self::Collection(error) => error,
        }
    }
}

struct CapturedStream {
    receiver: Receiver<ReadEvent>,
    bytes: Vec<u8>,
    done: bool,
}

enum ReadEvent {
    Bytes(Vec<u8>),
    Done,
    Failed(String),
}

#[derive(Debug)]
enum StreamPollFailure {
    OutputLimit,
    Collection(String),
}

impl CapturedStream {
    fn start(mut stream: impl Read + Send + 'static) -> Self {
        let (sender, receiver) = mpsc::sync_channel(CHANNEL_MESSAGES);
        std::thread::spawn(move || {
            let mut buffer = [0_u8; READ_CHUNK_BYTES];
            loop {
                match stream.read(&mut buffer) {
                    Ok(0) => {
                        let _ = sender.send(ReadEvent::Done);
                        break;
                    }
                    Ok(count) => {
                        if sender
                            .send(ReadEvent::Bytes(buffer[..count].to_vec()))
                            .is_err()
                        {
                            break;
                        }
                    }
                    Err(error) => {
                        let _ = sender.send(ReadEvent::Failed(error.to_string()));
                        break;
                    }
                }
            }
        });
        Self {
            receiver,
            bytes: Vec::new(),
            done: false,
        }
    }

    fn poll(
        &mut self,
        stream: &str,
        label: &str,
        remaining_output_bytes: usize,
    ) -> Result<(), StreamPollFailure> {
        let mut captured_this_poll = 0;
        for _ in 0..STREAM_POLL_MESSAGES {
            match self.receiver.try_recv() {
                Ok(ReadEvent::Bytes(bytes)) => {
                    let remaining = remaining_output_bytes.saturating_sub(captured_this_poll);
                    if bytes.len() > remaining {
                        self.bytes.extend(&bytes[..remaining]);
                        return Err(StreamPollFailure::OutputLimit);
                    }
                    captured_this_poll += bytes.len();
                    self.bytes.extend(bytes);
                }
                Ok(ReadEvent::Done) => {
                    self.done = true;
                    return Ok(());
                }
                Ok(ReadEvent::Failed(error)) => {
                    return Err(StreamPollFailure::Collection(format!(
                        "collect {label} {stream}: {error}"
                    )));
                }
                Err(TryRecvError::Empty) => return Ok(()),
                Err(TryRecvError::Disconnected) if self.done => return Ok(()),
                Err(TryRecvError::Disconnected) => {
                    return Err(StreamPollFailure::Collection(format!(
                        "collect {label} {stream}: reader disconnected"
                    )));
                }
            }
        }
        Ok(())
    }

    fn discard_poll(&mut self, stream: &str, label: &str) -> Result<(), String> {
        for _ in 0..STREAM_POLL_MESSAGES {
            match self.receiver.try_recv() {
                Ok(ReadEvent::Bytes(_)) => {}
                Ok(ReadEvent::Done) => {
                    self.done = true;
                    return Ok(());
                }
                Ok(ReadEvent::Failed(error)) => {
                    return Err(format!("collect {label} {stream}: {error}"));
                }
                Err(TryRecvError::Empty) => return Ok(()),
                Err(TryRecvError::Disconnected) if self.done => return Ok(()),
                Err(TryRecvError::Disconnected) => {
                    return Err(format!("collect {label} {stream}: reader disconnected"));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CapturedStream, ReadEvent, CHANNEL_MESSAGES, READ_CHUNK_BYTES, STREAM_POLL_MESSAGES,
    };
    use std::sync::mpsc;

    #[test]
    fn one_stream_poll_processes_a_bounded_batch() {
        let (sender, receiver) = mpsc::sync_channel(CHANNEL_MESSAGES);
        for _ in 0..CHANNEL_MESSAGES {
            sender
                .try_send(ReadEvent::Bytes(vec![b'x'; READ_CHUNK_BYTES]))
                .expect("preload bounded capture channel");
        }
        drop(sender);
        let mut stream = CapturedStream {
            receiver,
            bytes: Vec::new(),
            done: false,
        };
        stream
            .poll("stdout", "batch test", usize::MAX)
            .expect("poll bounded batch");
        assert_eq!(stream.bytes.len(), STREAM_POLL_MESSAGES * READ_CHUNK_BYTES);
    }
}
