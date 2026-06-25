#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedInvalidationAuthorityInventoryError {
    kind: DerivedInvalidationAuthorityInventoryErrorKind,
    detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DerivedInvalidationAuthorityInventoryErrorKind {
    MissingCoveredProductCategory {
        category: &'static str,
    },
    InvalidOrdinaryDisposition {
        surface: String,
        disposition: &'static str,
    },
    MissingRowExitCondition {
        surface: String,
    },
    InvalidCertificationResidue {
        surface: String,
    },
    UncappedWholeViewResidue {
        surface: String,
    },
    QueryGapDispositionMismatch {
        surface: String,
    },
    MissingConfiguredSource {
        source_path: String,
    },
    UncoveredAuthorityPatterns {
        patterns: Vec<String>,
    },
    CertificationResidueCannotSatisfyOrdinaryInvalidation {
        surface: String,
    },
}

impl DerivedInvalidationAuthorityInventoryError {
    pub(crate) fn new(
        kind: DerivedInvalidationAuthorityInventoryErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> &DerivedInvalidationAuthorityInventoryErrorKind {
        &self.kind
    }
}

impl std::fmt::Display for DerivedInvalidationAuthorityInventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "derived invalidation authority inventory: {}",
            self.detail
        )
    }
}

impl std::error::Error for DerivedInvalidationAuthorityInventoryError {}
