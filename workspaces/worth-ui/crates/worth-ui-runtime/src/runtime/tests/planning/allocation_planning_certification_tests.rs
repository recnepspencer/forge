use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{
    admitted_measurement_basis, planning_graph_authority,
};
use crate::facade::evidence::{
    certify_allocation_planning_determinism, certify_allocation_planning_suite,
    UiAllocationPlanningCertificationSuiteKind, UiAllocationPlanningDeterminismPosture,
};

#[test]
fn equivalent_planning_certification_reports_deterministic_kernel() {
    let inputs = activation_staging_inputs();
    let (_, runtime, pending) = inputs.into_app_runtime_and_pending();
    let first_basis = admitted_measurement_basis("allocation-inspection.cert");
    let second_basis = admitted_measurement_basis("allocation-inspection.cert");
    let (first_snapshot, first_selected) =
        planning_graph_authority("allocation-inspection.cert", "operator:stack");
    let (second_snapshot, second_selected) =
        planning_graph_authority("allocation-inspection.cert", "operator:stack");
    let first = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &first_snapshot, first_basis, &first_selected)
            .expect("first certification input admits through graph authority"),
    );
    let second = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &second_snapshot, second_basis, &second_selected)
            .expect("second certification input admits through graph authority"),
    );

    let report = certify_allocation_planning_determinism(first.planning(), second.planning());
    assert_eq!(
        report.determinism_posture(),
        UiAllocationPlanningDeterminismPosture::Equivalent
    );
    assert!(report.neighborhood_width_is_bounded());
    assert!(report.emitted_edges_match());
    assert!(report.solve_trace_converges());
    assert!(report.handoff_identity_matches());
    assert!(report.denied_broadening_matches());
}

#[test]
fn planning_suite_certification_binds_runtime_invariants_to_named_suite_kinds() {
    let inputs = activation_staging_inputs();
    let (_, runtime, pending) = inputs.into_app_runtime_and_pending();
    let first_basis = admitted_measurement_basis("allocation-inspection.suite");
    let second_basis = admitted_measurement_basis("allocation-inspection.suite");
    let (first_snapshot, first_selected) =
        planning_graph_authority("allocation-inspection.suite", "operator:stack");
    let (second_snapshot, second_selected) =
        planning_graph_authority("allocation-inspection.suite", "operator:stack");
    let first = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &first_snapshot, first_basis, &first_selected)
            .expect("first suite input admits through graph authority"),
    );
    let second = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &second_snapshot, second_basis, &second_selected)
            .expect("second suite input admits through graph authority"),
    );

    let report = certify_allocation_planning_suite(
        UiAllocationPlanningCertificationSuiteKind::BoundedReconciliation,
        first.planning(),
        second.planning(),
    );
    assert_eq!(
        report.suite_kind(),
        Some(UiAllocationPlanningCertificationSuiteKind::BoundedReconciliation)
    );
    assert!(report.suite_verified());
    assert!(report.neighborhood_width_is_bounded());
    assert!(report.emitted_edges_match());
}
