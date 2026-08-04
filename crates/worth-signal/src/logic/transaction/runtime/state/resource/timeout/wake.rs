use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::temporal::TemporalWakeId;
use std::collections::BTreeSet;

impl ResourceRuntimeState {
    pub fn active_timeout_wake_for_handle(
        &self,
        handle: ResourceRequestHandle,
    ) -> Option<TemporalWakeId> {
        self.in_flight_by_request
            .get(&handle.request_id())
            .filter(|in_flight| in_flight.handle() == handle)
            .filter(|in_flight| in_flight.status() == ResourceInFlightStatus::Active)
            .filter(|in_flight| in_flight.lifecycle() == ResourceLifecycleClass::Pending)
            .and_then(|in_flight| in_flight.timeout_wake_id())
    }
    pub fn active_timeout_wakes_for_cancellation_footprint(
        &self,
        handle: ResourceRequestHandle,
    ) -> Vec<TemporalWakeId> {
        let mut visited_requests = BTreeSet::new();
        let mut collected_wakes = BTreeSet::new();
        self.collect_active_timeout_wakes_for_cancellation_footprint(
            handle.request_id(),
            handle,
            &mut visited_requests,
            &mut collected_wakes,
        );
        collected_wakes.into_iter().collect()
    }
    pub fn active_timeout_wake_for_node(&self, node: ResourceNodeId) -> Option<TemporalWakeId> {
        let request_id = self.active_request_by_node.get(&node)?;
        self.in_flight_by_request
            .get(request_id)
            .filter(|in_flight| in_flight.status() == ResourceInFlightStatus::Active)
            .filter(|in_flight| in_flight.lifecycle() == ResourceLifecycleClass::Pending)
            .and_then(|in_flight| in_flight.timeout_wake_id())
    }
    pub fn active_stale_after_wake_for_node(&self, node: ResourceNodeId) -> Option<TemporalWakeId> {
        self.stale_after_wake_by_node.get(&node).copied()
    }
    pub fn lifecycle_summary_for_node(
        &self,
        node: ResourceNodeId,
    ) -> Option<ResourceLifecycleSummary> {
        self.lifecycle_by_node.get(&node).copied()
    }
    pub fn active_request_handle_for_node(
        &self,
        node: ResourceNodeId,
    ) -> Option<ResourceRequestHandle> {
        let request_id = self.active_request_by_node.get(&node)?;
        self.in_flight_by_request
            .get(request_id)
            .filter(|in_flight| in_flight.status() == ResourceInFlightStatus::Active)
            .map(|in_flight| in_flight.handle())
    }
    pub fn attach_stale_after_wake(&mut self, node: ResourceNodeId, wake_id: TemporalWakeId) {
        self.stale_after_wake_by_node.insert(node, wake_id);
    }
    pub fn clear_stale_after_wake_for_node(
        &mut self,
        node: ResourceNodeId,
    ) -> Option<TemporalWakeId> {
        self.stale_after_wake_by_node.remove(&node)
    }
}
