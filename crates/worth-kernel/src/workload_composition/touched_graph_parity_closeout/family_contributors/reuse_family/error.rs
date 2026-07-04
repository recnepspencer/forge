#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReuseFamilyContributorCatalogErrorKind {
    CurrentSurfaceUnavailable,
    MissingRequiredRow,
    MissingCarriedIdentity,
    MismatchedReuseSemantics,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReuseFamilyContributorCatalogError {
    kind: ReuseFamilyContributorCatalogErrorKind,
    detail: String,
}

impl ReuseFamilyContributorCatalogError {
    pub(crate) fn new(
        kind: ReuseFamilyContributorCatalogErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> ReuseFamilyContributorCatalogErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
