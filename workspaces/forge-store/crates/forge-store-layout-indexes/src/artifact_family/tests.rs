use super::{
    ArtifactFamilyAccessLane, ArtifactFamilyAuthorityClass, ArtifactFamilyDenial,
    S8ArtifactFamilyInventory,
};
use crate::layout_declarations;
use forge_store_contracts::{
    CompatibilityFamilyKind, DerivedFamilyRetentionPolicy, DurableArtifactFamilyId,
    DurableArtifactOwningBoundary, LayoutCompactionFamilyKind, LayoutFamilyCompactionUnit,
    MaintenanceArtifactFamily, PlacementArtifactFamily, PublicationFamily, SupportArtifactFamily,
    WalRecordFamily,
};

const EXPECTED_FAMILIES: &[DurableArtifactFamilyId] = &[
    DurableArtifactFamilyId::PhysicalPage,
    DurableArtifactFamilyId::PhysicalSegment,
    DurableArtifactFamilyId::PhysicalExtent,
    DurableArtifactFamilyId::PhysicalRootManifest,
    DurableArtifactFamilyId::WalDurableMutationIntent,
    DurableArtifactFamilyId::WalHostedRuntimeCommitResult,
    DurableArtifactFamilyId::WalBulkCheckpointPublicationIntent,
    DurableArtifactFamilyId::WalDurablePublicationProgress,
    DurableArtifactFamilyId::WalRecoveryDecision,
    DurableArtifactFamilyId::BlobChunk,
    DurableArtifactFamilyId::BlobManifest,
    DurableArtifactFamilyId::BlobStream,
    DurableArtifactFamilyId::ChunkTreeRoot,
    DurableArtifactFamilyId::DedupeIndex,
    DurableArtifactFamilyId::ReachabilityEdge,
    DurableArtifactFamilyId::RetentionHold,
    DurableArtifactFamilyId::ReclaimReceipt,
    DurableArtifactFamilyId::ResidencyRecord,
    DurableArtifactFamilyId::CorruptionRecord,
    DurableArtifactFamilyId::QuarantineRecord,
    DurableArtifactFamilyId::RepairRecord,
    DurableArtifactFamilyId::ReadmissionRecord,
    DurableArtifactFamilyId::SecurityCustodyLookup,
    DurableArtifactFamilyId::ExportBundle,
    DurableArtifactFamilyId::ImportBundle,
    DurableArtifactFamilyId::CapsuleArtifact,
    DurableArtifactFamilyId::OfflineVerificationRecord,
    DurableArtifactFamilyId::CompatibilityCommitEnvelope,
    DurableArtifactFamilyId::CompatibilityBranchVersionDagRecord,
    DurableArtifactFamilyId::CompatibilityWalRestartRecord,
    DurableArtifactFamilyId::CompatibilitySchemaLineageCursorCheckpointSupport,
    DurableArtifactFamilyId::CompatibilityEmbeddedCheckpointAuthority,
    DurableArtifactFamilyId::CompatibilitySnapshotRecord,
    DurableArtifactFamilyId::CompatibilityDeltaRecord,
    DurableArtifactFamilyId::CompatibilityMilestone6LayoutBlockChunkRecord,
    DurableArtifactFamilyId::CompatibilityMilestone8BasisContinuationDescriptor,
    DurableArtifactFamilyId::CompatibilityMilestone9BulkRecord,
    DurableArtifactFamilyId::CompatibilityMilestone10RetentionRebuildRecord,
    DurableArtifactFamilyId::CompatibilityMilestone11MaintenanceRecord,
    DurableArtifactFamilyId::CompatibilityMilestone13TieringRecord,
    DurableArtifactFamilyId::MaintenanceSnapshot,
    DurableArtifactFamilyId::MaintenanceCompaction,
    DurableArtifactFamilyId::MaintenanceReclaim,
    DurableArtifactFamilyId::MaintenanceCapsule,
    DurableArtifactFamilyId::SupportSchema,
    DurableArtifactFamilyId::SupportLineage,
    DurableArtifactFamilyId::SupportCursor,
    DurableArtifactFamilyId::SupportEmbeddedCheckpoint,
    DurableArtifactFamilyId::PlacementAuthoritativeBranchHead,
    DurableArtifactFamilyId::PlacementRetainedAuthority,
    DurableArtifactFamilyId::PlacementStableBasis,
    DurableArtifactFamilyId::PlacementSnapshotFamily,
    DurableArtifactFamilyId::PlacementBranchDeltaFamily,
    DurableArtifactFamilyId::PlacementMilestone6LayoutFamily,
    DurableArtifactFamilyId::PublicationWalIntent,
    DurableArtifactFamilyId::PublicationWalCanonicalResult,
    DurableArtifactFamilyId::PublicationWalPublicationProgress,
    DurableArtifactFamilyId::PublicationAuthoritativeCommitAppendUnit,
    DurableArtifactFamilyId::PublicationBranchHeadPublication,
    DurableArtifactFamilyId::PublicationAcknowledgmentEligibility,
    DurableArtifactFamilyId::PublicationSnapshotBasis,
    DurableArtifactFamilyId::PublicationSnapshotImage,
    DurableArtifactFamilyId::DerivedRetentionMilestone6LayoutMaterialization,
    DurableArtifactFamilyId::DerivedRetentionMilestone6ScopeSliceMembership,
    DurableArtifactFamilyId::DerivedRetentionMilestone6StructuralBlock,
    DurableArtifactFamilyId::DerivedRetentionMilestone6ChunkMembership,
    DurableArtifactFamilyId::LayoutCompactionUnit,
    DurableArtifactFamilyId::SnapshotArtifact,
    DurableArtifactFamilyId::BranchDeltaArtifact,
];

