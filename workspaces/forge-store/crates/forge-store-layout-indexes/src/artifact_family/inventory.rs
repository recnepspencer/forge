use super::PhysicalArtifactFamilyDeclaration;
use crate::ArtifactFamilyDenial;
use forge_store_contracts::{
    ArtifactFamilyAccessLane, ArtifactFamilyAuthorityClass, ArtifactFamilyLifecycleClass,
    CompatibilityFamilyKind, DerivedFamilyRetentionPolicy, DurableArtifactFamilyId,
    DurableArtifactMigrationPosture, DurableArtifactOwningBoundary, DurableArtifactProjectionClass,
    DurableArtifactRebuildPosture, LayoutFamilyCompactionUnit, MaintenanceArtifactFamily,
    PlacementArtifactFamily, PublicationFamily, SupportArtifactFamily, WalRecordFamily,
};

const NONE: &[DurableArtifactProjectionClass] = &[];
const OFFLINE_ONLY: &[DurableArtifactProjectionClass] =
    &[DurableArtifactProjectionClass::OfflineObservation];
const TERMINAL_ONLY: &[DurableArtifactProjectionClass] =
    &[DurableArtifactProjectionClass::TerminalReport];
const EVIDENCE_ONLY: &[DurableArtifactProjectionClass] = &[
    DurableArtifactProjectionClass::OfflineObservation,
    DurableArtifactProjectionClass::CertificationEvidence,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactFamilyInventoryRow {
    declaration: PhysicalArtifactFamilyDeclaration,
}

impl ArtifactFamilyInventoryRow {
    pub const fn declaration(&self) -> &PhysicalArtifactFamilyDeclaration {
        &self.declaration
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S8ArtifactFamilyInventory;

pub trait ExistingArtifactFamilySurface: private::Sealed {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId;
}

mod private {
    pub trait Sealed {}
}

const fn row(
    family_id: DurableArtifactFamilyId,
    authority: ArtifactFamilyAuthorityClass,
    lifecycle: ArtifactFamilyLifecycleClass,
    access_lane: ArtifactFamilyAccessLane,
    owning_boundary: DurableArtifactOwningBoundary,
    rebuild_posture: DurableArtifactRebuildPosture,
    migration_posture: DurableArtifactMigrationPosture,
    projection_classes: &'static [DurableArtifactProjectionClass],
) -> ArtifactFamilyInventoryRow {
    ArtifactFamilyInventoryRow {
        declaration: PhysicalArtifactFamilyDeclaration::declare(
            family_id,
            authority,
            lifecycle,
            access_lane,
            owning_boundary,
            rebuild_posture,
            migration_posture,
            projection_classes,
        ),
    }
}

const ROWS: &[ArtifactFamilyInventoryRow] = &[
    row(
        DurableArtifactFamilyId::PhysicalPage,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStorePhysicalFormat,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::VersionedMigration,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::PhysicalSegment,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStorePhysicalFormat,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::VersionedMigration,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::PhysicalExtent,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStorePhysicalFormat,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::VersionedMigration,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::PhysicalRootManifest,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStorePhysicalFormat,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::VersionedMigration,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::WalDurableMutationIntent,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStoreWal,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::VersionedReadmission,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::WalHostedRuntimeCommitResult,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStoreWal,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::VersionedReadmission,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::WalBulkCheckpointPublicationIntent,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::RecoveryState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreWal,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::VersionedReadmission,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::WalDurablePublicationProgress,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::RecoveryState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreWal,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::VersionedReadmission,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::WalRecoveryDecision,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::RecoveryState,
        ArtifactFamilyAccessLane::VerifierPath,
        DurableArtifactOwningBoundary::ForgeStoreRecoveryPhysics,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::VersionedReadmission,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::BlobChunk,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStoreBlobChunks,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::VersionedMigration,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::BlobManifest,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStoreBlobChunks,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::VersionedMigration,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::BlobStream,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStoreBlobChunks,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::VersionedMigration,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::ChunkTreeRoot,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::DerivedState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreBlobChunks,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::DedupeIndex,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::DerivedState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreBlobChunks,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::ReachabilityEdge,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::DerivedState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreRetention,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::RetentionHold,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreRetention,
        DurableArtifactRebuildPosture::QuarantineOnly,
        DurableArtifactMigrationPosture::VersionedMigration,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::ReclaimReceipt,
        ArtifactFamilyAuthorityClass::Diagnostic,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreMaintenance,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::StableNoMigration,
        EVIDENCE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::PlacementAuthoritativeBranchHead,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreTiering,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::PlacementRetainedAuthority,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreTiering,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::PlacementStableBasis,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreTiering,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::PlacementSnapshotFamily,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreTiering,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::PlacementBranchDeltaFamily,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreTiering,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::PlacementMilestone6LayoutFamily,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreTiering,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::ResidencyRecord,
        ArtifactFamilyAuthorityClass::Diagnostic,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreBufferPool,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CorruptionRecord,
        ArtifactFamilyAuthorityClass::Diagnostic,
        ArtifactFamilyLifecycleClass::EvidenceOnly,
        ArtifactFamilyAccessLane::VerifierPath,
        DurableArtifactOwningBoundary::ForgeStorePhysicalIntegrity,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::VersionedReadmission,
        EVIDENCE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::QuarantineRecord,
        ArtifactFamilyAuthorityClass::Diagnostic,
        ArtifactFamilyLifecycleClass::RecoveryState,
        ArtifactFamilyAccessLane::VerifierPath,
        DurableArtifactOwningBoundary::ForgeStoreRecoveryPhysics,
        DurableArtifactRebuildPosture::QuarantineOnly,
        DurableArtifactMigrationPosture::VersionedReadmission,
        EVIDENCE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::RepairRecord,
        ArtifactFamilyAuthorityClass::Diagnostic,
        ArtifactFamilyLifecycleClass::RecoveryState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreOperations,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::VersionedReadmission,
        EVIDENCE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::ReadmissionRecord,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::RecoveryState,
        ArtifactFamilyAccessLane::VerifierPath,
        DurableArtifactOwningBoundary::ForgeStoreRecoveryPhysics,
        DurableArtifactRebuildPosture::QuarantineOnly,
        DurableArtifactMigrationPosture::VersionedReadmission,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::SecurityCustodyLookup,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStoreSecurity,
        DurableArtifactRebuildPosture::QuarantineOnly,
        DurableArtifactMigrationPosture::VersionedMigration,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::ExportBundle,
        ArtifactFamilyAuthorityClass::Terminal,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::TerminalPath,
        DurableArtifactOwningBoundary::ForgeStoreOperations,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::OfflineImportOnly,
        TERMINAL_ONLY,
    ),
    row(
        DurableArtifactFamilyId::ImportBundle,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreOperations,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::OfflineImportOnly,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CapsuleArtifact,
        ArtifactFamilyAuthorityClass::Terminal,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::TerminalPath,
        DurableArtifactOwningBoundary::ForgeStoreOperations,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::OfflineImportOnly,
        TERMINAL_ONLY,
    ),
    row(
        DurableArtifactFamilyId::OfflineVerificationRecord,
        ArtifactFamilyAuthorityClass::Diagnostic,
        ArtifactFamilyLifecycleClass::EvidenceOnly,
        ArtifactFamilyAccessLane::VerifierPath,
        DurableArtifactOwningBoundary::ForgeStoreOfflineVerifier,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::StableNoMigration,
        EVIDENCE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilityCommitEnvelope,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilityBranchVersionDagRecord,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilityWalRestartRecord,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilitySchemaLineageCursorCheckpointSupport,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilityEmbeddedCheckpointAuthority,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::NoRebuild,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilitySnapshotRecord,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilityDeltaRecord,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilityMilestone6LayoutBlockChunkRecord,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilityMilestone8BasisContinuationDescriptor,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilityMilestone9BulkRecord,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilityMilestone10RetentionRebuildRecord,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilityMilestone11MaintenanceRecord,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::CompatibilityMilestone13TieringRecord,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::TransferBoundary,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreCompatibility,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::VersionedMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::MaintenanceSnapshot,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreMaintenance,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::MaintenanceCompaction,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreMaintenance,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::MaintenanceReclaim,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreMaintenance,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::MaintenanceCapsule,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreMaintenance,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::SupportSchema,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreRecoveryPhysics,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::SupportLineage,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreRecoveryPhysics,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::SupportCursor,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreRecoveryPhysics,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::SupportEmbeddedCheckpoint,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreRecoveryPhysics,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::PublicationWalIntent,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStoreOperations,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::RollbackCapable,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::PublicationWalCanonicalResult,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStoreOperations,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::RollbackCapable,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::PublicationWalPublicationProgress,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStoreOperations,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::RollbackCapable,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::PublicationAuthoritativeCommitAppendUnit,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStoreOperations,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::RollbackCapable,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::PublicationBranchHeadPublication,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStoreOperations,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::RollbackCapable,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::PublicationAcknowledgmentEligibility,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::OperationalSupport,
        ArtifactFamilyAccessLane::HotPath,
        DurableArtifactOwningBoundary::ForgeStoreOperations,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::RollbackCapable,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::PublicationSnapshotBasis,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreOperations,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::RollbackCapable,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::PublicationSnapshotImage,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreOperations,
        DurableArtifactRebuildPosture::ReplayRebuildable,
        DurableArtifactMigrationPosture::RollbackCapable,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::DerivedRetentionMilestone6LayoutMaterialization,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::DerivedState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreRetention,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::DerivedRetentionMilestone6ScopeSliceMembership,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::DerivedState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreRetention,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::DerivedRetentionMilestone6StructuralBlock,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::DerivedState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreRetention,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::DerivedRetentionMilestone6ChunkMembership,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::DerivedState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreRetention,
        DurableArtifactRebuildPosture::RebuildFromAuthoritativeState,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::LayoutCompactionUnit,
        ArtifactFamilyAuthorityClass::Derived,
        ArtifactFamilyLifecycleClass::DerivedState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreRetention,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::StableNoMigration,
        OFFLINE_ONLY,
    ),
    row(
        DurableArtifactFamilyId::SnapshotArtifact,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreSnapshots,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::RollbackCapable,
        NONE,
    ),
    row(
        DurableArtifactFamilyId::BranchDeltaArtifact,
        ArtifactFamilyAuthorityClass::Authoritative,
        ArtifactFamilyLifecycleClass::CoreState,
        ArtifactFamilyAccessLane::MaintenancePath,
        DurableArtifactOwningBoundary::ForgeStoreBranchDeltas,
        DurableArtifactRebuildPosture::PartialRebuildOnly,
        DurableArtifactMigrationPosture::RollbackCapable,
        NONE,
    ),
];

impl S8ArtifactFamilyInventory {
    pub const fn current() -> Self {
        Self
    }

    pub const fn rows(&self) -> &'static [ArtifactFamilyInventoryRow] {
        ROWS
    }

    pub fn declaration(
        &self,
        family_id: DurableArtifactFamilyId,
    ) -> Result<&'static PhysicalArtifactFamilyDeclaration, ArtifactFamilyDenial> {
        declaration_in_rows(ROWS, family_id)
    }

    pub fn admit_existing_family(
        &self,
        family: &impl ExistingArtifactFamilySurface,
    ) -> Result<&'static PhysicalArtifactFamilyDeclaration, ArtifactFamilyDenial> {
        self.declaration(family.canonical_family_id())
    }
}

