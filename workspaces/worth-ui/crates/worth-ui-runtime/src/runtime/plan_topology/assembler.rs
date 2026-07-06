use std::collections::BTreeMap;

use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiEguiPlanBoundary, WorthUiExecutionLaneDescriptor,
    WorthUiExecutionPlan, WorthUiLaneAdmission, WorthUiPlanChildRange, WorthUiPlanExecutionLane,
    WorthUiPlanLanePartition, WorthUiPlanLookupIndex, WorthUiPlanNode, WorthUiPlanNodeFamily,
    WorthUiPlanNodeInput, WorthUiPlanNodeInputFamily, WorthUiPlanRegionStructure,
    WorthUiPlanTopology, WorthUiPlanTopologyCounters, WorthUiPlanTopologyDenial,
    WorthUiPlanTopologyDenialReason, WorthUiRenderResourceRef, WorthUiRuntimeHandle,
    WorthUiRuntimeHandleAllocation, WorthUiRuntimeHandleAllocationBasis,
};

pub(crate) struct WorthUiPlanTopologyAssembler;

impl WorthUiPlanTopologyAssembler {
    pub(crate) fn assemble_with_lane_admission(
        allocation_planning: &WorthUiAllocationPlanning,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
        let mut counters = WorthUiPlanTopologyCounters::default();
        if !allocation_planning.is_admitted() {
            return Err(denial(
                WorthUiPlanTopologyDenialReason::AllocationPlanningDenied,
                counters,
            ));
        }
        let node_inputs = allocation_planning
            .node_inputs()
            .expect("admitted allocation planning must expose lowered node inputs");
        validate_receipt(allocation_planning, handle_allocation, &mut counters)?;
        validate_lane_admission(
            node_inputs,
            lane_admission,
            handle_allocation.receipt().basis_digest(),
            &mut counters,
        )?;
        validate_runtime_handles(node_inputs, handle_allocation, &mut counters)?;
        validate_child_range_handles(node_inputs, handle_allocation, &mut counters)?;
        assemble_validated_topology(node_inputs, handle_allocation, counters)
    }
}

fn assemble_validated_topology(
    node_inputs: &[WorthUiPlanNodeInput],
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    mut counters: WorthUiPlanTopologyCounters,
) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
    let mut lookup_index = WorthUiPlanLookupIndex::new();
    let mut child_ranges = Vec::new();
    let mut nodes = Vec::with_capacity(node_inputs.len());
    let mut lanes = BTreeMap::<WorthUiPlanExecutionLane, Vec<u32>>::new();

    for (position, node_input) in node_inputs.iter().enumerate() {
        counters.record_plan_node_input();
        let runtime_handle = handle_allocation.runtime_handles()[position];
        let plan_index = runtime_handle.plan_index();
        let family = node_input.family();
        let region_structure = region_structure_for_node(node_input, counters)?;
        let child_range = child_range_for_node(region_structure, plan_index, counters)?;
        if let Some(range) = child_range {
            child_ranges.push(range);
            counters.record_child_range();
        }
        let egui_boundary = egui_boundary_for_node(node_input, plan_index, counters)?;
        if egui_boundary.is_some() {
            counters.record_egui_boundary();
        }
        let render_resource_ref = render_resource_for_node(family, runtime_handle);
        if render_resource_ref.is_some() {
            counters.record_render_resource_ref();
        }
        if lookup_index.record(family, plan_index) {
            counters.record_lookup_entry();
        }
        lanes
            .entry(lane_for_family(family))
            .or_default()
            .push(plan_index);
        counters.record_topology_node();
        nodes.push(WorthUiPlanNode::new(
            runtime_handle,
            WorthUiPlanNodeFamily::from_input_family(family),
            child_range,
            region_structure,
            egui_boundary,
            render_resource_ref,
        ));
    }

    let lane_partitions = lane_partitions_from(lanes, &mut counters)?;
    let topology = WorthUiPlanTopology::new(nodes, child_ranges);
    Ok(WorthUiExecutionPlan::new(
        handle_allocation.receipt(),
        topology,
        lane_partitions,
        lookup_index,
        counters,
    ))
}

fn validate_lane_admission(
    node_inputs: &[WorthUiPlanNodeInput],
    lane_admission: &WorthUiLaneAdmission,
    expected_basis_digest: u64,
    counters: &mut WorthUiPlanTopologyCounters,
) -> Result<(), WorthUiPlanTopologyDenial> {
    counters.record_validation();
    if lane_admission.plan_input_basis_digest() != expected_basis_digest {
        return Err(denial(
            WorthUiPlanTopologyDenialReason::LaneAdmissionMismatch,
            *counters,
        ));
    }
    if lane_admission.counters().plan_node_count() != node_inputs.len() {
        return Err(denial(
            WorthUiPlanTopologyDenialReason::LaneAdmissionMismatch,
            *counters,
        ));
    }
    for node_input in node_inputs {
        let descriptor = WorthUiExecutionLaneDescriptor::from_node_input(node_input);
        if !lane_admission.includes_lane(descriptor.lane()) {
            return Err(denial(
                WorthUiPlanTopologyDenialReason::LaneAdmissionMismatch,
                *counters,
            ));
        }
    }
    Ok(())
}

