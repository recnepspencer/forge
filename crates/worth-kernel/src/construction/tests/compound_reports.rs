use crate::construction::tests::support::compound_corpus::{
    expected_grazing_scenario_ids, expected_motion_scenario_ids, sorted_ids,
};
use crate::construction::tests::support::compound_lane_support::{
    compound_authoring_order_parity_verified, compound_canonical_rows, compound_lane_names,
    compound_report_digest, compound_required_scenario_coverage_verified, compound_row_for,
};
use crate::construction::tests::support::compound_parity_view::prepare_compound_parity_view;
use crate::construction::tests::support::compound_row_support::{
    attempted_realization_strategies, exhaustion_reason, grazing_digest, grazing_kind,
    motion_digest, motion_kind, query_surface_digest, realization_strategy, row_digest,
    stability_class,
};
use crate::construction::tests::support::compound_runtime::{
    prepare_primitive_construction_compound_adversarial_lanes,
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundTopologyClass,
    PrimitiveConstructionCompoundWorkloadFamily,
};
use worth_geom::facade::{
    PrimitiveRealizationExhaustionReason, PrimitiveRealizationStrategy, PrimitiveStabilityClass,
};

#[test]
fn compound_adversarial_siege_report_carries_shell_wire_and_motion_truth() {
    let report =
        prepare_primitive_construction_compound_adversarial_lanes().expect("compound report");
    let shell =
        compound_row_for(&report, "sheet_patch_reorient_grazing_workplane").expect("shell row");
    let wire = compound_row_for(&report, "wire_open_endpoint_graze").expect("wire row");
    let pyramid =
        compound_row_for(&report, "pyramid_threshold_admitted_exact_support").expect("pyramid row");
    let exhausted_pyramid =
        compound_row_for(&report, "pyramid_semantic_exhaustion").expect("exhausted pyramid row");
    let simplex = compound_row_for(&report, "simplex_world_collapsed_admitted_local_or_exact")
        .expect("simplex row");
    let exhausted_simplex =
        compound_row_for(&report, "simplex_world_collapsed_explicit_exhaustion")
            .expect("exhausted simplex row");

    assert!(compound_authoring_order_parity_verified(&report));
    assert!(compound_required_scenario_coverage_verified(&report));
    assert_eq!(
        sorted_ids(compound_lane_names(&report)),
        vec![
            "canonical".to_string(),
            "escalation_clustered".to_string(),
            "family_clustered".to_string(),
            "rejected_first".to_string(),
            "reversed".to_string(),
        ]
    );
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
        motion_kind(shell),
        Some(PrimitiveConstructionCompoundMotionKind::Reorient)
    );
    assert_eq!(
        grazing_kind(shell),
        Some(PrimitiveConstructionCompoundGrazingKind::NearFrameNormalAlignment)
    );
    assert_eq!(
        query_surface_digest(shell),
        query_surface_digest(
            compound_row_for(&report, "sheet_patch_reorient_grazing_workplane").expect("shell row")
        )
    );
    assert_ne!(
        query_surface_digest(shell).expect("shell query surface digest"),
        row_digest(shell)
    );
    assert_eq!(
        wire.topology_class(),
        PrimitiveConstructionCompoundTopologyClass::OpenWire
    );
    assert_eq!(
        motion_kind(wire),
        Some(PrimitiveConstructionCompoundMotionKind::Offset)
    );
    assert_eq!(
        grazing_kind(wire),
        Some(PrimitiveConstructionCompoundGrazingKind::NearReferenceAnchorDistance)
    );
    assert_eq!(
        simplex.row_class(),
        PrimitiveConstructionCompoundRowClass::EscalatedStableExactSupport
    );
    assert_eq!(
        realization_strategy(simplex),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(realization_strategy(exhausted_simplex), None);
    assert_eq!(
        exhausted_simplex.row_class(),
        PrimitiveConstructionCompoundRowClass::StructuredRealizationExhaustion
    );
    assert_eq!(
        attempted_realization_strategies(exhausted_simplex),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        stability_class(exhausted_simplex),
        Some(PrimitiveStabilityClass::RejectedBelowConditioningFloor)
    );
    assert_eq!(
        exhaustion_reason(exhausted_simplex),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
    assert_eq!(
        realization_strategy(pyramid),
        Some(PrimitiveRealizationStrategy::ExactSupport)
    );
    assert_eq!(
        exhausted_pyramid.row_class(),
        PrimitiveConstructionCompoundRowClass::StructuredRealizationExhaustion
    );
    assert_eq!(realization_strategy(exhausted_pyramid), None);
    assert_eq!(
        attempted_realization_strategies(exhausted_pyramid),
        &[
            PrimitiveRealizationStrategy::DirectWorld,
            PrimitiveRealizationStrategy::ExactSupport,
        ]
    );
    assert_eq!(
        stability_class(exhausted_pyramid),
        Some(PrimitiveStabilityClass::RejectedBelowConditioningFloor)
    );
    assert_eq!(
        exhaustion_reason(exhausted_pyramid),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
    assert_ne!(compound_report_digest(&report), row_digest(shell));
}

#[test]
fn compound_motion_and_grazing_reports_summarize_their_specialized_rows() {
    let parity = prepare_compound_parity_view().expect("parity report");
    let report = parity.siege();
    let motion_ids = sorted_ids(
        parity
            .motion_rows()
            .iter()
            .map(|row| row.scenario_id().to_string()),
    );
    let grazing_ids = sorted_ids(
        parity
            .grazing_rows()
            .iter()
            .map(|row| row.scenario_id().to_string()),
    );

    assert!(parity.motion_parity_verified());
    assert_eq!(motion_ids, expected_motion_scenario_ids());
    assert_eq!(
        parity
            .motion_row_for("sheet_patch_reorient_grazing_workplane")
            .expect("shell motion row")
            .motion_kind(),
        PrimitiveConstructionCompoundMotionKind::Reorient
    );
    assert_eq!(
        parity
            .motion_row_for("sheet_patch_reorient_grazing_workplane")
            .expect("shell motion row")
            .motion_digest(),
        motion_digest(
            compound_row_for(report, "sheet_patch_reorient_grazing_workplane")
                .expect("shell siege row")
        )
        .expect("shell siege motion digest")
    );
    assert_eq!(
        parity
            .motion_row_for("wire_open_endpoint_graze")
            .expect("wire grazing motion row")
            .motion_kind(),
        PrimitiveConstructionCompoundMotionKind::Offset
    );
    assert_eq!(
        parity
            .motion_row_for("wire_open_endpoint_graze")
            .expect("wire grazing motion row")
            .motion_digest(),
        motion_digest(
            compound_row_for(report, "wire_open_endpoint_graze").expect("wire siege row")
        )
        .expect("wire siege motion digest")
    );
    assert_eq!(
        parity
            .motion_row_for("wire_open_motion_relocation")
            .expect("wire relocation motion row")
            .motion_kind(),
        PrimitiveConstructionCompoundMotionKind::Move
    );
    assert_eq!(
        parity
            .motion_row_for("wire_open_motion_relocation")
            .expect("wire relocation motion row")
            .motion_digest(),
        motion_digest(
            compound_row_for(report, "wire_open_motion_relocation")
                .expect("wire relocation siege row")
        )
        .expect("wire relocation siege motion digest")
    );
    assert!(parity.grazing_parity_verified());
    assert_eq!(grazing_ids, expected_grazing_scenario_ids());
    assert_eq!(
        parity
            .grazing_row_for("sheet_patch_reorient_grazing_workplane")
            .expect("shell grazing row")
            .grazing_kind(),
        PrimitiveConstructionCompoundGrazingKind::NearFrameNormalAlignment
    );
    assert_eq!(
        parity
            .grazing_row_for("sheet_patch_reorient_grazing_workplane")
            .expect("shell grazing row")
            .grazing_digest(),
        grazing_digest(
            compound_row_for(report, "sheet_patch_reorient_grazing_workplane")
                .expect("shell siege row")
        )
        .expect("shell siege grazing digest")
    );
    assert_eq!(
        parity
            .grazing_row_for("wire_open_endpoint_graze")
            .expect("wire grazing row")
            .grazing_kind(),
        PrimitiveConstructionCompoundGrazingKind::NearReferenceAnchorDistance
    );
    assert_eq!(
        parity
            .grazing_row_for("wire_open_endpoint_graze")
            .expect("wire grazing row")
            .grazing_digest(),
        grazing_digest(
            compound_row_for(report, "wire_open_endpoint_graze").expect("wire siege row")
        )
        .expect("wire siege grazing digest")
    );
    assert_ne!(
        parity.motion_report_digest(),
        parity.grazing_report_digest()
    );
}

#[test]
fn compound_siege_report_anchors_public_rows_on_named_canonical_lane_not_vector_position() {
    let report =
        prepare_primitive_construction_compound_adversarial_lanes().expect("compound report");
    let mut reordered_lanes = report.clone();
    reordered_lanes.rotate_left(1);
    let reordered = reordered_lanes;

    assert!(compound_authoring_order_parity_verified(&reordered));
    assert!(compound_required_scenario_coverage_verified(&reordered));
    assert_eq!(
        sorted_ids(
            compound_canonical_rows(&reordered)
                .iter()
                .map(|row| row.scenario_id().to_string()),
        ),
        sorted_ids(
            report
                .iter()
                .find(|(lane, _)| lane.as_str() == "canonical")
                .expect("canonical lane")
                .1
                .iter()
                .map(|row| row.scenario_id().to_string()),
        )
    );
    assert_eq!(
        row_digest(
            compound_row_for(&reordered, "sheet_patch_reorient_grazing_workplane")
                .expect("shell row"),
        ),
        row_digest(
            report
                .iter()
                .find(|(lane, _)| lane.as_str() == "canonical")
                .expect("canonical lane")
                .1
                .iter()
                .find(|row| row.scenario_id() == "sheet_patch_reorient_grazing_workplane")
                .expect("canonical shell row"),
        )
    );
}
