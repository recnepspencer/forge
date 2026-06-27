#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReplayUndoMilestoneTwelvePublicCloseoutErrorKind {
    UnpublishedHardDeletionProof,
    IncompleteInventoryClassification,
    MissingRemovalTrigger,
    MissingResidueCap,
    UncleanFirewall,
    UncappedResidue,
    MismatchedProofProducts,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoMilestoneTwelvePublicCloseoutError {
    kind: ReplayUndoMilestoneTwelvePublicCloseoutErrorKind,
    detail: String,
}

impl ReplayUndoMilestoneTwelvePublicCloseoutError {
    pub(crate) fn new(
        kind: ReplayUndoMilestoneTwelvePublicCloseoutErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> &ReplayUndoMilestoneTwelvePublicCloseoutErrorKind {
        &self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
