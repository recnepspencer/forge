#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MosaicStatePersistencePolicy {
    EphemeralDuringRuntime,
    RestoreAcrossHotReload,
    PersistAcrossRuntimeRestart,
    MissingForDiagnostics,
}

impl MosaicStatePersistencePolicy {
    pub fn ephemeral_during_runtime() -> Self {
        Self::EphemeralDuringRuntime
    }

    pub fn restore_across_hot_reload() -> Self {
        Self::RestoreAcrossHotReload
    }

    pub fn persist_across_runtime_restart() -> Self {
        Self::PersistAcrossRuntimeRestart
    }

    pub fn missing_for_diagnostics() -> Self {
        Self::MissingForDiagnostics
    }

    pub(crate) fn is_missing(&self) -> bool {
        matches!(self, Self::MissingForDiagnostics)
    }

    pub(crate) fn digest_basis(&self) -> &'static str {
        match self {
            Self::EphemeralDuringRuntime => "ephemeral_during_runtime",
            Self::RestoreAcrossHotReload => "restore_across_hot_reload",
            Self::PersistAcrossRuntimeRestart => "persist_across_runtime_restart",
            Self::MissingForDiagnostics => "missing",
        }
    }
}
