#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedInvalidationSelectionErrorKind {
    CatalogSeedMismatch,
    TouchedClosureEmpty,
    CounterLeakage,
    ConflictRoutingContractRejected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedInvalidationSelectionError {
    kind: DerivedInvalidationSelectionErrorKind,
    message: String,
}

impl DerivedInvalidationSelectionError {
    pub(crate) fn new(
        kind: DerivedInvalidationSelectionErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub const fn kind(&self) -> DerivedInvalidationSelectionErrorKind {
        self.kind
    }
}

impl std::fmt::Display for DerivedInvalidationSelectionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for DerivedInvalidationSelectionError {}
