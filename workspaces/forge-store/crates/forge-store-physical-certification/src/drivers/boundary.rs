#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DriverBoundaryKind {
    ProductionStorage,
    AdversarialStorage,
    CrashRuntimeIsolation,
    MemoryPressure,
    IoPressure,
    OfflineVerifier,
    ShortcutRejection,
    FutureExtensionSlot,
}

impl DriverBoundaryKind {
    pub const fn token(self) -> &'static str {
        match self {
            Self::ProductionStorage => "production-storage",
            Self::AdversarialStorage => "adversarial-storage",
            Self::CrashRuntimeIsolation => "crash-runtime-isolation",
            Self::MemoryPressure => "memory-pressure",
            Self::IoPressure => "io-pressure",
            Self::OfflineVerifier => "offline-verifier",
            Self::ShortcutRejection => "shortcut-rejection",
            Self::FutureExtensionSlot => "future-extension-slot",
        }
    }
}
