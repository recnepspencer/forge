//! Spawns the external rail binary as a genuinely separate OS process and
//! discovers the TCP address it bound.
//!
//! This is the load-bearing proof for Gate 8.2: nothing here shares memory,
//! a runtime, or a truth source with the caller.

use std::io::{BufRead, BufReader};
use std::net::SocketAddr;
use std::path::Path;
use std::process::{Child, Command, Stdio};

use crate::protocol::support_profile::RailProtocolSupportProfile;

/// A running, separate-process external rail.
///
/// Killing the process is this handle's responsibility: dropping it always
/// terminates the child, so a test cannot leak a rail process past its own
/// scope.
pub struct RailProcessHandle {
    child: Child,
    local_addr: SocketAddr,
}

/// Failure to spawn the rail process or discover its bound address.
#[derive(Debug)]
pub enum RailSpawnError {
    Spawn(std::io::Error),
    NoStdout,
    ReadLine(std::io::Error),
    ProcessExitedBeforeListening,
    MalformedListeningLine(String),
}

impl RailProcessHandle {
    /// Spawns the rail binary at `binary_path` (for example
    /// `env!("CARGO_BIN_EXE_bank-external-rail")` from an integration test)
    /// with the given bind address (`"127.0.0.1:0"` lets the OS assign a
    /// free port) and waits for it to report the address it actually bound.
    pub fn spawn(binary_path: impl AsRef<Path>, bind_addr: &str) -> Result<Self, RailSpawnError> {
        Self::spawn_with_protocol_support(
            binary_path,
            bind_addr,
            RailProtocolSupportProfile::Current,
        )
    }

    pub fn spawn_with_protocol_support(
        binary_path: impl AsRef<Path>,
        bind_addr: &str,
        protocol_support: RailProtocolSupportProfile,
    ) -> Result<Self, RailSpawnError> {
        let mut child = Command::new(binary_path.as_ref())
            .arg(bind_addr)
            .arg(protocol_support.command_line_name())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .map_err(RailSpawnError::Spawn)?;

        let stdout = child.stdout.take().ok_or(RailSpawnError::NoStdout)?;
        let mut lines = BufReader::new(stdout).lines();
        let listening_line = match lines.next() {
            Some(Ok(line)) => line,
            Some(Err(error)) => return Err(RailSpawnError::ReadLine(error)),
            None => return Err(RailSpawnError::ProcessExitedBeforeListening),
        };
        let local_addr = match parse_listening_line(&listening_line) {
            Some(addr) => addr,
            None => return Err(RailSpawnError::MalformedListeningLine(listening_line)),
        };

        Ok(Self { child, local_addr })
    }

    pub fn pid(&self) -> u32 {
        self.child.id()
    }

    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }
}

impl Drop for RailProcessHandle {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn parse_listening_line(line: &str) -> Option<SocketAddr> {
    line.strip_prefix("LISTENING ")?.trim().parse().ok()
}
