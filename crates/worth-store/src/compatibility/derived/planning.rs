use super::super::admission::{
    CompatibilityAdmissionCounters, CompatibilityRejection, CompatibilityRejectionKind,
    CompatibilityRelation, ReadCompatibilityReceipt,
};
use super::super::catalog::DerivedFamilyDeclaration;
use super::super::decoding::QuarantinedDecodedArtifact;
use super::super::manifests::ArtifactCompatibilityWindow;
use super::declarations::{
    DerivedBasisCompatibilityPosture, DerivedCompatibilityLaneDeclaration,
    DerivedCompatibilityLaneKind, DerivedInvalidationReason,
};
use super::lane_plans::{
    BulkResumeCompatibilityPlan, BulkResumeCompatibilityRejection, BulkResumeInterpretation,
    DerivedBasisCompatibilityInput, DerivedLaneCompatibilityPlan, TierManifestCompatibilityPlan,
    TierManifestCompatibilityRejection,
};
use super::maintenance_admission::plan_invalidation;
use super::reuse::DerivedBasisCompatibilityPlan;

pub(crate) fn plan_derived_basis_compatibility(
    counters: &mut CompatibilityAdmissionCounters,
    derived_family: &DerivedFamilyDeclaration,
    artifact: &QuarantinedDecodedArtifact,
    read_receipt: &ReadCompatibilityReceipt,
    required_window: ArtifactCompatibilityWindow,
) -> Result<DerivedBasisCompatibilityPlan, CompatibilityRejection> {
    if artifact.family_id() != derived_family.declaration().family_id()
        || artifact.family_id() != read_receipt.receipt().family_id()
        || artifact.manifest_digest() != read_receipt.receipt().manifest_digest()
        || artifact.semantic_version() != read_receipt.receipt().observed_semantic_version()
    {
        counters.record_derived_reuse_incompatible();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedBasisIncompatible,
            artifact.family_id().clone(),
            "derived basis declaration, receipt, and artifact do not describe the same artifact",
        ));
    }

    if !required_window.contains_format(artifact.format_version()) {
        return Ok(plan_invalidation(
            counters,
            artifact,
            required_window,
            DerivedInvalidationReason::FormatWindowMismatch,
            "derived artifact format version is outside the required rebuild window",
        ));
    }

    if !required_window.contains_semantic(artifact.semantic_version()) {
        return Ok(plan_invalidation(
            counters,
            artifact,
            required_window,
            DerivedInvalidationReason::SemanticWindowMismatch,
            "derived artifact semantic version is outside the required rebuild window",
        ));
    }

    if read_receipt.receipt().relation() != CompatibilityRelation::Native {
        return Ok(plan_invalidation(
            counters,
            artifact,
            required_window,
            DerivedInvalidationReason::NonNativeReadRelation,
            "derived artifact was admitted through a non-native read relation",
        ));
    }

    Ok(DerivedBasisCompatibilityPlan::reusable(
        artifact.family_id().clone(),
    ))
}

