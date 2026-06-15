use super::lane::ProjectionFactParityLane;
use crate::workload_platform::evidence_ledger::WorkloadEvidenceLedgerError;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionFactParityDenialKind {
    MissingDeclaration,
    MissingLane,
    DuplicateLane,
    LiveProjectionMismatch,
    RetainedReplayMismatch,
    RecoveryMismatch,
    TransformParityMismatch,
    LocalRebuildMismatch,
    DiagnosticsMismatch,
    DeniedLaneUpgraded,
    PolicyRequired,
    EvidenceLedgerRejected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionFactParityDenial {
    kind: ProjectionFactParityDenialKind,
    failed_lane: Option<ProjectionFactParityLane>,
    workload_basis_identity: String,
    human_reason: String,
}

impl ProjectionFactParityDenial {
    pub(crate) fn new(
        kind: ProjectionFactParityDenialKind,
        failed_lane: Option<ProjectionFactParityLane>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self::new_with_workload_basis(kind, failed_lane, "", human_reason)
    }

    pub(crate) fn new_with_workload_basis(
        kind: ProjectionFactParityDenialKind,
        failed_lane: Option<ProjectionFactParityLane>,
        workload_basis_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            failed_lane,
            workload_basis_identity: workload_basis_identity.into(),
            human_reason: human_reason.into(),
        }
    }

    pub(crate) fn from_ledger_error(error: WorkloadEvidenceLedgerError) -> Self {
        Self::new(
            ProjectionFactParityDenialKind::EvidenceLedgerRejected,
            None,
            error.human_reason(),
        )
    }

    pub fn kind(&self) -> ProjectionFactParityDenialKind {
        self.kind
    }

    pub fn failed_lane(&self) -> Option<ProjectionFactParityLane> {
        self.failed_lane
    }

    pub fn workload_basis_identity(&self) -> &str {
        &self.workload_basis_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub(crate) fn user_response_evidence_identity(&self) -> String {
        projection_fact_parity_denial_evidence_identity(
            self.kind,
            self.failed_lane,
            &self.workload_basis_identity,
            &self.human_reason,
        )
    }
}

pub(crate) fn projection_fact_parity_denial_evidence_identity(
    kind: ProjectionFactParityDenialKind,
    failed_lane: Option<ProjectionFactParityLane>,
    workload_basis_identity: &str,
    human_reason: &str,
) -> String {
    worth_primitives::truth_digest_parts(
        worth_primitives::TruthDigestScope::ArtifactIdentity,
        &[
            "projection-fact-parity-denial".to_string(),
            format!("{kind:?}"),
            failed_lane
                .map(|lane| format!("{lane:?}"))
                .unwrap_or_else(|| "no-lane".to_string()),
            format!("basis:{workload_basis_identity}"),
            human_reason.to_string(),
        ],
    )
}
