#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupDiagnosticDenialReason {
    RequiredQuerySupport,
    MissingProjectionConsumptionFact,
    WrongSpatialTouchDigest,
    WrongStageReceiptIdentity,
    ProductSwapDetected,
    RequiredTopologySupport,
    ExecutionDeniedBeforeLookup,
}