fn validate_receipt(
    allocation_planning: &WorthUiAllocationPlanning,
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    counters: &mut WorthUiPlanTopologyCounters,
) -> Result<(), WorthUiPlanTopologyDenial> {
    counters.record_validation();
    let basis = WorthUiRuntimeHandleAllocationBasis::from_allocation_planning(allocation_planning);
    if handle_allocation.receipt().certifies_basis(&basis) {
        Ok(())
    } else {
        Err(denial(
            WorthUiPlanTopologyDenialReason::HandleAllocationReceiptMismatch,
            *counters,
        ))
    }
}

fn validate_runtime_handles(
    node_inputs: &[WorthUiPlanNodeInput],
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    counters: &mut WorthUiPlanTopologyCounters,
) -> Result<(), WorthUiPlanTopologyDenial> {
    counters.record_validation();
    if handle_allocation.runtime_handles().len() != node_inputs.len() {
        return Err(denial(
            WorthUiPlanTopologyDenialReason::MissingRuntimeHandle,
            *counters,
        ));
    }
    let generation = handle_allocation.receipt().plan_generation();
    for (position, (node_input, runtime_handle)) in node_inputs
        .iter()
        .zip(handle_allocation.runtime_handles())
        .enumerate()
    {
        let plan_index = u32::try_from(position).map_err(|_| {
            denial(
                WorthUiPlanTopologyDenialReason::RuntimeHandleOutOfBounds,
                *counters,
            )
        })?;
        if runtime_handle.plan_index() != plan_index
            || runtime_handle.plan_generation() != generation
        {
            return Err(denial(
                WorthUiPlanTopologyDenialReason::RuntimeHandleOutOfBounds,
                *counters,
            ));
        }
        if runtime_handle.family() != node_input.family() {
            return Err(denial(
                WorthUiPlanTopologyDenialReason::RuntimeHandleFamilyMismatch,
                *counters,
            ));
        }
    }
    Ok(())
}

fn validate_child_range_handles(
    node_inputs: &[WorthUiPlanNodeInput],
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    counters: &mut WorthUiPlanTopologyCounters,
) -> Result<(), WorthUiPlanTopologyDenial> {
    counters.record_validation();
    let generation = handle_allocation.receipt().plan_generation();
    let expected_child_range_plan_indexes =
        expected_child_range_plan_indexes(node_inputs, counters)?;
    if expected_child_range_plan_indexes.len() != handle_allocation.child_range_handles().len() {
        return Err(denial(
            WorthUiPlanTopologyDenialReason::MissingChildOrLaneLink,
            *counters,
        ));
    }
    for (expected_plan_index, handle) in expected_child_range_plan_indexes
        .iter()
        .zip(handle_allocation.child_range_handles())
    {
        let plan_index = handle.plan_index() as usize;
        let Some(node_input) = node_inputs.get(plan_index) else {
            return Err(denial(
                WorthUiPlanTopologyDenialReason::OrphanedChildRangeHandle,
                *counters,
            ));
        };
        if handle.plan_index() != *expected_plan_index
            || handle.plan_generation() != generation
            || node_input.family() != WorthUiPlanNodeInputFamily::ChildRange
        {
            return Err(denial(
                WorthUiPlanTopologyDenialReason::OrphanedChildRangeHandle,
                *counters,
            ));
        }
    }
    Ok(())
}

fn expected_child_range_plan_indexes(
    node_inputs: &[WorthUiPlanNodeInput],
    counters: &mut WorthUiPlanTopologyCounters,
) -> Result<Vec<u32>, WorthUiPlanTopologyDenial> {
    node_inputs
        .iter()
        .enumerate()
        .filter_map(|(position, node_input)| {
            matches!(node_input.family(), WorthUiPlanNodeInputFamily::ChildRange)
                .then_some(position)
        })
        .map(|position| {
            u32::try_from(position).map_err(|_| {
                denial(
                    WorthUiPlanTopologyDenialReason::RuntimeHandleOutOfBounds,
                    *counters,
                )
            })
        })
        .collect()
}

fn child_range_for_node(
    region_structure: Option<WorthUiPlanRegionStructure>,
    plan_index: u32,
    counters: WorthUiPlanTopologyCounters,
) -> Result<Option<WorthUiPlanChildRange>, WorthUiPlanTopologyDenial> {
    let Some(region_structure) = region_structure else {
        return Ok(None);
    };
    let root_region_count = u32::try_from(region_structure.root_region_count()).map_err(|_| {
        denial(
            WorthUiPlanTopologyDenialReason::RuntimeHandleOutOfBounds,
            counters,
        )
    })?;
    Ok((root_region_count > 0)
        .then(|| WorthUiPlanChildRange::from_root_region_count(plan_index, root_region_count)))
}

