use crate::construction::tests::support::compound_corpus::{
    expected_motion_scenario_ids, expected_required_scenario_ids, sorted_ids,
};
use crate::construction::tests::support::compound_lane_support::{
    compound_lane_names, compound_report_digest, compound_required_lane_coverage_verified,
    compound_required_scenario_coverage_verified, compound_row_for,
    compound_stable_scenario_coverage_verified,
};
use crate::construction::tests::support::compound_parity_view::prepare_compound_parity_view;
use crate::construction::tests::support::compound_row_support::{
    blocking_boundary, branch_basis_digest, exhaustion_reason, outcome_digest, query_surface_digest,
};
use crate::construction::tests::support::compound_runtime::compound_parity_registry;
use std::collections::BTreeSet;
use worth_geom::facade::PrimitiveRealizationExhaustionReason;

#[test]
fn compound_required_inventory_binds_siege_motion_grazing_and_parity_gate() {
    let report = prepare_compound_parity_view().expect("parity report");
    let requirements = compound_parity_registry().required_scenario_inventory();
    let required_ids = sorted_ids(requirements.scenario_ids().iter().cloned());
    let required_rows_present = compound_required_scenario_coverage_verified(report.siege());

    assert!(required_rows_present);
    assert!(compound_required_lane_coverage_verified(report.siege()));
    assert!(compound_required_scenario_coverage_verified(report.siege()));
    assert!(report.motion_expected_inventory_coverage_verified());
    assert!(report.grazing_expected_inventory_coverage_verified());
    assert!(report.exhaustion_expected_inventory_coverage_verified());
    assert_eq!(required_ids, expected_required_scenario_ids());
    for scenario_id in requirements.scenario_ids() {
        let row = requirements
            .row_for(scenario_id, |required| {
                compound_row_for(report.siege(), required)
            })
            .expect("required closeout row");
        assert_eq!(row.scenario_id(), scenario_id);
    }
    let required_rows = requirements
        .scenario_ids()
        .iter()
        .map(|scenario_id| {
            requirements
                .row_for(scenario_id, |required| {
                    compound_row_for(report.siege(), required)
                })
                .expect("required closeout row")
        })
        .collect::<Vec<_>>();
    assert_eq!(
        required_rows
            .iter()
            .map(|row| outcome_digest(row))
            .collect::<BTreeSet<_>>()
            .len(),
        required_rows.len()
    );
    assert_eq!(
        required_rows
            .iter()
            .map(|row| branch_basis_digest(row))
            .collect::<BTreeSet<_>>()
            .len(),
        6
    );
    assert!(required_rows
        .iter()
        .all(|row| !branch_basis_digest(row).is_empty()));
    assert!(query_surface_digest(
        compound_row_for(report.siege(), "sheet_patch_reorient_grazing_workplane")
            .expect("required shell row")
    )
    .is_some());
    assert_eq!(
        exhaustion_reason(
            compound_row_for(report.siege(), "pyramid_semantic_exhaustion")
                .expect("required pyramid exhaustion row")
        ),
        Some(PrimitiveRealizationExhaustionReason::DegenerateSupportNormals)
    );
    assert!(blocking_boundary(
        compound_row_for(report.siege(), "pyramid_semantic_exhaustion")
            .expect("required pyramid exhaustion row")
    )
    .is_some());
    assert_eq!(
        compound_report_digest(report.siege()),
        compound_report_digest(report.siege())
    );
    assert!(compound_stable_scenario_coverage_verified(report.siege()));
    assert_eq!(
        sorted_ids(compound_lane_names(report.siege())),
        vec![
            "canonical".to_string(),
            "escalation_clustered".to_string(),
            "family_clustered".to_string(),
            "rejected_first".to_string(),
            "reversed".to_string(),
        ]
    );
    assert_eq!(
        sorted_ids(report.motion_scenario_ids()),
        expected_motion_scenario_ids()
    );
    assert_eq!(
        sorted_ids(report.grazing_scenario_ids()),
        vec![
            "sheet_patch_reorient_grazing_workplane".to_string(),
            "wire_open_endpoint_graze".to_string(),
        ]
    );
    assert_eq!(
        sorted_ids(report.exhaustion_scenario_ids()),
        vec![
            "pyramid_semantic_exhaustion".to_string(),
            "simplex_world_collapsed_explicit_exhaustion".to_string(),
        ]
    );
    assert_ne!(
        compound_report_digest(report.siege()),
        report.report_digest()
    );
}
