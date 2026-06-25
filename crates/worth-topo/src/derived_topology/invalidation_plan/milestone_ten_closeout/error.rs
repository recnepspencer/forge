#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedInvalidationMilestoneTenErrorKind {
    CatalogMismatch,
    SelectedPlanMismatch,
    ExecutionReceiptMismatch,
    TouchedClosureMismatch,
    QuerySupportMismatch,
    LegalitySupportMismatch,
    IncompleteProductMigration,
    OperatorCutoverMismatch,
    DeletionCloseoutMismatch,
    SourceFirewallViolation,
    OldAuthorityResidue,
    WholeViewFallback,
    CallerOwnedGraphWork,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedInvalidationMilestoneTenError {
    kind: DerivedInvalidationMilestoneTenErrorKind,
    message: String,
}

impl DerivedInvalidationMilestoneTenError {
    pub(crate) fn new(
        kind: DerivedInvalidationMilestoneTenErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub const fn kind(&self) -> DerivedInvalidationMilestoneTenErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn reason(&self) -> &str {
        &self.message
    }
}

impl std::fmt::Display for DerivedInvalidationMilestoneTenError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for DerivedInvalidationMilestoneTenError {}
