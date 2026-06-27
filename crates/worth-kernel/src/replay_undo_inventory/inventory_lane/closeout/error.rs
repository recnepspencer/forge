#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoInventoryErrorKind {
    DuplicateDeclaredSource,
    DuplicateLoweredSource,
    DeclaredSourceNotLowered,
    LoweredSourceNotDeclared,
    AuthorityRoleMismatch,
    ObservabilityRoleMismatch,
    UnclassifiedSource,
    MissingGapTrigger,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoInventoryError {
    kind: ReplayUndoInventoryErrorKind,
    detail: String,
}

impl ReplayUndoInventoryError {
    pub(crate) fn new(kind: ReplayUndoInventoryErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> ReplayUndoInventoryErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
