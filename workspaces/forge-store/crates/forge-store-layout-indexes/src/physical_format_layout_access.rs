use crate::phase23_rules::{
    AdmittedBranchDeltaLayoutRule, AdmittedContinuationLayoutRule, AdmittedSnapshotLayoutRule,
    AdmittedStableBasisLayoutRule,
};
use crate::phase24_rules::{
    AdmittedBlobObjectLayoutRule, AdmittedChunkTreeLayoutRule, AdmittedStreamingLayoutRule,
};
use crate::phase25_rules::{
    AdmittedCompactionLayoutRule, AdmittedDedupeLayoutRule, AdmittedQuarantineLayoutRule,
    AdmittedReachabilityLayoutRule, AdmittedReclaimLayoutRule, AdmittedRetentionLayoutRule,
};
use crate::{
    access_shapes,
    artifact_family::ArtifactFamilyDenial,
    materialization::{
        S8LayoutCoverageWitness, S8LayoutMaterializationState, S8PhysicalCoverageBasis,
    },
    PhysicalArtifactFamilyDeclaration, S8AccessAuthorityPosture, S8AccessShape,
    S8AccessShapeContract, S8AccessShapeUnsupportedDenial,
};
use forge_store_contracts::DurableArtifactFamilyId;
use forge_store_physical_format::PhysicalEpoch;
use forge_store_physical_isolation::AdmittedPlacementLayoutRule;
use forge_store_recovery_physics::AdmittedRecoveryManifestLayoutRule;
use forge_store_recovery_physics::{
    AdmittedBoundedWalTailLayoutRule, AdmittedCrashBoundaryLayoutRule,
    AdmittedRecoverySourceLayoutRule, AdmittedReplayIndexLayoutRule,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Phase19LayoutRuleDenial {
    Family(ArtifactFamilyDenial),
    AccessShape(S8AccessShapeUnsupportedDenial),
    WrongFamily(DurableArtifactFamilyId),
    WrongAuthorityPosture(S8AccessAuthorityPosture),
    WrongShape(S8AccessShape),
}

pub fn phase20_placement_rule() -> Result<AdmittedPlacementLayoutRule, Phase19LayoutRuleDenial> {
    validate_bounded_scan_family(DurableArtifactFamilyId::PlacementStableBasis)?;
    Ok(AdmittedPlacementLayoutRule::phase20())
}
pub fn phase21_recovery_manifest_rule(
) -> Result<AdmittedRecoveryManifestLayoutRule, Phase19LayoutRuleDenial> {
    validate_maintenance_bounded_scan_family(DurableArtifactFamilyId::WalRecoveryDecision)?;
    Ok(AdmittedRecoveryManifestLayoutRule::phase21())
}
pub fn phase22_replay_index_rule() -> Result<AdmittedReplayIndexLayoutRule, Phase19LayoutRuleDenial>
{
    validate_maintenance_bounded_scan_family(DurableArtifactFamilyId::WalRecoveryDecision)?;
    Ok(AdmittedReplayIndexLayoutRule::phase22())
}
pub fn phase22_recovery_source_rule(
) -> Result<AdmittedRecoverySourceLayoutRule, Phase19LayoutRuleDenial> {
    validate_maintenance_bounded_scan_family(DurableArtifactFamilyId::WalRecoveryDecision)?;
    Ok(AdmittedRecoverySourceLayoutRule::phase22())
}
pub fn phase22_crash_boundary_rule(
) -> Result<AdmittedCrashBoundaryLayoutRule, Phase19LayoutRuleDenial> {
    validate_maintenance_bounded_scan_family(DurableArtifactFamilyId::WalRecoveryDecision)?;
    Ok(AdmittedCrashBoundaryLayoutRule::phase22())
}
pub fn phase22_bounded_wal_tail_rule(
) -> Result<AdmittedBoundedWalTailLayoutRule, Phase19LayoutRuleDenial> {
    validate_bounded_scan_family(DurableArtifactFamilyId::WalHostedRuntimeCommitResult)?;
    Ok(AdmittedBoundedWalTailLayoutRule::phase22())
}
pub fn phase23_snapshot_rule() -> Result<AdmittedSnapshotLayoutRule, Phase19LayoutRuleDenial> {
    validate_maintenance_bounded_scan_family(DurableArtifactFamilyId::PublicationSnapshotImage)?;
    Ok(AdmittedSnapshotLayoutRule::internal_phase23())
}
pub fn phase23_branch_delta_rule() -> Result<AdmittedBranchDeltaLayoutRule, Phase19LayoutRuleDenial>
{
    validate_maintenance_bounded_scan_family(DurableArtifactFamilyId::BranchDeltaArtifact)?;
    Ok(AdmittedBranchDeltaLayoutRule::internal_phase23())
}
pub fn phase23_stable_basis_rule() -> Result<AdmittedStableBasisLayoutRule, Phase19LayoutRuleDenial>
{
    validate_bounded_scan_family(DurableArtifactFamilyId::PlacementStableBasis)?;
    Ok(AdmittedStableBasisLayoutRule::internal_phase23())
}
pub fn phase23_continuation_support_rule(
) -> Result<AdmittedContinuationLayoutRule, Phase19LayoutRuleDenial> {
    validate_streaming_continuation_family(DurableArtifactFamilyId::SupportCursor)?;
    Ok(AdmittedContinuationLayoutRule::internal_phase23())
}
pub fn phase24_blob_object_rule() -> Result<AdmittedBlobObjectLayoutRule, Phase19LayoutRuleDenial> {
    validate_bounded_scan_family(DurableArtifactFamilyId::BlobManifest)?;
    Ok(AdmittedBlobObjectLayoutRule::phase24())
}
pub fn phase24_chunk_tree_rule() -> Result<AdmittedChunkTreeLayoutRule, Phase19LayoutRuleDenial> {
    validate_chunk_tree_walk_family(DurableArtifactFamilyId::BlobChunk)?;
    Ok(AdmittedChunkTreeLayoutRule::phase24())
}
pub fn phase24_streaming_rule() -> Result<AdmittedStreamingLayoutRule, Phase19LayoutRuleDenial> {
    validate_streaming_family(DurableArtifactFamilyId::BlobStream)?;
    Ok(AdmittedStreamingLayoutRule::phase24())
}
pub fn phase25_dedupe_rule() -> Result<AdmittedDedupeLayoutRule, Phase19LayoutRuleDenial> {
    validate_exact_point_family(DurableArtifactFamilyId::DedupeIndex)?;
    Ok(AdmittedDedupeLayoutRule::phase25())
}
pub fn phase25_reachability_rule() -> Result<AdmittedReachabilityLayoutRule, Phase19LayoutRuleDenial>
{
    validate_bounded_scan_family(DurableArtifactFamilyId::ReachabilityEdge)?;
    Ok(AdmittedReachabilityLayoutRule::phase25())
}
pub fn phase25_retention_rule() -> Result<AdmittedRetentionLayoutRule, Phase19LayoutRuleDenial> {
    validate_maintenance_bounded_scan_family(DurableArtifactFamilyId::RetentionHold)?;
    Ok(AdmittedRetentionLayoutRule::phase25())
}
pub fn phase25_reclaim_rule() -> Result<AdmittedReclaimLayoutRule, Phase19LayoutRuleDenial> {
    validate_maintenance_bounded_scan_family(DurableArtifactFamilyId::ReclaimReceipt)?;
    Ok(AdmittedReclaimLayoutRule::phase25())
}
pub fn phase25_compaction_rule() -> Result<AdmittedCompactionLayoutRule, Phase19LayoutRuleDenial> {
    validate_compaction_read_family(DurableArtifactFamilyId::MaintenanceCompaction)?;
    Ok(AdmittedCompactionLayoutRule::phase25())
}
pub fn phase25_quarantine_rule() -> Result<AdmittedQuarantineLayoutRule, Phase19LayoutRuleDenial> {
    validate_quarantine_read_family(DurableArtifactFamilyId::QuarantineRecord)?;
    Ok(AdmittedQuarantineLayoutRule::phase25())
}

fn validate_exact_point_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = crate::layout_declarations()
        .declaration(family_id)
        .map_err(Phase19LayoutRuleDenial::Family)?;
    if declaration.family_id() != family_id {
        return Err(Phase19LayoutRuleDenial::WrongFamily(
            declaration.family_id(),
        ));
    }
    let point_lookup = exact_point_lookup_shape(declaration)?;
    if point_lookup.authority_posture() != S8AccessAuthorityPosture::ExactMaterialized {
        return Err(Phase19LayoutRuleDenial::WrongAuthorityPosture(
            point_lookup.authority_posture(),
        ));
    }
    if point_lookup.shape() != S8AccessShape::PointLookup {
        return Err(Phase19LayoutRuleDenial::WrongShape(point_lookup.shape()));
    }
    Ok(())
}

