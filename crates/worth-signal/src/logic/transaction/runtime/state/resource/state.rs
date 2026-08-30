use super::retry::budget::ResourceRetryBudgetLedger;
use crate::data::resource::{
    AsyncDenialId, DeniedResourceCompletion, FrozenResourcePolicyRegistry, InFlightResourceRequest,
    LoweredResourceDescriptor, ResourceBranchRestoreReport, ResourceCancellationOrdinal,
    ResourceCompletionOrdinal, ResourceDescriptorId, ResourceGeneration, ResourceLifecycleOrdinal,
    ResourceLifecycleSummary, ResourceNodeId, ResourceRejectionOrdinal, ResourceRequestId,
    ResourceRetainedDeniedCompletionAvailability, ResourceRetainedHistoryAvailability,
    ResourceRetainedRetryLineageAvailability, ResourceRetryOrdinal,
    ResourceSafePointObservationOrdinal, ResourceSupersessionOrdinal, ResourceTimeoutOrdinal,
    RetainedResourceRetryLineage, ScheduledResourceRetry,
};
use crate::data::temporal::TemporalWakeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) struct ResourceRuntimeState {
    pub(super) next_descriptor_id: ResourceDescriptorId,
    pub(super) next_request_id: ResourceRequestId,
    pub(super) next_generation: ResourceGeneration,
    pub(super) next_lifecycle_ordinal: ResourceLifecycleOrdinal,
    pub(super) next_denial_id: AsyncDenialId,
    pub(super) next_completion_ordinal: ResourceCompletionOrdinal,
    pub(super) next_cancellation_ordinal: ResourceCancellationOrdinal,
    pub(super) next_timeout_ordinal: ResourceTimeoutOrdinal,
    pub(super) next_rejection_ordinal: ResourceRejectionOrdinal,
    pub(super) next_supersession_ordinal: ResourceSupersessionOrdinal,
    pub(super) next_retry_ordinal: ResourceRetryOrdinal,
    pub(super) next_safe_point_observation_ordinal: ResourceSafePointObservationOrdinal,
    pub(super) restore_epoch: u64,
    pub(super) policy_registry: FrozenResourcePolicyRegistry,
    pub(super) descriptors: im::OrdMap<ResourceDescriptorId, LoweredResourceDescriptor>,
    pub(super) descriptors_by_node: im::OrdMap<ResourceNodeId, ResourceDescriptorId>,
    pub(super) lifecycle_by_node: im::OrdMap<ResourceNodeId, ResourceLifecycleSummary>,
    pub(super) in_flight_by_request: im::OrdMap<ResourceRequestId, InFlightResourceRequest>,
    pub(super) retained_in_flight_history_by_request:
        im::OrdMap<ResourceRequestId, InFlightResourceRequest>,
    pub(super) pruned_in_flight_history_by_request:
        im::OrdMap<ResourceRequestId, ResourceRetainedHistoryAvailability>,
    pub(super) terminal_in_flight_by_request: im::OrdSet<ResourceRequestId>,
    pub(super) active_request_by_node: im::OrdMap<ResourceNodeId, ResourceRequestId>,
    pub(super) stale_after_wake_by_node: im::OrdMap<ResourceNodeId, TemporalWakeId>,
    pub(super) pending_retry_by_request: im::OrdMap<ResourceRequestId, ScheduledResourceRetry>,
    pub(super) pending_retry_by_wake: im::OrdMap<TemporalWakeId, ResourceRequestId>,
    pub(super) pending_retry_by_node: im::OrdMap<ResourceNodeId, ScheduledResourceRetry>,
    pub(super) retained_retry_lineage_by_ordinal:
        im::OrdMap<ResourceRetryOrdinal, RetainedResourceRetryLineage>,
    pub(super) pruned_retry_lineage_by_ordinal:
        im::OrdMap<ResourceRetryOrdinal, ResourceRetainedRetryLineageAvailability>,
    pub(super) retry_budget_ledger: ResourceRetryBudgetLedger,
    pub(super) denied_completions: im::OrdMap<AsyncDenialId, DeniedResourceCompletion>,
    pub(super) pruned_denied_completions_by_id:
        im::OrdMap<AsyncDenialId, ResourceRetainedDeniedCompletionAvailability>,
    pub(super) latest_denied_completion_by_node:
        im::OrdMap<ResourceNodeId, DeniedResourceCompletion>,
    pub(super) latest_branch_restore_report: Option<ResourceBranchRestoreReport>,
}

