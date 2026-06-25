use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedInvalidationFamilyCatalogError {
    kind: DerivedInvalidationFamilyCatalogErrorKind,
    detail: String,
}

impl DerivedInvalidationFamilyCatalogError {
    pub(crate) fn new(
        kind: DerivedInvalidationFamilyCatalogErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> &DerivedInvalidationFamilyCatalogErrorKind {
        &self.kind
    }
}

impl fmt::Display for DerivedInvalidationFamilyCatalogError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for DerivedInvalidationFamilyCatalogError {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivedInvalidationFamilyCatalogErrorKind {
    InventorySeedMismatch,
    MissingInventorySourceForFamily { family: &'static str },
    MissingCatalogFamilyForInventorySource { family: &'static str },
    MissingRequiredFamily { family: &'static str },
    DuplicateFamily { family: &'static str },
    MissingConsumedGraphFacts { family: &'static str },
    EmptyConsumedGraphFacts { family: &'static str },
    MissingInvalidationPredicate { family: &'static str },
    MissingUpdatePosture { family: &'static str },
    MissingSpatialEvidencePosture { family: &'static str },
    MissingQueryReceiptPosture { family: &'static str },
    MissingLegalityReceiptPosture { family: &'static str },
    MissingDiagnosticPosture { family: &'static str },
    MissingSupportPosture { family: &'static str },
    QuerySupportRequired { family: &'static str },
}
