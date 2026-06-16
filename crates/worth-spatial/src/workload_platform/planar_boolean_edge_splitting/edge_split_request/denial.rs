use crate::workload_platform::evidence_ledger::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanEdgeSplitRequestDenialKind {
    MissingEventLedgerEvidence,
    ManualEventLedgerEvidence,
    CounterlessEventLedgerEvidence,
    MismatchedEventLedgerEvidence,
    UnsupportedEventLedgerEvidence,
    CandidateIndexGateEventLedgerMismatch,
    CandidateIndexGateDownstreamMismatch,
    CandidateIndexGateReducedPairMismatch,
    CandidateIndexGateSegmentPairMismatch,
    NonProductionCandidateIndexGate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanEdgeSplitRequestDenial {
    kind: PlanarBooleanEdgeSplitRequestDenialKind,
    evidence_identity: String,
    human_reason: String,
}

impl PlanarBooleanEdgeSplitRequestDenial {
    pub(crate) fn new(
        kind: PlanarBooleanEdgeSplitRequestDenialKind,
        evidence_identity: impl Into<String>,
        human_reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            evidence_identity: evidence_identity.into(),
            human_reason: human_reason.into(),
        }
    }

    pub(crate) fn from_event_ledger_evidence_error(
        error: WorkloadEvidenceLedgerError,
        event_ledger_identity: impl Into<String>,
    ) -> Self {
        Self::new(
            split_request_denial_kind_for_event_ledger_evidence_error(&error),
            event_ledger_identity,
            error.human_reason(),
        )
    }

    pub fn kind(&self) -> PlanarBooleanEdgeSplitRequestDenialKind {
        self.kind
    }

    pub fn evidence_identity(&self) -> &str {
        &self.evidence_identity
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

fn split_request_denial_kind_for_event_ledger_evidence_error(
    error: &WorkloadEvidenceLedgerError,
) -> PlanarBooleanEdgeSplitRequestDenialKind {
    match error {
        WorkloadEvidenceLedgerError::MissingBooleanStage(
            WorkloadEvidenceStage::BooleanEventLedger,
        ) => PlanarBooleanEdgeSplitRequestDenialKind::MissingEventLedgerEvidence,
        WorkloadEvidenceLedgerError::ManualBooleanStage(
            WorkloadEvidenceStage::BooleanEventLedger,
        ) => PlanarBooleanEdgeSplitRequestDenialKind::ManualEventLedgerEvidence,
        WorkloadEvidenceLedgerError::CounterlessBooleanStage(
            WorkloadEvidenceStage::BooleanEventLedger,
        ) => PlanarBooleanEdgeSplitRequestDenialKind::CounterlessEventLedgerEvidence,
        WorkloadEvidenceLedgerError::MismatchedBooleanStage(
            WorkloadEvidenceStage::BooleanEventLedger,
        ) => PlanarBooleanEdgeSplitRequestDenialKind::MismatchedEventLedgerEvidence,
        WorkloadEvidenceLedgerError::UnsupportedBooleanStage(
            WorkloadEvidenceStage::BooleanEventLedger,
        ) => PlanarBooleanEdgeSplitRequestDenialKind::UnsupportedEventLedgerEvidence,
        _ => PlanarBooleanEdgeSplitRequestDenialKind::MismatchedEventLedgerEvidence,
    }
}
