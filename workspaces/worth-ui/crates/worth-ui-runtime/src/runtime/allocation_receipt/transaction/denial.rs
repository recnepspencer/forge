/// Typed failure with immutable lineage; it cannot mutate committed truth.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiAllocationReceiptCommitDenial {
    CatalogBindingCardinalityMismatch,
    CatalogBindingIdentityMismatch {
        ordinal: u16,
    },
    CatalogActivationAuthority(Box<super::UiCommittedAllocationCatalogActivationDenial>),
    CandidatePlanningDenied(Box<super::UiAllocationReceiptDenialReport>),
    #[cfg(test)]
    ReuseDenied(Box<super::UiAllocationReceiptDenialReport>),
    AuthorityCounterExhausted(super::UiAllocationAuthorityCounterExhaustion),
    EvidenceCounterExhausted,
}

impl UiAllocationReceiptCommitDenial {
    pub(super) fn catalog_activation(
        denial: super::UiCommittedAllocationCatalogActivationDenial,
    ) -> Self {
        Self::CatalogActivationAuthority(Box::new(denial))
    }

    pub(super) fn candidate_planning(report: super::UiAllocationReceiptDenialReport) -> Self {
        Self::CandidatePlanningDenied(Box::new(report))
    }

    #[cfg(test)]
    pub(super) fn reuse(report: super::UiAllocationReceiptDenialReport) -> Self {
        Self::ReuseDenied(Box::new(report))
    }
}
