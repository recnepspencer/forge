use super::super::{
    prepare_primitive_construction_compound_adversarial_siege_report,
    prepare_primitive_construction_compound_grazing_boundary_report,
    prepare_primitive_construction_compound_motion_parity_report,
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundTopologyClass,
    PrimitiveConstructionCompoundWorkloadFamily,
};
use super::support::{
    compound_workspace, expected_grazing_scenario_ids, expected_motion_scenario_ids,
    parity_truth_from_siege, sorted_ids,
};
use worth_geom::facade::{
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

#[test]
fn compound_adversarial_siege_report_carries_shell_wire_and_motion_truth() {
    let mut workspace = compound_workspace("worth-kernel.compound-adversarial-siege");
    let report = prepare_primitive_construction_compound_adversarial_siege_report(&mut workspace)
        .expect("compound report");
    let shell = report
        .row_for("sheet_patch_reorient_grazing_workplane")
        .expect("shell row");
    let wire = report
        .row_for("wire_open_endpoint_graze")
        .expect("wire row");
    let mixed = report
        .row_for("mixed_topology_class_batch")
        .expect("mixed row");
    let pyramid = report
        .row_for("pyramid_threshold_admitted_exact_support")
        .expect("pyramid row");
    let exhausted_pyramid = report
        .row_for("pyramid_semantic_exhaustion")
        .expect("exhausted pyramid row");
    let simplex = report
        .row_for("simplex_world_collapsed_admitted_local_or_exact")
        .expect("simplex row");
    let exhausted_simplex = report
        .row_for("simplex_world_collapsed_explicit_exhaustion")
        .expect("exhausted simplex row");

    assert!(report.authoring_order_parity_verified());
    assert_eq!(report.authoring_order_rows().len(), 5);
    assert_eq!(
        shell.workload_family(),
        PrimitiveConstructionCompoundWorkloadFamily::SheetPatch
    );
    assert_eq!(
        shell.topology_class(),
        PrimitiveConstructionCompoundTopologyClass::OpenShell
    );
    assert_eq!(
        shell.row_class(),
        PrimitiveConstructionCompoundRowClass::MotionHostileReorientation
    );
    assert_eq!(
        shell.motion_kind(),
        Some(PrimitiveConstructionCompoundMotionKind::Reorient)
    );
    assert_eq!(
        shell.grazing_kind(),
        Some(PrimitiveConstructionCompoundGrazingKind::NearFrameNormalAlignment)
    );
    assert_eq!(
        shell.inspection_digest(),
        report
            .row_for("sheet_patch_reorient_grazing_workplane")
            .expect("shell row")
            .inspection_digest()
    );
    assert_ne!(
        shell.inspection_digest().expect("shell inspection digest"),
        shell.row_digest()
    );
    assert_eq!(
        wire.topology_class(),
        PrimitiveConstructionCompoundTopologyClass::OpenWire
    );
    assert_eq!(
        wire.motion_kind(),
        Some(PrimitiveConstructionCompoundMotionKind::Offset)
    );
    assert_eq!(
        wire.grazing_kind(),
        Some(PrimitiveConstructionCompoundGrazingKind::NearReferenceAnchorDistance)
    );
    assert_eq!(
        mixed.workload_family(),
        PrimitiveConstructionCompoundWorkloadFamily::MixedTopologyClassBatch
    );
    assert_eq!(
        mixed.topology_class(),
        PrimitiveConstructionCompoundTopologyClass::MixedBatch
    );
    assert_eq!(
        mixed.row_class(),
        PrimitiveConstructionCompoundRowClass::MixedTopologyBatch
    );
    assert_eq!(
        simplex.row_class(),
        PrimitiveConstructionCompoundRowClass::EscalatedStableExactSupport
    );
    assert_eq!(
        simplex.realization_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(exhausted_simplex.realization_strategy(), None);
    assert_eq!(
        exhausted_simplex.row_class(),
        PrimitiveConstructionCompoundRowClass::StructuredRealizationExhaustion
    );
    assert_eq!(
        exhausted_simplex.attempted_realization_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        exhausted_simplex.stability_class(),
        Some(PrimitiveStabilityClass::RejectedBelowConditioningFloor)
    );
    assert_eq!(
        exhausted_simplex.exhaustion_reason(),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
    assert_eq!(
        pyramid.realization_strategy(),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        exhausted_pyramid.row_class(),
        PrimitiveConstructionCompoundRowClass::StructuredRealizationExhaustion
    );
    assert_eq!(exhausted_pyramid.realization_strategy(), None);
    assert_eq!(
        exhausted_pyramid.attempted_realization_strategies(),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        exhausted_pyramid.stability_class(),
        Some(PrimitiveStabilityClass::RejectedBelowConditioningFloor)
    );
    assert_eq!(
        exhausted_pyramid.exhaustion_reason(),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
    assert_ne!(report.report_digest(), shell.row_digest());
}

#[test]
fn compound_motion_and_grazing_reports_summarize_their_specialized_rows() {
    let mut workspace = compound_workspace("worth-kernel.compound-motion-grazing");
    let report = prepare_primitive_construction_compound_adversarial_siege_report(&mut workspace)
        .expect("compound report");
    let truth = parity_truth_from_siege(&report);
    let motion = prepare_primitive_construction_compound_motion_parity_report(&mut workspace)
        .expect("motion report");
    let grazing = prepare_primitive_construction_compound_grazing_boundary_report(&mut workspace)
        .expect("grazing report");
    let motion_ids = sorted_ids(
        motion
            .rows()
            .iter()
            .map(|row| row.scenario_id().to_string()),
    );
    let grazing_ids = sorted_ids(
        grazing
            .rows()
            .iter()
            .map(|row| row.scenario_id().to_string()),
    );

    assert!(motion.parity_verified());
    assert_eq!(motion_ids, expected_motion_scenario_ids(&truth));
    assert_eq!(
        motion
            .row_for("sheet_patch_reorient_grazing_workplane")
            .expect("shell motion row")
            .motion_kind(),
        PrimitiveConstructionCompoundMotionKind::Reorient
    );
    assert_eq!(
        motion
            .row_for("sheet_patch_reorient_grazing_workplane")
            .expect("shell motion row")
            .motion_digest(),
        report
            .row_for("sheet_patch_reorient_grazing_workplane")
            .expect("shell siege row")
            .motion_digest()
            .expect("shell siege motion digest")
    );
    assert_eq!(
        motion
            .row_for("wire_open_endpoint_graze")
            .expect("wire grazing motion row")
            .motion_kind(),
        PrimitiveConstructionCompoundMotionKind::Offset
    );
    assert_eq!(
        motion
            .row_for("wire_open_endpoint_graze")
            .expect("wire grazing motion row")
            .motion_digest(),
        report
            .row_for("wire_open_endpoint_graze")
            .expect("wire siege row")
            .motion_digest()
            .expect("wire siege motion digest")
    );
    assert_eq!(
        motion
            .row_for("wire_open_motion_relocation")
            .expect("wire relocation motion row")
            .motion_kind(),
        PrimitiveConstructionCompoundMotionKind::Move
    );
    assert_eq!(
        motion
            .row_for("wire_open_motion_relocation")
            .expect("wire relocation motion row")
            .motion_digest(),
        report
            .row_for("wire_open_motion_relocation")
            .expect("wire relocation siege row")
            .motion_digest()
            .expect("wire relocation siege motion digest")
    );
    assert!(grazing.parity_verified());
    assert_eq!(grazing_ids, expected_grazing_scenario_ids(&truth));
    assert_eq!(
        grazing
            .row_for("sheet_patch_reorient_grazing_workplane")
            .expect("shell grazing row")
            .grazing_kind(),
        PrimitiveConstructionCompoundGrazingKind::NearFrameNormalAlignment
    );
    assert_eq!(
        grazing
            .row_for("sheet_patch_reorient_grazing_workplane")
            .expect("shell grazing row")
            .grazing_digest(),
        report
            .row_for("sheet_patch_reorient_grazing_workplane")
            .expect("shell siege row")
            .grazing_digest()
            .expect("shell siege grazing digest")
    );
    assert_eq!(
        grazing
            .row_for("wire_open_endpoint_graze")
            .expect("wire grazing row")
            .grazing_kind(),
        PrimitiveConstructionCompoundGrazingKind::NearReferenceAnchorDistance
    );
    assert_eq!(
        grazing
            .row_for("wire_open_endpoint_graze")
            .expect("wire grazing row")
            .grazing_digest(),
        report
            .row_for("wire_open_endpoint_graze")
            .expect("wire siege row")
            .grazing_digest()
            .expect("wire siege grazing digest")
    );
    assert_ne!(motion.report_digest(), grazing.report_digest());
}

#[test]
fn compound_siege_report_anchors_public_rows_on_named_canonical_lane_not_vector_position() {
    let mut workspace = compound_workspace("worth-kernel.compound-siege-canonical-lane");
    let report = prepare_primitive_construction_compound_adversarial_siege_report(&mut workspace)
        .expect("compound report");
    let mut reordered_lanes = report.lane_reports().to_vec();
    reordered_lanes.rotate_left(1);
    let reordered =
        super::super::PrimitiveConstructionCompoundAdversarialSiegeReport::new(reordered_lanes);

    assert!(reordered.authoring_order_parity_verified());
    assert_eq!(
        sorted_ids(
            reordered
                .rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        ),
        sorted_ids(
            report
                .lane_reports()
                .iter()
                .find(|lane| lane.lane_name() == "canonical")
                .expect("canonical lane")
                .rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        )
    );
    assert_eq!(
        reordered
            .row_for("sheet_patch_reorient_grazing_workplane")
            .expect("shell row")
            .row_digest(),
        report
            .lane_reports()
            .iter()
            .find(|lane| lane.lane_name() == "canonical")
            .expect("canonical lane")
            .row_for("sheet_patch_reorient_grazing_workplane")
            .expect("canonical shell row")
            .row_digest()
    );
}
