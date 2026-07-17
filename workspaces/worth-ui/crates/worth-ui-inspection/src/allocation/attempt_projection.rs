use super::{
    UiAllocationInspectionEvidenceRef, UiAllocationInspectionInvalidationFamily,
    UiAllocationInspectionSelection, UiAllocationInspectionStreamFamily,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationInspectionReuseDenialPosture {
    NotApplicable,
    ReceiptIdentityMismatch,
    GenerationMismatch,
    EquivalenceBasisMismatch,
    UnsupportedPartialReuse,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationInspectionAttemptResult {
    PriorCommittedReceiptUnchanged,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAllocationInspectionDenialFamily {
    MissingSelection,
    CandidateMismatch,
    CandidatePlanning,
    Reuse,
    RecomputePending,
    TransactionIdentity,
    GenerationMismatch,
    CommitBudget,
    DurableMutationBudget,
    ResizeBasis,
    PortalAnchor,
    DurableSemanticState,
    CatalogBinding,
    CounterExhaustion,
    SourceSequence,
    SourcePolicy,
    SourceAuthority,
    StaleHostEvidence,
    UnsupportedScrollOwnership,
    ContradictoryScrollOwnership,
    BrokenPortalAnchor,
    NeighborhoodLocality,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiAllocationInspectionDeniedAttempt {
    stream_families: Box<[UiAllocationInspectionStreamFamily]>,
    invalidation_families: Box<[UiAllocationInspectionInvalidationFamily]>,
    selection: UiAllocationInspectionSelection,
    reuse_denial: UiAllocationInspectionReuseDenialPosture,
    denial_family: UiAllocationInspectionDenialFamily,
    result: UiAllocationInspectionAttemptResult,
    invalidation_evidence_ref: UiAllocationInspectionEvidenceRef,
    denial_evidence_ref: UiAllocationInspectionEvidenceRef,
}

impl UiAllocationInspectionDeniedAttempt {
    pub fn from_runtime_projection(
        stream_families: Box<[UiAllocationInspectionStreamFamily]>,
        invalidation_families: Box<[UiAllocationInspectionInvalidationFamily]>,
        selection: UiAllocationInspectionSelection,
        reuse_denial: UiAllocationInspectionReuseDenialPosture,
        denial_family: UiAllocationInspectionDenialFamily,
        invalidation_evidence_ref: UiAllocationInspectionEvidenceRef,
        denial_evidence_ref: UiAllocationInspectionEvidenceRef,
    ) -> Self {
        Self {
            stream_families,
            invalidation_families,
            selection,
            reuse_denial,
            denial_family,
            result: UiAllocationInspectionAttemptResult::PriorCommittedReceiptUnchanged,
            invalidation_evidence_ref,
            denial_evidence_ref,
        }
    }

    pub fn stream_families(&self) -> &[UiAllocationInspectionStreamFamily] {
        &self.stream_families
    }
    pub fn invalidation_families(&self) -> &[UiAllocationInspectionInvalidationFamily] {
        &self.invalidation_families
    }
    pub fn selection(&self) -> &UiAllocationInspectionSelection {
        &self.selection
    }
    pub fn reuse_denial(&self) -> UiAllocationInspectionReuseDenialPosture {
        self.reuse_denial
    }
    pub fn denial_family(&self) -> UiAllocationInspectionDenialFamily {
        self.denial_family
    }
    pub fn result(&self) -> UiAllocationInspectionAttemptResult {
        self.result
    }
    pub fn invalidation_evidence_ref(&self) -> UiAllocationInspectionEvidenceRef {
        self.invalidation_evidence_ref
    }
    pub fn denial_evidence_ref(&self) -> UiAllocationInspectionEvidenceRef {
        self.denial_evidence_ref
    }
}
