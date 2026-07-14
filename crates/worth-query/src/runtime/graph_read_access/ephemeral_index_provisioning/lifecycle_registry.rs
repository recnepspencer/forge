use super::{WorthQueryEphemeralGraphIndex, WorthQueryEphemeralGraphIndexCounters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryEphemeralGraphIndexLifecycleRegistry {
    allocation_attempt_count: usize,
    successful_allocation_count: usize,
    release_count: usize,
    active_resource_count: usize,
    rejected_before_allocation_count: usize,
    touched_node_count: usize,
    touched_edge_count: usize,
}

impl WorthQueryEphemeralGraphIndexLifecycleRegistry {
    pub fn allocation_attempt_count(&self) -> usize {
        self.allocation_attempt_count
    }

    pub fn successful_allocation_count(&self) -> usize {
        self.successful_allocation_count
    }

    pub fn release_count(&self) -> usize {
        self.release_count
    }

    pub fn active_resource_count(&self) -> usize {
        self.active_resource_count
    }

    pub fn rejected_before_allocation_count(&self) -> usize {
        self.rejected_before_allocation_count
    }

    pub(in crate::runtime::graph_read_access::ephemeral_index_provisioning) fn open_scope() -> Self
    {
        Self {
            allocation_attempt_count: 0,
            successful_allocation_count: 0,
            release_count: 0,
            active_resource_count: 0,
            rejected_before_allocation_count: 0,
            touched_node_count: 0,
            touched_edge_count: 0,
        }
    }

    pub(in crate::runtime::graph_read_access::ephemeral_index_provisioning) fn reject_before_allocation(
        &mut self,
    ) {
        self.rejected_before_allocation_count += 1;
    }

    pub(in crate::runtime::graph_read_access::ephemeral_index_provisioning) fn register_allocation(
        &mut self,
        index: &WorthQueryEphemeralGraphIndex,
    ) {
        self.allocation_attempt_count += 1;
        self.successful_allocation_count += 1;
        self.active_resource_count += 1;
        self.touched_node_count += index.touched_node_count();
        self.touched_edge_count += index.touched_edge_count();
    }

    pub(in crate::runtime::graph_read_access::ephemeral_index_provisioning) fn release_index(
        &mut self,
        _index: WorthQueryEphemeralGraphIndex,
    ) {
        self.release_count += 1;
        self.active_resource_count = self.active_resource_count.saturating_sub(1);
    }

    pub(in crate::runtime::graph_read_access::ephemeral_index_provisioning) fn close_scope_counters(
        &self,
    ) -> WorthQueryEphemeralGraphIndexCounters {
        WorthQueryEphemeralGraphIndexCounters::from_lifecycle_registry(self)
    }

    pub fn orphan_resource_count(&self) -> usize {
        self.active_resource_count
    }

    pub fn touched_node_count(&self) -> usize {
        self.touched_node_count
    }

    pub fn touched_edge_count(&self) -> usize {
        self.touched_edge_count
    }
}
