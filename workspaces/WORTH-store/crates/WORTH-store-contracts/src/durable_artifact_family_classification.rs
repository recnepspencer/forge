#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactFamilyAuthorityClass {
    Authoritative,
    Derived,
    Diagnostic,
    Terminal,
    CertificationEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactFamilyAccessLane {
    HotPath,
    MaintenancePath,
    VerifierPath,
    TerminalPath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactFamilyLifecycleClass {
    CoreState,
    RecoveryState,
    DerivedState,
    OperationalSupport,
    TransferBoundary,
    EvidenceOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurableArtifactOwningBoundary {
    WORTHStorePhysicalFormat,
    WORTHStoreWal,
    WORTHStoreRecoveryPhysics,
    WORTHStoreBufferPool,
    WORTHStorePhysicalIntegrity,
    WORTHStorePhysicalIsolation,
    WORTHStoreIoScheduler,
    WORTHStoreBlobChunks,
    WORTHStoreSecurity,
    WORTHStoreOperations,
    WORTHStoreCompatibility,
    WORTHStoreMaintenance,
    WORTHStoreRetention,
    WORTHStoreTiering,
    WORTHStoreSnapshots,
    WORTHStoreBranchDeltas,
    WORTHStoreOfflineVerifier,
}

impl DurableArtifactOwningBoundary {
    pub const fn crate_name(self) -> &'static str {
        match self {
            Self::WORTHStorePhysicalFormat => "worth-store-physical-format",
            Self::WORTHStoreWal => "worth-store-wal",
            Self::WORTHStoreRecoveryPhysics => "worth-store-recovery-physics",
            Self::WORTHStoreBufferPool => "worth-store-buffer-pool",
            Self::WORTHStorePhysicalIntegrity => "worth-store-physical-integrity",
            Self::WORTHStorePhysicalIsolation => "worth-store-physical-isolation",
            Self::WORTHStoreIoScheduler => "worth-store-io-scheduler",
            Self::WORTHStoreBlobChunks => "worth-store-blob-chunks",
            Self::WORTHStoreSecurity => "worth-store-security",
            Self::WORTHStoreOperations => "worth-store-operations",
            Self::WORTHStoreCompatibility => "worth-store-compatibility",
            Self::WORTHStoreMaintenance => "worth-store-maintenance",
            Self::WORTHStoreRetention => "worth-store-retention",
            Self::WORTHStoreTiering => "worth-store-tiering",
            Self::WORTHStoreSnapshots => "worth-store-snapshots",
            Self::WORTHStoreBranchDeltas => "worth-store-branch-deltas",
            Self::WORTHStoreOfflineVerifier => "worth-store-offline-verifier",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurableArtifactRebuildPosture {
    NoRebuild,
    RebuildFromAuthoritativeState,
    ReplayRebuildable,
    PartialRebuildOnly,
    QuarantineOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurableArtifactMigrationPosture {
    StableNoMigration,
    VersionedReadmission,
    VersionedMigration,
    RollbackCapable,
    OfflineImportOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurableArtifactProjectionClass {
    TerminalReport,
    OfflineObservation,
    CertificationEvidence,
    JsonExport,
    CounterSummary,
    TestFixture,
}
