use crate::data::resource::{
    AsyncDenialId, ResourceLifecycleRetentionCompactionReport, ResourceRequestId,
    ResourceRetainedDeniedCompletionAvailability, ResourceRetainedHistoryAvailability,
    ResourceRetainedRetryLineageAvailability, ResourceRetentionCompactionBudget,
    ResourceRetryOrdinal, RetainedResourceRetryLineage,
};

use super::super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn compact_resource_lifecycle_history(
        &mut self,
        max_reclaimed: u32,
    ) -> ResourceLifecycleRetentionCompactionReport {
        self.resource
            .compact_lifecycle_history(max_reclaimed, &mut self.telemetry.resource)
    }

    pub fn compact_resource_lifecycle_history_with_retained_limit(
        &mut self,
        max_reclaimed: u32,
        retained_history_limit: u32,
    ) -> ResourceLifecycleRetentionCompactionReport {
        self.resource.compact_lifecycle_history_with_retained_limit(
            max_reclaimed,
            Some(retained_history_limit),
            &mut self.telemetry.resource,
        )
    }

    pub fn compact_resource_lifecycle_history_with_budget(
        &mut self,
        max_reclaimed: u32,
        budget: ResourceRetentionCompactionBudget,
    ) -> ResourceLifecycleRetentionCompactionReport {
        self.resource.compact_lifecycle_history_with_budget(
            max_reclaimed,
            budget,
            &mut self.telemetry.resource,
        )
    }

    pub fn retained_history_availability_for_request(
        &self,
        request_id: ResourceRequestId,
    ) -> Option<&ResourceRetainedHistoryAvailability> {
        self.resource
            .retained_history_availability_for_request(request_id)
    }

    pub fn retained_denied_completion_availability(
        &self,
        denial_id: AsyncDenialId,
    ) -> Option<&ResourceRetainedDeniedCompletionAvailability> {
        self.resource
            .retained_denied_completion_availability(denial_id)
    }

    pub fn retained_retry_lineage(
        &self,
        retry_ordinal: ResourceRetryOrdinal,
    ) -> Option<&RetainedResourceRetryLineage> {
        self.resource.retained_retry_lineage(retry_ordinal)
    }

    pub fn retained_retry_lineage_availability(
        &self,
        retry_ordinal: ResourceRetryOrdinal,
    ) -> Option<&ResourceRetainedRetryLineageAvailability> {
        self.resource
            .retained_retry_lineage_availability(retry_ordinal)
    }
}
