#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadRequirementDerivationErrorKind {
    MissingCatalogRecord,
    MissingQueryRequirementCapabilityInventory,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadRequirementDerivationError {
    kind: WorthGraphReadRequirementDerivationErrorKind,
}

impl WorthGraphReadRequirementDerivationError {
    pub(crate) const fn new(kind: WorthGraphReadRequirementDerivationErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadRequirementDerivationErrorKind {
        self.kind
    }
}
