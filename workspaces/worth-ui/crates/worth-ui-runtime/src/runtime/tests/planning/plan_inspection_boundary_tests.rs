use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::allocation_planning;
use super::plan_inspection_expected_provenance::{
    expected_provenance_for_node_input, expected_query_links_from_plan_input,
};
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiExecutionPlan, WorthUiExecutionPlanInput,
    WorthUiPlanInspectionDenialReason, WorthUiPlanNodeInputFamily, WorthUiPlanProvenanceSource,
};

#[test]
fn plan_inspection_explains_artifact_and_capability_origin() {
    let (runtime, plan_input, planning, plan) = inspection_fixture();

    let inspection = runtime
        .inspect_execution_plan(&plan, &planning)
        .expect("plan inspection succeeds");

    assert_eq!(inspection.nodes().len(), plan_input.node_inputs().len());
    assert_eq!(
        inspection.provenance().len(),
        plan_input.node_inputs().len()
    );
    assert_eq!(inspection.lanes().len(), plan.lane_partitions().len());
    assert_eq!(
        inspection.plan_digest(),
        runtime.digest_execution_plan(&plan)
    );
    assert!(inspection.nodes().iter().all(|node| node
        .artifact_provenance()
        .identity_basis()
        .len()
        > 0));
    assert!(inspection
        .nodes()
        .iter()
        .any(|node| node.capability_provenance().is_some()));
    for ((inspected_node, plan_node), node_input) in inspection
        .nodes()
        .iter()
        .zip(plan.topology().traversal_order())
        .zip(plan_input.node_inputs())
    {
        assert_eq!(
            inspected_node.plan_index(),
            plan_node.runtime_handle().plan_index()
        );
        assert_eq!(inspected_node.runtime_handle(), plan_node.runtime_handle());
        assert_eq!(inspected_node.family(), plan_node.family());
        assert_eq!(inspected_node.child_range(), plan_node.child_range());
        assert_eq!(
            inspected_node.region_structure(),
            plan_node.region_structure()
        );
        assert_eq!(inspected_node.egui_boundary(), plan_node.egui_boundary());
        assert_eq!(
            inspected_node.render_resource_ref(),
            plan_node.render_resource_ref()
        );
        assert_eq!(
            inspected_node.artifact_provenance(),
            &expected_provenance_for_node_input(
                plan_node.runtime_handle().plan_index(),
                node_input
            )
        );
        if node_input.family() == WorthUiPlanNodeInputFamily::QueryViewBinding {
            if node_input.query_binding_identity().is_some() {
                assert!(node_input.query_binding_posture().is_some());
                assert_eq!(
                    inspected_node.artifact_provenance().source(),
                    WorthUiPlanProvenanceSource::QueryBinding
                );
                assert!(inspected_node.query_inspection_links().is_some());
            } else {
                assert_eq!(
                    inspected_node.artifact_provenance().source(),
                    WorthUiPlanProvenanceSource::ReplacementClassification
                );
                assert_eq!(
                    inspected_node.capability_provenance(),
                    Some(node_input.identity_basis())
                );
                assert!(inspected_node.query_inspection_links().is_none());
            }
        }
    }
    for (inspected_lane, plan_lane) in inspection.lanes().iter().zip(plan.lane_partitions()) {
        assert_eq!(inspected_lane.lane(), plan_lane.lane());
        assert_eq!(inspected_lane.plan_indexes(), plan_lane.plan_indexes());
        assert_eq!(inspected_lane.node_count(), plan_lane.node_count());
    }
    assert_eq!(
        inspection.counters().node_inspection_count(),
        plan.topology().traversal_order().len()
    );
    assert_eq!(inspection.counters().plan_digest_count(), 1);
    assert_eq!(inspection.counters().inspection_count(), 1);
    assert_eq!(
        inspection.counters().lane_inspection_count(),
        plan.lane_partitions().len()
    );
    assert_eq!(inspection.counters().artifact_tree_scan_count(), 0);
    assert_eq!(inspection.counters().source_archaeology_count(), 0);
    assert_eq!(inspection.counters().registry_lookup_count(), 0);
    assert_eq!(inspection.counters().diagnostic_policy_read_count(), 0);
    assert_eq!(inspection.counters().frame_path_materialization_count(), 0);
}

#[test]
fn plan_provenance_replay_is_deterministic() {
    let (left_runtime, _, left_planning, left_plan) = inspection_fixture();
    let (_, _, right_planning, right_plan) = inspection_fixture();

    let left = left_runtime
        .inspect_execution_plan(&left_plan, &left_planning)
        .expect("left inspection succeeds");
    let right = left_runtime
        .inspect_execution_plan(&right_plan, &right_planning)
        .expect("right inspection succeeds");

    assert_eq!(left.plan_digest(), right.plan_digest());
    assert_eq!(left.nodes(), right.nodes());
    assert_eq!(left.lanes(), right.lanes());
    assert_eq!(left.provenance(), right.provenance());
    assert_eq!(left.counters(), right.counters());
}

