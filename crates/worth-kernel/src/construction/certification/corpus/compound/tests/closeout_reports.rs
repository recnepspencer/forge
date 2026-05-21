use super::super::prepare_primitive_construction_compound_milestone_closeout_report;
use super::support::{compound_workspace, expected_required_scenario_ids, sorted_ids};
use std::collections::BTreeSet;
use worth_geom::facade::PrimitiveRealizationExhaustionReason;

#[test]
fn compound_milestone_closeout_report_binds_siege_motion_grazing_and_parity_gate() {
    let mut workspace = compound_workspace("worth-kernel.compound-closeout");
    let report = prepare_primitive_construction_compound_milestone_closeout_report(&mut workspace)
        .expect("closeout report");
    let required_ids = sorted_ids(report.required_scenarios().iter().cloned());

    assert!(report.required_rows_present());
    assert!(report.closeout_gate_verified());
    assert_eq!(required_ids, expected_required_scenario_ids());
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
    assert_eq!(
        report.parity().truth().siege().report_digest(),
        report.siege().report_digest()
    );
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
