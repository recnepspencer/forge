#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind {
    MissingInventory,
    MissingCurrentProofSurface,
    CoveredOrdinaryConsumerBypass,
    MissingTypedQueryBackedProof,
    BroadScanFallbackStillOrdinary,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthWorkloadOrdinaryConsumerSweepCloseoutError {
    kind: WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind,
    detail: String,
}

impl WorthWorkloadOrdinaryConsumerSweepCloseoutError {
    pub(crate) fn new(
        kind: WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> WorthWorkloadOrdinaryConsumerSweepCloseoutErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
