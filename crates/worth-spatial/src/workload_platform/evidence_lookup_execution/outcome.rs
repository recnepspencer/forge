#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupExecutionOutcome {
    IndexedHit,
    IndexedMiss,
    BoundedRebuild,
    RequiredQuerySupport,
    MissingProjectionConsumptionFact,
    DeniedBeforeExecution,
    CappedResidue,
}
