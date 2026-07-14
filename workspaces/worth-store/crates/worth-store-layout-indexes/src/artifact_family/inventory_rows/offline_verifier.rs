use super::super::ArtifactFamilyInventoryRow;
use super::row::{row, EVIDENCE_ONLY};
use worth_store_contracts::{
    ArtifactFamilyAccessLane as Lane, ArtifactFamilyAuthorityClass as Authority,
    ArtifactFamilyLifecycleClass as Lifecycle, DurableArtifactFamilyId as Family,
    DurableArtifactMigrationPosture as Migration, DurableArtifactOwningBoundary as Owner,
    DurableArtifactRebuildPosture as Rebuild,
};

pub(super) const ROWS: &[ArtifactFamilyInventoryRow] = &[row(
    Family::OfflineVerificationRecord,
    Authority::Diagnostic,
    Lifecycle::EvidenceOnly,
    Lane::VerifierPath,
    Owner::WorthStoreOfflineVerifier,
    Rebuild::NoRebuild,
    Migration::StableNoMigration,
    EVIDENCE_ONLY,
)];
