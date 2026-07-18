use std::ffi::{OsStr, OsString};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use sha2::{Digest, Sha256};

use super::bounded_process;
use super::{UiCompilerToolIdentity, UiCompilerToolchainIdentity, UiProofRunFailure};

pub(super) fn observe(root: &Path) -> Result<UiCompilerToolchainIdentity, UiProofRunFailure> {
    Ok(UiCompilerToolchainIdentity {
        cargo: observe_program(root, selected_program("CARGO", "cargo"), &["-Vv"])?,
        rustc: observe_program(root, selected_program("RUSTC", "rustc"), &["-Vv"])?,
        version_probe_timeout_millis: 10_000,
        compile_timeout_millis: 300_000,
        output_cap_bytes_per_stream: 16 * 1024 * 1024,
        resource_posture: "one-process-per-fixture; shared-environment-target; offline; bounded-output"
            .to_owned(),
    })
}

fn observe_program(
    root: &Path,
    program: OsString,
    arguments: &[&str],
) -> Result<UiCompilerToolIdentity, UiProofRunFailure> {
    let path = resolve_program(&program).map_err(UiProofRunFailure::EnvironmentObservation)?;
    let executable_sha256 = file_digest(&path)?;
    let mut command = Command::new(&path);
    command.args(arguments).current_dir(root);
    let output = bounded_process::run(
        &mut command,
        Duration::from_secs(10),
        1024 * 1024,
    )
    .map_err(UiProofRunFailure::EnvironmentObservation)?;
    if output.timed_out || !output.status.success() {
        return Err(UiProofRunFailure::EnvironmentObservation(format!(
            "{} version probe failed with {:?}",
            path.display(),
            output.status.code()
        )));
    }
    let version_identity = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
    .trim()
    .to_owned();
    if version_identity.is_empty() {
        return Err(UiProofRunFailure::EnvironmentObservation(format!(
            "{} returned no version identity",
            path.display()
        )));
    }
    Ok(UiCompilerToolIdentity {
        executable_path: normalized_path(&path),
        executable_sha256,
        version_identity,
    })
}

fn selected_program(environment: &str, fallback: &str) -> OsString {
    std::env::var_os(environment).unwrap_or_else(|| OsString::from(fallback))
}

fn resolve_program(program: &OsStr) -> Result<PathBuf, String> {
    let path = Path::new(program);
    if path.is_absolute() || path.components().count() > 1 {
        return std::fs::canonicalize(path)
            .map_err(|error| format!("could not resolve {}: {error}", path.display()));
    }
    for root in std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()) {
        for extension in executable_extensions() {
            let candidate = root.join(format!("{}{}", path.to_string_lossy(), extension));
            if candidate.is_file() {
                return std::fs::canonicalize(&candidate)
                    .map_err(|error| format!("could not resolve {}: {error}", candidate.display()));
            }
        }
    }
    Err(format!("{} is not on PATH", path.display()))
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

fn file_digest(path: &Path) -> Result<String, UiProofRunFailure> {
    let mut file = std::fs::File::open(path)
        .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
    let mut digest = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|error| UiProofRunFailure::EnvironmentObservation(error.to_string()))?;
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