fn validate_bounded_scan_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = crate::layout_declarations()
        .declaration(family_id)
        .map_err(Phase19LayoutRuleDenial::Family)?;
    if declaration.family_id() != family_id {
        return Err(Phase19LayoutRuleDenial::WrongFamily(
            declaration.family_id(),
        ));
    }
    let coverage = exact_coverage(declaration);
    let bounded = access_shapes()
        .bounded_scan(
            coverage,
            crate::S8AccessLaneClassification::Foreground,
            crate::S8BoundedScanBasis::LocalityBoundedTraversal,
        )
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if bounded.shape() != S8AccessShape::BoundedScan {
        return Err(Phase19LayoutRuleDenial::WrongShape(bounded.shape()));
    }
    Ok(())
}

fn validate_maintenance_bounded_scan_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = crate::layout_declarations()
        .declaration(family_id)
        .map_err(Phase19LayoutRuleDenial::Family)?;
    if declaration.family_id() != family_id {
        return Err(Phase19LayoutRuleDenial::WrongFamily(
            declaration.family_id(),
        ));
    }
    let bounded = access_shapes()
        .bounded_scan(
            exact_coverage(declaration),
            crate::S8AccessLaneClassification::Maintenance,
            crate::S8BoundedScanBasis::LocalityBoundedTraversal,
        )
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if bounded.shape() != S8AccessShape::BoundedScan {
        return Err(Phase19LayoutRuleDenial::WrongShape(bounded.shape()));
    }
    Ok(())
}

