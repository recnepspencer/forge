#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlobHarnessSizeClass {
    TinyShortcut,
    LocalDeterministic,
    MemoryEnvelopeExceeding,
    HeavyMultiGbDeclared,
}

impl BlobHarnessSizeClass {
    pub const fn declared_logical_bytes(self) -> u64 {
        match self {
            Self::TinyShortcut => 1024,
            Self::LocalDeterministic => 8 * 1024 * 1024,
            Self::MemoryEnvelopeExceeding => 768 * 1024 * 1024,
            Self::HeavyMultiGbDeclared => 2 * 1024 * 1024 * 1024,
        }
    }

    pub const fn is_shortcut_sized(self) -> bool {
        matches!(self, Self::TinyShortcut)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlobHarnessChunkSizeClass {
    Fixed64KiB,
    Fixed1MiB,
    Fixed8MiB,
}

impl BlobHarnessChunkSizeClass {
    pub const fn chunk_bytes(self) -> u64 {
        match self {
            Self::Fixed64KiB => 64 * 1024,
            Self::Fixed1MiB => 1024 * 1024,
            Self::Fixed8MiB => 8 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlobHarnessPlacementClass {
    StoreLocal,
    ExternalPlacementObserved,
    ColdTierObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlobHarnessSecurityScopeClass {
    ScopePreserving,
    CrossScopeDenied,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlobHarnessAccessMode {
    ReadOnlyReplay,
    ResumableIngestSeed,
    ExportBoundary,
    ImportReadmission,
    PartialReplication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlobHarnessFailurePoint {
    NoFaultSeed,
    AfterChunkWrite,
    AfterSessionCheckpoint,
    AfterRootPublication,
    DuringTierMove,
    DuringExport,
    DuringReclaim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlobHarnessActorMix {
    SeedReplayOnly,
    IngestReadVerify,
    ResumeRecovery,
    DedupeReclaim,
    ExportImport,
    PlacementMovePartialReplication,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BlobHarnessChunkTopology {
    chunk_count: u64,
    logical_bytes: u64,
    chunk_bytes: u64,
}

impl BlobHarnessChunkTopology {
    pub fn from_classes(
        size_class: BlobHarnessSizeClass,
        chunk_size_class: BlobHarnessChunkSizeClass,
    ) -> Result<Self, BlobHarnessTopologyDenial> {
        if size_class.is_shortcut_sized() {
            return Err(BlobHarnessTopologyDenial::TinyBlobShortcut);
        }
        let logical_bytes = size_class.declared_logical_bytes();
        let chunk_bytes = chunk_size_class.chunk_bytes();
        let chunk_count = logical_bytes.div_ceil(chunk_bytes);
        if chunk_count == 0 {
            return Err(BlobHarnessTopologyDenial::MissingChunkCounters);
        }
        Ok(Self {
            chunk_count,
            logical_bytes,
            chunk_bytes,
        })
    }

    #[cfg(feature = "certification-test-authority")]
    pub(crate) fn from_executed_projection(
        chunk_count: u64,
        logical_bytes: u64,
        chunk_bytes: u64,
    ) -> Result<Self, BlobHarnessTopologyDenial> {
        if chunk_count == 0 || logical_bytes == 0 || chunk_bytes == 0 {
            return Err(BlobHarnessTopologyDenial::MissingChunkCounters);
        }
        if chunk_count != logical_bytes.div_ceil(chunk_bytes) {
            return Err(BlobHarnessTopologyDenial::InconsistentChunkCounters);
        }
        Ok(Self {
            chunk_count,
            logical_bytes,
            chunk_bytes,
        })
    }

    pub const fn chunk_count(self) -> u64 {
        self.chunk_count
    }

    pub const fn logical_bytes(self) -> u64 {
        self.logical_bytes
    }

    pub const fn chunk_bytes(self) -> u64 {
        self.chunk_bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlobHarnessTopologyDenial {
    TinyBlobShortcut,
    MissingChunkCounters,
    InconsistentChunkCounters,
}
