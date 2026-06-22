use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::boolean_readiness_workload::{
    PlanarBooleanReadinessWorkloadDenial, PlanarBooleanReadinessWorkloadDenialKind,
    PlanarBooleanReadinessWorkloadReceipt,
};
use crate::workload_platform::user_response::{
    source::WorthUserResponseSourceKind, WorthPolicyDecision, WorthUserOutcomeCauseKind,
    WorthUserResponseSource,
};

impl WorthUserResponseSource {
    pub fn from_boolean_readiness_workload(
        receipt: &PlanarBooleanReadinessWorkloadReceipt,
    ) -> Self {
        Self {
            kind: WorthUserResponseSourceKind::Admitted {
                message: format!(
                    "M7 may proceed with a complete pre-boolean readiness bundle backed by {} evidence stages.",
                    receipt.counters().required_evidence_stages_consumed()
                ),
                evidence_digest: receipt.workload_digest().to_string(),
                source_identity: receipt.workload_digest().to_string(),
            },
        }
    }

    pub fn from_boolean_readiness_workload_denial(
        denial: &PlanarBooleanReadinessWorkloadDenial,
    ) -> Self {
        let evidence_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "boolean-readiness-workload-denial".to_string(),
                format!("{:?}", denial.kind()),
                denial.human_reason().to_string(),
                denial.evidence_digest().to_string(),
            ],
        );
        if denial.kind() == PlanarBooleanReadinessWorkloadDenialKind::PolicyRequired {
            return Self {
                kind: WorthUserResponseSourceKind::PolicyRequired {
                    message: denial.human_reason().to_string(),
                    evidence_digest: evidence_digest.clone(),
                    source_identity: evidence_digest,
                    choices: vec![WorthPolicyDecision::PauseForManualInspection],
                },
            };
        }
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: cause_kind(denial.kind()),
                message: denial.human_reason().to_string(),
                evidence_digest: evidence_digest.clone(),
                source_identity: evidence_digest,
            },
        }
    }
}

fn cause_kind(kind: PlanarBooleanReadinessWorkloadDenialKind) -> WorthUserOutcomeCauseKind {
    match kind {
        PlanarBooleanReadinessWorkloadDenialKind::UnsupportedWorkloadFamily => {
            WorthUserOutcomeCauseKind::UnsupportedInput
        }
        PlanarBooleanReadinessWorkloadDenialKind::CleanFailure => {
            WorthUserOutcomeCauseKind::DirtyInput
        }
        PlanarBooleanReadinessWorkloadDenialKind::PredicateUncertainty => {
            WorthUserOutcomeCauseKind::PredicateUncertain
        }
        PlanarBooleanReadinessWorkloadDenialKind::ProjectionOrParityMismatch
        | PlanarBooleanReadinessWorkloadDenialKind::RecoveryOrReplayMismatch
        | PlanarBooleanReadinessWorkloadDenialKind::OrientationFlipLocalization
        | PlanarBooleanReadinessWorkloadDenialKind::KernelSummarySubstitution
        | PlanarBooleanReadinessWorkloadDenialKind::QueryBoundaryMismatch
        | PlanarBooleanReadinessWorkloadDenialKind::BooleanExecutionAlreadyPresent => {
            WorthUserOutcomeCauseKind::IntegrityMismatch
        }
        PlanarBooleanReadinessWorkloadDenialKind::PolicyRequired => {
            WorthUserOutcomeCauseKind::PolicyRequired
        }
        PlanarBooleanReadinessWorkloadDenialKind::MissingDeclaration
        | PlanarBooleanReadinessWorkloadDenialKind::MissingRequiredStage => {
            WorthUserOutcomeCauseKind::MissingEvidence
        }
    }
}
