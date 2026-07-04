#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReplayUndoFamilyContributorCatalogErrorKind {
    CurrentSurfaceUnavailable,
    MissingRequiredRow,
    MissingCarriedIdentity,
    MismatchedRouteFamily,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReplayUndoFamilyContributorCatalogError {
    kind: ReplayUndoFamilyContributorCatalogErrorKind,
    detail: String,
}

impl ReplayUndoFamilyContributorCatalogError {
    pub(crate) fn new(
        kind: ReplayUndoFamilyContributorCatalogErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> ReplayUndoFamilyContributorCatalogErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
