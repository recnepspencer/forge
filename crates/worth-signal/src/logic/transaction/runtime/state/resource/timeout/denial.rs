use super::super::ResourceRuntimeState;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub(in crate::logic::transaction::runtime::state::resource) fn deny_timeout_heartbeat_extension(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceTimeoutHeartbeatExtensionDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceTimeoutHeartbeatExtensionReport {
        telemetry.resource_timeout_heartbeat_extension_denial_count += 1;
        if matches!(
            class,
            ResourceTimeoutHeartbeatExtensionDenialClass::PolicyDoesNotAllowHeartbeatExtension
        ) {
            telemetry.resource_timeout_heartbeat_policy_denial_count += 1;
        }
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::timeout_heartbeat_extension(0, 1, 0),
        );
        ResourceTimeoutHeartbeatExtensionReport::denied(
            DeniedResourceTimeoutHeartbeatExtension::new(request_id, class),
            performance,
        )
    }
    pub fn deny_timeout_heartbeat_extension_for_report(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceTimeoutHeartbeatExtensionDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceTimeoutHeartbeatExtensionReport {
        self.deny_timeout_heartbeat_extension(request_id, class, telemetry)
    }
    pub(in crate::logic::transaction::runtime::state::resource) fn deny_timeout(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceTimeoutDenialClass,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceTimeoutReport {
        telemetry.resource_timeout_denial_count += 1;
        match class {
            ResourceTimeoutDenialClass::UnknownOrStaleRequest => {
                telemetry.resource_stale_timeout_denial_count += 1
            }
            ResourceTimeoutDenialClass::NonActiveRequest => {
                telemetry.resource_non_active_timeout_denial_count += 1
            }
            ResourceTimeoutDenialClass::MissingTimeoutWake => {
                telemetry.resource_missing_timeout_wake_denial_count += 1
            }
            ResourceTimeoutDenialClass::WakeMismatch => {
                telemetry.resource_timeout_wake_mismatch_denial_count += 1
            }
        }
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::timeout_admission(
                0,
                1,
                u32::from(matches!(class, ResourceTimeoutDenialClass::WakeMismatch)),
            ),
        );
        ResourceTimeoutReport::denied(DeniedResourceTimeout::new(request_id, class), performance)
    }
}
