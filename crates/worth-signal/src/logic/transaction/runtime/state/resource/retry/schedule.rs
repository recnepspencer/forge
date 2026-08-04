use super::super::ResourceRuntimeState;
use super::admission::PreparedScheduledResourceRetry;
use super::budget::ResourceRetryBudgetCharge;
use crate::data::resource::*;
use crate::data::telemetry::ResourceTelemetry;
use crate::data::temporal::{ReadyTemporalWake, TemporalWakeId};

struct ResourceRetryScheduleInput {
    handle: ResourceRequestHandle,
    reason: ResourceRetryReason,
    backoff_wake_id: TemporalWakeId,
    next_attempt: ResourceAttemptId,
    scheduled_delay: crate::data::temporal::TemporalDuration,
    retry_decision_digest: ResourcePolicyDigest,
    retry_budget_charge: Option<ResourceRetryBudgetCharge>,
}

struct ResourceRetryScheduleCandidate {
    request_id: ResourceRequestId,
    node: ResourceNodeId,
    in_flight: InFlightResourceRequest,
    retry_budget_charge: Option<ResourceRetryBudgetCharge>,
}

struct PreparedResourceRetrySchedule {
    request_id: ResourceRequestId,
    node: ResourceNodeId,
    scheduled: ScheduledResourceRetry,
    budget_charged: bool,
}

struct InstalledResourceRetrySchedule {
    scheduled: ScheduledResourceRetry,
    budget_charged: bool,
}

struct RetryScheduleEligibilityDenial {
    class: ResourceRetryDenialClass,
    retry_budget_charge: Option<ResourceRetryBudgetCharge>,
}

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
        let input = ResourceRetryScheduleInput {
            handle,
            reason,
            backoff_wake_id,
            next_attempt,
            scheduled_delay,
            retry_decision_digest,
            retry_budget_charge,
        };
        let candidate = match self.classify_retry_schedule(&input) {
            Ok(candidate) => candidate,
            Err(denial) => {
                return self.deny_retry_schedule(
                    input.handle.request_id(),
                    denial.class,
                    input.retry_decision_digest,
                    denial.retry_budget_charge,
                    telemetry,
                )
            }
        };
        self.consume_retry_schedule_budget(&candidate);
        let prepared = self.prepare_resource_retry_schedule(input, candidate);
        let installed = self.install_resource_retry_schedule(prepared, telemetry);
        let performance = Self::record_boundary_performance(
            telemetry,
            ResourceBoundaryPerformanceEnvelope::retry_schedule(
                1,
                0,
                u32::from(installed.budget_charged),
            ),
        );

        ResourceRetryScheduleReport::admitted(installed.scheduled, performance)
    }

    fn classify_retry_schedule(
        &self,
        input: &ResourceRetryScheduleInput,
    ) -> Result<ResourceRetryScheduleCandidate, RetryScheduleEligibilityDenial> {
        let request_id = input.handle.request_id();
        let Some(in_flight) = self.in_flight_by_request.get(&request_id).cloned() else {
            return Err(RetryScheduleEligibilityDenial {
                class: ResourceRetryDenialClass::UnknownOrStaleRequest,
                retry_budget_charge: None,
            });
        };
        if in_flight.handle() != input.handle {
            return Err(RetryScheduleEligibilityDenial {
                class: ResourceRetryDenialClass::UnknownOrStaleRequest,
                retry_budget_charge: None,
            });
        }
        if in_flight.status() != ResourceInFlightStatus::TimedOut
            || in_flight.lifecycle() != ResourceLifecycleClass::TimedOut
        {
            return Err(RetryScheduleEligibilityDenial {
                class: ResourceRetryDenialClass::NonRetryableRequest,
                retry_budget_charge: None,
            });
        }
        if self.pending_retry_by_request.contains_key(&request_id) {
            return Err(RetryScheduleEligibilityDenial {
                class: ResourceRetryDenialClass::RetryAlreadyScheduled,
                retry_budget_charge: input.retry_budget_charge,
            });
        }
        if input
            .retry_budget_charge
            .is_some_and(|charge| charge.spent_before() >= charge.limit())
        {
            return Err(RetryScheduleEligibilityDenial {
                class: ResourceRetryDenialClass::RetryBudgetExhausted,
                retry_budget_charge: input.retry_budget_charge,
            });
        }
        Ok(ResourceRetryScheduleCandidate {
            request_id,
            node: in_flight.node(),
            in_flight,
            retry_budget_charge: input.retry_budget_charge,
        })
    }

    fn consume_retry_schedule_budget(&mut self, candidate: &ResourceRetryScheduleCandidate) {
        if let Some(charge) = candidate.retry_budget_charge {
            self.retry_budget_ledger
                .consume(&candidate.in_flight, charge);
        }
    }

    fn prepare_resource_retry_schedule(
        &mut self,
        input: ResourceRetryScheduleInput,
        candidate: ResourceRetryScheduleCandidate,
    ) -> PreparedResourceRetrySchedule {
        let retry_budget_charge = candidate.retry_budget_charge;
        let scheduled = ScheduledResourceRetry::new(
            input.handle,
            self.issue_retry_ordinal(),
            input.reason,
            input.next_attempt,
            input.backoff_wake_id,
            input.scheduled_delay,
            input.retry_decision_digest,
            retry_budget_charge.map(|charge| charge.scope()),
            retry_budget_charge.map(|charge| charge.limit()),
            retry_budget_charge.map(|charge| charge.spent_before().saturating_add(1)),
        );
        PreparedResourceRetrySchedule {
            request_id: candidate.request_id,
            node: candidate.node,
            scheduled,
            budget_charged: retry_budget_charge.is_some(),
        }
    }

    fn install_resource_retry_schedule(
        &mut self,
        prepared: PreparedResourceRetrySchedule,
        telemetry: &mut ResourceTelemetry,
    ) -> InstalledResourceRetrySchedule {
        let PreparedResourceRetrySchedule {
            request_id,
            node,
            scheduled,
            budget_charged,
        } = prepared;
        self.pending_retry_by_request
            .insert(request_id, scheduled.clone());
        self.pending_retry_by_wake
            .insert(scheduled.backoff_wake_id(), request_id);
        self.pending_retry_by_node.insert(node, scheduled.clone());
        telemetry.resource_retry_schedule_count += 1;
        telemetry.resource_retry_temporal_wake_footprint = telemetry
            .resource_retry_temporal_wake_footprint
            .saturating_add(1);
        InstalledResourceRetrySchedule {
            scheduled,
            budget_charged,
        }
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
