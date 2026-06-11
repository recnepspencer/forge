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
    human_reason: String,
}

impl ProjectionFactParityDenial {
    pub(crate) fn new(
        kind: ProjectionFactParityDenialKind,
        failed_lane: Option<ProjectionFactParityLane>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            failed_lane,
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

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}