fn validate_append_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = crate::layout_declarations()
        .declaration(family_id)
        .map_err(Phase19LayoutRuleDenial::Family)?;
    if declaration.family_id() != family_id {
        return Err(Phase19LayoutRuleDenial::WrongFamily(
            declaration.family_id(),
        ));
    }
    let append = access_shapes()
        .append(crate::S8PhysicalMutationShape::LogStructuredAppend)
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if append.shape() != S8AccessShape::Append {
        return Err(Phase19LayoutRuleDenial::WrongShape(append.shape()));
    }
    Ok(())
}

fn validate_compaction_read_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = crate::layout_declarations()
        .declaration(family_id)
        .map_err(Phase19LayoutRuleDenial::Family)?;
    if declaration.family_id() != family_id {
        return Err(Phase19LayoutRuleDenial::WrongFamily(
            declaration.family_id(),
        ));
    }
    let compaction = access_shapes()
        .compaction_read(crate::S8PhysicalMutationShape::CompactionRewrite)
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if compaction.shape() != S8AccessShape::CompactionRead {
        return Err(Phase19LayoutRuleDenial::WrongShape(compaction.shape()));
    }
    Ok(())
}

fn validate_quarantine_read_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = crate::layout_declarations()
        .declaration(family_id)
        .map_err(Phase19LayoutRuleDenial::Family)?;
    if declaration.family_id() != family_id {
        return Err(Phase19LayoutRuleDenial::WrongFamily(
            declaration.family_id(),
        ));
    }
    let quarantine = access_shapes()
        .quarantine_read(
            exact_coverage(declaration),
            crate::S8AccessLaneClassification::Verifier,
        )
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if quarantine.shape() != S8AccessShape::QuarantineRead {
        return Err(Phase19LayoutRuleDenial::WrongShape(quarantine.shape()));
    }
    Ok(())
}

