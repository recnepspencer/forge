#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicProjectionContributorCatalogErrorKind {
    CurrentSurfaceUnavailable,
    MissingRequiredRow,
    MissingCarriedIdentity,
    MismatchedProjectionAuthority,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicProjectionContributorCatalogError {
    kind: PublicProjectionContributorCatalogErrorKind,
    detail: String,
}

impl PublicProjectionContributorCatalogError {
    pub(crate) fn new(
        kind: PublicProjectionContributorCatalogErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> PublicProjectionContributorCatalogErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
