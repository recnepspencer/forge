/// Typed failure with immutable lineage; it cannot mutate committed truth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiAllocationReceiptCommitDenial {
    CatalogBindingCardinalityMismatch,
    CatalogBindingIdentityMismatch { ordinal: u16 },
    CatalogActivationAuthority(super::UiCommittedAllocationCatalogActivationDenial),
    CandidatePlanningDenied(super::UiAllocationReceiptDenialReport),
    ReuseDenied(super::UiAllocationReceiptDenialReport),
    AuthorityCounterExhausted(super::UiAllocationAuthorityCounterExhaustion),
    EvidenceCounterExhausted,
}
