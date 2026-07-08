use crate::runtime::tests::activation_staging_test_support::activation_staging_inputs;
use crate::runtime::tests::allocation_planning_test_support::{
    admitted_allocation_neighborhood, admitted_measurement_basis,
};
use crate::runtime::tests::durable_state_inventory_test_support::platform_inventory;
use crate::runtime::activation::WorthUiActivationLaneInput;
use crate::runtime::WorthUiRuntimeHost;

#[test]
fn lifecycle_path_parity_orchestrator_matches_stepwise_replacement_chain() {
    let manual_inputs = activation_staging_inputs();
    let (_, manual_pending) = manual_inputs.into_runtime_and_pending();

    let orchestrated_inputs = activation_staging_inputs();
    let inventory = platform_inventory(&orchestrated_inputs.runtime)
        .build_for_replacement(&orchestrated_inputs.node_plan)
        .expect("inventory builds");
    let lowering = orchestrated_inputs
        .runtime
        .prepare_replacement_lowering(orchestrated_inputs.admitted, &inventory)
        .expect("replacement lowering orchestrator succeeds");
    let orchestrated_pending = orchestrated_inputs
        .runtime
        .stage_replacement_activation_from_lane_input(WorthUiActivationLaneInput::from_lowering(
            lowering,
        ))
        .expect("orchestrated staging succeeds");

    assert_eq!(
        manual_pending.frame_epoch(),
        orchestrated_pending.frame_epoch()
    );
    assert_eq!(
        manual_pending
            .staged_replacement()
            .node_plan()
            .classifications()
            .len(),
        orchestrated_pending
            .staged_replacement()
            .node_plan()
            .classifications()
            .len()
    );
}

#[test]
fn lifecycle_path_parity_planning_lane_matches_direct_plan_allocation() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let measurement_basis = admitted_measurement_basis("lifecycle-path.parity");
    let allocation_neighborhood = admitted_allocation_neighborhood("lifecycle-path.parity");

    let direct = runtime.plan_allocation(
        &pending,
        &measurement_basis,
        &allocation_neighborhood,
    );
    let via_lane_input = runtime.plan_allocation_from_lane_input(
        crate::runtime::WorthUiPlanningLaneInput::new(
            &pending,
            measurement_basis.clone(),
            allocation_neighborhood.clone(),
        ),
    );

    assert_eq!(
        direct.planning_identity_digest(),
        via_lane_input.planning_identity_digest()
    );
    assert_eq!(
        direct.denial_posture().is_some(),
        via_lane_input.denial_posture().is_some()
    );
}

#[test]
fn lifecycle_path_parity_execution_lane_matches_direct_handle_allocation() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let measurement_basis = admitted_measurement_basis("lifecycle-path.execution");
    let allocation_neighborhood = admitted_allocation_neighborhood("lifecycle-path.execution");
    let planning = runtime.plan_allocation(
        &pending,
        &measurement_basis,
        &allocation_neighborhood,
    );

    let direct = runtime
        .allocate_runtime_handles(&planning)
        .expect("direct handle allocation succeeds");
    let via_lane = runtime
        .allocate_runtime_handles_from_lane_input(crate::runtime::WorthUiExecutionLaneInput::new(
            &planning,
        ))
        .expect("lane handle allocation succeeds");

    assert_eq!(
        direct.receipt().basis_digest(),
        via_lane.receipt().basis_digest()
    );
}

fn _host_type_check(_: &WorthUiRuntimeHost) {}