#[test]
fn every_expected_family_is_declared() {
    let inventory = S8ArtifactFamilyInventory::current();
    for family in EXPECTED_FAMILIES {
        assert!(
            inventory.declaration(*family).is_ok(),
            "missing durable artifact family declaration for {}",
            family.label()
        );
    }
    assert_eq!(inventory.rows().len(), EXPECTED_FAMILIES.len());
}

#[test]
fn every_declaration_carries_complete_phase_one_classification() {
    for row in S8ArtifactFamilyInventory::current().rows() {
        let declaration = row.declaration();
        assert_ne!(
            declaration.owning_boundary().crate_name(),
            "",
            "every declaration must name an owning lower-crate boundary"
        );
        assert!(
            !matches!(
                (declaration.authority(), declaration.access_lane()),
                (
                    ArtifactFamilyAuthorityClass::Terminal
                        | ArtifactFamilyAuthorityClass::CertificationEvidence,
                    ArtifactFamilyAccessLane::HotPath
                )
            ),
            "terminal or courtroom evidence cannot masquerade as hot-path authority"
        );
    }
}

#[test]
fn named_existing_inputs_map_to_real_family_boundaries() {
    let inventory = S8ArtifactFamilyInventory::current();
    assert_eq!(
        inventory
            .declaration(DurableArtifactFamilyId::WalDurableMutationIntent)
            .unwrap()
            .owning_boundary(),
        DurableArtifactOwningBoundary::ForgeStoreWal
    );
    assert_eq!(
        inventory
            .declaration(DurableArtifactFamilyId::CompatibilityCommitEnvelope)
            .unwrap()
            .owning_boundary(),
        DurableArtifactOwningBoundary::ForgeStoreCompatibility
    );
    assert_eq!(
        inventory
            .declaration(DurableArtifactFamilyId::MaintenanceSnapshot)
            .unwrap()
            .owning_boundary(),
        DurableArtifactOwningBoundary::ForgeStoreMaintenance
    );
    assert_eq!(
        inventory
            .declaration(DurableArtifactFamilyId::SupportSchema)
            .unwrap()
            .owning_boundary(),
        DurableArtifactOwningBoundary::ForgeStoreRecoveryPhysics
    );
    assert_eq!(
        inventory
            .declaration(DurableArtifactFamilyId::PlacementAuthoritativeBranchHead)
            .unwrap()
            .owning_boundary(),
        DurableArtifactOwningBoundary::ForgeStoreTiering
    );
    assert_eq!(
        inventory
            .declaration(DurableArtifactFamilyId::PublicationWalIntent)
            .unwrap()
            .owning_boundary(),
        DurableArtifactOwningBoundary::ForgeStoreOperations
    );
    assert_eq!(
        inventory
            .declaration(DurableArtifactFamilyId::DerivedRetentionMilestone6LayoutMaterialization)
            .unwrap()
            .owning_boundary(),
        DurableArtifactOwningBoundary::ForgeStoreRetention
    );
    assert_eq!(
        inventory
            .declaration(DurableArtifactFamilyId::LayoutCompactionUnit)
            .unwrap()
            .owning_boundary(),
        DurableArtifactOwningBoundary::ForgeStoreRetention
    );
}

#[test]
fn non_authority_families_remain_non_authority() {
    let inventory = S8ArtifactFamilyInventory::current();
    for family in [
        DurableArtifactFamilyId::OfflineVerificationRecord,
        DurableArtifactFamilyId::CorruptionRecord,
        DurableArtifactFamilyId::ReclaimReceipt,
        DurableArtifactFamilyId::ExportBundle,
        DurableArtifactFamilyId::CapsuleArtifact,
        DurableArtifactFamilyId::CompatibilitySnapshotRecord,
        DurableArtifactFamilyId::DerivedRetentionMilestone6LayoutMaterialization,
    ] {
        let declaration = inventory.declaration(family).unwrap();
        assert!(
            !matches!(
                declaration.authority(),
                ArtifactFamilyAuthorityClass::Authoritative
            ),
            "{} must not become production authority",
            family.label()
        );
        assert!(
            !declaration.non_authority_projection_classes().is_empty()
                || matches!(
                    declaration.authority(),
                    ArtifactFamilyAuthorityClass::Diagnostic
                ),
            "{} must declare explicit weaker projection posture",
            family.label()
        );
    }
}

