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
    ForgeStorePhysicalFormat,
    ForgeStoreWal,
    ForgeStoreRecoveryPhysics,
    ForgeStoreBufferPool,
    ForgeStorePhysicalIntegrity,
    ForgeStorePhysicalIsolation,
    ForgeStoreIoScheduler,
    ForgeStoreBlobChunks,
    ForgeStoreSecurity,
    ForgeStoreOperations,
    ForgeStoreCompatibility,
    ForgeStoreMaintenance,
    ForgeStoreRetention,
    ForgeStoreTiering,
    ForgeStoreSnapshots,
    ForgeStoreBranchDeltas,
    ForgeStoreOfflineVerifier,
}

impl DurableArtifactOwningBoundary {
    pub const fn crate_name(self) -> &'static str {
        match self {
            Self::ForgeStorePhysicalFormat => "forge-store-physical-format",
            Self::ForgeStoreWal => "forge-store-wal",
            Self::ForgeStoreRecoveryPhysics => "forge-store-recovery-physics",
            Self::ForgeStoreBufferPool => "forge-store-buffer-pool",
            Self::ForgeStorePhysicalIntegrity => "forge-store-physical-integrity",
            Self::ForgeStorePhysicalIsolation => "forge-store-physical-isolation",
            Self::ForgeStoreIoScheduler => "forge-store-io-scheduler",
            Self::ForgeStoreBlobChunks => "forge-store-blob-chunks",
            Self::ForgeStoreSecurity => "forge-store-security",
            Self::ForgeStoreOperations => "forge-store-operations",
            Self::ForgeStoreCompatibility => "forge-store-compatibility",
            Self::ForgeStoreMaintenance => "forge-store-maintenance",
            Self::ForgeStoreRetention => "forge-store-retention",
            Self::ForgeStoreTiering => "forge-store-tiering",
            Self::ForgeStoreSnapshots => "forge-store-snapshots",
            Self::ForgeStoreBranchDeltas => "forge-store-branch-deltas",
            Self::ForgeStoreOfflineVerifier => "forge-store-offline-verifier",
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
