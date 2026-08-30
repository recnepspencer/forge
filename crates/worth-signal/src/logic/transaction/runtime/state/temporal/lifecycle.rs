use crate::data::error::SignalError;
use crate::data::telemetry::TemporalTelemetry;
use crate::data::temporal::{
    ClockTick, IntervalCondition, IntervalWakeRegeneration, MissedTickPolicy, ReadyTemporalWake,
    RetiredTemporalWake, ScheduledTemporalWake, TemporalCondition, TemporalWakeId,
    TemporalWakeOwner, TemporalWakeReschedule, TemporalWakeRetirementBatch,
    TemporalWakeRetirementReason,
};

use super::TemporalRuntimeState;

impl TemporalRuntimeState {
    pub fn schedule_wake(
        &mut self,
        condition: TemporalCondition,
        due_tick: ClockTick,
        telemetry: Option<&mut TemporalTelemetry>,
    ) -> Result<ScheduledTemporalWake, SignalError> {
        self.admit_scheduled_wake(
            TemporalWakeOwner::Manual,
            condition,
            due_tick,
            telemetry,
            false,
        )
    }

    fn admit_scheduled_wake(
        &mut self,
        owner: TemporalWakeOwner,
        condition: TemporalCondition,
        due_tick: ClockTick,
        telemetry: Option<&mut TemporalTelemetry>,
        allow_past_due: bool,
    ) -> Result<ScheduledTemporalWake, SignalError> {
        if !allow_past_due && due_tick < self.clock_basis.current_tick() {
            return Err(SignalError::invalid_input(format!(
                "cannot schedule temporal wake in the past: current tick is {}, due tick was {}",
                self.clock_basis.current_tick().get(),
                due_tick.get()
            )));
        }
        if condition.clock_domain() != self.clock_basis.domain() {
            return Err(SignalError::invalid_input(format!(
                "temporal wake clock-domain mismatch: runtime basis is {:?}, wake declared {:?}",
                self.clock_basis.domain(),
                condition.clock_domain()
            )));
        }

        let wake = ScheduledTemporalWake::new(
            self.issue_wake_id(),
            self.issue_wake_ordinal(),
            owner,
            condition,
            due_tick,
        );
        self.insert_scheduled_frontier_entry(&wake);
        self.insert_owner_frontier_entry(wake.owner(), wake.ordinal(), wake.id());
        self.scheduled_wakes.insert(wake.id(), wake.clone());
        if let Some(telemetry) = telemetry {
            telemetry.temporal_wake_count += 1;
            telemetry.scheduled_frontier_width = telemetry
                .scheduled_frontier_width
                .max(self.scheduled_frontier.len() as u64);
            telemetry.wake_allocation_count += 1;
            telemetry.ready_queue_width = telemetry
                .ready_queue_width
                .max(self.ready_wakes.len() as u64);
        }
        Ok(wake)
    }

    pub fn reschedule_wake(
        &mut self,
        wake_id: TemporalWakeId,
        due_tick: ClockTick,
        mut telemetry: Option<&mut TemporalTelemetry>,
    ) -> Result<TemporalWakeReschedule, SignalError> {
        let (owner, condition) = if let Some(wake) = self.scheduled_wakes.get(&wake_id) {
            (wake.owner(), wake.condition().clone())
        } else if let Some(wake) = self.ready_wakes.get(&wake_id) {
            (wake.owner(), wake.condition().clone())
        } else {
            return Err(SignalError::invalid_input(format!(
                "cannot reschedule unknown temporal wake {}",
                wake_id.get()
            )));
        };

        if due_tick < self.clock_basis.current_tick() {
            return Err(SignalError::invalid_input(format!(
                "cannot reschedule temporal wake {} into the past: current tick is {}, due tick was {}",
                wake_id.get(),
                self.clock_basis.current_tick().get(),
                due_tick.get()
            )));
        }

        let retired = self.retire_wake(
            wake_id,
            TemporalWakeRetirementReason::Superseded,
            telemetry.as_deref_mut(),
        )?;
        let scheduled =
            self.admit_scheduled_wake(owner, condition, due_tick, telemetry.as_deref_mut(), false)?;
        if let Some(telemetry) = telemetry {
            telemetry.rescheduled_wake_count += 1;
        }
        Ok(TemporalWakeReschedule::new(retired, scheduled))
    }

