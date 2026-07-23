use super::activation_staging_test_support::activation_staging_inputs;
use super::plan_inspection_expected_provenance::{
    expected_provenance_for_node_input, expected_query_links_from_plan_input,
};
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiExecutionPlanInput, WorthUiPlanInspectionDenialReason,
    WorthUiPlanNode, WorthUiPlanNodeFamily, WorthUiPlanNodeInputFamily,
    WorthUiPlanProvenanceSource, WorthUiPlanTopology,
};

#[test]
fn plan_inspection_explains_artifact_and_capability_origin() {
    let (runtime, plan_input, lowering_facts, plan) = inspection_fixture();

    let inspection = runtime
        .inspect_execution_plan(&plan, &lowering_facts)
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
    assert!(inspection
        .nodes()
        .iter()
        .all(|node| !node.artifact_provenance().identity_basis().is_empty()));
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
                assert!(node_input.query_settled_fact_link().is_some());
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
fn semantic_plan_provenance_replay_is_deterministic_without_sharing_handles() {
    let (left_runtime, _, left_facts, left_plan) = inspection_fixture();
    let (_, _, right_facts, right_plan) = inspection_fixture();

    let left = left_runtime
        .inspect_execution_plan(&left_plan, &left_facts)
        .expect("left inspection succeeds");
    let right = left_runtime
        .inspect_execution_plan(&right_plan, &right_facts)
        .expect("right inspection succeeds");

    assert_eq!(left.plan_digest(), right.plan_digest());
    assert_ne!(left.handle_arena_identity(), right.handle_arena_identity());
    assert!(!left
        .lowering_identity()
        .shares_authority_with(right.lowering_identity()));
    assert_eq!(left.nodes().len(), right.nodes().len());
    for (left_node, right_node) in left.nodes().iter().zip(right.nodes()) {
        assert_eq!(left_node.plan_index(), right_node.plan_index());
        assert_eq!(left_node.family(), right_node.family());
        assert_eq!(left_node.child_range(), right_node.child_range());
        assert_eq!(left_node.region_structure(), right_node.region_structure());
        assert_ne!(
            left_node.runtime_handle().arena_identity(),
            right_node.runtime_handle().arena_identity()
        );
    }
    assert_eq!(left.lanes(), right.lanes());
    assert_eq!(left.provenance().len(), right.provenance().len());
    for (left_row, right_row) in left.provenance().iter().zip(right.provenance()) {
        assert_eq!(left_row.plan_index(), right_row.plan_index());
        assert_eq!(left_row.identity_basis(), right_row.identity_basis());
        assert_eq!(left_row.input_family(), right_row.input_family());
        assert_eq!(
            left_row.authored_provenance_digest(),
            right_row.authored_provenance_digest()
        );
        assert_eq!(left_row.source(), right_row.source());
        assert_eq!(
            left_row.capability_reference(),
            right_row.capability_reference()
        );
        match (left_row.query_links(), right_row.query_links()) {
            (Some(left_links), Some(right_links)) => {
                assert_eq!(
                    left_links.binding_identity(),
                    right_links.binding_identity()
                );
                assert_eq!(
                    left_links.settled_fact_link(),
                    right_links.settled_fact_link()
                );
            }
            (None, None) => {}
            _ => panic!("semantic replay must preserve Query link presence"),
        }
    }
    assert_eq!(left.counters(), right.counters());
}

#[test]
fn query_owned_inspection_links_are_preserved_not_reauthored() {
    let (runtime, plan_input, lowering_facts, plan) = inspection_fixture();
    let expected_query_links = expected_query_links_from_plan_input(&plan_input);

    let inspection = runtime
        .inspect_execution_plan(&plan, &lowering_facts)
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
            links.settled_fact_link(),
            expected_links.settled_fact_link()
        );
        assert_eq!(
            links.preservation_receipt(),
            expected_links.preservation_receipt()
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
fn plan_inspection_rejects_mismatched_plan_family_before_provenance() {
    let (runtime, _, lowering_facts, plan) = inspection_fixture();
    let mismatched_plan = plan_with_first_different_family(plan);

    let denial = runtime
        .inspect_execution_plan(&mismatched_plan, &lowering_facts)
        .expect_err("mismatched plan family denies inspection");

    assert_eq!(
        denial.reason(),
        WorthUiPlanInspectionDenialReason::PlanNodeFamilyMismatch
    );
    assert_eq!(denial.counters().denial_count(), 1);
    assert_eq!(denial.counters().node_inspection_count(), 0);
    assert_eq!(denial.counters().provenance_link_count(), 0);
    assert_eq!(denial.counters().artifact_tree_scan_count(), 0);
}

#[test]
fn plan_inspection_rejects_same_shape_foreign_lowering_authority() {
    let (runtime, plan_input, lowering_facts, plan) = inspection_fixture();
    let wrong_provenance_input = plan_input_with_first_identity_basis_changed(plan_input);
    let wrong_provenance_facts = runtime.execution_plan_lowering_facts_below_authority_for_test(
        lowering_facts.committed_input().clone(),
        wrong_provenance_input,
    );

    let denial = runtime
        .inspect_execution_plan(&plan, &wrong_provenance_facts)
        .expect_err("same-shape wrong provenance input denies inspection");

    assert_eq!(
        denial.reason(),
        WorthUiPlanInspectionDenialReason::ForeignLoweringAuthority
    );
    assert_eq!(denial.counters().denial_count(), 1);
    assert_eq!(denial.counters().node_inspection_count(), 0);
    assert_eq!(denial.counters().provenance_link_count(), 0);
    assert_eq!(denial.counters().source_archaeology_count(), 0);
}

fn inspection_fixture() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiExecutionPlanInput,
    crate::runtime::planning::WorthUiExecutionPlanLoweringFacts,
    WorthUiExecutionPlan,
) {
    let inputs = activation_staging_inputs();
    let plan_input = inputs
        .runtime
        .prepare_reconstructive_plan_input_for_test(&inputs.admitted, &[]);
    let (runtime, pending) = inputs.into_runtime_and_pending();
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
    let receipt = runtime.detached_allocation_receipt_for_test(&candidate);
    let lowering_facts = runtime.execution_plan_lowering_facts_below_authority_for_test(
        receipt
            .lowering_input()
            .expect("fresh receipt admits execution lowering"),
        plan_input.clone(),
    );
    let allocation = runtime
        .allocate_runtime_handles(&lowering_facts)
        .expect("handles allocate");
    let plan = runtime
        .assemble_execution_plan_topology(&lowering_facts, &allocation)
        .expect("topology assembles");
    (runtime, plan_input, lowering_facts, plan)
}

fn plan_with_first_different_family(plan: WorthUiExecutionPlan) -> WorthUiExecutionPlan {
    let mut nodes = plan.topology().traversal_order().to_vec();
    let node = nodes[0].clone();
    nodes[0] = WorthUiPlanNode::new(
        node.runtime_handle(),
        WorthUiPlanNodeFamily::from_input_family(alternate_family(node.family().input_family())),
        node.child_range(),
        node.region_structure(),
        node.render_resource_ref(),
    );
    plan.with_test_parts(
        WorthUiPlanTopology::new(nodes, plan.topology().child_ranges().to_vec()),
        plan.lane_partitions().to_vec(),
        plan.lookup_index().clone(),
        plan.counters(),
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
