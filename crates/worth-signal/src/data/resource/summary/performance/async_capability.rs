use super::{
    ResourceBoundaryKind, ResourceBoundaryPerformanceEnvelope, ResourceCostContractId,
    ResourceCostPosture, ResourceDensityStrategy,
};

impl ResourceBoundaryPerformanceEnvelope {
    pub(crate) fn async_node_gate_state(
        upstream_dependency_count: u32,
        downstream_subscriber_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::AsyncNodeGateState,
            upstream_dependency_count.saturating_add(downstream_subscriber_count),
            0,
            1,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(22),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn async_node_hierarchy_replay(
        hierarchy_width: u32,
        in_flight_width: u32,
        hierarchy_depth: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::AsyncNodeHierarchyReplay,
            hierarchy_width,
            hierarchy_depth,
            in_flight_width,
            0,
            0,
            0,
            0,
            0,
            0,
            hierarchy_width,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(23),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn async_node_hierarchy_cancellation(
        hierarchy_width: u32,
        propagated_width: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::AsyncNodeHierarchyCancellation,
            hierarchy_width,
            propagated_width.saturating_add(1),
            1,
            propagated_width,
            0,
            0,
            0,
            0,
            propagated_width.saturating_add(1),
            0,
            1,
            ResourceDensityStrategy::NotApplicable,
            ResourceCostContractId::new(24),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn async_node_historical_parity(
        branch_restore_width: u32,
        diagnostics_allocation_count: u32,
        denied_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::AsyncNodeHistoricalParity,
            1,
            0,
            u32::from(denied_count == 0),
            denied_count,
            denied_count,
            0,
            0,
            2,
            branch_restore_width,
            diagnostics_allocation_count,
            1,
            ResourceDensityStrategy::SparseIndexedLookup,
            ResourceCostContractId::new(27),
            ResourceCostPosture::Verified,
        )
    }

    pub(crate) fn async_node_capability_equivalence(
        branch_restore_width: u32,
        observation_width: u32,
        diagnostics_allocation_count: u32,
        denied_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::AsyncNodeCapabilityEquivalence,
            1,
            0,
            u32::from(denied_count == 0),
            denied_count,
            denied_count,
            0,
            0,
            2,
            branch_restore_width,
            diagnostics_allocation_count,
            1,
            ResourceDensityStrategy::SparseIndexedLookup,
            ResourceCostContractId::new(28),
            ResourceCostPosture::Verified,
        )
        .with_output_continuity_classification_width(observation_width)
    }

    pub(crate) fn async_keyed_node_historical_parity(
        branch_restore_width: u32,
        observation_width: u32,
        diagnostics_allocation_count: u32,
        denied_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::AsyncKeyedNodeHistoricalParity,
            1,
            0,
            u32::from(denied_count == 0),
            denied_count,
            denied_count,
            0,
            0,
            2,
            branch_restore_width,
            diagnostics_allocation_count,
            1,
            ResourceDensityStrategy::SparseIndexedLookup,
            ResourceCostContractId::new(29),
            ResourceCostPosture::Verified,
        )
        .with_output_continuity_classification_width(observation_width)
    }

    pub(crate) fn async_keyed_node_capability_equivalence(
        branch_restore_width: u32,
        observation_width: u32,
        diagnostics_allocation_count: u32,
        denied_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::AsyncKeyedNodeCapabilityEquivalence,
            1,
            0,
            u32::from(denied_count == 0),
            denied_count,
            denied_count,
            0,
            0,
            2,
            branch_restore_width,
            diagnostics_allocation_count,
            1,
            ResourceDensityStrategy::SparseIndexedLookup,
            ResourceCostContractId::new(30),
            ResourceCostPosture::Verified,
        )
        .with_output_continuity_classification_width(observation_width)
    }

    pub(crate) fn async_node_hierarchy_historical_parity(
        hierarchy_width: u32,
        branch_restore_width: u32,
        observation_width: u32,
        diagnostics_allocation_count: u32,
        denied_count: u32,
    ) -> Self {
        Self::new(
            ResourceBoundaryKind::AsyncNodeHierarchyHistoricalParity,
            hierarchy_width,
            0,
            u32::from(denied_count == 0),
            denied_count,
            denied_count,
            0,
            0,
            hierarchy_width,
            branch_restore_width,
            diagnostics_allocation_count,
            1,
            ResourceDensityStrategy::BurstySortedDeduplicated,
            ResourceCostContractId::new(31),
            ResourceCostPosture::Verified,
        )
        .with_output_continuity_classification_width(observation_width)
    }
}