pub(crate) fn declaration_in_rows<'a>(
    rows: &'a [ArtifactFamilyInventoryRow],
    family_id: DurableArtifactFamilyId,
) -> Result<&'a PhysicalArtifactFamilyDeclaration, ArtifactFamilyDenial> {
    let mut index = 0;
    while index < rows.len() {
        let row = &rows[index];
        if row.declaration().family_id() == family_id {
            return Ok(row.declaration());
        }
        index += 1;
    }
    Err(ArtifactFamilyDenial::MissingFamilyDeclaration)
}

impl private::Sealed for WalRecordFamily {}
impl ExistingArtifactFamilySurface for WalRecordFamily {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::DurableMutationIntent => DurableArtifactFamilyId::WalDurableMutationIntent,
            Self::HostedRuntimeCommitResult => {
                DurableArtifactFamilyId::WalHostedRuntimeCommitResult
            }
            Self::BulkCheckpointPublicationIntent => {
                DurableArtifactFamilyId::WalBulkCheckpointPublicationIntent
            }
            Self::DurablePublicationProgress => {
                DurableArtifactFamilyId::WalDurablePublicationProgress
            }
            Self::RecoveryDecision => DurableArtifactFamilyId::WalRecoveryDecision,
        }
    }
}

impl private::Sealed for CompatibilityFamilyKind {}
impl ExistingArtifactFamilySurface for CompatibilityFamilyKind {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::CommitEnvelope => DurableArtifactFamilyId::CompatibilityCommitEnvelope,
            Self::BranchVersionDagRecord => {
                DurableArtifactFamilyId::CompatibilityBranchVersionDagRecord
            }
            Self::WalRestartRecord => DurableArtifactFamilyId::CompatibilityWalRestartRecord,
            Self::SchemaLineageCursorCheckpointSupport => {
                DurableArtifactFamilyId::CompatibilitySchemaLineageCursorCheckpointSupport
            }
            Self::EmbeddedCheckpointAuthority => {
                DurableArtifactFamilyId::CompatibilityEmbeddedCheckpointAuthority
            }
            Self::SnapshotRecord => DurableArtifactFamilyId::CompatibilitySnapshotRecord,
            Self::DeltaRecord => DurableArtifactFamilyId::CompatibilityDeltaRecord,
            Self::Milestone6LayoutBlockChunkRecord => {
                DurableArtifactFamilyId::CompatibilityMilestone6LayoutBlockChunkRecord
            }
            Self::Milestone8BasisContinuationDescriptor => {
                DurableArtifactFamilyId::CompatibilityMilestone8BasisContinuationDescriptor
            }
            Self::Milestone9BulkRecord => {
                DurableArtifactFamilyId::CompatibilityMilestone9BulkRecord
            }
            Self::Milestone10RetentionRebuildRecord => {
                DurableArtifactFamilyId::CompatibilityMilestone10RetentionRebuildRecord
            }
            Self::Milestone11MaintenanceRecord => {
                DurableArtifactFamilyId::CompatibilityMilestone11MaintenanceRecord
            }
            Self::Milestone13TieringRecord => {
                DurableArtifactFamilyId::CompatibilityMilestone13TieringRecord
            }
        }
    }
}

