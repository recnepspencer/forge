/// Closed destinations for a future retention-owner transfer. Phase 1 carries
/// the vocabulary only; no transfer constructor or executable transition is
/// available until the retention lane owns the component leases.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ComponentBasisObligationTransferDestination {
    ProductBranchHead,
    RetainedCompositeHistory,
    AdmittedObservation,
    ActivePublicationAttempt,
    ProductUnpublishedOwnerEffects,
    HistoricalInspection,
    Release,
}
