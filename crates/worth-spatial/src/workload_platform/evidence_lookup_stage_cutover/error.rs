#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupStageCutoverErrorKind {
    MissingCoveredFamily,
    StageMismatch,
    StageReceiptMismatch,
    SpatialTouchMismatch,
    SelectedPlanMismatch,
    UncoveredFamilyOutcome,
    MissingTopologyDerivedReceipt,
    ScopeExpansionDenied,
    RawEvidenceFallbackDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupStageCutoverError {
    kind: EvidenceLookupStageCutoverErrorKind,
    detail: String,
}

impl EvidenceLookupStageCutoverError {
    pub(crate) fn new(
        kind: EvidenceLookupStageCutoverErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> EvidenceLookupStageCutoverErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