    pub fn supersede_wake_with_condition(
        &mut self,
        wake_id: TemporalWakeId,
        condition: TemporalCondition,
        due_tick: ClockTick,
        mut telemetry: Option<&mut TemporalTelemetry>,
    ) -> Result<TemporalWakeReschedule, SignalError> {
        let owner = self.active_wake_owner(wake_id).ok_or_else(|| {
            SignalError::invalid_input(format!(
                "cannot supersede unknown temporal wake {}",
                wake_id.get()
            ))
        })?;

        if due_tick < self.clock_basis.current_tick() {
            return Err(SignalError::invalid_input(format!(
                "cannot supersede temporal wake {} into the past: current tick is {}, due tick was {}",
                wake_id.get(),
                self.clock_basis.current_tick().get(),
                due_tick.get()
            )));
        }
        if condition.clock_domain() != self.clock_basis.domain() {
            return Err(SignalError::invalid_input(format!(
                "temporal supersession clock-domain mismatch: runtime basis is {:?}, wake declared {:?}",
                self.clock_basis.domain(),
                condition.clock_domain()
            )));
        }

        let retired = self.retire_wake(
            wake_id,
            TemporalWakeRetirementReason::Superseded,
            telemetry.as_deref_mut(),
        )?;
        let scheduled =
            self.admit_scheduled_wake(owner, condition, due_tick, telemetry.as_deref_mut(), false)?;
        if let Some(telemetry) = telemetry {
            telemetry.rescheduled_wake_count += 1;
        }
        Ok(TemporalWakeReschedule::new(retired, scheduled))
    }

    pub fn promote_wake_ready(
        &mut self,
        wake_id: TemporalWakeId,
        telemetry: Option<&mut TemporalTelemetry>,
    ) -> Result<ReadyTemporalWake, SignalError> {
        let scheduled = self.scheduled_wakes.remove(&wake_id).ok_or_else(|| {
            SignalError::invalid_input(format!(
                "cannot promote unknown scheduled temporal wake {}",
                wake_id.get()
            ))
        })?;
        self.remove_scheduled_frontier_entry(&scheduled);
        self.remove_owner_frontier_entry(scheduled.owner(), scheduled.ordinal(), scheduled.id());

        let ready_tick = self.clock_basis.current_tick();
        if ready_tick < scheduled.due_tick() {
            self.insert_scheduled_frontier_entry(&scheduled);
            self.insert_owner_frontier_entry(
                scheduled.owner(),
                scheduled.ordinal(),
                scheduled.id(),
            );
            self.scheduled_wakes.insert(wake_id, scheduled.clone());
            return Err(SignalError::invalid_input(format!(
                "cannot promote temporal wake {} before due tick {} at current tick {}",
                wake_id.get(),
                scheduled.due_tick().get(),
                ready_tick.get()
            )));
        }

        let ready =
            ReadyTemporalWake::from_scheduled(scheduled, self.issue_wake_ordinal(), ready_tick);
        self.insert_owner_frontier_entry(ready.owner(), ready.ready_ordinal(), ready.id());
        self.ready_frontier
            .insert(ready.ready_ordinal(), ready.id());
        self.ready_wakes.insert(wake_id, ready.clone());
        if let Some(telemetry) = telemetry {
            telemetry.ready_queue_width = telemetry
                .ready_queue_width
                .max(self.ready_wakes.len() as u64);
        }
        Ok(ready)
    }

    pub fn retire_wake(
        &mut self,
        wake_id: TemporalWakeId,
        reason: TemporalWakeRetirementReason,
        telemetry: Option<&mut TemporalTelemetry>,
    ) -> Result<RetiredTemporalWake, SignalError> {
        if let Some(ready) = self.ready_wakes.remove(&wake_id) {
            self.ready_frontier.remove(&ready.ready_ordinal());
            self.remove_owner_frontier_entry(ready.owner(), ready.ready_ordinal(), ready.id());
            let retired = RetiredTemporalWake::new(
                wake_id,
                ready.ready_ordinal(),
                self.issue_wake_ordinal(),
                ready.owner(),
                self.clock_basis.current_tick(),
                reason,
            );
            self.retired_wakes.insert(wake_id, retired.clone());
            if let Some(telemetry) = telemetry {
                telemetry.retired_wake_count += 1;
                telemetry.ready_queue_width = telemetry
                    .ready_queue_width
                    .max(self.ready_wakes.len() as u64);
            }
            return Ok(retired);
        }

        if let Some(scheduled) = self.scheduled_wakes.remove(&wake_id) {
            self.remove_scheduled_frontier_entry(&scheduled);
            self.remove_owner_frontier_entry(
                scheduled.owner(),
                scheduled.ordinal(),
                scheduled.id(),
            );
            let retired = RetiredTemporalWake::new(
                wake_id,
                scheduled.ordinal(),
                self.issue_wake_ordinal(),
                scheduled.owner(),
                self.clock_basis.current_tick(),
                reason,
            );
            self.retired_wakes.insert(wake_id, retired.clone());
            if let Some(telemetry) = telemetry {
                telemetry.retired_wake_count += 1;
            }
            return Ok(retired);
        }

        Err(SignalError::invalid_input(format!(
            "cannot retire unknown temporal wake {}",
            wake_id.get()
        )))
    }

