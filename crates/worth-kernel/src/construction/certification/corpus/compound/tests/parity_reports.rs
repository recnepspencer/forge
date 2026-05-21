use super::super::{
    prepare_primitive_construction_compound_exhaustion_witness_parity_report,
    prepare_primitive_construction_compound_ordering_parity_report,
    prepare_primitive_construction_compound_parity_report,
};
use std::collections::BTreeSet;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::PrimitiveRealizationExhaustionWitnessKind;

fn sorted_ids(ids: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut ids = ids.into_iter().collect::<Vec<_>>();
    ids.sort();
    ids
}

#[test]
fn compound_ordering_parity_report_requires_the_full_spec_order_matrix() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.compound-ordering-parity".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_compound_ordering_parity_report(&mut workspace)
        .expect("ordering parity report");

    assert!(report.parity_verified());
    assert_eq!(
        sorted_ids(
            report
                .authoring_order_rows()
                .iter()
                .map(|row| row.lane_name().to_string()),
        ),
        vec![
            "canonical".to_string(),
            "escalation_clustered".to_string(),
            "family_clustered".to_string(),
            "rejected_first".to_string(),
            "reversed".to_string(),
        ]
    );
    let lane_digests = report
        .authoring_order_rows()
        .iter()
        .map(|row| row.lane_digest().to_string())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(lane_digests.len(), report.authoring_order_rows().len());
    assert_eq!(report.lane_reports().len(), 5);
    assert_eq!(
        sorted_ids(
            report
                .scenario_rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        ),
        vec![
            "mixed_topology_class_batch".to_string(),
            "orthotope_boundary_neighbor_rejected".to_string(),
            "orthotope_direct_stable".to_string(),
            "pyramid_direct_stable_comparison".to_string(),
            "pyramid_semantic_exhaustion".to_string(),
            "pyramid_threshold_admitted_exact_support".to_string(),
            "pyramid_threshold_rejected_neighbor".to_string(),
            "regular_prism_boundary_neighbor_rejected".to_string(),
            "regular_prism_direct_stable".to_string(),
            "sheet_patch_reorient_grazing_workplane".to_string(),
            "simplex_world_collapsed_admitted_local_or_exact".to_string(),
            "simplex_world_collapsed_explicit_exhaustion".to_string(),
            "simplex_world_collapsed_threshold_rejected".to_string(),
            "wire_open_endpoint_graze".to_string(),
            "wire_open_motion_relocation".to_string(),
        ]
    );
    let shell_graze = report
        .scenario_row_for("sheet_patch_reorient_grazing_workplane")
        .expect("shell grazing scenario row");
    assert!(shell_graze.stable_across_orders());
    assert!(shell_graze.grazing_kind_stable());
    let simplex_exhausted = report
        .scenario_row_for("simplex_world_collapsed_explicit_exhaustion")
        .expect("simplex exhausted scenario row");
    assert!(simplex_exhausted.stable_across_orders());
    assert!(simplex_exhausted.exhaustion_reason_stable());
    let rejected_pyramid = report
        .scenario_row_for("pyramid_threshold_rejected_neighbor")
        .expect("rejected pyramid scenario row");
    assert!(rejected_pyramid.rejection_class_stable());
    assert!(rejected_pyramid.rejection_locality_stable());
    assert_ne!(report.normalized_matrix_digest(), report.report_digest());
    assert_ne!(
        report.report_digest(),
        report
            .scenario_row_for("orthotope_direct_stable")
            .expect("orthotope scenario row")
            .row_digest()
    );
}

#[test]
fn compound_exhaustion_witness_parity_report_binds_kernel_rows_to_lower_layer_witnesses() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.compound-exhaustion-parity".to_string(),
    )
    .expect("workspace");
    let report =
        prepare_primitive_construction_compound_exhaustion_witness_parity_report(&mut workspace)
            .expect("exhaustion parity report");

    assert!(report.parity_verified());
    assert_eq!(
        sorted_ids(
            report
                .rows()
                .iter()
                .map(|row| row.scenario_id().to_string())
        ),
        vec![
            "pyramid_semantic_exhaustion".to_string(),
            "simplex_world_collapsed_explicit_exhaustion".to_string(),
        ]
    );
    assert_eq!(
        report
            .row_for("pyramid_semantic_exhaustion")
            .expect("pyramid exhaustion row")
            .witness_kind(),
        PrimitiveRealizationExhaustionWitnessKind::ZeroRadiusPyramidSupportCollapse
    );
    assert_eq!(
        report
            .row_for("simplex_world_collapsed_explicit_exhaustion")
            .expect("simplex exhaustion row")
            .witness_kind(),
        PrimitiveRealizationExhaustionWitnessKind::AltitudeSqueezedSimplexSupportCollapse
    );
    assert_eq!(
        report
            .rows()
            .iter()
            .map(|row| row.siege_row_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        report.rows().len()
    );
    assert_eq!(
        report
            .rows()
            .iter()
            .map(|row| row.witness_row_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        report.rows().len()
    );
}

#[test]
fn compound_parity_report_bundles_ordering_motion_grazing_and_exhaustion_truth() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.compound-parity".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_compound_parity_report(&mut workspace)
        .expect("compound parity report");

    assert!(report.parity_verified());
    assert!(report.ordering().parity_verified());
    assert!(report.motion().parity_verified());
    assert!(report.grazing().parity_verified());
    assert!(report.exhaustion().parity_verified());
    assert_eq!(report.motion().rows().len(), 3);
    assert_eq!(report.grazing().rows().len(), 2);
    assert_eq!(report.exhaustion().rows().len(), 2);
    assert_eq!(
        sorted_ids(
            report
                .motion()
                .rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        ),
        vec![
            "sheet_patch_reorient_grazing_workplane".to_string(),
            "wire_open_endpoint_graze".to_string(),
            "wire_open_motion_relocation".to_string(),
        ]
    );
    assert_eq!(
        sorted_ids(
            report
                .grazing()
                .rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        ),
        vec![
            "sheet_patch_reorient_grazing_workplane".to_string(),
            "wire_open_endpoint_graze".to_string(),
        ]
    );
    assert_eq!(
        sorted_ids(
            report
                .exhaustion()
                .rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        ),
        vec![
            "pyramid_semantic_exhaustion".to_string(),
            "simplex_world_collapsed_explicit_exhaustion".to_string(),
        ]
    );
    assert_ne!(
        report.motion().report_digest(),
        report.grazing().report_digest()
    );
    assert_ne!(
        report.grazing().report_digest(),
        report.exhaustion().report_digest()
    );
}

#[test]
fn compound_ordering_parity_report_anchors_stability_on_named_canonical_lane_not_vector_position() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.compound-ordering-canonical-lane".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_compound_ordering_parity_report(&mut workspace)
        .expect("ordering parity report");
    let mut reordered_lanes = report.lane_reports().to_vec();
    reordered_lanes.rotate_left(2);
    let reordered =
        super::super::PrimitiveConstructionCompoundOrderingParityReport::new(reordered_lanes);

    assert!(reordered.parity_verified());
    assert_eq!(
        reordered.normalized_matrix_digest(),
        report.normalized_matrix_digest()
    );
    assert_eq!(
        sorted_ids(
            reordered
                .scenario_rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        ),
        sorted_ids(
            report
                .scenario_rows()
                .iter()
                .map(|row| row.scenario_id().to_string()),
        )
    );
    assert!(reordered
        .scenario_row_for("pyramid_semantic_exhaustion")
        .expect("pyramid scenario row")
        .stable_across_orders());
}
