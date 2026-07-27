use std::fmt;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use serde::Serialize;

use crate::external_observation::LifecycleFailureSnapshot;
use crate::native_platform::{current_platform_posture, NativePlatformPosture};

use super::report::{ExecutableWorldFailureTeardown, PulseExecutableWorldFailure};

const MAXIMUM_ARTIFACT_BYTES: usize = 64 * 1_024 * 1_024;
static NEXT_ARTIFACT: AtomicU64 = AtomicU64::new(1);

#[derive(Debug)]
pub(crate) struct RetainedFailureArtifact {
    root: PathBuf,
    retained_bytes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FailureArtifactDiscardEvidence {
    removed_owned_root: bool,
}

#[derive(Debug)]
pub(crate) enum FailureArtifactFailure {
    Encode(serde_json::Error),
    BudgetExceeded(usize),
    CreateRoot(std::io::Error),
    WriteFile {
        path: PathBuf,
        error: std::io::Error,
    },
    SyncFile {
        path: PathBuf,
        error: std::io::Error,
    },
    Cleanup(std::io::Error),
    Residue(PathBuf),
}

pub(super) struct FailureArtifactInputs {
    pub(super) source_snapshot: Option<Box<[u8]>>,
    pub(super) lifecycle: Option<LifecycleFailureSnapshot>,
}

#[derive(Serialize)]
struct FailureManifest<'a> {
    schema: &'static str,
    primary: String,
    teardown: String,
    environment: FailureEnvironment,
    lifecycle: Option<FailureLifecycle<'a>>,
    source_snapshot: Option<&'static str>,
    retained_by_default: bool,
    maximum_artifact_bytes: usize,
}

#[derive(Serialize)]
struct FailureEnvironment {
    operating_system: &'static str,
    architecture: &'static str,
    native_posture: &'static str,
}

#[derive(Serialize)]
struct FailureLifecycle<'a> {
    accepted_events: usize,
    accepted_bytes: usize,
    trace: &'a [crate::external_observation::LifecycleTraceEntry],
}

impl FailureArtifactInputs {
    pub(super) fn none() -> Self {
        Self {
            source_snapshot: None,
            lifecycle: None,
        }
    }
}

impl RetainedFailureArtifact {
    pub(super) fn create(
        primary: &PulseExecutableWorldFailure,
        teardown: &ExecutableWorldFailureTeardown,
        inputs: FailureArtifactInputs,
    ) -> Result<Self, FailureArtifactFailure> {
        let manifest = manifest(primary, teardown, &inputs);
        let manifest_bytes =
            serde_json::to_vec_pretty(&manifest).map_err(FailureArtifactFailure::Encode)?;
        let source_bytes = inputs.source_snapshot.as_deref().unwrap_or_default();
        let retained_bytes = manifest_bytes.len().saturating_add(source_bytes.len());
        if retained_bytes > MAXIMUM_ARTIFACT_BYTES {
            return Err(FailureArtifactFailure::BudgetExceeded(retained_bytes));
        }
        let root = artifact_root();
        fs::create_dir(&root).map_err(FailureArtifactFailure::CreateRoot)?;
        if let Err(failure) = write_bundle(&root, &manifest_bytes, source_bytes) {
            let cleanup = fs::remove_dir_all(&root);
            return match cleanup {
                Ok(()) => Err(failure),
                Err(error) => Err(FailureArtifactFailure::Cleanup(error)),
            };
        }
        Ok(Self {
            root,
            retained_bytes,
        })
    }

    pub(crate) fn path(&self) -> &Path {
        &self.root
    }

    pub(crate) fn retained_bytes(&self) -> usize {
        self.retained_bytes
    }

    pub(crate) fn maximum_bytes(&self) -> usize {
        MAXIMUM_ARTIFACT_BYTES
    }

    pub(crate) fn discard(self) -> Result<FailureArtifactDiscardEvidence, FailureArtifactFailure> {
        fs::remove_dir_all(&self.root).map_err(FailureArtifactFailure::Cleanup)?;
        if self.root.exists() {
            return Err(FailureArtifactFailure::Residue(self.root));
        }
        Ok(FailureArtifactDiscardEvidence {
            removed_owned_root: true,
        })
    }
}

impl FailureArtifactDiscardEvidence {
    pub(crate) fn removed_owned_root(self) -> bool {
        self.removed_owned_root
    }
}

fn manifest<'a>(
    primary: &PulseExecutableWorldFailure,
    teardown: &ExecutableWorldFailureTeardown,
    inputs: &'a FailureArtifactInputs,
) -> FailureManifest<'a> {
    let lifecycle = inputs.lifecycle.as_ref().map(|snapshot| {
        let measurement = snapshot.measurement();
        FailureLifecycle {
            accepted_events: measurement.accepted_events(),
            accepted_bytes: measurement.accepted_bytes(),
            trace: snapshot.trace(),
        }
    });
    FailureManifest {
        schema: "worth-ui.platform-pulse.failure-artifact.v1",
        primary: primary.to_string(),
        teardown: teardown.to_string(),
        environment: FailureEnvironment {
            operating_system: std::env::consts::OS,
            architecture: std::env::consts::ARCH,
            native_posture: posture_name(current_platform_posture()),
        },
        lifecycle,
        source_snapshot: inputs.source_snapshot.as_ref().map(|_| "source.wui"),
        retained_by_default: true,
        maximum_artifact_bytes: MAXIMUM_ARTIFACT_BYTES,
    }
}

fn posture_name(posture: NativePlatformPosture) -> &'static str {
    match posture {
        NativePlatformPosture::CertifiedExecutable => "certified_executable",
    }
}

fn artifact_root() -> PathBuf {
    let ordinal = NEXT_ARTIFACT.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "worth-ui-platform-pulse-failure-{}-{ordinal}",
        std::process::id()
    ))
}

fn write_bundle(
    root: &Path,
    manifest_bytes: &[u8],
    source_bytes: &[u8],
) -> Result<(), FailureArtifactFailure> {
    write_synced(&root.join("manifest.json"), manifest_bytes)?;
    if !source_bytes.is_empty() {
        write_synced(&root.join("source.wui"), source_bytes)?;
    }
    Ok(())
}

fn write_synced(path: &Path, bytes: &[u8]) -> Result<(), FailureArtifactFailure> {
    let mut file = File::create(path).map_err(|error| FailureArtifactFailure::WriteFile {
        path: path.to_owned(),
        error,
    })?;
    file.write_all(bytes)
        .map_err(|error| FailureArtifactFailure::WriteFile {
            path: path.to_owned(),
            error,
        })?;
    file.sync_all()
        .map_err(|error| FailureArtifactFailure::SyncFile {
            path: path.to_owned(),
            error,
        })
}

impl fmt::Display for FailureArtifactFailure {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Encode(error) => write!(formatter, "encode failure manifest: {error}"),
            Self::BudgetExceeded(bytes) => {
                write!(
                    formatter,
                    "failure artifact is {bytes} bytes; limit is {MAXIMUM_ARTIFACT_BYTES}"
                )
            }
            Self::CreateRoot(error) => write!(formatter, "create failure artifact root: {error}"),
            Self::WriteFile { path, error } => {
                write!(formatter, "write {}: {error}", path.display())
            }
            Self::SyncFile { path, error } => {
                write!(formatter, "sync {}: {error}", path.display())
            }
            Self::Cleanup(error) => write!(formatter, "remove failure artifact: {error}"),
            Self::Residue(path) => {
                write!(formatter, "failure artifact remained: {}", path.display())
            }
        }
    }
}
