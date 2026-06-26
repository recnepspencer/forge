#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessHardDeletionErrorKind {
    SeedAlreadyClaimsValidatorSelection,
    MissingReceiptAccountingProof,
    MissingCounterAccountingProof,
    MissingBatchAccountingProof,
    UnresolvedMigratedExecutionPath,
    CappedResidueCapExceeded,
    SourceFirewallViolation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessHardDeletionError {
    kind: WorthGraphReadAccessHardDeletionErrorKind,
}

impl WorthGraphReadAccessHardDeletionError {
    pub(crate) const fn new(kind: WorthGraphReadAccessHardDeletionErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessHardDeletionErrorKind {
        self.kind
    }
}
