use super::diagnostics::{assert_tiny_rotation_diagnostic, certify_tiny_rotation_diagnostic};
use super::outcome_matrix::assert_mb_m6_outcome_matrix;
use super::platform_storm_subject::{
    certify_platform_storm, certify_platform_storm_with_transform, manual_stage_substitution_error,
    mismatched_operator_stage_link_error,
};
use super::storm_extraction_subject::{
    certify_projected_storm_bridge_authority, certify_projected_storm_context,
    deny_storm_tiny_rotation,
};
use worth_kernel::workload_composition::{
    TransformRecipe, WorkloadCatalog, WorkloadTopologyBreadth,
};
use worth_spatial::facade::coplanar_overlap_storm::CoplanarOverlapStormWorkloadError;
use worth_spatial::facade::planar_overlap::CoplanarOverlapUserOutcomeKind;
use worth_spatial::facade::projected_overlap_faces::{
    CertifiedProjectedOverlapFaceSet, CoplanarOverlapExtractionBundle, ProjectedOverlapFaceSet,
};
use worth_spatial::facade::workload_certification_context::{
    WorkloadMotionAdversary, WorkloadMotionBinding,
};
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
};

#[test]
fn mb_m6_1_coplanar_overlap_storm_end_to_end_receipts() {
    let subject = certify_platform_storm("mb-real-coplanar-storm-identity");
    let counters = subject.storm_receipt.counters();

    assert_eq!(counters.topology_face_count(), 64);
    assert!(counters.topology_entity_count() > 64);
    assert!(counters.topology_relation_count() > counters.topology_face_count());
    assert!(counters.projected_entity_count() > 64);
    assert!(counters.transform_step_count() > 0);
    assert!(counters.transform_cancellation_step_count() > 0);
    assert!(counters.retained_artifact_count() > 0);
    assert!(counters.replay_checkpoint_count() > 0);
    assert_eq!(counters.operator_input_count(), 40);
    assert!(counters.operator_receipt_count() > 0);
    assert_eq!(counters.overlap_extraction_receipt_count(), 32);
    assert!(counters.overlap_candidate_pair_breadth() >= 512);
    assert!(counters.overlap_segment_contacts_certified() >= 192);
    assert!(counters.overlap_shared_intervals() >= 8);
    assert!(counters.overlap_islands() >= 8);
    assert!(counters.overlap_ambiguous_contacts() >= 8);
    assert!(counters.overlap_policy_required_exits() >= 8);
    assert!(!subject.storm_receipt.storm_digest().is_empty());
    assert!(!subject.storm_receipt.workload_identity().is_empty());
    assert_eq!(
        subject.storm_receipt.operator_identity(),
        subject.operator_receipt.operator_digest()
    );
    assert_eq!(
        subject.user_outcome.kind(),
        CoplanarOverlapUserOutcomeKind::ContractsCertified
    );
    assert!(!subject.user_outcome.message().contains('_'));

    let tiny_rotation_source =
        real_storm_tiny_rotation_workload("mb-real-coplanar-storm-tiny-rotation");
    let tiny_rotation_denial = deny_storm_tiny_rotation(
        "mb-real-coplanar-storm-tiny-rotation",
        tiny_rotation_source.projected_workload(),
        tiny_rotation_source.transform_receipts(),
    );
    assert_eq!(
        tiny_rotation_denial.reason(),
        "movement and rotation posture must match before coplanar overlap extraction"
    );
    let diagnostic = certify_tiny_rotation_diagnostic(tiny_rotation_denial.reason());
    assert_tiny_rotation_diagnostic(&diagnostic, tiny_rotation_denial.reason());
}

#[test]
fn mb_m6_1_equivalent_motion_subset_converges_without_full_storm_replay() {
    let movement_stack = certify_platform_storm_with_transform(
        "mb-real-coplanar-subset-movement-stack",
        WorkloadTopologyBreadth::MultiFaceShell { face_count: 8 },
        TransformRecipe::MovementRotationStack,
    );
    let hostile_cancellation = certify_platform_storm_with_transform(
        "mb-real-coplanar-subset-hostile-cancellation",
        WorkloadTopologyBreadth::MultiFaceShell { face_count: 8 },
        TransformRecipe::HostileCancellation,
    );

    assert_eq!(
        movement_stack
            .storm_receipt
            .counters()
            .topology_face_count(),
        8
    );
    assert_eq!(
        movement_stack
            .storm_receipt
            .counters()
            .topology_face_count(),
        hostile_cancellation
            .storm_receipt
            .counters()
            .topology_face_count()
    );
    assert_eq!(
        movement_stack
            .storm_receipt
            .counters()
            .topology_relation_count(),
        hostile_cancellation
            .storm_receipt
            .counters()
            .topology_relation_count()
    );
    assert_eq!(
        movement_stack
            .storm_receipt
            .counters()
            .projected_entity_count(),
        hostile_cancellation
            .storm_receipt
            .counters()
            .projected_entity_count()
    );
    assert_ne!(
        movement_stack
            .storm_receipt
            .counters()
            .transform_cancellation_step_count(),
        hostile_cancellation
            .storm_receipt
            .counters()
            .transform_cancellation_step_count(),
        "movement and hostile cancellation branches must prove different replay breadth"
    );
    assert_eq!(
        movement_stack
            .storm_receipt
            .counters()
            .operator_input_count(),
        hostile_cancellation
            .storm_receipt
            .counters()
            .operator_input_count(),
        "operator consumption must converge after replay despite transform breadth differences"
    );
    assert_ne!(
        movement_stack.storm_receipt.workload_identity(),
        hostile_cancellation.storm_receipt.workload_identity(),
        "distinct Query declarations must remain visible even when storm breadth converges"
    );
}