impl private::Sealed for MaintenanceArtifactFamily {}
impl ExistingArtifactFamilySurface for MaintenanceArtifactFamily {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::Snapshot => DurableArtifactFamilyId::MaintenanceSnapshot,
            Self::Compaction => DurableArtifactFamilyId::MaintenanceCompaction,
            Self::Reclaim => DurableArtifactFamilyId::MaintenanceReclaim,
            Self::Capsule => DurableArtifactFamilyId::MaintenanceCapsule,
        }
    }
}

impl private::Sealed for SupportArtifactFamily {}
impl ExistingArtifactFamilySurface for SupportArtifactFamily {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::SchemaSupport => DurableArtifactFamilyId::SupportSchema,
            Self::LineageSupport => DurableArtifactFamilyId::SupportLineage,
            Self::CursorSupport => DurableArtifactFamilyId::SupportCursor,
            Self::EmbeddedCheckpoint => DurableArtifactFamilyId::SupportEmbeddedCheckpoint,
        }
    }
}

impl private::Sealed for PlacementArtifactFamily {}
impl ExistingArtifactFamilySurface for PlacementArtifactFamily {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::AuthoritativeBranchHead => {
                DurableArtifactFamilyId::PlacementAuthoritativeBranchHead
            }
            Self::RetainedAuthority => DurableArtifactFamilyId::PlacementRetainedAuthority,
            Self::StableBasis => DurableArtifactFamilyId::PlacementStableBasis,
            Self::SnapshotFamily => DurableArtifactFamilyId::PlacementSnapshotFamily,
            Self::BranchDeltaFamily => DurableArtifactFamilyId::PlacementBranchDeltaFamily,
            Self::Milestone6LayoutFamily => {
                DurableArtifactFamilyId::PlacementMilestone6LayoutFamily
            }
        }
    }
}

