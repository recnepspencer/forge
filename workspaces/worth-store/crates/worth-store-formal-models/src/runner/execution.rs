use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use sha2::{Digest, Sha256};

use super::{
    interpret_tlc_output, ExecutedProtocolCheck, ExternalToolIdentity, ExternalToolObservation,
    ExternalToolResourcePosture, PinnedTlcToolchain, ProtocolCheckArtifactIdentity,
    ProtocolCheckInvocation, ProtocolCheckVerdict,
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
    StateDirectoryNotFresh,
    ProcessLaunch(String),
    ProcessTimedOut,
    ToolIdentityUnavailable(String),
    ArtifactChangedDuringExecution(&'static str),
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
        &runner.state_directory,
        tool_sha256,
        invocation.bounds().maximum_runtime_millis().get(),
    )?;
    prepare_state_directory(&runner.state_directory)?;

    let mut child = Command::new(external_tool_identity.executable_path())
        .current_dir(
            invocation
                .model_path()
                .parent()
                .ok_or(ProtocolRunnerFailure::MissingModel)?,
        )
        .args(["-cp"])
        .arg(external_tool_identity.tool_artifact_path())
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
    verify_execution_artifacts(invocation, &artifact_identity, &external_tool_identity)?;
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
    state_directory: &Path,
    tool_sha256: [u8; 32],
    timeout_millis: u64,
) -> Result<ExternalToolIdentity, ProtocolRunnerFailure> {
    let executable_path = std::fs::canonicalize(java_executable)
        .map_err(|error| ProtocolRunnerFailure::ToolIdentityUnavailable(error.to_string()))?;
    let tool_artifact_path = std::fs::canonicalize(tool_jar)
        .map_err(|error| ProtocolRunnerFailure::ToolIdentityUnavailable(error.to_string()))?;
    let executable_sha256 = file_digest(&executable_path)?;
    let version =
        command_output_with_timeout(&executable_path, &["-version"], Duration::from_secs(10))?;
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
    Ok(ExternalToolIdentity::observed(ExternalToolObservation {
        adapter_name: "tlc2.TLC".to_owned(),
        adapter_version: PinnedTlcToolchain::VERSION.to_owned(),
        provenance: PinnedTlcToolchain::DOWNLOAD_URL.to_owned(),
        executable_path,
        executable_sha256,
        executable_version,
        tool_artifact_path,
        tool_artifact_sha256: tool_sha256,
        timeout_millis,
        resource_posture: ExternalToolResourcePosture::tlc(state_directory),
    }))
}

fn verify_execution_artifacts(
    invocation: &ProtocolCheckInvocation,
    artifact: &ProtocolCheckArtifactIdentity,
    tool: &ExternalToolIdentity,
) -> Result<(), ProtocolRunnerFailure> {
    for (name, path, expected) in [
        ("model", invocation.model_path(), artifact.model_sha256()),
        (
            "configuration",
            invocation.configuration_path(),
            artifact.configuration_sha256(),
        ),
        (
            "java executable",
            tool.executable_path(),
            tool.executable_sha256(),
        ),
        (
            "TLC artifact",
            tool.tool_artifact_path(),
            tool.tool_artifact_sha256(),
        ),
    ] {
        if &file_digest(path)? != expected {
            return Err(ProtocolRunnerFailure::ArtifactChangedDuringExecution(name));
        }
    }
    Ok(())
}

fn prepare_state_directory(path: &Path) -> Result<(), ProtocolRunnerFailure> {
    let parent = path
        .parent()
        .ok_or(ProtocolRunnerFailure::StateDirectoryNotFresh)?;
    std::fs::create_dir_all(parent)
        .map_err(|error| ProtocolRunnerFailure::ProcessLaunch(error.to_string()))?;
    std::fs::create_dir(path).map_err(|error| {
        if error.kind() == std::io::ErrorKind::AlreadyExists {
            ProtocolRunnerFailure::StateDirectoryNotFresh
        } else {
            ProtocolRunnerFailure::ProcessLaunch(error.to_string())
        }
    })
}

fn command_output_with_timeout(
    executable: &Path,
    arguments: &[&str],
    timeout: Duration,
) -> Result<std::process::Output, ProtocolRunnerFailure> {
    let mut child = Command::new(executable)
        .args(arguments)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| ProtocolRunnerFailure::ToolIdentityUnavailable(error.to_string()))?;
    let stdout = child.stdout.take().expect("piped identity stdout");
    let stderr = child.stderr.take().expect("piped identity stderr");
    let stdout_reader = std::thread::spawn(move || read_process_stream(stdout));
    let stderr_reader = std::thread::spawn(move || read_process_stream(stderr));
    let status = match wait_for_checker(&mut child, Instant::now() + timeout) {
        Ok(status) => status,
        Err(failure) => {
            let _ = stdout_reader.join();
            let _ = stderr_reader.join();
            return Err(failure);
        }
    };
    let stdout = stdout_reader.join().map_err(|_| {
        ProtocolRunnerFailure::ToolIdentityUnavailable("stdout reader panicked".into())
    })??;
    let stderr = stderr_reader.join().map_err(|_| {
        ProtocolRunnerFailure::ToolIdentityUnavailable("stderr reader panicked".into())
    })??;
    Ok(std::process::Output {
        status,
        stdout,
        stderr,
    })
}

fn wait_for_checker(
    child: &mut Child,
    deadline: Instant,
) -> Result<ExitStatus, ProtocolRunnerFailure> {
    loop {
        let status = match child.try_wait() {
            Ok(status) => status,
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(ProtocolRunnerFailure::ProcessLaunch(error.to_string()));
            }
        };
        if let Some(status) = status {
            return Ok(status);
        }
        if Instant::now() >= deadline {
            let kill_error = child.kill().err();
            child
                .wait()
                .map_err(|error| ProtocolRunnerFailure::ProcessLaunch(error.to_string()))?;
            if let Some(error) = kill_error {
                return Err(ProtocolRunnerFailure::ProcessLaunch(format!(
                    "could not kill timed-out checker: {error}"
                )));
            }
            return Err(ProtocolRunnerFailure::ProcessTimedOut);
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn read_process_stream(mut stream: impl Read) -> Result<Vec<u8>, ProtocolRunnerFailure> {
    const MAX_CHECKER_OUTPUT_BYTES: usize = 64 * 1024 * 1024;
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = stream
            .read(&mut buffer)
            .map_err(|error| ProtocolRunnerFailure::ProcessLaunch(error.to_string()))?;
        if read == 0 {
            break;
        }
        if bytes.len().saturating_add(read) > MAX_CHECKER_OUTPUT_BYTES {
            return Err(ProtocolRunnerFailure::ProcessLaunch(
                "checker output exceeded 64 MiB".to_owned(),
            ));
        }
        bytes.extend_from_slice(&buffer[..read]);
    }
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
    let mut file = std::fs::File::open(path)
        .map_err(|error| ProtocolRunnerFailure::ProcessLaunch(error.to_string()))?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|error| ProtocolRunnerFailure::ProcessLaunch(error.to_string()))?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(digest.finalize().into())
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