#[test]
fn mb_m6_1_user_outcome_matrix_branches_every_stop() {
    let tiny_rotation_source =
        real_storm_tiny_rotation_workload("mb-real-coplanar-matrix-tiny-rotation");
    let tiny_rotation_denial = deny_storm_tiny_rotation(
        "mb-real-coplanar-matrix-tiny-rotation",
        tiny_rotation_source.projected_workload(),
        tiny_rotation_source.transform_receipts(),
    );
    assert_mb_m6_outcome_matrix(&tiny_rotation_denial);
}

fn real_storm_tiny_rotation_workload(
    world: &'static str,
) -> worth_kernel::workload_composition::BuiltWorkloadCatalogRecipe {
    WorkloadCatalog::coplanar_overlap_storm()
        .with_topology_breadth(WorkloadTopologyBreadth::MultiFaceShell { face_count: 8 })
        .declared(format!(
            "MB-M6-1 real tiny-rotation candidate source {world}"
        ))
        .build()
        .expect("real storm workload should build before tiny-rotation denial")
}

#[test]
fn mb_m6_1_certified_projected_face_bridge_is_extraction_authority() {
    let built = real_storm_tiny_rotation_workload("mb-real-coplanar-certified-bridge");
    let authority = certify_projected_storm_bridge_authority(
        "mb-real-coplanar-certified-bridge",
        built.projected_workload(),
        built.transform_receipts(),
    );

    assert_eq!(authority.certified_face_count(), 8);
    assert_eq!(authority.candidate_pair_count(), 4);
    assert_eq!(
        authority.extraction_receipt_count(),
        authority.candidate_pair_count()
    );
    assert_eq!(
        authority.context_identity(),
        authority.extraction_bundle().context_identity()
    );
    assert_eq!(
        authority.projection_stage_identity(),
        authority.extraction_bundle().projection_stage_identity()
    );
    assert_eq!(
        authority.movement_rotation_posture_identity(),
        authority
            .extraction_bundle()
            .movement_rotation_posture_identity()
    );
    assert_eq!(
        authority.certified_face_set_digest(),
        authority.certified_faces().certified_face_set_digest()
    );
    assert!(!authority.certified_face_set_digest().is_empty());
    assert!(!authority
        .extraction_bundle()
        .extraction_bundle_digest()
        .is_empty());
    assert!(!authority.authority_digest().is_empty());
    let pair = authority
        .candidate_pairs()
        .first_pair()
        .expect("certified storm bridge should expose candidate pairs");
    assert_eq!(
        pair.projection_stage_identity(),
        authority.projection_stage_identity()
    );
    assert_eq!(
        pair.first_face().projection_stage_identity(),
        authority.projection_stage_identity()
    );
    assert_eq!(
        pair.first_face().movement_rotation_posture_identity(),
        built
            .transform_receipts()
            .transform_posture_receipt()
            .posture_identity()
    );
    assert!(!pair.first_face().winding_fact_digest().is_empty());
    assert!(!pair.first_face().signed_area_fact_digest().is_empty());
    assert!(!pair.first_face().local_frame_fact_digest().is_empty());
    assert!(!pair.first_face().precision_fact_digest().is_empty());
    assert!(!pair.pair_identity().is_empty());
}

#[test]
fn mb_m6_1_certified_bridge_rejects_cross_context_face_authority() {
    let projected_source = real_storm_tiny_rotation_workload("mb-real-coplanar-projected-source");
    let context_source = real_storm_tiny_rotation_workload("mb-real-coplanar-context-source");
    let projected_faces =
        ProjectedOverlapFaceSet::from_projected_workload(projected_source.projected_workload())
            .expect("projected source should expose projected overlap faces");
    let foreign_context = certify_projected_storm_context(
        "mb-real-coplanar-context-source",
        context_source.projected_workload(),
        context_source.transform_receipts(),
    );

    let denial =
        CertifiedProjectedOverlapFaceSet::from_projected_faces(projected_faces, &foreign_context)
            .expect_err("projected faces must not certify under a foreign context");

    assert_eq!(
        denial.human_reason(),
        "certified projected overlap faces require the projected workload and certification context to share the same projection stage"
    );
    assert!(!denial.human_reason().contains('_'));
}

