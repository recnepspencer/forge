use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedInvalidationDeletionErrorKind {
    MissingPhaseEightSeed,
    PhaseEightSeedDoesNotMatchMigrationSweep,
    IncompleteMigrationSweep,
    OrdinaryResidueCannotClose,
    TrueQueryGapCannotClose,
    SourceFirewallViolation,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedInvalidationDeletionError {
    kind: DerivedInvalidationDeletionErrorKind,
    reason: String,
}

impl DerivedInvalidationDeletionError {
    pub(crate) fn new(
        kind: DerivedInvalidationDeletionErrorKind,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            reason: reason.into(),
        }
    }

    pub const fn kind(&self) -> DerivedInvalidationDeletionErrorKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl fmt::Display for DerivedInvalidationDeletionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.reason)
    }
}

impl std::error::Error for DerivedInvalidationDeletionError {}
