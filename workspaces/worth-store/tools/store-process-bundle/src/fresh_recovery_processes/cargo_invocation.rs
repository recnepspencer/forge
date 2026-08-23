use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use super::compiler_artifacts::{parse, CompilerTranscript};
use super::metadata_graph::Metadata;
use super::targets::TargetSpec;
use super::{BuildProfile, FreshProcessCargoTarget};

pub(crate) struct BuiltTarget<R> {
    pub(crate) path: PathBuf,
    pub(crate) transcript: CompilerTranscript,
    pub(crate) _role: std::marker::PhantomData<fn() -> R>,
}

pub(crate) fn build_target<R>(
    cargo: &OsStr,
    workspace: &Path,
    target_directory: &FreshProcessCargoTarget,
    metadata: &Metadata,
    target: &TargetSpec<R>,
    profile: BuildProfile,
) -> Result<BuiltTarget<R>, String> {
    let mut command = Command::new(cargo);
    command
        .current_dir(workspace)
        .env("CARGO_TARGET_DIR", target_directory.path())
        .env("RUSTFLAGS", warnings_denied_flags())
        .args([
            "build",
            "--locked",
            "-j",
            "1",
            "--message-format",
            "json",
            "--no-default-features",
        ]);
    if profile == BuildProfile::Release {
        command.arg("--release");
    }
    command.args(["-p", target.package, "--bin", target.binary]);
    if !target.features.is_empty() {
        command.arg("--features").arg(target.features.join(","));
    }
    let started = Instant::now();
    let output = command.output().map_err(|error| {
        format!(
            "spawn source-bound Cargo build for {}: {error}",
            target.binary
        )
    })?;
    if !output.status.success() {
        return Err(format!(
            "source-bound Cargo build for {} failed after {:?}:\nstdout:\n{}\nstderr:\n{}",
            target.binary,
            started.elapsed(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr),
        ));
    }
    let (transcript, path) = parse(output.stdout, metadata, target)?;
    Ok(BuiltTarget {
        path,
        transcript,
        _role: std::marker::PhantomData,
    })
}

fn warnings_denied_flags() -> String {
    let existing = std::env::var("RUSTFLAGS").unwrap_or_default();
    if existing.split_whitespace().any(|flag| flag == "-D") && existing.contains("warnings") {
        existing
    } else if existing.is_empty() {
        "-D warnings".to_owned()
    } else {
        format!("{existing} -D warnings")
    }
}
