use forge_query::facade::runtime::{
    ForgeQueryGraphReadAccessExecutionCounters, ForgeQueryGraphReadAccessRequirementCounters,
    ForgeQueryGraphReadCostEstimateCounters,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueryGraphReadCostCounterField {
    RequirementRowCount,
    DirectionalAdjacencyCount,
    ReverseAdjacencyCount,
    TraversalWorksetCount,
    ResultBufferCount,
    MaterializationLifecycleCount,
    LiveMaintenanceSupportCount,
    DomainOperationCapabilityRegistrationCount,
    ExecutorEntryCount,
    StrategyRecomputeCount,
    EphemeralIndexAllocationCount,
    AccessExecutionEdgeScanCount,
    PerResultNeighborLookupCount,
    PersistentArtifactBypassCount,
    MaterializedRowCount,
    CostEstimateRequirementRowCount,
    EstimatedRelationRowCount,
    EstimatedWorksetRowCount,
    EstimatedBufferRowCount,
    CostEstimateEdgeScanCount,
    AccessBufferAllocationCount,
}

impl QueryGraphReadCostCounterField {
    pub const ALL: [Self; 21] = [
        Self::RequirementRowCount,
        Self::DirectionalAdjacencyCount,
        Self::ReverseAdjacencyCount,
        Self::TraversalWorksetCount,
        Self::ResultBufferCount,
        Self::MaterializationLifecycleCount,
        Self::LiveMaintenanceSupportCount,
        Self::DomainOperationCapabilityRegistrationCount,
        Self::ExecutorEntryCount,
        Self::StrategyRecomputeCount,
        Self::EphemeralIndexAllocationCount,
        Self::AccessExecutionEdgeScanCount,
        Self::PerResultNeighborLookupCount,
        Self::PersistentArtifactBypassCount,
        Self::MaterializedRowCount,
        Self::CostEstimateRequirementRowCount,
        Self::EstimatedRelationRowCount,
        Self::EstimatedWorksetRowCount,
        Self::EstimatedBufferRowCount,
        Self::CostEstimateEdgeScanCount,
        Self::AccessBufferAllocationCount,
    ];

    pub fn query_label(&self) -> &'static str {
        match self {
            Self::RequirementRowCount => "requirement_counters.row_count",
            Self::DirectionalAdjacencyCount => "requirement_counters.directional_adjacency_count",
            Self::ReverseAdjacencyCount => "requirement_counters.reverse_adjacency_count",
            Self::TraversalWorksetCount => "requirement_counters.traversal_workset_count",
            Self::ResultBufferCount => "requirement_counters.buffer_count",
            Self::MaterializationLifecycleCount => {
                "requirement_counters.materialization_lifecycle_count"
            }
            Self::LiveMaintenanceSupportCount => {
                "requirement_counters.live_maintenance_support_count"
            }
            Self::DomainOperationCapabilityRegistrationCount => {
                "requirement_counters.domain_operation_capability_registration_count"
            }
            Self::ExecutorEntryCount => "access_execution_counters.executor_entry_count",
            Self::StrategyRecomputeCount => "access_execution_counters.strategy_recompute_count",
            Self::EphemeralIndexAllocationCount => {
                "access_execution_counters.ephemeral_index_allocation_count"
            }
            Self::AccessExecutionEdgeScanCount => "access_execution_counters.edge_scan_count",
            Self::PerResultNeighborLookupCount => {
                "access_execution_counters.per_result_neighbor_lookup_count"
            }
            Self::PersistentArtifactBypassCount => {
                "access_execution_counters.persistent_artifact_bypass_count"
            }
            Self::MaterializedRowCount => "access_execution_counters.materialized_row_count",
            Self::CostEstimateRequirementRowCount => "cost_estimate_counters.requirement_row_count",
            Self::EstimatedRelationRowCount => {
                "cost_estimate_counters.estimated_relation_row_count"
            }
            Self::EstimatedWorksetRowCount => "cost_estimate_counters.estimated_workset_row_count",
            Self::EstimatedBufferRowCount => "cost_estimate_counters.estimated_buffer_row_count",
            Self::CostEstimateEdgeScanCount => "cost_estimate_counters.edge_scan_count",
            Self::AccessBufferAllocationCount => {
                "cost_estimate_counters.access_buffer_allocation_count"
            }
        }
    }
}

