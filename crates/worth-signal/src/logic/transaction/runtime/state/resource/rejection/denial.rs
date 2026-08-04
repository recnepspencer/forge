use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime::state::resource) fn deny_rejection(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceRejectionDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRejectionReport {
        telemetry.resource_rejection_denial_count += 1;
        match class {
            ResourceRejectionDenialClass::UnknownOrStaleRequest => {
                telemetry.resource_stale_rejection_denial_count += 1
            }
            ResourceRejectionDenialClass::NonActiveRequest => {
                telemetry.resource_non_active_rejection_denial_count += 1
            }
        }
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::rejection_admission(0, 1),
        );
        ResourceRejectionReport::denied(
            DeniedResourceRejection::new(request_id, class),
            performance,
        )
    }
}
