use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use sha2::{Digest, Sha256};

use super::{
    interpret_tlc_output, ExecutedProtocolCheck, ExternalToolIdentity, PinnedTlcToolchain,
    ProtocolCheckArtifactIdentity, ProtocolCheckInvocation, ProtocolCheckVerdict,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlcRunnerPaths {
    java_executable: PathBuf,
    tool_jar: PathBuf,
    state_directory: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolRunnerFailure {
    MissingJavaExecutable,
    MissingToolJar,
    ToolDigestMismatch {
        actual: String,
    },
    MissingModel,
    MissingConfiguration,
    ProcessLaunch(String),
    ProcessTimedOut,
    ToolIdentityUnavailable(String),
    CheckerOutput {
        denial: super::ProtocolCheckerOutputDenial,
        output: String,
    },
}

impl TlcRunnerPaths {
    pub fn new(
        java_executable: impl Into<PathBuf>,
        tool_jar: impl Into<PathBuf>,
        state_directory: impl Into<PathBuf>,
    ) -> Self {
        Self {
            java_executable: java_executable.into(),
            tool_jar: tool_jar.into(),
            state_directory: state_directory.into(),
        }
    }
}

pub fn execute_protocol_check(
    invocation: &ProtocolCheckInvocation,
    runner: &TlcRunnerPaths,
) -> Result<ProtocolCheckVerdict, ProtocolRunnerFailure> {
    execute_protocol_check_with_identity(invocation, runner)
        .map(ExecutedProtocolCheck::into_verdict)
}

pub fn execute_protocol_check_with_identity(
    invocation: &ProtocolCheckInvocation,
    runner: &TlcRunnerPaths,
) -> Result<ExecutedProtocolCheck, ProtocolRunnerFailure> {
    require_file(
        &runner.java_executable,
        ProtocolRunnerFailure::MissingJavaExecutable,
    )?;
    require_file(&runner.tool_jar, ProtocolRunnerFailure::MissingToolJar)?;
    let tool_sha256 = require_pinned_tool_digest(&runner.tool_jar)?;
    require_file(invocation.model_path(), ProtocolRunnerFailure::MissingModel)?;
    require_file(
        invocation.configuration_path(),
        ProtocolRunnerFailure::MissingConfiguration,
    )?;
    let artifact_identity = ProtocolCheckArtifactIdentity::observed(
        file_digest(invocation.model_path())?,
        file_digest(invocation.configuration_path())?,
        tool_sha256,
    );
    let external_tool_identity = observe_external_tool_identity(
        &runner.java_executable,
        &runner.tool_jar,
        tool_sha256,
        invocation.bounds().maximum_runtime_millis().get(),
    )?;
    std::fs::create_dir_all(&runner.state_directory)
        .map_err(|error| ProtocolRunnerFailure::ProcessLaunch(error.to_string()))?;

    let mut child = Command::new(&runner.java_executable)
        .current_dir(
            invocation
                .model_path()
                .parent()
                .ok_or(ProtocolRunnerFailure::MissingModel)?,
        )
        .args(["-cp"])
        .arg(&runner.tool_jar)
        .args(["tlc2.TLC", "-deadlock", "-workers", "auto", "-metadir"])
        .arg(&runner.state_directory)
        .arg("-config")
        .arg(invocation.configuration_path())
        .arg(invocation.model_path())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| ProtocolRunnerFailure::ProcessLaunch(error.to_string()))?;
    let stdout = child.stdout.take().expect("piped checker stdout");
    let stderr = child.stderr.take().expect("piped checker stderr");
    let stdout_reader = std::thread::spawn(move || read_process_stream(stdout));
    let stderr_reader = std::thread::spawn(move || read_process_stream(stderr));
    let deadline =
        Instant::now() + Duration::from_millis(invocation.bounds().maximum_runtime_millis().get());
    let status = match wait_for_checker(&mut child, deadline) {
        Ok(status) => status,
        Err(failure) => {
            let _ = stdout_reader.join();
            let _ = stderr_reader.join();
            return Err(failure);
        }
    };
    let stdout = stdout_reader.join().map_err(|_| {
        ProtocolRunnerFailure::ProcessLaunch("checker stdout reader panicked".into())
    })??;
    let stderr = stderr_reader.join().map_err(|_| {
        ProtocolRunnerFailure::ProcessLaunch("checker stderr reader panicked".into())
    })??;
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&stdout),
        String::from_utf8_lossy(&stderr)
    );
    let verdict = interpret_tlc_output(
        invocation.protocol(),
        invocation.bounds(),
        &combined,
        status.success(),
    )
    .map_err(|denial| ProtocolRunnerFailure::CheckerOutput {
        denial,
        output: combined,
    })?;
    Ok(ExecutedProtocolCheck::observed(
        invocation.clone(),
        artifact_identity,
        external_tool_identity,
        verdict,
    ))
}