    pub fn regenerate_interval_wake(
        &mut self,
        wake_id: TemporalWakeId,
        mut telemetry: Option<&mut TemporalTelemetry>,
    ) -> Result<IntervalWakeRegeneration, SignalError> {
        let ready = self.ready_wakes.get(&wake_id).cloned().ok_or_else(|| {
            SignalError::invalid_input(format!(
                "cannot regenerate non-ready interval wake {}",
                wake_id.get()
            ))
        })?;
        let interval = match ready.condition() {
            TemporalCondition::Interval(interval) => interval.clone(),
            other => {
                return Err(SignalError::invalid_input(format!(
                    "cannot regenerate non-interval temporal wake {} with condition {other:?}",
                    wake_id.get()
                )))
            }
        };

        let (successor_due, suppressed_interval_count) =
            self.compute_interval_successor_due(ready.due_tick(), &interval)?;
        let retired = self.retire_wake(
            wake_id,
            TemporalWakeRetirementReason::Consumed,
            telemetry.as_deref_mut(),
        )?;
        let scheduled = self.admit_scheduled_wake(
            ready.owner(),
            TemporalCondition::Interval(interval),
            successor_due,
            telemetry.as_deref_mut(),
            true,
        )?;
        if let Some(telemetry) = telemetry {
            telemetry.rescheduled_wake_count += 1;
            telemetry.interval_wake_regeneration_count += 1;
            telemetry.missed_interval_count = telemetry
                .missed_interval_count
                .saturating_add(suppressed_interval_count);
        }

        Ok(IntervalWakeRegeneration::new(
            retired,
            scheduled,
            suppressed_interval_count,
        ))
    }

    fn compute_interval_successor_due(
        &self,
        consumed_due_tick: ClockTick,
        interval: &IntervalCondition,
    ) -> Result<(ClockTick, u64), SignalError> {
        let period = interval.period().get();
        let current_tick = self.clock_basis.current_tick().get();
        let consumed_due = consumed_due_tick.get();
        let next_due = consumed_due.saturating_add(period);

        if current_tick < next_due {
            return Ok((ClockTick::new(next_due), 0));
        }

        let elapsed_boundaries = ((current_tick - consumed_due) / period).max(1);
        let successor_due = match interval.missed_tick_policy() {
            MissedTickPolicy::CatchUpAll => ClockTick::new(next_due),
            MissedTickPolicy::SkipToLatest => ClockTick::new(
                consumed_due.saturating_add(period.saturating_mul(elapsed_boundaries)),
            ),
            MissedTickPolicy::CollapseToOne => ClockTick::new(
                consumed_due
                    .saturating_add(period.saturating_mul(elapsed_boundaries.saturating_add(1))),
            ),
        };
        let suppressed_interval_count = match interval.missed_tick_policy() {
            MissedTickPolicy::CatchUpAll => 0,
            MissedTickPolicy::SkipToLatest => elapsed_boundaries.saturating_sub(1),
            MissedTickPolicy::CollapseToOne => elapsed_boundaries,
        };

        if successor_due.get() <= consumed_due {
            return Err(SignalError::internal(format!(
                "interval regeneration failed to advance due tick for wake due at {} with period {}",
                consumed_due, period
            )));
        }

        Ok((successor_due, suppressed_interval_count))
    }

    pub fn schedule_owned_wake(
        &mut self,
        owner: TemporalWakeOwner,
        condition: TemporalCondition,
        due_tick: ClockTick,
        telemetry: Option<&mut TemporalTelemetry>,
    ) -> Result<ScheduledTemporalWake, SignalError> {
        self.admit_scheduled_wake(owner, condition, due_tick, telemetry, false)
    }

    pub fn retire_wakes_for_owner(
        &mut self,
        owner: TemporalWakeOwner,
        reason: TemporalWakeRetirementReason,
        mut telemetry: Option<&mut TemporalTelemetry>,
    ) -> Result<TemporalWakeRetirementBatch, SignalError> {
        let Some(owned) = self.owner_frontier.get(&owner) else {
            return Ok(TemporalWakeRetirementBatch::new(owner, reason, Vec::new()));
        };
        let wake_ids = owned.values().copied().collect::<Vec<_>>();
        let unique_wake_ids = wake_ids
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>();
        if unique_wake_ids.len() != wake_ids.len() {
            return Err(SignalError::internal(format!(
                "owner frontier for {owner:?} contained duplicate active wake ids"
            )));
        }
        let mut retired = Vec::with_capacity(wake_ids.len());
        for wake_id in wake_ids {
            retired.push(self.retire_wake(wake_id, reason, telemetry.as_deref_mut())?);
        }
        Ok(TemporalWakeRetirementBatch::new(owner, reason, retired))
    }
}