#[test]
fn query_owned_inspection_links_are_preserved_not_reauthored() {
    let (runtime, plan_input, planning, plan) = inspection_fixture();
    let expected_query_links = expected_query_links_from_plan_input(&plan_input);

    let inspection = runtime
        .inspect_execution_plan(&plan, &planning)
        .expect("plan inspection succeeds");
    let query_nodes = inspection
        .nodes()
        .iter()
        .filter_map(|node| node.query_inspection_links())
        .collect::<Vec<_>>();

    assert!(!query_nodes.is_empty());
    assert_eq!(query_nodes.len(), expected_query_links.len());
    for (links, expected_links) in query_nodes.iter().zip(expected_query_links) {
        assert_eq!(
            links.binding_identity().view_binding_id(),
            "workspace.view_binding.selection"
        );
        assert_eq!(*links, &expected_links);
        assert_eq!(
            links.support_admission_digest(),
            expected_links.support_admission_digest()
        );
        assert_eq!(
            links.basis_capability_digest(),
            expected_links.basis_capability_digest()
        );
        assert_eq!(
            links.live_compatibility_digest(),
            expected_links.live_compatibility_digest()
        );
        assert_eq!(
            links.inspection_digest(),
            expected_links.inspection_digest()
        );
        assert_eq!(
            links.projection_consumption_digest(),
            expected_links.projection_consumption_digest()
        );
        assert_eq!(
            links.async_result_state_digest(),
            expected_links.async_result_state_digest()
        );
        assert_eq!(links.recovery_digest(), expected_links.recovery_digest());
        assert_eq!(
            links.preservation_receipt(),
            expected_links.preservation_receipt()
        );
        assert_eq!(
            links.required_surfaces(),
            expected_links.required_surfaces()
        );
    }
    assert_eq!(
        inspection.counters().query_link_preservation_count(),
        query_nodes.len()
    );
    assert_eq!(
        inspection.counters().projection_consumption_link_count(),
        query_nodes.len()
    );
    assert_eq!(inspection.counters().causal_inspection_reference_count(), 0);
    assert_eq!(inspection.counters().ordinary_outcome_reference_count(), 0);
}

#[test]
fn plan_inspection_rejects_mismatched_plan_input_before_provenance() {
    let (runtime, plan_input, _planning, plan) = inspection_fixture();
    let mismatched_input = plan_input_with_first_different_family(plan_input);
    let mismatched_planning =
        allocation_planning(&runtime, &mismatched_input, "plan-inspection.mismatch");

    let denial = runtime
        .inspect_execution_plan(&plan, mismatched_planning.planning())
        .expect_err("mismatched plan input denies inspection");

    assert_eq!(
        denial.reason(),
        WorthUiPlanInspectionDenialReason::PlanInputReceiptMismatch
    );
    assert_eq!(denial.counters().denial_count(), 1);
    assert_eq!(denial.counters().node_inspection_count(), 0);
    assert_eq!(denial.counters().provenance_link_count(), 0);
    assert_eq!(denial.counters().artifact_tree_scan_count(), 0);
}

#[test]
fn plan_inspection_rejects_same_shape_wrong_plan_input_receipt() {
    let (runtime, plan_input, _planning, plan) = inspection_fixture();
    let wrong_provenance_input = plan_input_with_first_identity_basis_changed(plan_input);
    let wrong_provenance_planning = allocation_planning(
        &runtime,
        &wrong_provenance_input,
        "plan-inspection.wrong-provenance",
    );

    let denial = runtime
        .inspect_execution_plan(&plan, wrong_provenance_planning.planning())
        .expect_err("same-shape wrong provenance input denies inspection");

    assert_eq!(
        denial.reason(),
        WorthUiPlanInspectionDenialReason::PlanInputReceiptMismatch
    );
    assert_eq!(denial.counters().denial_count(), 1);
    assert_eq!(denial.counters().node_inspection_count(), 0);
    assert_eq!(denial.counters().provenance_link_count(), 0);
    assert_eq!(denial.counters().source_archaeology_count(), 0);
}

fn inspection_fixture() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiExecutionPlanInput,
    WorthUiAllocationPlanning,
    WorthUiExecutionPlan,
) {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let (measurement_basis, graph_snapshot, selected_obligations) =
        super::allocation_planning_test_support::admitted_planning_admission(
            "plan-inspection.fixture",
            "operator:stack",
        );
    let candidate = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(
                &pending,
                &graph_snapshot,
                measurement_basis,
                &selected_obligations,
            )
            .expect("inspection fixture admits through graph authority"),
    );
    let planning = candidate.planning().clone();
    let receipt = runtime.detached_allocation_receipt_for_test(&candidate);
    let allocation = runtime
        .allocate_runtime_handles(&receipt)
        .expect("handles allocate");
    let plan = runtime
        .assemble_execution_plan_topology(&receipt, &allocation)
        .expect("topology assembles");
    (runtime, plan_input, planning, plan)
}

fn plan_input_with_first_different_family(
    plan_input: WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    let mut node_inputs = plan_input.node_inputs().to_vec();
    let replacement_family = alternate_family(node_inputs[0].family());
    node_inputs[0] = node_inputs[0]
        .clone()
        .with_family_for_test(replacement_family);
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}

fn plan_input_with_first_identity_basis_changed(
    plan_input: WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    let mut node_inputs = plan_input.node_inputs().to_vec();
    let changed_identity = format!("{}:wrong-provenance", node_inputs[0].identity_basis());
    node_inputs[0] = node_inputs[0]
        .clone()
        .with_identity_basis_for_test(changed_identity);
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}

fn alternate_family(family: WorthUiPlanNodeInputFamily) -> WorthUiPlanNodeInputFamily {
    match family {
        WorthUiPlanNodeInputFamily::TokenStyle => WorthUiPlanNodeInputFamily::DiagnosticsRef,
        _ => WorthUiPlanNodeInputFamily::TokenStyle,
    }
}
