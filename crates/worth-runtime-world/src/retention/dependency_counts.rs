/// Closed Runtime World dependency vocabulary for one exact component-basis
/// pin. Phase 1 freezes the classes; the Phase 2 retention owner will attach
/// counts to independently keyed Relational and Signal admissions.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ComponentBasisDependencyClass {
    ProductBranchHead,
    RetainedCompositeHistory,
    AdmittedObservation,
    ActivePublicationAttempt,
    ProductUnpublishedOwnerEffects,
    HistoricalInspection,
}

#[allow(dead_code)]
impl ComponentBasisDependencyClass {
    pub(crate) const ALL: [Self; 6] = [
        Self::ProductBranchHead,
        Self::RetainedCompositeHistory,
        Self::AdmittedObservation,
        Self::ActivePublicationAttempt,
        Self::ProductUnpublishedOwnerEffects,
        Self::HistoricalInspection,
    ];
}
