#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicSizingPersistence {
    Ephemeral,
    Restorable,
    PersistedUserOverride,
    MissingForDiagnostics,
}

impl MosaicSizingPersistence {
    pub fn ephemeral() -> Self {
        Self::Ephemeral
    }

    pub fn restorable() -> Self {
        Self::Restorable
    }

    pub fn persisted_user_override() -> Self {
        Self::PersistedUserOverride
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
            Self::PersistedUserOverride => "persisted_user_override",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
