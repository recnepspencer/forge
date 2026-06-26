#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessPlanAdoptionCloseoutErrorKind {
    SeedAlreadyClaimsValidatorSelection,
    MissingReceiptOrPostureProof,
    MissingCounterProof,
    MissingBatchAccountingProof,
    BatchCounterReceiptAssociationLost,
    CallerOwnedGraphWorkDetected,
    UnresolvedDeletionProof,
    UncappedResidue,
    SourceFirewallViolation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionCloseoutError {
    kind: WorthGraphReadAccessPlanAdoptionCloseoutErrorKind,
}

impl WorthGraphReadAccessPlanAdoptionCloseoutError {
    pub(crate) const fn new(kind: WorthGraphReadAccessPlanAdoptionCloseoutErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessPlanAdoptionCloseoutErrorKind {
        self.kind
    }
}
