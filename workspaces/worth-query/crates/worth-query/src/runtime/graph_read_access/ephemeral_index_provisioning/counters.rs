use super::WorthQueryEphemeralGraphIndexLifecycleRegistry;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryEphemeralGraphIndexCounters {
    allocation_attempt_count: usize,
    allocation_count: usize,
    cleanup_count: usize,
    orphan_resource_count: usize,
    rejected_before_allocation_count: usize,
    touched_node_count: usize,
    touched_edge_count: usize,
}

impl WorthQueryEphemeralGraphIndexCounters {
    pub fn allocation_attempt_count(&self) -> usize {
        self.allocation_attempt_count
    }

    pub fn allocation_count(&self) -> usize {
        self.allocation_count
    }

    pub fn successful_allocation_count(&self) -> usize {
        self.allocation_count
    }

    pub fn cleanup_count(&self) -> usize {
        self.cleanup_count
    }

    pub fn release_count(&self) -> usize {
        self.cleanup_count
    }

    pub fn orphan_resource_count(&self) -> usize {
        self.orphan_resource_count
    }

    pub fn rejected_before_allocation_count(&self) -> usize {
        self.rejected_before_allocation_count
    }

    pub fn touched_node_count(&self) -> usize {
        self.touched_node_count
    }

    pub fn touched_edge_count(&self) -> usize {
        self.touched_edge_count
    }

    pub(in crate::runtime::graph_read_access::ephemeral_index_provisioning) fn from_lifecycle_registry(
        registry: &WorthQueryEphemeralGraphIndexLifecycleRegistry,
    ) -> Self {
        Self {
            allocation_attempt_count: registry.allocation_attempt_count(),
            allocation_count: registry.successful_allocation_count(),
            cleanup_count: registry.release_count(),
            orphan_resource_count: registry.orphan_resource_count(),
            rejected_before_allocation_count: registry.rejected_before_allocation_count(),
            touched_node_count: registry.touched_node_count(),
            touched_edge_count: registry.touched_edge_count(),
        }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!(
            "ephemeral_index_counters:attempts:{}:allocations:{}:cleanups:{}:orphans:{}:rejected_before_allocation:{}:nodes:{}:edges:{}",
            self.allocation_attempt_count,
            self.allocation_count,
            self.cleanup_count,
            self.orphan_resource_count,
            self.rejected_before_allocation_count,
            self.touched_node_count,
            self.touched_edge_count
        )
    }
}
