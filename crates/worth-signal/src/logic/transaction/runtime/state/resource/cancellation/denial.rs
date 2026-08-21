use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime::state::resource) fn deny_cancellation(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceCancellationDenialClass,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceCancellationReport {
        if let Some(telemetry) = telemetry.as_deref_mut() {
            telemetry.resource_cancellation_denial_count += 1;
            match class {
                ResourceCancellationDenialClass::UnknownOrStaleRequest => {
                    telemetry.resource_stale_cancellation_denial_count += 1
                }
                ResourceCancellationDenialClass::NonActiveRequest => {
                    telemetry.resource_non_active_cancellation_denial_count += 1
                }
            }
        }
        let performance = Self::record_boundary_performance_optional(
            telemetry.as_deref_mut(),
            ResourceBoundaryPerformanceEnvelope::cancellation(0, 1),
        );
        ResourceCancellationReport::denied(
            DeniedResourceCancellation::new(request_id, class),
            performance,
        )
    }
}
