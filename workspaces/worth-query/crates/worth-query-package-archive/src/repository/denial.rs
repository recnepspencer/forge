//! Typed no-effect denials and uncertain-effect store outcomes.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPackageArchiveRepositoryDenialKind {
    EnvelopeByteBudgetExceeded,
    Unavailable,
    CapacityExhausted,
    DeadlineExceeded,
}

/// Repository operation refusal for which no store effect began.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageArchiveRepositoryDenial {
    kind: WorthQueryPackageArchiveRepositoryDenialKind,
}

impl WorthQueryPackageArchiveRepositoryDenial {
    pub const fn new(kind: WorthQueryPackageArchiveRepositoryDenialKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthQueryPackageArchiveRepositoryDenialKind {
        self.kind
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPackageArchiveStoreIndeterminateKind {
    ConnectionLostAfterEffectStart,
    DeadlineAfterEffectStart,
    ResponseLost,
}

/// Store attempt whose physical completion is not yet known.
///
/// Repeating the same exact record is safe because the repository contract is
/// immutable and idempotent. Substituting different bytes under the claimed
/// identity is never a recovery operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPackageArchiveStoreIndeterminate {
    kind: WorthQueryPackageArchiveStoreIndeterminateKind,
}

impl WorthQueryPackageArchiveStoreIndeterminate {
    pub const fn new(kind: WorthQueryPackageArchiveStoreIndeterminateKind) -> Self {
        Self { kind }
    }

    pub const fn kind(&self) -> WorthQueryPackageArchiveStoreIndeterminateKind {
        self.kind
    }
}