pub(crate) fn anchor_query_graph_read_cost_counter_accessors() {
    let _: fn(&ForgeQueryGraphReadAccessRequirementCounters) -> usize =
        ForgeQueryGraphReadAccessRequirementCounters::row_count;
    let _: fn(&ForgeQueryGraphReadAccessRequirementCounters) -> usize =
        ForgeQueryGraphReadAccessRequirementCounters::directional_adjacency_count;
    let _: fn(&ForgeQueryGraphReadAccessRequirementCounters) -> usize =
        ForgeQueryGraphReadAccessRequirementCounters::reverse_adjacency_count;
    let _: fn(&ForgeQueryGraphReadAccessRequirementCounters) -> usize =
        ForgeQueryGraphReadAccessRequirementCounters::traversal_workset_count;
    let _: fn(&ForgeQueryGraphReadAccessRequirementCounters) -> usize =
        ForgeQueryGraphReadAccessRequirementCounters::buffer_count;
    let _: fn(&ForgeQueryGraphReadAccessRequirementCounters) -> usize =
        ForgeQueryGraphReadAccessRequirementCounters::materialization_lifecycle_count;
    let _: fn(&ForgeQueryGraphReadAccessRequirementCounters) -> usize =
        ForgeQueryGraphReadAccessRequirementCounters::live_maintenance_support_count;
    let _: fn(&ForgeQueryGraphReadAccessRequirementCounters) -> usize =
        ForgeQueryGraphReadAccessRequirementCounters::domain_operation_capability_registration_count;

    let _: fn(&ForgeQueryGraphReadAccessExecutionCounters) -> usize =
        ForgeQueryGraphReadAccessExecutionCounters::executor_entry_count;
    let _: fn(&ForgeQueryGraphReadAccessExecutionCounters) -> usize =
        ForgeQueryGraphReadAccessExecutionCounters::strategy_recompute_count;
    let _: fn(&ForgeQueryGraphReadAccessExecutionCounters) -> usize =
        ForgeQueryGraphReadAccessExecutionCounters::ephemeral_index_allocation_count;
    let _: fn(&ForgeQueryGraphReadAccessExecutionCounters) -> usize =
        ForgeQueryGraphReadAccessExecutionCounters::edge_scan_count;
    let _: fn(&ForgeQueryGraphReadAccessExecutionCounters) -> usize =
        ForgeQueryGraphReadAccessExecutionCounters::per_result_neighbor_lookup_count;
    let _: fn(&ForgeQueryGraphReadAccessExecutionCounters) -> usize =
        ForgeQueryGraphReadAccessExecutionCounters::persistent_artifact_bypass_count;
    let _: fn(&ForgeQueryGraphReadAccessExecutionCounters) -> usize =
        ForgeQueryGraphReadAccessExecutionCounters::materialized_row_count;

    let _: fn(&ForgeQueryGraphReadCostEstimateCounters) -> usize =
        ForgeQueryGraphReadCostEstimateCounters::requirement_row_count;
    let _: fn(&ForgeQueryGraphReadCostEstimateCounters) -> usize =
        ForgeQueryGraphReadCostEstimateCounters::estimated_relation_row_count;
    let _: fn(&ForgeQueryGraphReadCostEstimateCounters) -> usize =
        ForgeQueryGraphReadCostEstimateCounters::estimated_workset_row_count;
    let _: fn(&ForgeQueryGraphReadCostEstimateCounters) -> usize =
        ForgeQueryGraphReadCostEstimateCounters::estimated_buffer_row_count;
    let _: fn(&ForgeQueryGraphReadCostEstimateCounters) -> usize =
        ForgeQueryGraphReadCostEstimateCounters::edge_scan_count;
    let _: fn(&ForgeQueryGraphReadCostEstimateCounters) -> usize =
        ForgeQueryGraphReadCostEstimateCounters::access_buffer_allocation_count;
}