impl private::Sealed for PublicationFamily {}
impl ExistingArtifactFamilySurface for PublicationFamily {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::WalIntent => DurableArtifactFamilyId::PublicationWalIntent,
            Self::WalCanonicalResult => DurableArtifactFamilyId::PublicationWalCanonicalResult,
            Self::WalPublicationProgress => {
                DurableArtifactFamilyId::PublicationWalPublicationProgress
            }
            Self::AuthoritativeCommitAppendUnit => {
                DurableArtifactFamilyId::PublicationAuthoritativeCommitAppendUnit
            }
            Self::BranchHeadPublication => {
                DurableArtifactFamilyId::PublicationBranchHeadPublication
            }
            Self::AcknowledgmentEligibility => {
                DurableArtifactFamilyId::PublicationAcknowledgmentEligibility
            }
            Self::SnapshotBasis => DurableArtifactFamilyId::PublicationSnapshotBasis,
            Self::SnapshotImage => DurableArtifactFamilyId::PublicationSnapshotImage,
        }
    }
}

impl private::Sealed for DerivedFamilyRetentionPolicy {}
impl ExistingArtifactFamilySurface for DerivedFamilyRetentionPolicy {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self {
            Self::Milestone6LayoutMaterialization => {
                DurableArtifactFamilyId::DerivedRetentionMilestone6LayoutMaterialization
            }
            Self::Milestone6ScopeSliceMembership => {
                DurableArtifactFamilyId::DerivedRetentionMilestone6ScopeSliceMembership
            }
            Self::Milestone6StructuralBlock => {
                DurableArtifactFamilyId::DerivedRetentionMilestone6StructuralBlock
            }
            Self::Milestone6ChunkMembership => {
                DurableArtifactFamilyId::DerivedRetentionMilestone6ChunkMembership
            }
        }
    }
}

impl private::Sealed for LayoutFamilyCompactionUnit {}
impl ExistingArtifactFamilySurface for LayoutFamilyCompactionUnit {
    fn canonical_family_id(&self) -> DurableArtifactFamilyId {
        match self.family_kind() {
            forge_store_contracts::LayoutCompactionFamilyKind::LayoutCompactionUnit => {
                DurableArtifactFamilyId::LayoutCompactionUnit
            }
        }
    }
}
