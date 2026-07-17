#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProductionStorageBoundarySeam {
    WalAppendBeforeFlush,
    WalFlush,
    DirectorySync,
    RootLoad,
    RootPublicationBeforeObserve,
    RootSwap,
    PagePin,
    LeasePublish,
    ReclaimEligibility,
    CheckpointManifestWrite,
    CompactionCutover,
    ReplicationProgressSnapshotWrite,
    ReplicationProgressSnapshotDurable,
    CrashSeam,
    FutureExtensionSlot,
}

pub const PHASE4_PRODUCTION_STORAGE_BOUNDARY_SEAMS: &[ProductionStorageBoundarySeam] = &[
    ProductionStorageBoundarySeam::WalAppendBeforeFlush,
    ProductionStorageBoundarySeam::WalFlush,
    ProductionStorageBoundarySeam::DirectorySync,
    ProductionStorageBoundarySeam::RootLoad,
    ProductionStorageBoundarySeam::RootPublicationBeforeObserve,
    ProductionStorageBoundarySeam::RootSwap,
    ProductionStorageBoundarySeam::PagePin,
    ProductionStorageBoundarySeam::LeasePublish,
    ProductionStorageBoundarySeam::ReclaimEligibility,
    ProductionStorageBoundarySeam::CheckpointManifestWrite,
    ProductionStorageBoundarySeam::CompactionCutover,
    ProductionStorageBoundarySeam::ReplicationProgressSnapshotWrite,
    ProductionStorageBoundarySeam::ReplicationProgressSnapshotDurable,
    ProductionStorageBoundarySeam::CrashSeam,
    ProductionStorageBoundarySeam::FutureExtensionSlot,
];

impl ProductionStorageBoundarySeam {
    pub const fn registered_backend_operation_seams() -> &'static [Self] {
        PHASE4_PRODUCTION_STORAGE_BOUNDARY_SEAMS
    }

    pub const fn token(self) -> &'static str {
        match self {
            Self::WalAppendBeforeFlush => "wal-append-before-flush",
            Self::WalFlush => "wal-flush",
            Self::DirectorySync => "directory-sync",
            Self::RootLoad => "root-load",
            Self::RootPublicationBeforeObserve => "root-publication-before-observe",
            Self::RootSwap => "root-swap",
            Self::PagePin => "page-pin",
            Self::LeasePublish => "lease-publish",
            Self::ReclaimEligibility => "reclaim-eligibility",
            Self::CheckpointManifestWrite => "checkpoint-manifest-write",
            Self::CompactionCutover => "compaction-cutover",
            Self::ReplicationProgressSnapshotWrite => "replication-progress-snapshot-write",
            Self::ReplicationProgressSnapshotDurable => "replication-progress-snapshot-durable",
            Self::CrashSeam => "crash-seam",
            Self::FutureExtensionSlot => "future-extension-slot",
        }
    }
}
