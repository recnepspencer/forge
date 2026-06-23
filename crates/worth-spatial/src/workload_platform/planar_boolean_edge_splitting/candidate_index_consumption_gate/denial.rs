use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanCandidateIndexConsumptionDenialKind {
    MissingSegmentPairEnumerationEvidence,
    MissingEventLedgerEvidence,
    ManualSegmentPairEnumerationEvidence,
    ManualEventLedgerEvidence,
    CounterlessSegmentPairEnumerationEvidence,
    CounterlessEventLedgerEvidence,
    MismatchedSegmentPairEnumerationEvidence,
    MismatchedEventLedgerEvidence,
    UnsupportedSegmentPairEnumerationEvidence,
    UnsupportedEventLedgerEvidence,
    EventLedgerSegmentPairEnumerationMismatch,
    NonProductionCandidateIndexFallback,
    UnsupportedCandidateIndexLifecycleOutcome,
    CandidateIndexCounterMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCandidateIndexConsumptionDenial {
    kind: PlanarBooleanCandidateIndexConsumptionDenialKind,
    evidence_identity: String,
    human_reason: String,
}

impl PlanarBooleanCandidateIndexConsumptionDenial {
    pub(crate) fn new(
        kind: PlanarBooleanCandidateIndexConsumptionDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
        }
    }

    pub(crate) fn from_evidence_error(
        error: WorkloadEvidenceLedgerError,
        evidence_identity: impl Into<String>,
    ) -> Self {
        let kind = candidate_index_consumption_denial_kind_for_evidence_error(&error);
        Self::new(kind, evidence_identity, error.human_reason())
    }

    pub fn kind(&self) -> PlanarBooleanCandidateIndexConsumptionDenialKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

fn candidate_index_consumption_denial_kind_for_evidence_error(
    error: &WorkloadEvidenceLedgerError,
) -> PlanarBooleanCandidateIndexConsumptionDenialKind {
    match error {
        WorkloadEvidenceLedgerError::MissingBooleanStage(
            WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
        ) => PlanarBooleanCandidateIndexConsumptionDenialKind::MissingSegmentPairEnumerationEvidence,
        WorkloadEvidenceLedgerError::MissingBooleanStage(WorkloadEvidenceStage::BooleanEventLedger) => {
            PlanarBooleanCandidateIndexConsumptionDenialKind::MissingEventLedgerEvidence
        }
        WorkloadEvidenceLedgerError::ManualBooleanStage(
            WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
        ) => PlanarBooleanCandidateIndexConsumptionDenialKind::ManualSegmentPairEnumerationEvidence,
        WorkloadEvidenceLedgerError::ManualBooleanStage(WorkloadEvidenceStage::BooleanEventLedger) => {
            PlanarBooleanCandidateIndexConsumptionDenialKind::ManualEventLedgerEvidence
        }
        WorkloadEvidenceLedgerError::CounterlessBooleanStage(
            WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
        ) => PlanarBooleanCandidateIndexConsumptionDenialKind::CounterlessSegmentPairEnumerationEvidence,
        WorkloadEvidenceLedgerError::CounterlessBooleanStage(
            WorkloadEvidenceStage::BooleanEventLedger,
        ) => PlanarBooleanCandidateIndexConsumptionDenialKind::CounterlessEventLedgerEvidence,
        WorkloadEvidenceLedgerError::MismatchedBooleanStage(
            WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
        ) => PlanarBooleanCandidateIndexConsumptionDenialKind::MismatchedSegmentPairEnumerationEvidence,
        WorkloadEvidenceLedgerError::MismatchedBooleanStage(
            WorkloadEvidenceStage::BooleanEventLedger,
        ) => PlanarBooleanCandidateIndexConsumptionDenialKind::MismatchedEventLedgerEvidence,
        WorkloadEvidenceLedgerError::UnsupportedBooleanStage(
            WorkloadEvidenceStage::BooleanSegmentPairEnumeration,
        ) => PlanarBooleanCandidateIndexConsumptionDenialKind::UnsupportedSegmentPairEnumerationEvidence,
        WorkloadEvidenceLedgerError::UnsupportedBooleanStage(
            WorkloadEvidenceStage::BooleanEventLedger,
        ) => PlanarBooleanCandidateIndexConsumptionDenialKind::UnsupportedEventLedgerEvidence,
        _ => PlanarBooleanCandidateIndexConsumptionDenialKind::MismatchedSegmentPairEnumerationEvidence,
    }
}
