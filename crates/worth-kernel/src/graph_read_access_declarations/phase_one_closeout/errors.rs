#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessDeclarationPhaseOneErrorKind {
    SeedClaimedExecutionAuthority,
    SeedContainsUncappedOldGraphReadFolklore,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessDeclarationPhaseOneError {
    kind: WorthGraphReadAccessDeclarationPhaseOneErrorKind,
}

impl WorthGraphReadAccessDeclarationPhaseOneError {
    pub(crate) const fn new(kind: WorthGraphReadAccessDeclarationPhaseOneErrorKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthGraphReadAccessDeclarationPhaseOneErrorKind {
        self.kind
    }
}
