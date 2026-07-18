use serde::{Deserialize, Serialize};

use super::{CheckedCompilerDiagnostic, UiFixtureIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiRunResult {
    pub environment_identity: String,
    pub shared_target_root: String,
    pub fixtures: Vec<UiFixtureResult>,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiFixtureResult {
    pub fixture: UiFixtureIdentity,
    pub dependency_artifacts_compiled: usize,
    pub dependency_artifacts_reused: usize,
    pub target_artifact_count_before: usize,
    pub target_artifact_count_after: usize,
    pub diagnostics: Vec<CheckedCompilerDiagnostic>,
    pub semantic_denial_matched: bool,
}

#[derive(Debug)]
pub enum UiRunFailure {
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
}

impl std::fmt::Display for UiRunFailure {
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
        }
    }
}

impl std::error::Error for UiRunFailure {}
