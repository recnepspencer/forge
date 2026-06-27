/// Persistence posture for runtime-owned placement state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicPlacementPersistence {
    Ephemeral,
    Restorable,
    Persistent,
    MissingForDiagnostics,
}

impl MosaicPlacementPersistence {
    pub fn ephemeral() -> Self {
        Self::Ephemeral
    }

    pub fn restorable() -> Self {
        Self::Restorable
    }

    pub fn persistent() -> Self {
        Self::Persistent
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::Ephemeral => "ephemeral",
            Self::Restorable => "restorable",
            Self::Persistent => "persistent",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
