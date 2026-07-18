use serde::{Deserialize, Serialize};

use super::{CheckedCompilerDiagnostic, UiFixtureIdentity};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiProofRunEvidence {
    pub schema_version: u32,
    pub suite_identity: String,
    pub environment_identity: String,
    pub environment_manifest_path: String,
    pub shared_target_root: String,
    pub environment_manifest_created: bool,
    pub fixtures: Vec<UiFixtureRunEvidence>,
    pub evidence_identity: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UiFixtureRunEvidence {
    pub fixture: UiFixtureIdentity,
    pub cargo_process_id: u32,
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
