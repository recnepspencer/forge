#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConflictFamilyContributorCatalogErrorKind {
    CurrentSurfaceUnavailable,
    MissingRequiredRow,
    MissingCarriedIdentity,
    MismatchedRouteFamily,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictFamilyContributorCatalogError {
    kind: ConflictFamilyContributorCatalogErrorKind,
    detail: String,
}

impl ConflictFamilyContributorCatalogError {
    pub(crate) fn new(
        kind: ConflictFamilyContributorCatalogErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> ConflictFamilyContributorCatalogErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
