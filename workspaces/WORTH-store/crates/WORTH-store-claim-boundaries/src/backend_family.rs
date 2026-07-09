#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LegacyBackendFamily {
    Heap,
    File,
    Sqlite,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BackendFamily {
    Legacy(LegacyBackendFamily),
    PhysicalFoundationCandidate,
    PlatformPhysicalFacade,
}

impl BackendFamily {
    pub const fn legacy(legacy_family: LegacyBackendFamily) -> Self {
        Self::Legacy(legacy_family)
    }

    pub const fn is_legacy(&self) -> bool {
        matches!(self, Self::Legacy(_))
    }
}
