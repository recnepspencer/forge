#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCommittedAllocationActivationInspectionOutcome {
    Committed,
    Denied,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiCommittedAllocationActivationInspectionDenialKind {
    Validation,
    GraphPredecessorMismatch,
    LedgerPredecessorMismatch,
    ScrollBinding,
    PortalBinding,
    FrameBoundary,
    CandidatePlanDigestMismatch,
    LedgerCommittedOutcomeMismatch,
    CommitResourceUnavailable,
    FrameReplacement,
    CounterExhausted,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiCommittedAllocationActivationInspection {
    outcome: UiCommittedAllocationActivationInspectionOutcome,
    attempt_identity_digest: u64,
    committed_row_count: usize,
    counters: super::UiCommittedAllocationActivationCounters,
    live_state_unchanged: bool,
    denial_kind: Option<UiCommittedAllocationActivationInspectionDenialKind>,
    scroll_owned_evidence: Box<[crate::evidence::UiScrollOwnedAllocationEvidence]>,
    scroll_catalog_identity_digest: Option<u64>,
}

impl UiCommittedAllocationActivationInspection {
    pub(super) fn committed(receipt: &super::WorthUiPlanSwapReceipt) -> Self {
        Self {
            outcome: UiCommittedAllocationActivationInspectionOutcome::Committed,
            attempt_identity_digest: receipt.attempt_identity_digest(),
            committed_row_count: receipt.committed_row_count(),
            counters: receipt.counters(),
            live_state_unchanged: false,
            denial_kind: None,
            scroll_owned_evidence: receipt
                .committed_allocation()
                .scroll_owned_evidence()
                .into(),
            scroll_catalog_identity_digest: receipt
                .scroll_owner_catalog()
                .map(|catalog| catalog.catalog_identity_digest()),
        }
    }

    pub(super) fn denied(denial: &super::UiCommittedAllocationActivationDenial) -> Self {
        let evidence = denial.evidence();
        Self {
            outcome: UiCommittedAllocationActivationInspectionOutcome::Denied,
            attempt_identity_digest: evidence.attempt_identity_digest(),
            committed_row_count: evidence.committed_row_count(),
            counters: evidence.counters(),
            live_state_unchanged: evidence.live_state_unchanged(),
            denial_kind: Some(denial_kind(denial.reason())),
            scroll_owned_evidence: Box::new([]),
            scroll_catalog_identity_digest: None,
        }
    }

    pub fn outcome(&self) -> UiCommittedAllocationActivationInspectionOutcome {
        self.outcome
    }
    pub fn attempt_identity_digest(&self) -> u64 {
        self.attempt_identity_digest
    }
    pub fn committed_row_count(&self) -> usize {
        self.committed_row_count
    }
    pub fn counters(&self) -> super::UiCommittedAllocationActivationCounters {
        self.counters
    }
    pub fn live_state_unchanged(&self) -> bool {
        self.live_state_unchanged
    }
    pub fn denial_kind(&self) -> Option<UiCommittedAllocationActivationInspectionDenialKind> {
        self.denial_kind
    }
    pub fn scroll_owned_evidence(&self) -> &[crate::evidence::UiScrollOwnedAllocationEvidence] {
        &self.scroll_owned_evidence
    }
    pub fn scroll_catalog_identity_digest(&self) -> Option<u64> {
        self.scroll_catalog_identity_digest
    }
}

fn denial_kind(
    reason: &super::UiCommittedAllocationActivationDenialReason,
) -> UiCommittedAllocationActivationInspectionDenialKind {
    match reason {
        super::UiCommittedAllocationActivationDenialReason::Validation(_) => {
            UiCommittedAllocationActivationInspectionDenialKind::Validation
        }
        super::UiCommittedAllocationActivationDenialReason::GraphPredecessorMismatch => {
            UiCommittedAllocationActivationInspectionDenialKind::GraphPredecessorMismatch
        }
        super::UiCommittedAllocationActivationDenialReason::LedgerPredecessorMismatch => {
            UiCommittedAllocationActivationInspectionDenialKind::LedgerPredecessorMismatch
        }
        super::UiCommittedAllocationActivationDenialReason::ScrollBinding(_) => {
            UiCommittedAllocationActivationInspectionDenialKind::ScrollBinding
        }
        super::UiCommittedAllocationActivationDenialReason::PortalBinding(_) => {
            UiCommittedAllocationActivationInspectionDenialKind::PortalBinding
        }
        super::UiCommittedAllocationActivationDenialReason::FrameBoundary(_) => {
            UiCommittedAllocationActivationInspectionDenialKind::FrameBoundary
        }
        super::UiCommittedAllocationActivationDenialReason::CandidatePlanDigestMismatch => {
            UiCommittedAllocationActivationInspectionDenialKind::CandidatePlanDigestMismatch
        }
        super::UiCommittedAllocationActivationDenialReason::LedgerCommittedOutcomeMismatch => {
            UiCommittedAllocationActivationInspectionDenialKind::LedgerCommittedOutcomeMismatch
        }
        super::UiCommittedAllocationActivationDenialReason::CommitResourceUnavailable => {
            UiCommittedAllocationActivationInspectionDenialKind::CommitResourceUnavailable
        }
        super::UiCommittedAllocationActivationDenialReason::FrameReplacement(_) => {
            UiCommittedAllocationActivationInspectionDenialKind::FrameReplacement
        }
        super::UiCommittedAllocationActivationDenialReason::CounterExhausted(_) => {
            UiCommittedAllocationActivationInspectionDenialKind::CounterExhausted
        }
    }
}
