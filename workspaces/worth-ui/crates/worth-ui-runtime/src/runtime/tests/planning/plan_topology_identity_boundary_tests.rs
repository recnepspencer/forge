use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{
    admitted_allocation_neighborhood_for_basis, admitted_measurement_basis_with_font_seed,
};
use crate::runtime::WorthUiPlanTopologyDenialReason;

#[test]
fn stale_lane_admission_denies_when_planning_identity_changes_without_topology_drift() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let first_basis =
        admitted_measurement_basis_with_font_seed("plan-topology.identity-drift", 100);
    let second_basis =
        admitted_measurement_basis_with_font_seed("plan-topology.identity-drift", 240);
    let first_neighborhood = admitted_allocation_neighborhood_for_basis(
        "plan-topology.identity-drift",
        first_basis.clone(),
    );
    let second_neighborhood = admitted_allocation_neighborhood_for_basis(
        "plan-topology.identity-drift",
        second_basis.clone(),
    );
    let first_planning = runtime.plan_allocation(&pending, &first_basis, &first_neighborhood);
    let second_planning = runtime.plan_allocation(&pending, &second_basis, &second_neighborhood);
    let stale_lane_admission = runtime
        .admit_execution_lanes(
            &first_planning,
            &crate::runtime::WorthUiExecutionLaneSupport::platform_default(),
        )
        .expect("first lane admission succeeds");
    let second_allocation = runtime
        .allocate_runtime_handles(&second_planning)
        .expect("second handles allocate");

    assert_eq!(first_planning.node_inputs(), second_planning.node_inputs());
    assert_ne!(
        first_planning.planning_identity_digest(),
        second_planning.planning_identity_digest()
    );
    assert_ne!(
        stale_lane_admission.plan_input_basis_digest(),
        second_allocation.receipt().basis_digest()
    );

    let denial = runtime
        .assemble_execution_plan_topology_with_lane_admission(
            &second_planning,
            &second_allocation,
            &stale_lane_admission,
        )
        .expect_err("stale lane admission must deny");

    assert_eq!(
        denial.reason(),
        WorthUiPlanTopologyDenialReason::LaneAdmissionMismatch
    );
}
