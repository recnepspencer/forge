use super::plan_topology_test_support::*;
use crate::runtime::{
    WorthUiChildRangeHandle, WorthUiEguiBoundaryContact, WorthUiEguiBoundaryInput,
    WorthUiExecutionPlanInput, WorthUiPlanNodeInputFamily, WorthUiPlanTopologyDenialReason,
};

#[test]
fn equivalent_plan_inputs_assemble_equivalent_topology() {
    let (_, _, left_planning, left_handles) = topology_fixture();
    let (_, _, right_planning, right_handles) = topology_fixture();
    let left = assemble(&left_planning, &left_handles);
    let right = assemble(&right_planning, &right_handles);

    assert_eq!(left.handle_receipt(), right.handle_receipt());
    assert_eq!(left.topology(), right.topology());
    assert_eq!(left.lane_partitions(), right.lane_partitions());
    assert_eq!(left.lookup_index(), right.lookup_index());
    assert_eq!(left.counters(), right.counters());
}

#[test]
fn plan_topology_assembly_rejects_missing_child_or_lane_links() {
    let (_, _plan_input, planning, allocation) = topology_fixture();
    let mut runtime_handles = allocation.runtime_handles().to_vec();
    runtime_handles.pop();
    let broken = allocation_with_runtime_handles(&allocation, runtime_handles);

    let denial = assemble_err(&planning, &broken);

    assert_eq!(
        denial.reason(),
        WorthUiPlanTopologyDenialReason::MissingRuntimeHandle
    );
    assert_eq!(denial.counters().denial_count(), 1);
    assert!(denial.counters().topology_node_count() == 0);
}

#[test]
fn frame_traversal_uses_plan_topology_without_artifact_tree_scan() {
    let (_, plan_input, planning, allocation) = topology_fixture();
    let plan = assemble(&planning, &allocation);
    let counters = plan.counters();

    assert_eq!(
        plan.topology().traversal_order().len(),
        plan_input.node_inputs().len()
    );
    assert!(plan
        .lane_partitions()
        .iter()
        .all(|lane| lane.node_count() > 0));
    assert_eq!(counters.artifact_tree_scan_count(), 0);
    assert_eq!(counters.registry_string_lookup_count(), 0);
    assert_eq!(counters.broad_registry_scan_count(), 0);
    assert_eq!(counters.topology_validation_count(), 4);

    let region_nodes = plan
        .topology()
        .traversal_order()
        .iter()
        .filter_map(|node| node.region_structure())
        .collect::<Vec<_>>();
    assert!(!region_nodes.is_empty());
    assert!(region_nodes
        .iter()
        .all(|structure| structure.structure_declared()));
    assert_eq!(
        plan.topology().child_ranges().len(),
        counters.child_range_count()
    );
    assert!(plan
        .topology()
        .child_ranges()
        .iter()
        .all(|range| range.len() > 0));
    for (node_input, topology_node) in plan_input
        .node_inputs()
        .iter()
        .zip(plan.topology().traversal_order())
    {
        if let Some(region_structure) = topology_node.region_structure() {
            assert_eq!(
                region_structure.root_region_count(),
                node_input.topology_input().root_region_count()
            );
            assert_eq!(
                region_structure.region_count(),
                node_input.topology_input().region_count()
            );
            assert_eq!(
                region_structure.mount_count(),
                node_input.topology_input().mount_count()
            );
            assert_eq!(
                region_structure.max_region_depth(),
                node_input.topology_input().max_region_depth()
            );
            assert_eq!(
                topology_node.child_range().map(|range| range.len()),
                (region_structure.root_region_count() > 0)
                    .then_some(region_structure.root_region_count() as u32)
            );
        }
    }
}

#[test]
fn plan_topology_rejects_missing_child_range_handles() {
    let (_, plan_input, _, _) = topology_fixture();
    let plan_input = plan_input_with_first_child_range_family(plan_input);
    let allocation_planning = topology_planning(&plan_input, "plan-topology.missing-child-range");
    let allocation = allocate_handles(&allocation_planning);
    assert!(!allocation.child_range_handles().is_empty());
    let broken = allocation_with_child_ranges(&allocation, Vec::new());

    let denial = assemble_err(&allocation_planning, &broken);

    assert_eq!(
        denial.reason(),
        WorthUiPlanTopologyDenialReason::MissingChildOrLaneLink
    );
    assert_eq!(denial.counters().denial_count(), 1);
}

