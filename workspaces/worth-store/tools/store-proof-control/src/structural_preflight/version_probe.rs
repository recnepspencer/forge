use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Serialize)]
pub(super) struct ObservedProgramVersion {
    pub program_path: String,
    pub program_sha256: String,
    pub version_output: String,
    pub timeout_millis: u64,
}

pub(super) fn observe(
    root: &Path,
    program: &str,
    arguments: &[&str],
) -> Result<ObservedProgramVersion, String> {
    const TIMEOUT: Duration = Duration::from_secs(10);
    let program_path = resolve_program(program)?;
    let program_sha256 = file_digest(&program_path)?;
    let mut child = Command::new(&program_path)
        .args(arguments)
        .current_dir(root)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("could not launch {program}: {error}"))?;
    let stdout = child.stdout.take().expect("piped version-probe stdout");
    let stderr = child.stderr.take().expect("piped version-probe stderr");
    let stdout_reader = std::thread::spawn(move || read_stream(stdout));
    let stderr_reader = std::thread::spawn(move || read_stream(stderr));
    let status = wait_bounded(&mut child, TIMEOUT)?;
    let stdout = join_stream(stdout_reader, program, "stdout")?;
    let stderr = join_stream(stderr_reader, program, "stderr")?;
    if !status.success() {
        return Err(format!(
            "{program} identity failed with {:?}: {}",
            status.code(),
            String::from_utf8_lossy(&stderr)
        ));
    }
    let version_output = format!(
        "{}{}",
        String::from_utf8_lossy(&stdout),
        String::from_utf8_lossy(&stderr)
    )
    .trim()
    .to_owned();
    if version_output.is_empty() {
        return Err(format!("{program} returned no version identity"));
    }
    Ok(ObservedProgramVersion {
        program_path: normalized_path(&program_path),
        program_sha256,
        version_output,
        timeout_millis: TIMEOUT.as_millis() as u64,
    })
}

fn wait_bounded(child: &mut std::process::Child, timeout: Duration) -> Result<ExitStatus, String> {
    let deadline = Instant::now() + timeout;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => return Ok(status),
            Ok(None) if Instant::now() < deadline => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Ok(None) => {
                let kill_error = child.kill().err();
                child
                    .wait()
                    .map_err(|error| format!("could not reap timed-out version probe: {error}"))?;
                if let Some(error) = kill_error {
                    return Err(format!("could not kill timed-out version probe: {error}"));
                }
                return Err(format!(
                    "version probe exceeded its {}ms timeout",
                    timeout.as_millis()
                ));
            }
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(format!("could not observe version probe: {error}"));
            }
        }
    }
}

fn read_stream(mut stream: impl Read) -> std::io::Result<Vec<u8>> {
    const MAX_VERSION_BYTES: usize = 1024 * 1024;
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 16 * 1024];
    loop {
        let read = stream.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        if bytes.len().saturating_add(read) > MAX_VERSION_BYTES {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "version output exceeded 1 MiB",
            ));
        }
        bytes.extend_from_slice(&buffer[..read]);
    }
    Ok(bytes)
}

fn join_stream(
    reader: std::thread::JoinHandle<std::io::Result<Vec<u8>>>,
    program: &str,
    stream: &str,
) -> Result<Vec<u8>, String> {
    reader
        .join()
        .map_err(|_| format!("{program} {stream} reader panicked"))?
        .map_err(|error| format!("could not read {program} {stream}: {error}"))
}

fn resolve_program(program: &str) -> Result<PathBuf, String> {
    let path = Path::new(program);
    if path.is_absolute() || path.components().count() > 1 {
        return std::fs::canonicalize(path)
            .map_err(|error| format!("could not resolve {program}: {error}"));
    }
    for root in std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()) {
        for extension in executable_extensions() {
            let candidate = root.join(format!("{program}{extension}"));
            if candidate.is_file() {
                return std::fs::canonicalize(&candidate)
                    .map_err(|error| format!("could not resolve {program}: {error}"));
            }
        }
    }
    Err(format!("{program} is not on PATH"))
}

fn executable_extensions() -> Vec<String> {
    if cfg!(windows) {
        let mut extensions = std::env::var("PATHEXT")
            .unwrap_or_else(|_| ".COM;.EXE;.BAT;.CMD".to_owned())
            .split(';')
            .map(str::to_ascii_lowercase)
            .collect::<Vec<_>>();
        extensions.insert(0, String::new());
        extensions
    } else {
        vec![String::new()]
    }
}

fn file_digest(path: &Path) -> Result<String, String> {
    let mut file = std::fs::File::open(path)
        .map_err(|error| format!("could not read {}: {error}", path.display()))?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|error| format!("could not read {}: {error}", path.display()))?;
        if read == 0 {
            break;
        }
        digest.update(&buffer[..read]);
    }
    Ok(format!("{:x}", digest.finalize()))
}

fn normalized_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
