use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

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

impl UiProofRunEvidence {
    pub fn validate_integrity(&self) -> Result<(), String> {
        if self.schema_version != 1
            || self.suite_identity.trim().is_empty()
            || self.environment_identity.trim().is_empty()
            || self.fixtures.is_empty()
            || self.fixtures.iter().any(|fixture| {
                fixture.fixture.suite_identity != self.suite_identity
                    || fixture.fixture.environment_identity != self.environment_identity
                    || !fixture.semantic_denial_matched
            })
        {
            return Err("UI proof evidence has inconsistent semantic fields".to_owned());
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
