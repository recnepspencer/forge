use crate::workload_platform::projection_fact_parity::{
    ProjectionFactParityDenial, ProjectionFactParityDenialKind, ProjectionFactParityReceipt,
};
use crate::workload_platform::user_response::{
    source::WorthUserResponseSourceKind, WorthPolicyDecision, WorthUserOutcomeCauseKind,
    WorthUserResponseSource,
};

impl WorthUserResponseSource {
    pub fn from_projection_fact_parity(receipt: &ProjectionFactParityReceipt) -> Self {
        Self {
            kind: WorthUserResponseSourceKind::Admitted {
                message: format!(
                    "Projection fact parity matched across {} receipt-backed lanes.",
                    receipt.counters().lanes_compared()
                ),
                evidence_digest: receipt.parity_digest().to_string(),
                source_identity: receipt.workload_basis_identity().to_string(),
            },
        }
    }

    pub fn from_projection_fact_parity_denial(denial: &ProjectionFactParityDenial) -> Self {
        let evidence_digest = denial.user_response_evidence_identity();
        let message = denial.human_reason().to_string();
        if denial.kind() == ProjectionFactParityDenialKind::PolicyRequired {
            return Self {
                kind: WorthUserResponseSourceKind::PolicyRequired {
                    message,
                    evidence_digest: evidence_digest.clone(),
                    source_identity: evidence_digest,
                    choices: vec![WorthPolicyDecision::PauseForManualInspection],
                },
            };
        }
        Self {
            kind: WorthUserResponseSourceKind::NoOptions {
                cause_kind: cause_kind(denial.kind()),
                message,
                evidence_digest: evidence_digest.clone(),
                source_identity: evidence_digest,
            },
        }
    }
}

fn cause_kind(kind: ProjectionFactParityDenialKind) -> WorthUserOutcomeCauseKind {
    match kind {
        ProjectionFactParityDenialKind::DeniedLaneUpgraded => {
            WorthUserOutcomeCauseKind::DeniedMovementOrRotation
        }
        ProjectionFactParityDenialKind::PolicyRequired => WorthUserOutcomeCauseKind::PolicyRequired,
        ProjectionFactParityDenialKind::MissingDeclaration
        | ProjectionFactParityDenialKind::MissingLane
        | ProjectionFactParityDenialKind::DuplicateLane
        | ProjectionFactParityDenialKind::EvidenceLedgerRejected => {
            WorthUserOutcomeCauseKind::MissingEvidence
        }
        ProjectionFactParityDenialKind::LiveProjectionMismatch
        | ProjectionFactParityDenialKind::RetainedReplayMismatch
        | ProjectionFactParityDenialKind::RecoveryMismatch
        | ProjectionFactParityDenialKind::TransformParityMismatch
        | ProjectionFactParityDenialKind::LocalRebuildMismatch
        | ProjectionFactParityDenialKind::DiagnosticsMismatch => {
            WorthUserOutcomeCauseKind::IntegrityMismatch
        }
    }
}
