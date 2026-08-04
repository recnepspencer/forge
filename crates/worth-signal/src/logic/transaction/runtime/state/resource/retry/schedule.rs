use super::super::ResourceRuntimeState;
use super::admission::PreparedScheduledResourceRetry;
use super::budget::ResourceRetryBudgetCharge;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;
use crate::data::temporal::{ReadyTemporalWake, TemporalWakeId};

impl ResourceRuntimeState {
    pub fn pending_retry_wake_for_handle(
        &self,
        handle: ResourceRequestHandle,
    ) -> Option<TemporalWakeId> {
        self.pending_retry_by_request
            .get(&handle.request_id())
            .filter(|scheduled| scheduled.previous() == handle)
            .map(|scheduled| scheduled.backoff_wake_id())
    }
    pub fn pending_retry_wake_for_node(&self, node: ResourceNodeId) -> Option<TemporalWakeId> {
        self.pending_retry_by_node
            .get(&node)
            .map(|scheduled| scheduled.backoff_wake_id())
    }
    pub fn clear_pending_retry_for_node(
        &mut self,
        node: ResourceNodeId,
    ) -> Option<ScheduledResourceRetry> {
        let scheduled = self.pending_retry_by_node.remove(&node)?;
        self.pending_retry_by_request
            .remove(&scheduled.previous().request_id());
        self.pending_retry_by_wake
            .remove(&scheduled.backoff_wake_id());
        self.retain_retry_lineage(node, scheduled.clone());
        Some(scheduled)
    }
    pub fn schedule_resource_retry(
        &mut self,
        handle: ResourceRequestHandle,
        reason: ResourceRetryReason,
        backoff_wake_id: TemporalWakeId,
        next_attempt: ResourceAttemptId,
        scheduled_delay: crate::data::temporal::TemporalDuration,
        retry_decision_digest: crate::data::resource::ResourcePolicyDigest,
        retry_budget_charge: Option<ResourceRetryBudgetCharge>,
        telemetry: &mut ResourceTelemetry,
    ) -> ResourceRetryScheduleReport {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get(&request_id).cloned() else {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                retry_decision_digest.clone(),
                None,
                telemetry,
            );
        };

        if in_flight.handle() != handle {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                retry_decision_digest.clone(),
                None,
                telemetry,
            );
        }
        if in_flight.status() != ResourceInFlightStatus::TimedOut
            || in_flight.lifecycle() != ResourceLifecycleClass::TimedOut
        {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::NonRetryableRequest,
                retry_decision_digest.clone(),
                None,
                telemetry,
            );
        }
        if self.pending_retry_by_request.contains_key(&request_id) {
            return self.deny_retry_schedule(
                request_id,
                ResourceRetryDenialClass::RetryAlreadyScheduled,
                retry_decision_digest.clone(),
                retry_budget_charge,
                telemetry,
            );
        }
        if let Some(charge) = retry_budget_charge {
            if charge.spent_before() >= charge.limit() {
                return self.deny_retry_schedule(
                    request_id,
                    ResourceRetryDenialClass::RetryBudgetExhausted,
                    retry_decision_digest.clone(),
                    Some(charge),
                    telemetry,
                );
            }
            self.retry_budget_ledger.consume(&in_flight, charge);
        }
        let scheduled = ScheduledResourceRetry::new(
            handle,
            self.issue_retry_ordinal(),
            reason,
            next_attempt,
            backoff_wake_id,
            scheduled_delay,
            retry_decision_digest,
            retry_budget_charge.map(|charge| charge.scope()),
            retry_budget_charge.map(|charge| charge.limit()),
            retry_budget_charge.map(|charge| charge.spent_before().saturating_add(1)),
        );
        self.pending_retry_by_request
            .insert(request_id, scheduled.clone());
        self.pending_retry_by_wake
            .insert(backoff_wake_id, request_id);
        self.pending_retry_by_node
            .insert(in_flight.node(), scheduled.clone());
        telemetry.resource_retry_schedule_count += 1;
        telemetry.resource_retry_temporal_wake_footprint = telemetry
            .resource_retry_temporal_wake_footprint
            .saturating_add(1);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::retry_schedule(
                1,
                0,
                u32::from(retry_budget_charge.is_some()),
            ),
        );

        ResourceRetryScheduleReport::admitted(scheduled, performance)
    }
    pub(in crate::logic::transaction::runtime::state) fn prepare_scheduled_resource_retry(
        &mut self,
        handle: ResourceRequestHandle,
        ready_wake: &ReadyTemporalWake,
        telemetry: &mut ResourceTelemetry,
    ) -> Result<PreparedScheduledResourceRetry, ResourceRetryAdmissionReport> {
        telemetry.resource_hot_in_flight_lookup_count += 1;
        let request_id = handle.request_id();
        let Some(scheduled) = self.pending_retry_by_request.get(&request_id).cloned() else {
            return Err(self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::MissingRetryBackoffWake,
                self.retry_policy_decision_digest_for_request(request_id),
                telemetry,
            ));
        };
        if scheduled.previous() != handle {
            return Err(self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                scheduled.policy_decision_digest().clone(),
                telemetry,
            ));
        }
        if scheduled.backoff_wake_id() != ready_wake.id() {
            return Err(self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::WakeMismatch,
                scheduled.policy_decision_digest().clone(),
                telemetry,
            ));
        }

        let Some(previous) = self.in_flight_by_request.get(&request_id).cloned() else {
            return Err(self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                scheduled.policy_decision_digest().clone(),
                telemetry,
            ));
        };
        if previous.handle() != handle {
            return Err(self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::UnknownOrStaleRequest,
                scheduled.policy_decision_digest().clone(),
                telemetry,
            ));
        }
        if self
            .active_request_by_node
            .get(&previous.node())
            .is_some_and(|active| *active != request_id)
        {
            return Err(self.deny_retry_admission(
                request_id,
                ResourceRetryDenialClass::SupersededByNewerRequest,
                scheduled.policy_decision_digest().clone(),
                telemetry,
            ));
        }
        Ok(PreparedScheduledResourceRetry {
            scheduled,
            previous,
        })
    }
}