fn validate_streaming_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = crate::layout_declarations()
        .declaration(family_id)
        .map_err(Phase19LayoutRuleDenial::Family)?;
    if declaration.family_id() != family_id {
        return Err(Phase19LayoutRuleDenial::WrongFamily(
            declaration.family_id(),
        ));
    }
    let streaming = access_shapes()
        .streaming_read(
            exact_coverage(declaration),
            crate::S8AccessLaneClassification::Foreground,
        )
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if streaming.shape() != S8AccessShape::StreamingRead {
        return Err(Phase19LayoutRuleDenial::WrongShape(streaming.shape()));
    }
    Ok(())
}

fn validate_streaming_continuation_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = crate::layout_declarations()
        .declaration(family_id)
        .map_err(Phase19LayoutRuleDenial::Family)?;
    if declaration.family_id() != family_id {
        return Err(Phase19LayoutRuleDenial::WrongFamily(
            declaration.family_id(),
        ));
    }
    let continuation = access_shapes()
        .streaming_continuation_read(
            exact_coverage(declaration),
            crate::S8AccessLaneClassification::Foreground,
            crate::S8StreamingContinuationBasis::ResumeCursorContinuation,
        )
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if continuation.shape() != S8AccessShape::StreamingContinuationRead {
        return Err(Phase19LayoutRuleDenial::WrongShape(continuation.shape()));
    }
    Ok(())
}

fn validate_chunk_tree_walk_family(
    family_id: DurableArtifactFamilyId,
) -> Result<(), Phase19LayoutRuleDenial> {
    let declaration = crate::layout_declarations()
        .declaration(family_id)
        .map_err(Phase19LayoutRuleDenial::Family)?;
    if declaration.family_id() != family_id {
        return Err(Phase19LayoutRuleDenial::WrongFamily(
            declaration.family_id(),
        ));
    }
    let walk = access_shapes()
        .chunk_tree_walk(
            exact_coverage(declaration),
            crate::S8AccessLaneClassification::Foreground,
        )
        .map_err(Phase19LayoutRuleDenial::AccessShape)?;
    if walk.shape() != S8AccessShape::ChunkTreeWalk {
        return Err(Phase19LayoutRuleDenial::WrongShape(walk.shape()));
    }
    Ok(())
}

fn exact_point_lookup_shape(
    declaration: &'static PhysicalArtifactFamilyDeclaration,
) -> Result<S8AccessShapeContract, Phase19LayoutRuleDenial> {
    let coverage = exact_coverage(declaration);
    access_shapes()
        .point_lookup(coverage)
        .map_err(Phase19LayoutRuleDenial::AccessShape)
}

fn exact_coverage(
    declaration: &'static PhysicalArtifactFamilyDeclaration,
) -> S8LayoutCoverageWitness {
    let watermark = S8PhysicalCoverageBasis::root_epoch(
        PhysicalEpoch::from_raw(1).expect("phase-19 point lookup watermark must be non-zero"),
    )
    .watermark();
    S8LayoutCoverageWitness::exact_through(
        S8LayoutMaterializationState::exact_through_physical_basis(declaration.family()),
        watermark,
    )
    .expect("phase-20 exact physical basis coverage must stay well-formed")
}