#[test]
fn facade_exposes_family_inventory_through_the_public_lane() {
    let declaration = layout_declarations()
        .declaration(DurableArtifactFamilyId::PhysicalRootManifest)
        .expect("public facade should expose the declared family inventory");
    assert_eq!(
        declaration.owning_boundary(),
        DurableArtifactOwningBoundary::ForgeStorePhysicalFormat
    );
}

#[test]
fn every_real_existing_family_variant_is_individually_addressable() {
    let inventory = S8ArtifactFamilyInventory::current();
    for family in [
        DurableArtifactFamilyId::WalDurableMutationIntent,
        DurableArtifactFamilyId::WalHostedRuntimeCommitResult,
        DurableArtifactFamilyId::WalBulkCheckpointPublicationIntent,
        DurableArtifactFamilyId::WalDurablePublicationProgress,
        DurableArtifactFamilyId::WalRecoveryDecision,
        DurableArtifactFamilyId::CompatibilityCommitEnvelope,
        DurableArtifactFamilyId::CompatibilityMilestone13TieringRecord,
        DurableArtifactFamilyId::MaintenanceCapsule,
        DurableArtifactFamilyId::SupportEmbeddedCheckpoint,
        DurableArtifactFamilyId::PlacementMilestone6LayoutFamily,
        DurableArtifactFamilyId::PublicationSnapshotImage,
        DurableArtifactFamilyId::DerivedRetentionMilestone6ChunkMembership,
    ] {
        assert!(
            inventory.declaration(family).is_ok(),
            "{} must have its own canonical declaration row",
            family.label()
        );
    }
}

#[test]
fn existing_family_inputs_lower_directly_to_canonical_declarations() {
    let facade = layout_declarations();
    let layout_compaction_unit = LayoutFamilyCompactionUnit::new(
        "retained_basis",
        LayoutCompactionFamilyKind::LayoutCompactionUnit,
        "artifact",
    );
    assert_eq!(
        facade
            .admit_existing_family(&WalRecordFamily::RecoveryDecision)
            .unwrap()
            .family_id(),
        DurableArtifactFamilyId::WalRecoveryDecision
    );
    assert_eq!(
        facade
            .admit_existing_family(&CompatibilityFamilyKind::Milestone13TieringRecord)
            .unwrap()
            .family_id(),
        DurableArtifactFamilyId::CompatibilityMilestone13TieringRecord
    );
    assert_eq!(
        facade
            .admit_existing_family(&MaintenanceArtifactFamily::Capsule)
            .unwrap()
            .family_id(),
        DurableArtifactFamilyId::MaintenanceCapsule
    );
    assert_eq!(
        facade
            .admit_existing_family(&SupportArtifactFamily::EmbeddedCheckpoint)
            .unwrap()
            .family_id(),
        DurableArtifactFamilyId::SupportEmbeddedCheckpoint
    );
    assert_eq!(
        facade
            .admit_existing_family(&PlacementArtifactFamily::Milestone6LayoutFamily)
            .unwrap()
            .family_id(),
        DurableArtifactFamilyId::PlacementMilestone6LayoutFamily
    );
    assert_eq!(
        facade
            .admit_existing_family(&PublicationFamily::SnapshotImage)
            .unwrap()
            .family_id(),
        DurableArtifactFamilyId::PublicationSnapshotImage
    );
    assert_eq!(
        facade
            .admit_existing_family(&DerivedFamilyRetentionPolicy::Milestone6ChunkMembership)
            .unwrap()
            .family_id(),
        DurableArtifactFamilyId::DerivedRetentionMilestone6ChunkMembership
    );
    assert_eq!(
        facade
            .admit_existing_family(&layout_compaction_unit)
            .unwrap()
            .family_id(),
        DurableArtifactFamilyId::LayoutCompactionUnit
    );
}

#[test]
fn missing_declared_family_is_denied_before_lowering() {
    let rows = S8ArtifactFamilyInventory::current().rows();
    let denial = super::inventory::declaration_in_rows(
        &rows[..rows.len() - 1],
        DurableArtifactFamilyId::BranchDeltaArtifact,
    )
    .unwrap_err();
    assert_eq!(denial, ArtifactFamilyDenial::MissingFamilyDeclaration);
}
