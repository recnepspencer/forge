#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PhysicalShortcutBoundary {
    LiveRuntimeCache,
    BackendPrivateMap,
    RawDebugDump,
    FullStoreHeapMaterialization,
    BackendResidueGuessing,
}

impl PhysicalShortcutBoundary {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveRuntimeCache => "live_runtime_cache",
            Self::BackendPrivateMap => "backend_private_map",
            Self::RawDebugDump => "raw_debug_dump",
            Self::FullStoreHeapMaterialization => "full_store_heap_materialization",
            Self::BackendResidueGuessing => "backend_residue_guessing",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalShortcutBoundaryDenial {
    boundary: PhysicalShortcutBoundary,
}

impl PhysicalShortcutBoundaryDenial {
    pub(crate) const fn live_runtime_cache() -> Self {
        Self::new(PhysicalShortcutBoundary::LiveRuntimeCache)
    }

    pub(crate) const fn backend_private_map() -> Self {
        Self::new(PhysicalShortcutBoundary::BackendPrivateMap)
    }

    pub(crate) const fn raw_debug_dump() -> Self {
        Self::new(PhysicalShortcutBoundary::RawDebugDump)
    }

    pub(crate) const fn full_store_heap_materialization() -> Self {
        Self::new(PhysicalShortcutBoundary::FullStoreHeapMaterialization)
    }

    pub(crate) const fn backend_residue_guessing() -> Self {
        Self::new(PhysicalShortcutBoundary::BackendResidueGuessing)
    }

    pub const fn boundary(self) -> PhysicalShortcutBoundary {
        self.boundary
    }

    const fn new(boundary: PhysicalShortcutBoundary) -> Self {
        Self { boundary }
    }
}
