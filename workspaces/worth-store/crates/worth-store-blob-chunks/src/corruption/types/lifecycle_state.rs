#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobQuarantineLifecycleState {
    DetectedUnquarantined,
    Quarantined,
    RebuildableDerived,
    RepairRequiredAuthoritative,
    RestoreRequiredAuthoritative,
    DegradedTruthAuthoritative,
    ColdUnavailableCorrupt,
    ImportCorrupt,
}
