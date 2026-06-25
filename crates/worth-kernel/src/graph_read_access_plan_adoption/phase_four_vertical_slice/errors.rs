#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessFirstVerticalSliceErrorKind {
    EmptyPhaseFourSeed,
    SeedAlreadyClaimedAccessPlanConsumption,
    SeedAlreadyClaimedGraphReadExecution,
    SeedAlreadyClaimedGraphReadReceipt,
    MissingSelectedSlice,
    MissingExecutionBindingIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessFirstVerticalSliceError {
    kind: WorthGraphReadAccessFirstVerticalSliceErrorKind,
}

impl WorthGraphReadAccessFirstVerticalSliceError {
    pub const fn new(kind: WorthGraphReadAccessFirstVerticalSliceErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessFirstVerticalSliceErrorKind {
        self.kind
    }
}