fn observe_external_tool_identity(
    java_executable: &Path,
    tool_jar: &Path,
    tool_sha256: [u8; 32],
    timeout_millis: u64,
) -> Result<ExternalToolIdentity, ProtocolRunnerFailure> {
    let executable_path = std::fs::canonicalize(java_executable)
        .map_err(|error| ProtocolRunnerFailure::ToolIdentityUnavailable(error.to_string()))?;
    let tool_artifact_path = std::fs::canonicalize(tool_jar)
        .map_err(|error| ProtocolRunnerFailure::ToolIdentityUnavailable(error.to_string()))?;
    let executable_sha256 = file_digest(&executable_path)?;
    let version = Command::new(&executable_path)
        .arg("-version")
        .output()
        .map_err(|error| ProtocolRunnerFailure::ToolIdentityUnavailable(error.to_string()))?;
    if !version.status.success() {
        return Err(ProtocolRunnerFailure::ToolIdentityUnavailable(
            "java -version did not complete successfully".to_owned(),
        ));
    }
    let executable_version = format!(
        "{}{}",
        String::from_utf8_lossy(&version.stdout),
        String::from_utf8_lossy(&version.stderr)
    )
    .trim()
    .to_owned();
    if executable_version.is_empty() {
        return Err(ProtocolRunnerFailure::ToolIdentityUnavailable(
            "java -version returned no identity".to_owned(),
        ));
    }
    Ok(ExternalToolIdentity::observed(
        "tlc2.TLC",
        PinnedTlcToolchain::VERSION,
        PinnedTlcToolchain::DOWNLOAD_URL,
        executable_path,
        executable_sha256,
        executable_version,
        tool_artifact_path,
        tool_sha256,
        timeout_millis,
        "workers=auto; deadlock-check=true; state-directory=attempt-scoped",
    ))
}

fn wait_for_checker(
    child: &mut Child,
    deadline: Instant,
) -> Result<ExitStatus, ProtocolRunnerFailure> {
    loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| ProtocolRunnerFailure::ProcessLaunch(error.to_string()))?
        {
            return Ok(status);
        }
        if Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            return Err(ProtocolRunnerFailure::ProcessTimedOut);
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn read_process_stream(mut stream: impl Read) -> Result<Vec<u8>, ProtocolRunnerFailure> {
    let mut bytes = Vec::new();
    stream
        .read_to_end(&mut bytes)
        .map_err(|error| ProtocolRunnerFailure::ProcessLaunch(error.to_string()))?;
    Ok(bytes)
}

fn require_pinned_tool_digest(path: &Path) -> Result<[u8; 32], ProtocolRunnerFailure> {
    let digest = file_digest(path)?;
    let actual = hex_digest(&digest);
    if actual == PinnedTlcToolchain::SHA256 {
        Ok(digest)
    } else {
        Err(ProtocolRunnerFailure::ToolDigestMismatch { actual })
    }
}

fn file_digest(path: &Path) -> Result<[u8; 32], ProtocolRunnerFailure> {
    let bytes = std::fs::read(path)
        .map_err(|error| ProtocolRunnerFailure::ProcessLaunch(error.to_string()))?;
    Ok(Sha256::digest(bytes).into())
}

fn hex_digest(digest: &[u8; 32]) -> String {
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn require_file(path: &Path, denial: ProtocolRunnerFailure) -> Result<(), ProtocolRunnerFailure> {
    if path.is_file() {
        Ok(())
    } else {
        Err(denial)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checker_process_is_killed_when_its_runtime_bound_expires() {
        let mut child = sleeping_child();
        let result = wait_for_checker(&mut child, Instant::now());

        assert_eq!(result, Err(ProtocolRunnerFailure::ProcessTimedOut));
        assert!(child.try_wait().unwrap().is_some());
    }

    #[cfg(windows)]
    fn sleeping_child() -> Child {
        Command::new("cmd")
            .args(["/C", "ping -n 30 127.0.0.1 >NUL"])
            .spawn()
            .unwrap()
    }

    #[cfg(unix)]
    fn sleeping_child() -> Child {
        Command::new("sh").args(["-c", "sleep 30"]).spawn().unwrap()
    }
}
