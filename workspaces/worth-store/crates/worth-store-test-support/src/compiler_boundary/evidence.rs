use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{CheckedCompilerDiagnostic, UiFixtureIdentity};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiProofRunEvidence {
    pub schema_version: u32,
    pub suite_identity: String,
    pub environment_identity: String,
    pub environment_root_identity: String,
    pub profile_identity: String,
    pub toolchain: UiCompilerToolchainIdentity,
    pub environment_manifest_path: String,
    pub environment_lock_path: String,
    pub environment_lock_sha256: String,
    pub shared_target_root: String,
    pub environment_manifest_created: bool,
    pub environment_lock_created: bool,
    pub fixtures: Vec<UiFixtureRunEvidence>,
    pub evidence_identity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiCompilerToolchainIdentity {
    pub cargo: UiCompilerToolIdentity,
    pub rustc: UiCompilerToolIdentity,
    pub cargo_configuration: Vec<UiCargoConfigurationIdentity>,
    pub version_probe_timeout_millis: u64,
    pub compile_timeout_millis: u64,
    pub output_cap_bytes_per_stream: usize,
    pub resource_posture: UiCompilerResourcePosture,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiCargoConfigurationIdentity {
    pub path: String,
    pub content_sha256: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiCompilerResourcePosture {
    pub one_process_per_fixture: bool,
    pub shared_environment_target: bool,
    pub offline: bool,
    pub locked_dependencies: bool,
    pub declared_profile_applied: bool,
    pub bounded_output: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiCompilerToolIdentity {
    pub executable_path: String,
    pub executable_sha256: String,
    pub version_identity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiFixtureRunEvidence {
    pub fixture: UiFixtureIdentity,
    pub cargo_process_id: u32,
    pub cargo_exit_code: Option<i32>,
    pub cargo_stdout_sha256: String,
    pub cargo_stderr_sha256: String,
    pub dependency_artifacts_compiled: usize,
    pub dependency_artifacts_reused: usize,
    pub target_artifact_count_before: usize,
    pub target_artifact_count_after: usize,
    pub diagnostics: Vec<CheckedCompilerDiagnostic>,
    pub semantic_denial_matched: bool,
    pub evidence_path: String,
}

#[derive(Debug)]
pub enum UiProofRunFailure {
    InvalidDeclaration(String),
    EnvironmentObservation(String),
    FixtureRead(String),
    CompilerLaunch(String),
    CompilerTimedOut(String),
    UnexpectedCompilerSuccess(String),
    WrongCompilerDenial {
        fixture: String,
        reason: String,
        diagnostics: Vec<CheckedCompilerDiagnostic>,
    },
    EvidenceWrite(String),
}

impl std::fmt::Display for UiProofRunFailure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDeclaration(reason) => write!(formatter, "invalid UI declaration: {reason}"),
            Self::EnvironmentObservation(reason) => {
                write!(formatter, "could not establish UI environment: {reason}")
            }
            Self::FixtureRead(reason) => write!(formatter, "could not read UI fixture: {reason}"),
            Self::CompilerLaunch(reason) => write!(formatter, "could not launch UI compiler: {reason}"),
            Self::CompilerTimedOut(fixture) => {
                write!(formatter, "UI fixture {fixture} exceeded its compiler timeout")
            }
            Self::UnexpectedCompilerSuccess(fixture) => {
                write!(formatter, "UI fixture {fixture} unexpectedly compiled")
            }
            Self::WrongCompilerDenial {
                fixture,
                reason,
                diagnostics,
            } => write!(
                formatter,
                "UI fixture {fixture} failed for the wrong reason: {reason}; diagnostics={diagnostics:?}"
            ),
            Self::EvidenceWrite(reason) => write!(formatter, "could not persist UI evidence: {reason}"),
        }
    }
}

impl std::error::Error for UiProofRunFailure {}

impl UiProofRunEvidence {
    pub fn validate_integrity(&self) -> Result<(), String> {
        let fixture_identities = self
            .fixtures
            .iter()
            .map(|fixture| fixture.fixture.case_identity.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        let fixture_evidence_paths = self
            .fixtures
            .iter()
            .map(|fixture| fixture.evidence_path.as_str())
            .collect::<std::collections::BTreeSet<_>>();
        let expected_environment_identity = serde_json::to_vec(&(
            "worth-store-ui-environment-v5",
            &self.environment_root_identity,
            &self.environment_lock_sha256,
        ))
        .map(|bytes| format!("{:x}", Sha256::digest(bytes)))
        .map_err(|error| format!("could not validate UI environment identity: {error}"))?;
        if self.schema_version != 1
            || self.suite_identity.trim().is_empty()
            || !is_sha256(&self.environment_identity)
            || !is_sha256(&self.environment_root_identity)
            || self.environment_identity != expected_environment_identity
            || self.profile_identity.trim().is_empty()
            || !valid_toolchain(&self.toolchain)
            || !self
                .environment_manifest_path
                .ends_with(&format!("/{}/Cargo.toml", self.environment_root_identity))
            || !self
                .environment_lock_path
                .ends_with(&format!("/{}/Cargo.lock", self.environment_root_identity))
            || !is_sha256(&self.environment_lock_sha256)
            || !self
                .shared_target_root
                .ends_with(&format!("/{}", &self.environment_identity[..24]))
            || self.fixtures.is_empty()
            || fixture_identities.len() != self.fixtures.len()
            || fixture_evidence_paths.len() != self.fixtures.len()
            || self.fixtures.iter().any(|fixture| {
                fixture.fixture.suite_identity != self.suite_identity
                    || fixture.fixture.environment_identity != self.environment_identity
                    || fixture.fixture.case_identity.trim().is_empty()
                    || fixture.fixture.source_path.trim().is_empty()
                    || !is_sha256(&fixture.fixture.source_digest)
                    || !is_sha256(&fixture.fixture.expected_denial_identity)
                    || fixture.cargo_process_id == 0
                    || fixture.cargo_exit_code.is_none_or(|code| code == 0)
                    || !is_sha256(&fixture.cargo_stdout_sha256)
                    || !is_sha256(&fixture.cargo_stderr_sha256)
                    || fixture.dependency_artifacts_compiled + fixture.dependency_artifacts_reused
                        == 0
                    || fixture.target_artifact_count_after < fixture.target_artifact_count_before
                    || fixture.diagnostics.is_empty()
                    || fixture
                        .diagnostics
                        .iter()
                        .any(|diagnostic| diagnostic.level != "error")
                    || fixture.evidence_path.trim().is_empty()
                    || !fixture.semantic_denial_matched
            })
        {
            return Err("UI proof evidence has inconsistent semantic fields".to_owned());
        }
        for fixture in &self.fixtures {
            let mut unsigned = fixture.clone();
            unsigned.evidence_path.clear();
            let identity = serde_json::to_vec(&unsigned)
                .map(|bytes| format!("{:x}", Sha256::digest(bytes)))
                .map_err(|error| format!("could not validate UI fixture evidence: {error}"))?;
            if !fixture
                .evidence_path
                .ends_with(&format!("/{identity}.json"))
            {
                return Err("UI fixture evidence path identity mismatch".to_owned());
            }
        }
        let mut unsigned = self.clone();
        unsigned.evidence_identity.clear();
        let identity = serde_json::to_vec(&unsigned)
            .map(|bytes| format!("{:x}", Sha256::digest(bytes)))
            .map_err(|error| format!("could not validate UI proof evidence: {error}"))?;
        if identity != self.evidence_identity {
            return Err("UI proof evidence identity mismatch".to_owned());
        }
        Ok(())
    }
}

fn valid_toolchain(toolchain: &UiCompilerToolchainIdentity) -> bool {
    toolchain.version_probe_timeout_millis > 0
        && toolchain.compile_timeout_millis > 0
        && toolchain.output_cap_bytes_per_stream > 0
        && toolchain.resource_posture.one_process_per_fixture
        && toolchain.resource_posture.shared_environment_target
        && toolchain.resource_posture.offline
        && toolchain.resource_posture.locked_dependencies
        && toolchain.resource_posture.declared_profile_applied
        && toolchain.resource_posture.bounded_output
        && !toolchain.cargo_configuration.is_empty()
        && toolchain.cargo_configuration.iter().all(|config| {
            !config.path.trim().is_empty() && config.content_sha256.as_deref().is_none_or(is_sha256)
        })
        && [&toolchain.cargo, &toolchain.rustc]
            .into_iter()
            .all(|tool| {
                !tool.executable_path.trim().is_empty()
                    && is_sha256(&tool.executable_sha256)
                    && !tool.version_identity.trim().is_empty()
            })
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
