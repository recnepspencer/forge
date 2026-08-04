use std::collections::{BTreeMap, BTreeSet};

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
    pub(super) descriptors: BTreeMap<ResourceDescriptorId, LoweredResourceDescriptor>,
    pub(super) descriptors_by_node: BTreeMap<ResourceNodeId, ResourceDescriptorId>,
    pub(super) lifecycle_by_node: BTreeMap<ResourceNodeId, ResourceLifecycleSummary>,
    pub(super) in_flight_by_request: BTreeMap<ResourceRequestId, InFlightResourceRequest>,
    pub(super) retained_in_flight_history_by_request:
        BTreeMap<ResourceRequestId, InFlightResourceRequest>,
    pub(super) pruned_in_flight_history_by_request:
        BTreeMap<ResourceRequestId, ResourceRetainedHistoryAvailability>,
    pub(super) terminal_in_flight_by_request: BTreeSet<ResourceRequestId>,
    pub(super) active_request_by_node: BTreeMap<ResourceNodeId, ResourceRequestId>,
    pub(super) stale_after_wake_by_node: BTreeMap<ResourceNodeId, TemporalWakeId>,
    pub(super) pending_retry_by_request: BTreeMap<ResourceRequestId, ScheduledResourceRetry>,
    pub(super) pending_retry_by_wake: BTreeMap<TemporalWakeId, ResourceRequestId>,
    pub(super) pending_retry_by_node: BTreeMap<ResourceNodeId, ScheduledResourceRetry>,
    pub(super) retained_retry_lineage_by_ordinal:
        BTreeMap<ResourceRetryOrdinal, RetainedResourceRetryLineage>,
    pub(super) pruned_retry_lineage_by_ordinal:
        BTreeMap<ResourceRetryOrdinal, ResourceRetainedRetryLineageAvailability>,
    pub(super) retry_budget_ledger: ResourceRetryBudgetLedger,
    pub(super) denied_completions: BTreeMap<AsyncDenialId, DeniedResourceCompletion>,
    pub(super) pruned_denied_completions_by_id:
        BTreeMap<AsyncDenialId, ResourceRetainedDeniedCompletionAvailability>,
    pub(super) latest_denied_completion_by_node: BTreeMap<ResourceNodeId, DeniedResourceCompletion>,
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
            descriptors: BTreeMap::new(),
            descriptors_by_node: BTreeMap::new(),
            lifecycle_by_node: BTreeMap::new(),
            in_flight_by_request: BTreeMap::new(),
            retained_in_flight_history_by_request: BTreeMap::new(),
            pruned_in_flight_history_by_request: BTreeMap::new(),
            terminal_in_flight_by_request: BTreeSet::new(),
            active_request_by_node: BTreeMap::new(),
            stale_after_wake_by_node: BTreeMap::new(),
            pending_retry_by_request: BTreeMap::new(),
            pending_retry_by_wake: BTreeMap::new(),
            pending_retry_by_node: BTreeMap::new(),
            retained_retry_lineage_by_ordinal: BTreeMap::new(),
            pruned_retry_lineage_by_ordinal: BTreeMap::new(),
            retry_budget_ledger: ResourceRetryBudgetLedger::default(),
            denied_completions: BTreeMap::new(),
            pruned_denied_completions_by_id: BTreeMap::new(),
            latest_denied_completion_by_node: BTreeMap::new(),
            latest_branch_restore_report: None,
        }
    }
}
