use std::collections::BTreeMap;

use crate::runtime::{
    WorthUiExecutionPlan, WorthUiPlanChildRange, WorthUiPlanExecutionLane,
    WorthUiPlanLanePartition, WorthUiPlanLookupIndex, WorthUiPlanNodeInput,
    WorthUiPlanNodeInputFamily, WorthUiPlanRegionIdentity, WorthUiPlanRegionStructure,
    WorthUiPlanTopology, WorthUiPlanTopologyCounters, WorthUiPlanTopologyDenial,
    WorthUiPlanTopologyDenialReason, WorthUiRuntimeHandle, WorthUiRuntimeHandleAllocation,
};

use super::validation::denial;

mod regional_successor;
pub(crate) use regional_successor::construct_regional_successor_plan;

pub(crate) fn construct_execution_plan(
    authority: &crate::runtime::planning::WorthUiExecutionPlanLoweringFacts,
    node_inputs: &[WorthUiPlanNodeInput],
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    lane_admission_counters: crate::runtime::WorthUiLaneAdmissionCounters,
    region_successor: Option<super::WorthUiPlanRegionSuccessor>,
    mut counters: WorthUiPlanTopologyCounters,
) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
    let region_successor = match region_successor {
        Some(successor) => successor,
        None => super::WorthUiPlanRegionStore::try_launch(node_inputs.iter().cloned())
            .map_err(|store_denial| denial(topology_reason(store_denial), counters))?,
    };
    let mut lookup_index = WorthUiPlanLookupIndex::new();
    let mut child_ranges = Vec::new();
    let mut nodes = Vec::with_capacity(node_inputs.len());
    let mut lanes = BTreeMap::<WorthUiPlanExecutionLane, Vec<u32>>::new();

    for node_input in node_inputs {
        counters.record_plan_node_input();
        let identity = WorthUiPlanRegionIdentity::from_exact_basis(node_input.identity_basis());
        let region_handle = region_successor
            .store()
            .handle_for(&identity)
            .ok_or_else(|| {
                denial(
                    WorthUiPlanTopologyDenialReason::RegionalSuccessorMismatch,
                    counters,
                )
            })?;
        let plan_index = u32::try_from(region_handle.stable_slot()).map_err(|_| {
            denial(
                WorthUiPlanTopologyDenialReason::RuntimeHandleOutOfBounds,
                counters,
            )
        })?;
        let executable = region_successor
            .store()
            .executable_for(&identity)
            .ok_or_else(|| {
                denial(
                    WorthUiPlanTopologyDenialReason::RegionalSuccessorMismatch,
                    counters,
                )
            })?;
        let family = executable.family();
        let runtime_handle = WorthUiRuntimeHandle::new(
            family,
            plan_index,
            crate::runtime::WorthUiHandleSlotGeneration::new(region_handle.slot_generation()),
            handle_allocation.receipt().arena_identity(),
        );
        let region_structure = region_structure_for_node(executable, counters)?;
        let child_range = child_range_for_node(region_structure, plan_index, counters)?;
        if let Some(range) = child_range {
            child_ranges.push(range);
            counters.record_child_range();
        }
        if executable.has_render_resource() {
            counters.record_render_resource_ref();
        }
        if lookup_index.record(family, plan_index) {
            counters.record_lookup_entry();
        }
        lanes.entry(executable.lane()).or_default().push(plan_index);
        counters.record_topology_node();
        nodes.push(executable.materialize_node(runtime_handle, child_range));
    }

    let lane_partitions = lane_partitions_from(lanes, &mut counters)?;
    let topology = WorthUiPlanTopology::new(nodes, child_ranges);
    let region_storage_counters = region_successor.counters();
    let construction_counters = super::WorthUiPlanConstructionCounters::new(
        authority.plan_input().counters(),
        handle_allocation.counters(),
        lane_admission_counters,
        counters,
        region_storage_counters,
    );
    let regional_evidence =
        super::WorthUiPlanRegionalEvidence::from_lowering(authority, &region_successor);
    let region_store = region_successor.into_store();
    Ok(WorthUiExecutionPlan::new(
        authority,
        super::WorthUiExecutionPlanConstruction {
            handle_receipt: handle_allocation.receipt(),
            topology,
            lane_partitions,
            lookup_index,
            region_store,
            construction_counters,
            regional_evidence,
            counters,
        },
    ))
}

fn topology_reason(
    denial: super::region::WorthUiPlanRegionStoreDenial,
) -> WorthUiPlanTopologyDenialReason {
    match denial {
        super::region::WorthUiPlanRegionStoreDenial::HandleCapacity(exhaustion) => {
            WorthUiPlanTopologyDenialReason::HandleCapacityExhausted(exhaustion)
        }
        super::region::WorthUiPlanRegionStoreDenial::MissingLinkedRegion => {
            WorthUiPlanTopologyDenialReason::MissingChildOrLaneLink
        }
        super::region::WorthUiPlanRegionStoreDenial::DuplicateRegionIdentity => {
            WorthUiPlanTopologyDenialReason::DuplicateRegionIdentity
        }
        super::region::WorthUiPlanRegionStoreDenial::OrdinaryMeaningFamilyMismatch => {
            WorthUiPlanTopologyDenialReason::OrdinaryMeaningFamilyMismatch
        }
        super::region::WorthUiPlanRegionStoreDenial::SpatialMeaningFamilyMismatch => {
            WorthUiPlanTopologyDenialReason::SpatialMeaningFamilyMismatch
        }
        super::region::WorthUiPlanRegionStoreDenial::RealtimeMeaningFamilyMismatch => {
            WorthUiPlanTopologyDenialReason::RealtimeMeaningFamilyMismatch
        }
        super::region::WorthUiPlanRegionStoreDenial::QueryBindingFactsMismatch => {
            WorthUiPlanTopologyDenialReason::QueryBindingFactsMismatch
        }
        super::region::WorthUiPlanRegionStoreDenial::DuplicateChildTarget => {
            WorthUiPlanTopologyDenialReason::DuplicateChildTarget
        }
        super::region::WorthUiPlanRegionStoreDenial::OverlappingChildTarget => {
            WorthUiPlanTopologyDenialReason::OverlappingChildTarget
        }
        super::region::WorthUiPlanRegionStoreDenial::CyclicRegionDependency => {
            WorthUiPlanTopologyDenialReason::CyclicRegionDependency
        }
        super::region::WorthUiPlanRegionStoreDenial::OwnerManifestMismatch => {
            WorthUiPlanTopologyDenialReason::OwnerManifestMismatch
        }
        super::region::WorthUiPlanRegionStoreDenial::IncompleteSuccessor => {
            WorthUiPlanTopologyDenialReason::IncompleteRegionalSuccessor
        }
    }
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
    executable: &super::WorthUiPlanRegionExecutable,
    counters: WorthUiPlanTopologyCounters,
) -> Result<Option<WorthUiPlanRegionStructure>, WorthUiPlanTopologyDenial> {
    let region_structure = executable.region_structure();
    if family_requires_region_structure(executable) && region_structure.is_none() {
        return Err(denial(
            WorthUiPlanTopologyDenialReason::MissingRegionStructure,
            counters,
        ));
    }
    Ok(region_structure)
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

fn family_requires_region_structure(executable: &super::WorthUiPlanRegionExecutable) -> bool {
    executable.family() == WorthUiPlanNodeInputFamily::QueryViewBinding
        || (executable.is_root_shell()
            && matches!(
                executable.family(),
                WorthUiPlanNodeInputFamily::ComponentInvocation
                    | WorthUiPlanNodeInputFamily::LayoutRegion
            ))
}
