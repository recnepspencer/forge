use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime::state::resource) fn deny_rejection(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceRejectionDenialClass,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceRejectionReport {
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_rejection_denial_count += 1;
            match class {
                ResourceRejectionDenialClass::UnknownOrStaleRequest => {
                    telemetry.resource_stale_rejection_denial_count += 1
                }
                ResourceRejectionDenialClass::NonActiveRequest => {
                    telemetry.resource_non_active_rejection_denial_count += 1
                }
            }
        }
        let performance = Self::record_boundary_performance_optional(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::rejection_admission(0, 1),
        );
        ResourceRejectionReport::denied(
            DeniedResourceRejection::new(request_id, class),
            performance,
        )
    }
}
