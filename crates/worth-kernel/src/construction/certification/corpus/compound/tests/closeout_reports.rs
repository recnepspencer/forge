use super::super::prepare_primitive_construction_compound_milestone_closeout_report;
use std::collections::BTreeSet;
use topology::facade::{milestone_one_runtime_builder, topology_runtime, TopologyRuntimeAdapters};
use worth_geom::facade::PrimitiveRealizationExhaustionReason;

fn sorted_ids(ids: impl IntoIterator<Item = String>) -> Vec<String> {
    let mut ids = ids.into_iter().collect::<Vec<_>>();
    ids.sort();
    ids
}

#[test]
fn compound_milestone_closeout_report_binds_siege_motion_grazing_and_parity_gate() {
    let runtime = milestone_one_runtime_builder()
        .expect("runtime builder")
        .build();
    let mut workspace = topology_runtime(
        TopologyRuntimeAdapters::current_head(runtime),
        "worth-kernel.compound-closeout".to_string(),
    )
    .expect("workspace");
    let report = prepare_primitive_construction_compound_milestone_closeout_report(&mut workspace)
        .expect("closeout report");
    let required_ids = sorted_ids(report.required_scenarios().iter().cloned());

    assert!(report.required_rows_present());
    assert!(report.closeout_gate_verified());
    assert_eq!(
        required_ids,
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
    for scenario_id in report.required_scenarios() {
        let row = report
            .required_row_for(scenario_id)
            .expect("required closeout row");
        assert_eq!(row.scenario_id(), scenario_id);
    }
    let required_rows = report
        .required_scenarios()
        .iter()
        .map(|scenario_id| {
            report
                .required_row_for(scenario_id)
                .expect("required closeout row")
        })
        .collect::<Vec<_>>();
    assert_eq!(
        required_rows
            .iter()
            .map(|row| row.direct_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        required_rows.len()
    );
    assert_eq!(
        required_rows
            .iter()
            .map(|row| row.replay_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        required_rows.len()
    );
    assert_eq!(
        required_rows
            .iter()
            .map(|row| row.branch_local_digest())
            .collect::<BTreeSet<_>>()
            .len(),
        required_rows.len()
    );
    assert!(report
        .required_row_for("sheet_patch_reorient_grazing_workplane")
        .expect("required shell row")
        .inspection_digest()
        .is_some());
    assert!(report
        .required_row_for("sheet_patch_reorient_grazing_workplane")
        .expect("required shell row")
        .projection_consumption_digest()
        .is_some());
    assert_eq!(
        report
            .required_row_for("pyramid_semantic_exhaustion")
            .expect("required pyramid exhaustion row")
            .exhaustion_reason(),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
    assert!(report.parity().parity_verified());
    assert!(report
        .parity()
        .ordering()
        .scenario_rows()
        .iter()
        .all(|row| row.stable_across_orders()));
    assert_eq!(
        sorted_ids(
            report
                .parity()
                .ordering()
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
    assert_eq!(report.motion().rows().len(), 3);
    assert_eq!(report.grazing().rows().len(), 2);
    assert_eq!(report.parity().exhaustion().rows().len(), 2);
    assert_ne!(
        report.siege().report_digest(),
        report.parity().report_digest()
    );
}
