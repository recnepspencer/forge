#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HarnessEvidenceFamily {
    RuntimeReceipt,
    OperationReceipt,
    ArtifactDigest,
    ActivePlanObservation,
    ActivePlanDigest,
    SnapshotDigest,
    FrameEpoch,
    CommandIdentity,
    CounterFamily,
    StateReceipt,
    RuntimeDenial,
    VisibleFrameObservation,
}