#[test]
fn plan_topology_rejects_orphaned_child_range_handles() {
    let (_, plan_input, _, _) = topology_fixture();
    let plan_input = plan_input_with_first_child_range_family(plan_input);
    let allocation_planning = topology_planning(&plan_input, "plan-topology.orphaned-child-range");
    let allocation = allocate_handles(&allocation_planning);
    let mut child_ranges = allocation.child_range_handles().to_vec();
    child_ranges[0] =
        WorthUiChildRangeHandle::new(u32::MAX, allocation.receipt().plan_generation());
    let broken = allocation_with_child_ranges(&allocation, child_ranges);

    let denial = assemble_err(&allocation_planning, &broken);

    assert_eq!(
        denial.reason(),
        WorthUiPlanTopologyDenialReason::OrphanedChildRangeHandle
    );
    assert_eq!(denial.counters().denial_count(), 1);
}

fn plan_input_with_first_child_range_family(
    plan_input: WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    let mut node_inputs = plan_input.node_inputs().to_vec();
    let index = node_inputs
        .iter()
        .position(|input| input.family() != WorthUiPlanNodeInputFamily::ChildRange)
        .expect("fixture includes a non-child-range plan input");
    node_inputs[index] = node_inputs[index]
        .clone()
        .with_family_for_test(WorthUiPlanNodeInputFamily::ChildRange);
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}

#[test]
fn plan_topology_rejects_missing_declared_region_structure() {
    let (_, plan_input, _, _) = topology_fixture();
    let broken_input = plan_input_without_first_region_structure(plan_input);
    let broken_planning =
        topology_planning(&broken_input, "plan-topology.missing-region-structure");
    let broken_allocation = allocate_handles(&broken_planning);

    let denial = assemble_err(&broken_planning, &broken_allocation);

    assert_eq!(
        denial.reason(),
        WorthUiPlanTopologyDenialReason::MissingRegionStructure
    );
    assert_eq!(denial.counters().denial_count(), 1);
}

#[test]
fn egui_boundary_contact_is_plan_declared_not_ambient() {
    let (_, plan_input, planning, allocation) = topology_fixture();
    let plan = assemble(&planning, &allocation);
    let egui_nodes = plan
        .topology()
        .traversal_order()
        .iter()
        .filter(|node| node.egui_boundary().is_some())
        .count();

    assert!(egui_nodes > 0);
    assert_eq!(plan.counters().egui_boundary_count(), egui_nodes);
    assert_eq!(plan.counters().ambient_egui_access_count(), 0);
    assert!(plan
        .topology()
        .traversal_order()
        .iter()
        .filter_map(|node| node.egui_boundary())
        .all(|boundary| boundary.contacts() == expected_contacts(boundary.input())));

    let broken_input = plan_input_without_first_egui_boundary(plan_input);
    let broken_planning = topology_planning(&broken_input, "plan-topology.missing-egui-boundary");
    let broken_allocation = allocate_handles(&broken_planning);
    let denial = assemble_err(&broken_planning, &broken_allocation);
    assert_eq!(
        denial.reason(),
        WorthUiPlanTopologyDenialReason::MissingEguiBoundaryDeclaration
    );
}

fn expected_contacts(input: WorthUiEguiBoundaryInput) -> &'static [WorthUiEguiBoundaryContact] {
    match input {
        WorthUiEguiBoundaryInput::Component => &[
            WorthUiEguiBoundaryContact::Context,
            WorthUiEguiBoundaryContact::Ui,
            WorthUiEguiBoundaryContact::Response,
            WorthUiEguiBoundaryContact::Id,
            WorthUiEguiBoundaryContact::Input,
            WorthUiEguiBoundaryContact::FrameTiming,
        ],
        WorthUiEguiBoundaryInput::Surface => &[
            WorthUiEguiBoundaryContact::Context,
            WorthUiEguiBoundaryContact::Ui,
            WorthUiEguiBoundaryContact::LayoutAllocation,
            WorthUiEguiBoundaryContact::PaintSubmission,
            WorthUiEguiBoundaryContact::MemoryStateBridge,
            WorthUiEguiBoundaryContact::FrameTiming,
        ],
        WorthUiEguiBoundaryInput::QueryBinding => &[
            WorthUiEguiBoundaryContact::Context,
            WorthUiEguiBoundaryContact::Response,
            WorthUiEguiBoundaryContact::Input,
            WorthUiEguiBoundaryContact::MemoryStateBridge,
        ],
        WorthUiEguiBoundaryInput::Token => &[
            WorthUiEguiBoundaryContact::Context,
            WorthUiEguiBoundaryContact::PaintSubmission,
        ],
        WorthUiEguiBoundaryInput::Diagnostics => &[
            WorthUiEguiBoundaryContact::Context,
            WorthUiEguiBoundaryContact::Ui,
            WorthUiEguiBoundaryContact::FrameTiming,
        ],
    }
}
