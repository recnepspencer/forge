#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayUndoHardDeletionErrorKind {
    SourceFirewallViolation,
    MissingHardDeletionRemovalTrigger,
    MissingResidueCap,
    MissingResidueRemovalTrigger,
    UncappedResidue,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoHardDeletionError {
    kind: ReplayUndoHardDeletionErrorKind,
    detail: String,
}

impl ReplayUndoHardDeletionError {
    pub(crate) fn new(kind: ReplayUndoHardDeletionErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> &ReplayUndoHardDeletionErrorKind {
        &self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
