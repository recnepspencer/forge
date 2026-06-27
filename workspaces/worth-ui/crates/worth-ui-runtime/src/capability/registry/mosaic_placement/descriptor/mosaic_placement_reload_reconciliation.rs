/// Reload reconciliation posture for runtime-owned placement policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicPlacementReloadReconciliation {
    RestoreWhenPossible,
    DropWhenSourceMissing,
    ReopenDefault,
    RequireExplicitReplay,
    MissingForDiagnostics,
}

impl MosaicPlacementReloadReconciliation {
    pub fn restore_when_possible() -> Self {
        Self::RestoreWhenPossible
    }

    pub fn drop_when_source_missing() -> Self {
        Self::DropWhenSourceMissing
    }

    pub fn reopen_default() -> Self {
        Self::ReopenDefault
    }

    pub fn require_explicit_replay() -> Self {
        Self::RequireExplicitReplay
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::RestoreWhenPossible => "restore_when_possible",
            Self::DropWhenSourceMissing => "drop_when_source_missing",
            Self::ReopenDefault => "reopen_default",
            Self::RequireExplicitReplay => "require_explicit_replay",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