impl Default for ResourceRuntimeState {
    fn default() -> Self {
        Self {
            next_descriptor_id: ResourceDescriptorId::new(0),
            next_request_id: ResourceRequestId::new(0),
            next_generation: ResourceGeneration::new(0),
            next_lifecycle_ordinal: ResourceLifecycleOrdinal::ZERO,
            next_denial_id: AsyncDenialId::new(0),
            next_completion_ordinal: ResourceCompletionOrdinal::ZERO,
            next_cancellation_ordinal: ResourceCancellationOrdinal::ZERO,
            next_timeout_ordinal: ResourceTimeoutOrdinal::ZERO,
            next_rejection_ordinal: ResourceRejectionOrdinal::ZERO,
            next_supersession_ordinal: ResourceSupersessionOrdinal::ZERO,
            next_retry_ordinal: ResourceRetryOrdinal::ZERO,
            next_safe_point_observation_ordinal: ResourceSafePointObservationOrdinal::ZERO,
            restore_epoch: 0,
            policy_registry: FrozenResourcePolicyRegistry::built_in(),
            descriptors: im::OrdMap::new(),
            descriptors_by_node: im::OrdMap::new(),
            lifecycle_by_node: im::OrdMap::new(),
            in_flight_by_request: im::OrdMap::new(),
            retained_in_flight_history_by_request: im::OrdMap::new(),
            pruned_in_flight_history_by_request: im::OrdMap::new(),
            terminal_in_flight_by_request: im::OrdSet::new(),
            active_request_by_node: im::OrdMap::new(),
            stale_after_wake_by_node: im::OrdMap::new(),
            pending_retry_by_request: im::OrdMap::new(),
            pending_retry_by_wake: im::OrdMap::new(),
            pending_retry_by_node: im::OrdMap::new(),
            retained_retry_lineage_by_ordinal: im::OrdMap::new(),
            pruned_retry_lineage_by_ordinal: im::OrdMap::new(),
            retry_budget_ledger: ResourceRetryBudgetLedger::default(),
            denied_completions: im::OrdMap::new(),
            pruned_denied_completions_by_id: im::OrdMap::new(),
            latest_denied_completion_by_node: im::OrdMap::new(),
            latest_branch_restore_report: None,
        }
    }
}

#[cfg(test)]
impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime) fn shares_storage_with(&self, other: &Self) -> bool {
        self.policy_registry
            .shares_storage_with(&other.policy_registry)
            && self.descriptors.ptr_eq(&other.descriptors)
            && self.descriptors_by_node.ptr_eq(&other.descriptors_by_node)
            && self.lifecycle_by_node.ptr_eq(&other.lifecycle_by_node)
            && self
                .in_flight_by_request
                .ptr_eq(&other.in_flight_by_request)
            && self
                .retained_in_flight_history_by_request
                .ptr_eq(&other.retained_in_flight_history_by_request)
            && self
                .pruned_in_flight_history_by_request
                .ptr_eq(&other.pruned_in_flight_history_by_request)
            && self
                .terminal_in_flight_by_request
                .ptr_eq(&other.terminal_in_flight_by_request)
            && self
                .active_request_by_node
                .ptr_eq(&other.active_request_by_node)
            && self
                .stale_after_wake_by_node
                .ptr_eq(&other.stale_after_wake_by_node)
            && self
                .pending_retry_by_request
                .ptr_eq(&other.pending_retry_by_request)
            && self
                .pending_retry_by_wake
                .ptr_eq(&other.pending_retry_by_wake)
            && self
                .pending_retry_by_node
                .ptr_eq(&other.pending_retry_by_node)
            && self
                .retained_retry_lineage_by_ordinal
                .ptr_eq(&other.retained_retry_lineage_by_ordinal)
            && self
                .pruned_retry_lineage_by_ordinal
                .ptr_eq(&other.pruned_retry_lineage_by_ordinal)
            && self
                .retry_budget_ledger
                .shares_storage_with(&other.retry_budget_ledger)
            && self.denied_completions.ptr_eq(&other.denied_completions)
            && self
                .pruned_denied_completions_by_id
                .ptr_eq(&other.pruned_denied_completions_by_id)
            && self
                .latest_denied_completion_by_node
                .ptr_eq(&other.latest_denied_completion_by_node)
    }
}
