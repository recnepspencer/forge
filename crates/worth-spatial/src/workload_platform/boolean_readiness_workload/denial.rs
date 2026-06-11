use super::counters::PlanarBooleanReadinessWorkloadCounters;
use super::required_stage::PlanarBooleanReadinessRequiredStage;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanReadinessWorkloadDenialKind {
    MissingDeclaration,
    MissingRequiredStage,
    PolicyRequired,
    CleanFailure,
    UnsupportedWorkloadFamily,
    PredicateUncertainty,
    ProjectionOrParityMismatch,
    RecoveryOrReplayMismatch,
    OrientationFlipLocalization,
    KernelSummarySubstitution,
    QueryBoundaryMismatch,
    BooleanExecutionAlreadyPresent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanReadinessWorkloadDenial {
    kind: PlanarBooleanReadinessWorkloadDenialKind,
    failed_stage: Option<PlanarBooleanReadinessRequiredStage>,
    human_reason: String,
    evidence_digest: String,
    counters: PlanarBooleanReadinessWorkloadCounters,
}

impl PlanarBooleanReadinessWorkloadDenial {
    pub(crate) fn new(
        kind: PlanarBooleanReadinessWorkloadDenialKind,
        failed_stage: Option<PlanarBooleanReadinessRequiredStage>,
        human_reason: impl Into<String>,
        evidence_digest: impl Into<String>,
        required_evidence_stages_consumed: usize,
    ) -> Self {
        Self {
            kind,
            failed_stage,
            human_reason: human_reason.into(),
            evidence_digest: evidence_digest.into(),
            counters: PlanarBooleanReadinessWorkloadCounters::blocked(
                required_evidence_stages_consumed,
            ),
        }
    }

    pub fn kind(&self) -> PlanarBooleanReadinessWorkloadDenialKind {
        self.kind
    }

    pub fn failed_stage(&self) -> Option<PlanarBooleanReadinessRequiredStage> {
        self.failed_stage
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }

    pub fn evidence_digest(&self) -> &str {
        &self.evidence_digest
    }

    pub fn counters(&self) -> PlanarBooleanReadinessWorkloadCounters {
        self.counters
    }
}