fn region_structure_for_node(
    node_input: &WorthUiPlanNodeInput,
    counters: WorthUiPlanTopologyCounters,
) -> Result<Option<WorthUiPlanRegionStructure>, WorthUiPlanTopologyDenial> {
    let region_structure =
        WorthUiPlanRegionStructure::from_topology_input(node_input.topology_input());
    if family_requires_region_structure(node_input.family()) && region_structure.is_none() {
        return Err(denial(
            WorthUiPlanTopologyDenialReason::MissingRegionStructure,
            counters,
        ));
    }
    Ok(region_structure)
}

fn egui_boundary_for_node(
    node_input: &WorthUiPlanNodeInput,
    plan_index: u32,
    counters: WorthUiPlanTopologyCounters,
) -> Result<Option<WorthUiEguiPlanBoundary>, WorthUiPlanTopologyDenial> {
    if family_requires_egui(node_input.family()) && node_input.egui_boundary_input().is_none() {
        return Err(denial(
            WorthUiPlanTopologyDenialReason::MissingEguiBoundaryDeclaration,
            counters,
        ));
    }
    Ok(node_input
        .egui_boundary_input()
        .map(|input| WorthUiEguiPlanBoundary::new(input, plan_index)))
}

fn render_resource_for_node(
    family: WorthUiPlanNodeInputFamily,
    runtime_handle: WorthUiRuntimeHandle,
) -> Option<WorthUiRenderResourceRef> {
    match family {
        WorthUiPlanNodeInputFamily::RenderResourceRef => Some(WorthUiRenderResourceRef::new(
            runtime_handle.plan_index(),
            runtime_handle.plan_generation(),
        )),
        _ => None,
    }
}

fn lane_partitions_from(
    lanes: BTreeMap<WorthUiPlanExecutionLane, Vec<u32>>,
    counters: &mut WorthUiPlanTopologyCounters,
) -> Result<Vec<WorthUiPlanLanePartition>, WorthUiPlanTopologyDenial> {
    if lanes.values().any(Vec::is_empty) {
        return Err(denial(
            WorthUiPlanTopologyDenialReason::MissingChildOrLaneLink,
            *counters,
        ));
    }
    let mut partitions = Vec::with_capacity(lanes.len());
    for (lane, plan_indexes) in lanes {
        counters.record_lane_partition();
        partitions.push(WorthUiPlanLanePartition::new(lane, plan_indexes));
    }
    Ok(partitions)
}

fn lane_for_family(family: WorthUiPlanNodeInputFamily) -> WorthUiPlanExecutionLane {
    match family {
        WorthUiPlanNodeInputFamily::ComponentInvocation
        | WorthUiPlanNodeInputFamily::LayoutRegion
        | WorthUiPlanNodeInputFamily::ChildRange => WorthUiPlanExecutionLane::UiStructure,
        WorthUiPlanNodeInputFamily::QueryViewBinding => WorthUiPlanExecutionLane::QueryView,
        WorthUiPlanNodeInputFamily::Command => WorthUiPlanExecutionLane::Command,
        WorthUiPlanNodeInputFamily::TokenStyle => WorthUiPlanExecutionLane::Style,
        WorthUiPlanNodeInputFamily::Accessibility | WorthUiPlanNodeInputFamily::DiagnosticsRef => {
            WorthUiPlanExecutionLane::Diagnostics
        }
        WorthUiPlanNodeInputFamily::LanePartitionRef => WorthUiPlanExecutionLane::LaneBoundary,
        WorthUiPlanNodeInputFamily::EguiBoundaryRef => WorthUiPlanExecutionLane::EguiBoundary,
        WorthUiPlanNodeInputFamily::RenderResourceRef => WorthUiPlanExecutionLane::RenderResource,
    }
}

fn family_requires_egui(family: WorthUiPlanNodeInputFamily) -> bool {
    matches!(
        family,
        WorthUiPlanNodeInputFamily::ComponentInvocation
            | WorthUiPlanNodeInputFamily::LayoutRegion
            | WorthUiPlanNodeInputFamily::QueryViewBinding
            | WorthUiPlanNodeInputFamily::TokenStyle
            | WorthUiPlanNodeInputFamily::DiagnosticsRef
            | WorthUiPlanNodeInputFamily::EguiBoundaryRef
    )
}

fn family_requires_region_structure(family: WorthUiPlanNodeInputFamily) -> bool {
    matches!(
        family,
        WorthUiPlanNodeInputFamily::ComponentInvocation
            | WorthUiPlanNodeInputFamily::LayoutRegion
            | WorthUiPlanNodeInputFamily::QueryViewBinding
    )
}

fn denial(
    reason: WorthUiPlanTopologyDenialReason,
    counters: WorthUiPlanTopologyCounters,
) -> WorthUiPlanTopologyDenial {
    WorthUiPlanTopologyDenial::new(reason, counters)
}
