use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub fn summary_read_report(
        &self,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRuntimeSummaryReadReport {
        self.summary_read_report_optional(Some(telemetry))
    }

    pub fn summary_read_report_optional(
        &self,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceRuntimeSummaryReadReport {
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_retained_summary_read_count += 1;
        }
        let performance = Self::record_boundary_performance_optional(
            telemetry.as_deref_mut(),
            ResourceBoundaryPerformanceEnvelope::summary_read(),
        );
        ResourceRuntimeSummaryReadReport::new(self.summary(), performance)
    }

    pub fn descriptor_for_node(&self, node: ResourceNodeId) -> Option<&LoweredResourceDescriptor> {
        self.descriptors_by_node
            .get(&node)
            .and_then(|descriptor_id| self.descriptors.get(descriptor_id))
    }

    pub fn latest_branch_restore_report(&self) -> Option<ResourceBranchRestoreReport> {
        self.latest_branch_restore_report
    }

    pub fn retained_history_availability_for_request(
        &self,
        request_id: ResourceRequestId,
    ) -> Option<&ResourceRetainedHistoryAvailability> {
        self.pruned_in_flight_history_by_request.get(&request_id)
    }

    pub fn retained_denied_completion_availability(
        &self,
        denial_id: AsyncDenialId,
    ) -> Option<&ResourceRetainedDeniedCompletionAvailability> {
        self.pruned_denied_completions_by_id.get(&denial_id)
    }

    pub fn retained_retry_lineage(
        &self,
        retry_ordinal: ResourceRetryOrdinal,
    ) -> Option<&RetainedResourceRetryLineage> {
        self.retained_retry_lineage_by_ordinal.get(&retry_ordinal)
    }

    pub fn retained_retry_lineage_availability(
        &self,
        retry_ordinal: ResourceRetryOrdinal,
    ) -> Option<&ResourceRetainedRetryLineageAvailability> {
        self.pruned_retry_lineage_by_ordinal.get(&retry_ordinal)
    }
}
