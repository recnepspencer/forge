use crate::runtime::activation::WorthUiActivationLaneInput;
use crate::runtime::tests::activation_staging_test_support::activation_staging_inputs;
use crate::runtime::tests::allocation_planning_test_support::admitted_planning_admission;
use crate::runtime::tests::durable_state_inventory_test_support::platform_inventory;
use crate::runtime::WorthUiRuntime;

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
fn lifecycle_path_planning_uses_canonical_admission() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (basis, snapshot, selected) =
        admitted_planning_admission("lifecycle-path.parity", "operator:stack");
    let input = runtime
        .admit_planning_lane_input(&pending, &snapshot, basis, &selected)
        .expect("canonical planning admission succeeds");
    assert!(runtime.plan_allocation(input).is_admitted());
}

#[test]
fn lifecycle_path_parity_execution_lane_matches_direct_handle_allocation() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (measurement_basis, snapshot, selected) =
        admitted_planning_admission("lifecycle-path.execution", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("execution parity planning admits through graph authority"),
    );

    let direct = runtime
        .allocate_runtime_handles(&runtime.detached_allocation_receipt_for_test(&planning))
        .expect("direct handle allocation succeeds");
    let via_lane = runtime
        .allocate_runtime_handles_from_lane_input(crate::runtime::WorthUiExecutionLaneInput::new(
            &runtime.detached_allocation_receipt_for_test(&planning),
        ))
        .expect("lane handle allocation succeeds");

    assert_eq!(
        direct.receipt().basis_digest(),
        via_lane.receipt().basis_digest()
    );
}

fn _host_type_check(_: &WorthUiRuntime) {}
