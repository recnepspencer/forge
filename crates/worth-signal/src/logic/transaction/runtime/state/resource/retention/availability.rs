use super::super::ResourceRuntimeState;
use crate::data::resource::*;

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime::state::resource) fn retention_availability_class_for_lifecycle(
        descriptor: &LoweredResourceDescriptor,
        lifecycle: ResourceLifecycleClass,
    ) -> Option<ResourceRetainedHistoryAvailabilityClass> {
        match descriptor.retention_decision_plan().class() {
            crate::data::resource::ResourceRetentionDecisionClass::RetainAllTransitions => None,
            crate::data::resource::ResourceRetentionDecisionClass::TerminalSummariesOnly => {
                Some(ResourceRetainedHistoryAvailabilityClass::TerminalSummaryOnly)
            }
            crate::data::resource::ResourceRetentionDecisionClass::CompactSuperseded => {
                Some(ResourceRetainedHistoryAvailabilityClass::CompactSuperseded)
            }
            crate::data::resource::ResourceRetentionDecisionClass::CompactCancelled => {
                Some(ResourceRetainedHistoryAvailabilityClass::CompactCancelled)
            }
            crate::data::resource::ResourceRetentionDecisionClass::CompactTimedOut => {
                Some(ResourceRetainedHistoryAvailabilityClass::CompactTimedOut)
            }
        }
        .filter(|_| {
            descriptor
                .retention_decision_plan()
                .permits_compaction_for_lifecycle(lifecycle)
        })
    }
    pub(in crate::logic::transaction::runtime::state::resource) fn retention_availability_from_in_flight(
        descriptor: &LoweredResourceDescriptor,
        in_flight: InFlightResourceRequest,
        class: ResourceRetainedHistoryAvailabilityClass,
    ) -> ResourceRetainedHistoryAvailability {
        ResourceRetainedHistoryAvailability::new(
            in_flight.handle(),
            in_flight.attempt(),
            in_flight.node(),
            in_flight.lifecycle(),
            class,
            descriptor.retention_decision_plan().descriptor_id(),
            descriptor.retention_decision_plan().class(),
            descriptor
                .retention_decision_plan()
                .decision_digest()
                .clone(),
        )
    }
    pub(in crate::logic::transaction::runtime::state::resource) fn pruned_denied_completion_availability(
        denied: DeniedResourceCompletion,
    ) -> ResourceRetainedDeniedCompletionAvailability {
        ResourceRetainedDeniedCompletionAvailability::new(
            denied.denial_id(),
            denied.request_id(),
            denied.node(),
            denied.class(),
            ResourceRetainedDeniedCompletionAvailabilityClass::PrunedByRetainedDeniedCompletionLimit,
        )
    }
    pub(in crate::logic::transaction::runtime::state::resource) fn retain_retry_lineage(
        &mut self,
        node: ResourceNodeId,
        scheduled: ScheduledResourceRetry,
    ) {
        let retained = RetainedResourceRetryLineage::from_scheduled(node, scheduled);
        self.retained_retry_lineage_by_ordinal
            .insert(retained.retry_ordinal(), retained);
    }
}
