use super::super::ResourceRuntimeState;
use super::budget::ResourceRetryBudgetCharge;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;

impl ResourceRuntimeState {
    pub fn deny_resource_retry_schedule(
        &mut self,
        handle: ResourceRequestHandle,
        class: ResourceRetryDenialClass,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceRetryScheduleReport {
        let retry_budget_charge = if class == ResourceRetryDenialClass::RetryBudgetExhausted {
            self.in_flight_by_request
                .get(&handle.request_id())
                .cloned()
                .filter(|in_flight| in_flight.handle() == handle)
                .and_then(|in_flight| {
                    self.descriptors
                        .get(&in_flight.descriptor_id())
                        .and_then(|descriptor| {
                            self.retry_budget_ledger.charge_for(
                                &in_flight,
                                descriptor.retry_decision_plan().retry_budget_scope(),
                                descriptor.retry_decision_plan().retry_budget_limit(),
                            )
                        })
                })
        } else {
            None
        };
        let retry_decision_digest =
            self.retry_policy_decision_digest_for_request(handle.request_id());
        self.deny_retry_schedule(
            handle.request_id(),
            class,
            retry_decision_digest,
            retry_budget_charge,
            telemetry.as_deref_mut(),
        )
    }

    pub fn deny_resource_retry_admission_for_report(
        &mut self,
        handle: ResourceRequestHandle,
        class: ResourceRetryDenialClass,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceRetryAdmissionReport {
        let retry_decision_digest =
            self.retry_policy_decision_digest_for_request(handle.request_id());
        self.deny_retry_admission(
            handle.request_id(),
            class,
            retry_decision_digest,
            telemetry.as_deref_mut(),
        )
    }

    pub(in crate::logic::transaction::runtime::state::resource::retry) fn deny_retry_schedule(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceRetryDenialClass,
        retry_decision_digest: ResourcePolicyDigest,
        retry_budget_charge: Option<ResourceRetryBudgetCharge>,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceRetryScheduleReport {
        self.record_retry_denial(class, telemetry.as_deref_mut());
        let performance = Self::record_boundary_performance_optional(
            telemetry.as_deref_mut(),
            ResourceBoundaryPerformanceEnvelope::retry_schedule(
                0,
                1,
                u32::from(retry_budget_charge.is_some()),
            ),
        );
        ResourceRetryScheduleReport::denied(
            DeniedResourceRetry::new(
                request_id,
                class,
                retry_decision_digest,
                retry_budget_charge.map(|charge| charge.scope()),
                retry_budget_charge.map(|charge| charge.limit()),
                retry_budget_charge.map(|charge| charge.spent_before()),
            ),
            performance,
        )
    }

    pub(in crate::logic::transaction::runtime::state::resource::retry) fn deny_retry_admission(
        &mut self,
        request_id: ResourceRequestId,
        class: ResourceRetryDenialClass,
        retry_decision_digest: ResourcePolicyDigest,
        mut telemetry: Option<&mut ResourceTelemetry>,
    ) -> ResourceRetryAdmissionReport {
        self.record_retry_denial(class, telemetry.as_deref_mut());
        let performance = Self::record_boundary_performance_optional(
            telemetry.as_deref_mut(),
            ResourceBoundaryPerformanceEnvelope::retry_admission(
                0,
                1,
                0,
                u32::from(matches!(class, ResourceRetryDenialClass::WakeMismatch)),
            ),
        );
        ResourceRetryAdmissionReport::denied(
            DeniedResourceRetry::new(request_id, class, retry_decision_digest, None, None, None),
            performance,
        )
    }

    fn record_retry_denial(
        &mut self,
        class: ResourceRetryDenialClass,
        telemetry: Option<&mut ResourceTelemetry>,
    ) {
        if let Some(telemetry) = telemetry {
            telemetry.resource_retry_denial_count += 1;
            match class {
                ResourceRetryDenialClass::UnknownOrStaleRequest
                | ResourceRetryDenialClass::MissingRetryBackoffWake => {
                    telemetry.resource_stale_retry_denial_count += 1
                }
                ResourceRetryDenialClass::NonRetryableRequest => {
                    telemetry.resource_non_retryable_denial_count += 1
                }
                ResourceRetryDenialClass::RetryPolicyDisabled => {
                    telemetry.resource_retry_policy_disabled_denial_count += 1
                }
                ResourceRetryDenialClass::RetryAttemptLimitReached => {
                    telemetry.resource_retry_attempt_limit_denial_count += 1
                }
                ResourceRetryDenialClass::RetryBudgetExhausted => {
                    telemetry.resource_retry_budget_exhaustion_denial_count += 1
                }
                ResourceRetryDenialClass::RetryTimeoutWindowExhausted => {
                    telemetry.resource_retry_timeout_window_exhaustion_denial_count += 1
                }
                ResourceRetryDenialClass::RetryAlreadyScheduled => {
                    telemetry.resource_retry_already_scheduled_denial_count += 1
                }
                ResourceRetryDenialClass::WakeMismatch => {
                    telemetry.resource_retry_wake_mismatch_denial_count += 1
                }
                ResourceRetryDenialClass::SupersededByNewerRequest => {
                    telemetry.resource_retry_superseded_denial_count += 1
                }
            }
        }
    }
}