pub(crate) fn plan_derived_lane_compatibility(
    counters: &mut CompatibilityAdmissionCounters,
    input: &DerivedBasisCompatibilityInput,
    artifact: &QuarantinedDecodedArtifact,
    read_receipt: &ReadCompatibilityReceipt,
) -> Result<DerivedLaneCompatibilityPlan, CompatibilityRejection> {
    counters.record_derived_lane_plan();
    let lane = input.lane_declaration();
    if lane.family_id() != input.derived_family.declaration().family_id()
        || lane.family_id() != artifact.family_id()
    {
        counters.record_derived_lane_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::DerivedLaneRejected,
            artifact.family_id().clone(),
            "derived compatibility lane, declaration, and artifact family do not match",
        ));
    }

    match lane.lane_kind() {
        DerivedCompatibilityLaneKind::BulkResumeReuse => {
            return plan_bulk_resume_compatibility(counters, lane, artifact, read_receipt).map(
                |plan| DerivedLaneCompatibilityPlan::from_bulk_resume(plan, lane.lane_kind()),
            );
        }
        DerivedCompatibilityLaneKind::TierManifestSupport => {
            return plan_tier_manifest_compatibility(counters, lane, artifact, read_receipt).map(
                |plan| DerivedLaneCompatibilityPlan::from_tier_manifest(plan, lane.lane_kind()),
            );
        }
        DerivedCompatibilityLaneKind::LayoutBlockChunkReuse
            if !input
                .required_window
                .contains_format(artifact.format_version())
                || !input
                    .required_window
                    .contains_semantic(artifact.semantic_version()) =>
        {
            counters.record_derived_layout_basis_rejection();
            return Err(CompatibilityRejection::new(
                CompatibilityRejectionKind::DerivedBasisIncompatible,
                artifact.family_id().clone(),
                "layout/block/chunk compatibility lane rejects basis drift",
            ));
        }
        _ => {}
    }

    let plan = plan_derived_basis_compatibility(
        counters,
        &input.derived_family,
        artifact,
        read_receipt,
        input.required_window.clone(),
    )?;
    match plan.posture() {
        DerivedBasisCompatibilityPosture::ReuseStillValid => {
            counters.record_derived_lane_reuse();
            match lane.lane_kind() {
                DerivedCompatibilityLaneKind::SnapshotReuse => {
                    counters.record_derived_snapshot_reuse();
                }
                DerivedCompatibilityLaneKind::BranchDeltaReuse => {
                    counters.record_derived_delta_reuse();
                }
                _ => {}
            }
        }
        DerivedBasisCompatibilityPosture::InvalidateAndRebuild => {
            counters.record_derived_lane_invalidation();
            if lane.lane_kind() == DerivedCompatibilityLaneKind::MaintenanceSummarySupport {
                counters.record_derived_maintenance_summary_rebuild();
            }
        }
    }
    Ok(DerivedLaneCompatibilityPlan::from_basis(
        lane.lane_kind(),
        plan,
    ))
}

pub(crate) fn plan_bulk_resume_compatibility(
    counters: &mut CompatibilityAdmissionCounters,
    lane: &DerivedCompatibilityLaneDeclaration,
    artifact: &QuarantinedDecodedArtifact,
    read_receipt: &ReadCompatibilityReceipt,
) -> Result<BulkResumeCompatibilityPlan, CompatibilityRejection> {
    if lane.lane_kind() != DerivedCompatibilityLaneKind::BulkResumeReuse
        || lane.family_id() != artifact.family_id()
    {
        counters.record_derived_bulk_resume_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::BulkResumeCompatibilityRejected,
            artifact.family_id().clone(),
            "bulk resume compatibility requires the bulk resume lane",
        ));
    }
    if read_receipt.receipt().relation() != CompatibilityRelation::Native {
        counters.record_derived_bulk_resume_rejection();
        let _rejection = BulkResumeCompatibilityRejection::new(
            artifact.family_id().clone(),
            BulkResumeInterpretation::ChangedInterpretationRejected,
        );
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::BulkResumeCompatibilityRejected,
            artifact.family_id().clone(),
            "bulk resume support cannot resume under changed semantic interpretation",
        ));
    }
    counters.record_derived_lane_reuse();
    Ok(BulkResumeCompatibilityPlan::new(
        artifact.family_id().clone(),
        BulkResumeInterpretation::NativeResume,
    ))
}

pub(crate) fn plan_tier_manifest_compatibility(
    counters: &mut CompatibilityAdmissionCounters,
    lane: &DerivedCompatibilityLaneDeclaration,
    artifact: &QuarantinedDecodedArtifact,
    read_receipt: &ReadCompatibilityReceipt,
) -> Result<TierManifestCompatibilityPlan, CompatibilityRejection> {
    if lane.lane_kind() != DerivedCompatibilityLaneKind::TierManifestSupport
        || lane.family_id() != artifact.family_id()
    {
        counters.record_tier_manifest_rejection();
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::TierManifestCompatibilityRejected,
            artifact.family_id().clone(),
            "tier manifest compatibility requires the tier manifest lane",
        ));
    }
    if read_receipt.receipt().relation() != CompatibilityRelation::Native
        || !lane
            .required_window()
            .contains_format(artifact.format_version())
        || !lane
            .required_window()
            .contains_semantic(artifact.semantic_version())
    {
        counters.record_tier_manifest_rejection();
        let _rejection = TierManifestCompatibilityRejection::new(
            artifact.family_id().clone(),
            "tier manifest semantic drift rejected",
        );
        return Err(CompatibilityRejection::new(
            CompatibilityRejectionKind::TierManifestCompatibilityRejected,
            artifact.family_id().clone(),
            "tier manifest compatibility preserves placement non-authority by rejecting drift",
        ));
    }
    counters.record_tier_non_authority_preserved();
    Ok(TierManifestCompatibilityPlan::new(
        artifact.family_id().clone(),
    ))
}