#[test]
fn mb_m6_1_certified_bridge_rejects_cross_context_candidate_pairs_before_extraction() {
    let pair_source = real_storm_tiny_rotation_workload("mb-real-coplanar-pair-source");
    let context_source = real_storm_tiny_rotation_workload("mb-real-coplanar-bundle-context");
    let pair_authority = certify_projected_storm_bridge_authority(
        "mb-real-coplanar-pair-source",
        pair_source.projected_workload(),
        pair_source.transform_receipts(),
    );
    let foreign_context = certify_projected_storm_context(
        "mb-real-coplanar-bundle-context",
        context_source.projected_workload(),
        context_source.transform_receipts(),
    );

    let denial = CoplanarOverlapExtractionBundle::from_context_candidate_pairs(
        pair_authority.candidate_pairs(),
        &foreign_context,
    )
    .expect_err("candidate pairs must not extract under a foreign context");

    assert_eq!(
        denial.human_reason(),
        "certified projected overlap extraction requires candidate pairs from the same projection stage as the certification context"
    );
    assert!(!denial.human_reason().contains('_'));
}

#[test]
fn mb_m6_1_certified_bridge_rejects_cross_motion_candidate_pairs_before_extraction() {
    let built = real_storm_tiny_rotation_workload("mb-real-coplanar-motion-guard");
    let context = certify_projected_storm_context(
        "mb-real-coplanar-motion-guard",
        built.projected_workload(),
        built.transform_receipts(),
    );
    let projected_faces = ProjectedOverlapFaceSet::from_context(&context)
        .expect("projected storm context should expose projected overlap faces");
    let adversarial_context = context
        .with_motion_binding(WorkloadMotionBinding::adversarial_for_context(
            &context,
            WorkloadMotionAdversary::TinyRotationExitsCoplanarClass,
        ))
        .expect("motion adversary should preserve projection while changing posture");
    let adversarial_faces = CertifiedProjectedOverlapFaceSet::from_projected_faces(
        projected_faces,
        &adversarial_context,
    )
    .expect("same-projection adversarial context should certify faces");

    let denial = CoplanarOverlapExtractionBundle::from_context_candidate_pairs(
        adversarial_faces.candidate_pairs(),
        &context,
    )
    .expect_err("candidate pairs with a different motion posture must not extract");

    assert_eq!(
        denial.human_reason(),
        "certified projected overlap extraction requires candidate pairs from the same movement and rotation posture as the certification context"
    );
    assert!(!denial.human_reason().contains('_'));
}

#[test]
fn mb_m6_1_fixture_arithmetic_cannot_satisfy_storm_truth() {
    for stage in [
        WorkloadEvidenceStage::Topology,
        WorkloadEvidenceStage::GeometryBinding,
        WorkloadEvidenceStage::SurfaceSupport,
        WorkloadEvidenceStage::Projection,
        WorkloadEvidenceStage::Transform,
        WorkloadEvidenceStage::RetainedReplay,
        WorkloadEvidenceStage::Diagnostics,
        WorkloadEvidenceStage::Response,
    ] {
        let error = manual_stage_substitution_error(stage)
            .expect_err("hand-filled authority row must not complete storm evidence");

        assert_eq!(
            error,
            WorkloadEvidenceLedgerError::ManualAuthorityStage(stage)
        );
        assert_eq!(
            error.human_reason(),
            format!(
                "workload evidence ledger has hand-filled {} instead of a source receipt",
                stage.human_name()
            )
        );
    }
}

#[test]
fn mb_m6_1_operator_receipt_must_match_workload_ledger() {
    let error = mismatched_operator_stage_link_error()
        .expect_err("operator receipt from a different workload must not certify this storm");

    assert_eq!(
        error,
        CoplanarOverlapStormWorkloadError::MismatchedOperatorStageLink(
            WorkloadEvidenceStage::Projection
        )
    );
    assert_eq!(
        error.human_reason(),
        "coplanar overlap storm operator receipt must consume the same projection evidence as the workload ledger"
    );
}

#[test]
fn mb_m6_1_unpaired_projected_face_cannot_be_silently_dropped() {
    let built = worth_kernel::workload_composition::WorkloadCatalog::coplanar_overlap_storm()
        .with_topology_breadth(WorkloadTopologyBreadth::MultiFaceShell { face_count: 9 })
        .declared("mb-real-coplanar-storm-unpaired-projected-face")
        .build()
        .expect("odd-width storm workload should build before overlap pairing is denied");

    let denial = ProjectedOverlapFaceSet::from_projected_workload(built.projected_workload())
        .expect_err("unpaired projected face must deny instead of being dropped");

    assert_eq!(
        denial.human_reason(),
        "projected overlap candidate policy requires an even number of boundary-backed faces; one projected face has no adjacent overlap partner"
    );
    assert!(!denial.human_reason().contains('_'));
}
