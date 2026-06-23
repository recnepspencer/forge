#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessPhaseSixErrorKind {
    MissingInventoryRowIdentity,
    MissingReadFamilyTarget,
    MissingTouchedAuthorityInput,
    MissingRequirementVocabulary,
    MissingMilestoneSevenLoweringTarget,
    MissingQueryCapability,
    MissingExpectedDenialKind,
    MissingCapabilityGapCap,
    MissingCapabilityGapBlocker,
    MissingCapabilityGapRemovalTrigger,
    MissingDeletionTrigger,
    UnknownInventoryRow,
    DuplicateInventoryRowDisposition,
    MissingInventoryRowDisposition,
    InventoryRowDispositionMismatch,
    KeepLocalGraphReadDispositionDenied,
    EmptyPhaseSixCloseout,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPhaseSixError {
    kind: WorthGraphReadAccessPhaseSixErrorKind,
}

impl WorthGraphReadAccessPhaseSixError {
    pub(crate) const fn new(kind: WorthGraphReadAccessPhaseSixErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessPhaseSixErrorKind {
        self.kind
    }
}
