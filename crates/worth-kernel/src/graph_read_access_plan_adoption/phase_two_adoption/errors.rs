#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind {
    MissingReadFamilyIdentity,
    MissingRequirementRowEvidence,
    MissingStructuredSeedPairing,
    DuplicateStructuredSeedPairing,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionPhaseTwoError {
    kind: WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind,
}

impl WorthGraphReadAccessPlanAdoptionPhaseTwoError {
    pub(crate) const fn new(kind: WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessPlanAdoptionPhaseTwoErrorKind {
        self.kind
    }
}
