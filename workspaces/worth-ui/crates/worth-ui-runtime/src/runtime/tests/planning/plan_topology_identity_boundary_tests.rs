use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::admitted_planning_admission_with_font_seed;
use crate::runtime::WorthUiPlanTopologyDenialReason;

#[test]
fn stale_lane_admission_denies_when_planning_identity_changes_without_topology_drift() {
    let (first_runtime, first_pending) = activation_staging_inputs().into_runtime_and_pending();
    let (second_runtime, second_pending) = activation_staging_inputs().into_runtime_and_pending();
    let (first_basis, first_snapshot, first_selected) = admitted_planning_admission_with_font_seed(
        "plan-topology.identity-drift",
        100,
        "operator:stack",
    );
    let (second_basis, second_snapshot, second_selected) =
        admitted_planning_admission_with_font_seed(
            "plan-topology.identity-drift",
            240,
            "operator:stack",
        );
    let first_planning = first_runtime.plan_allocation(
        first_runtime
            .admit_planning_lane_input(
                &first_pending,
                &first_snapshot,
                first_basis,
                &first_selected,
            )
            .expect("first topology identity input admits through graph authority"),
    );
    let second_planning = second_runtime.plan_allocation(
        second_runtime
            .admit_planning_lane_input(
                &second_pending,
                &second_snapshot,
                second_basis,
                &second_selected,
            )
            .expect("second topology identity input admits through graph authority"),
    );
    let stale_lane_admission = first_runtime
        .admit_execution_lanes(
            &first_runtime.detached_allocation_receipt_for_test(&first_planning),
            &crate::runtime::WorthUiExecutionLaneSupport::platform_default(),
        )
        .expect("first lane admission succeeds");
    let second_receipt = second_runtime.detached_allocation_receipt_for_test(&second_planning);
    let second_allocation = second_runtime
        .allocate_runtime_handles(&second_receipt)
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

    let denial = second_runtime
        .assemble_execution_plan_topology_with_lane_admission(
            &second_receipt,
            &second_allocation,
            &stale_lane_admission,
        )
        .expect_err("stale lane admission must deny");

    assert_eq!(
        denial.reason(),
        WorthUiPlanTopologyDenialReason::LaneAdmissionMismatch
    );
}
