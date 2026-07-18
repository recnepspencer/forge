use serde::{Deserialize, Serialize};

use crate::selection::ProofProcessModel;

use super::{
    ExternalObservationReceipt, FormalToolEvidenceReference, ProcessProbeEvidenceReference,
    UiProofEvidenceReference,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ProofAttemptOutcome {
    Passed,
    Failed { exit_code: Option<i32> },
    TimedOut,
    TerminationDenied { reason: String },
    LaunchDenied { reason: String },
    EvidenceDenied { denials: Vec<String> },
}

impl ProofAttemptOutcome {
    pub const fn passed(&self) -> bool {
        matches!(self, Self::Passed)
    }

    pub const fn exit_code(&self) -> Option<i32> {
        match self {
            Self::Passed => Some(0),
            Self::Failed { exit_code } => *exit_code,
            Self::TimedOut
            | Self::TerminationDenied { .. }
            | Self::LaunchDenied { .. }
            | Self::EvidenceDenied { .. } => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofAttemptLog {
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofRunAttempt {
    pub attempt_identity: String,
    pub plan_digest: String,
    pub unit_identity: String,
    pub unit_index: usize,
    pub ordinal: usize,
    pub command: Vec<String>,
    pub started_unix_millis: u128,
    pub elapsed_millis: u128,
    pub outcome: ProofAttemptOutcome,
    pub stdout: ProofAttemptLog,
    pub stderr: ProofAttemptLog,
    pub cargo_compiler_artifact_messages: usize,
    pub linked_executable_artifacts: Vec<String>,
    pub evidence_denials: Vec<String>,
    pub external_observation: Option<ExternalObservationReceipt>,
    pub formal_tool_evidence: Option<FormalToolEvidenceReference>,
    pub ui_proof_evidence: Vec<UiProofEvidenceReference>,
    pub process_probe_evidence: Vec<ProcessProbeEvidenceReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProofUnitExecutionVerdict {
    pub unit_identity: String,
    pub case_filter: Option<String>,
    pub process_model: ProofProcessModel,
    pub behavioral_verdict: String,
    pub elapsed_millis: u128,
    pub attempt_identities: Vec<String>,
    pub ui_proof_evidence: Vec<UiProofEvidenceReference>,
    pub process_probe_evidence: Vec<ProcessProbeEvidenceReference>,
}

impl ProofUnitExecutionVerdict {
    pub(crate) fn from_attempts(
        unit_identity: String,
        case_filter: Option<String>,
        process_model: ProofProcessModel,
        attempts: &[ProofRunAttempt],
    ) -> Self {
        let passed = attempts
            .last()
            .is_some_and(|attempt| attempt.outcome.passed());
        let failed_before_pass = passed
            && attempts
                .iter()
                .take(attempts.len().saturating_sub(1))
                .any(|attempt| !attempt.outcome.passed());
        let behavioral_verdict = if failed_before_pass {
            "flaky-indeterminate"
        } else if passed {
            "passed"
        } else {
            "failed"
        };
        Self {
            unit_identity,
            case_filter,
            process_model,
            behavioral_verdict: behavioral_verdict.to_owned(),
            elapsed_millis: attempts.iter().map(|attempt| attempt.elapsed_millis).sum(),
            attempt_identities: attempts
                .iter()
                .map(|attempt| attempt.attempt_identity.clone())
                .collect(),
            ui_proof_evidence: attempts
                .iter()
                .flat_map(|attempt| attempt.ui_proof_evidence.clone())
                .collect(),
            process_probe_evidence: attempts
                .iter()
                .flat_map(|attempt| attempt.process_probe_evidence.clone())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn attempt(ordinal: usize, outcome: ProofAttemptOutcome) -> ProofRunAttempt {
        ProofRunAttempt {
            attempt_identity: format!("attempt-{ordinal}"),
            plan_digest: "plan".to_owned(),
            unit_identity: "unit".to_owned(),
            unit_index: 0,
            ordinal,
            command: Vec::new(),
            started_unix_millis: 1,
            elapsed_millis: 1,
            outcome,
            stdout: ProofAttemptLog {
                path: "stdout".to_owned(),
                sha256: "stdout".to_owned(),
                bytes: 0,
            },
            stderr: ProofAttemptLog {
                path: "stderr".to_owned(),
                sha256: "stderr".to_owned(),
                bytes: 0,
            },
            cargo_compiler_artifact_messages: 0,
            linked_executable_artifacts: Vec::new(),
            evidence_denials: Vec::new(),
            external_observation: None,
            formal_tool_evidence: None,
            ui_proof_evidence: Vec::new(),
            process_probe_evidence: Vec::new(),
        }
    }

    #[test]
    fn retry_success_remains_flaky_instead_of_green() {
        let attempts = vec![
            attempt(0, ProofAttemptOutcome::Failed { exit_code: Some(1) }),
            attempt(1, ProofAttemptOutcome::Passed),
        ];
        let verdict = ProofUnitExecutionVerdict::from_attempts(
            "unit".to_owned(),
            None,
            ProofProcessModel::LibtestProcess,
            &attempts,
        );
        assert_eq!(verdict.behavioral_verdict, "flaky-indeterminate");
        assert_eq!(verdict.attempt_identities, ["attempt-0", "attempt-1"]);
    }
}
