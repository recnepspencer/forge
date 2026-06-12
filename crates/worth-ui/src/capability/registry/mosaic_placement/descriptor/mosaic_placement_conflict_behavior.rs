/// Conflict behavior when a placement target already owns competing state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicPlacementConflictBehavior {
    RejectConflict,
    ReplaceExisting,
    StackWithExisting,
    DeferUntilExplicit,
    MissingForDiagnostics,
}

impl MosaicPlacementConflictBehavior {
    pub fn reject_conflict() -> Self {
        Self::RejectConflict
    }

    pub fn replace_existing() -> Self {
        Self::ReplaceExisting
    }

    pub fn stack_with_existing() -> Self {
        Self::StackWithExisting
    }

    pub fn defer_until_explicit() -> Self {
        Self::DeferUntilExplicit
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::RejectConflict => "reject_conflict",
            Self::ReplaceExisting => "replace_existing",
            Self::StackWithExisting => "stack_with_existing",
            Self::DeferUntilExplicit => "defer_until_explicit",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
