#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessSpatialDensePostureErrorKind {
    SeedAlreadyClaimsValidatorSelection,
    MissingPhaseFourReceiptAndUnresolvedWork,
    RequiredPostureMissingCap,
    RequiredPostureExceedsCap,
    UnboundedEphemeralIndexForDenseOrBroadRead,
    SourceFirewallViolation,
    ScalarizedCallerLoopDetected,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessSpatialDensePostureError {
    kind: WorthGraphReadAccessSpatialDensePostureErrorKind,
}

impl WorthGraphReadAccessSpatialDensePostureError {
    pub(crate) const fn new(kind: WorthGraphReadAccessSpatialDensePostureErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessSpatialDensePostureErrorKind {
        self.kind
    }
}
