use std::collections::{BTreeMap, BTreeSet};

use crate::data::error::SignalError;
use crate::data::node::{CheckpointNodeImage, NodeEvaluationConfig};
use crate::data::telemetry::TemporalTelemetry;
use crate::data::temporal::{
    ClockAdvanceRequest, ClockTick, IntervalCondition, IntervalWakeRegeneration, MissedTickPolicy,
    PreviousValueRevision, ReadyTemporalWake, RetiredTemporalWake, RuntimeClockBasis,
    ScheduledTemporalWake, TemporalCondition, TemporalFrontierSnapshot,
    TemporalPreviousValueAccess, TemporalPreviousValueReference, TemporalWakeId, TemporalWakeOwner,
    TemporalWakeReschedule, TemporalWakeRetirementBatch, TemporalWakeRetirementReason,
    TemporalWakeSummary, ValidatedClockAdvance, WakeOrdinal,
};
use crate::state::SignalBranchId;

use super::runtime_state::SignalRuntime;

/// Runtime-owned temporal state for authoritative clock basis semantics and wake lifecycle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::logic::transaction::runtime) struct TemporalRuntimeState {
    clock_basis: RuntimeClockBasis,
    previous_value_capability_epoch: u64,
    next_wake_id: TemporalWakeId,
    next_wake_ordinal: WakeOrdinal,
    next_previous_value_revision: PreviousValueRevision,
    scheduled_wakes: BTreeMap<TemporalWakeId, ScheduledTemporalWake>,
    scheduled_frontier: BTreeMap<ClockTick, BTreeMap<WakeOrdinal, TemporalWakeId>>,
    ready_wakes: BTreeMap<TemporalWakeId, ReadyTemporalWake>,
    ready_frontier: BTreeMap<WakeOrdinal, TemporalWakeId>,
    owner_frontier: BTreeMap<TemporalWakeOwner, BTreeMap<WakeOrdinal, TemporalWakeId>>,
    retired_wakes: BTreeMap<TemporalWakeId, RetiredTemporalWake>,
}

impl Default for TemporalRuntimeState {
    fn default() -> Self {
        Self {
            clock_basis: RuntimeClockBasis::default(),
            previous_value_capability_epoch: 0,
            next_wake_id: TemporalWakeId::new(0),
            next_wake_ordinal: WakeOrdinal::ZERO,
            next_previous_value_revision: PreviousValueRevision::ZERO,
            scheduled_wakes: BTreeMap::new(),
            scheduled_frontier: BTreeMap::new(),
            ready_wakes: BTreeMap::new(),
            ready_frontier: BTreeMap::new(),
            owner_frontier: BTreeMap::new(),
            retired_wakes: BTreeMap::new(),
        }
    }
}

impl TemporalRuntimeState {
    pub fn clock_basis(&self) -> RuntimeClockBasis {
        self.clock_basis
    }

    pub fn validate_clock_advance(
        &self,
        request: ClockAdvanceRequest,
    ) -> Result<ValidatedClockAdvance, SignalError> {
        self.clock_basis.validate_advance(request)
    }

    pub fn apply_clock_advance(&mut self, validated: ValidatedClockAdvance) {
        self.clock_basis.apply_validated_advance(validated);
    }

    pub fn bump_previous_value_capability_epoch(&mut self) {
        self.previous_value_capability_epoch =
            self.previous_value_capability_epoch.saturating_add(1);
    }

    pub fn wake_summary(&self) -> TemporalWakeSummary {
        TemporalWakeSummary::new(
            self.scheduled_wakes.len(),
            self.ready_wakes.len(),
            self.retired_wakes.len(),
            self.next_wake_id,
            self.next_wake_ordinal,
        )
    }

    pub fn frontier_snapshot(&self) -> TemporalFrontierSnapshot {
        let next_due = self
            .scheduled_frontier
            .iter()
            .next()
            .and_then(|(tick, wakes)| {
                wakes
                    .iter()
                    .next()
                    .map(|(ordinal, wake_id)| (*tick, *ordinal, *wake_id))
            });
        let next_ready = self
            .ready_frontier
            .iter()
            .next()
            .map(|(ordinal, wake_id)| (*ordinal, *wake_id));

        TemporalFrontierSnapshot::new(
            self.scheduled_frontier.len(),
            self.ready_frontier.len(),
            next_due.map(|(tick, _, _)| tick),
            next_due.map(|(_, _, wake_id)| wake_id),
            next_due.map(|(_, ordinal, _)| ordinal),
            next_ready.map(|(_, wake_id)| wake_id),
            next_ready.map(|(ordinal, _)| ordinal),
        )
    }

    pub fn schedule_wake(
        &mut self,
        condition: TemporalCondition,
        due_tick: ClockTick,
        telemetry: &mut TemporalTelemetry,
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
        telemetry: &mut TemporalTelemetry,
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
        telemetry.temporal_wake_count += 1;
        telemetry.scheduled_frontier_width = telemetry
            .scheduled_frontier_width
            .max(self.scheduled_frontier.len() as u64);
        telemetry.wake_allocation_count += 1;
        telemetry.ready_queue_width = telemetry
            .ready_queue_width
            .max(self.ready_wakes.len() as u64);
        Ok(wake)
    }

    pub fn reschedule_wake(
        &mut self,
        wake_id: TemporalWakeId,
        due_tick: ClockTick,
        telemetry: &mut TemporalTelemetry,
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

        let retired =
            self.retire_wake(wake_id, TemporalWakeRetirementReason::Superseded, telemetry)?;
        let scheduled = self.admit_scheduled_wake(owner, condition, due_tick, telemetry, false)?;
        telemetry.rescheduled_wake_count += 1;
        Ok(TemporalWakeReschedule::new(retired, scheduled))
    }

    pub fn promote_wake_ready(
        &mut self,
        wake_id: TemporalWakeId,
        telemetry: &mut TemporalTelemetry,
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
        telemetry.ready_queue_width = telemetry
            .ready_queue_width
            .max(self.ready_wakes.len() as u64);
        Ok(ready)
    }

    pub fn retire_wake(
        &mut self,
        wake_id: TemporalWakeId,
        reason: TemporalWakeRetirementReason,
        telemetry: &mut TemporalTelemetry,
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
            telemetry.retired_wake_count += 1;
            telemetry.ready_queue_width = telemetry
                .ready_queue_width
                .max(self.ready_wakes.len() as u64);
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
            telemetry.retired_wake_count += 1;
            return Ok(retired);
        }

        Err(SignalError::invalid_input(format!(
            "cannot retire unknown temporal wake {}",
            wake_id.get()
        )))
    }

    pub fn grant_previous_value_access(
        &self,
        branch_id: SignalBranchId,
        wake_id: TemporalWakeId,
    ) -> Result<TemporalPreviousValueAccess, SignalError> {
        let ready = self.ready_wakes.get(&wake_id).ok_or_else(|| {
            SignalError::invalid_input(format!(
                "cannot grant previous-value access from non-ready temporal wake {}",
                wake_id.get()
            ))
        })?;
        Ok(TemporalPreviousValueAccess::from_ready_wake(
            branch_id,
            self.previous_value_capability_epoch,
            ready,
        ))
    }

    pub fn capture_previous_value_reference(
        &mut self,
        access: &TemporalPreviousValueAccess,
        node: crate::data::handle::NodeId,
        aspect_version: crate::data::aspect::AspectVersion,
        output_identity: Option<crate::data::output::OutputIdentity>,
        telemetry: &mut TemporalTelemetry,
    ) -> Result<TemporalPreviousValueReference, SignalError> {
        let Some(ready) = self.ready_wakes.get(&access.wake_id()) else {
            return Err(SignalError::invalid_input(format!(
                "cannot capture previous value from inactive temporal access wake {}",
                access.wake_id().get()
            )));
        };
        if access.capability_epoch() != self.previous_value_capability_epoch {
            return Err(SignalError::invalid_input(format!(
                "temporal previous-value access for wake {} belongs to stale restore epoch {} but active epoch is {}",
                access.wake_id().get(),
                access.capability_epoch(),
                self.previous_value_capability_epoch
            )));
        }
        if ready.ready_ordinal() != access.ready_ordinal()
            || ready.ready_tick() != access.ready_tick()
        {
            return Err(SignalError::invalid_input(format!(
                "temporal previous-value access for wake {} no longer matches active ready proof",
                access.wake_id().get()
            )));
        }

        let revision = self.issue_previous_value_revision();
        telemetry.previous_value_reference_count += 1;
        Ok(TemporalPreviousValueReference::new(
            revision,
            access,
            node,
            aspect_version,
            output_identity,
        ))
    }

    pub fn active_wake_owner(&self, wake_id: TemporalWakeId) -> Option<TemporalWakeOwner> {
        self.scheduled_wakes
            .get(&wake_id)
            .map(ScheduledTemporalWake::owner)
            .or_else(|| self.ready_wakes.get(&wake_id).map(ReadyTemporalWake::owner))
    }

    fn issue_wake_id(&mut self) -> TemporalWakeId {
        let id = self.next_wake_id;
        self.next_wake_id = TemporalWakeId::new(id.get().saturating_add(1));
        id
    }

    fn issue_wake_ordinal(&mut self) -> WakeOrdinal {
        self.next_wake_ordinal = self.next_wake_ordinal.next();
        self.next_wake_ordinal
    }

    fn issue_previous_value_revision(&mut self) -> PreviousValueRevision {
        self.next_previous_value_revision = self.next_previous_value_revision.next();
        self.next_previous_value_revision
    }

    pub fn regenerate_interval_wake(
        &mut self,
        wake_id: TemporalWakeId,
        telemetry: &mut TemporalTelemetry,
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
        let retired =
            self.retire_wake(wake_id, TemporalWakeRetirementReason::Consumed, telemetry)?;
        let scheduled = self.admit_scheduled_wake(
            ready.owner(),
            TemporalCondition::Interval(interval),
            successor_due,
            telemetry,
            true,
        )?;
        telemetry.rescheduled_wake_count += 1;

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
                consumed_due,
                period
            )));
        }

        Ok((successor_due, suppressed_interval_count))
    }

    pub fn schedule_owned_wake(
        &mut self,
        owner: TemporalWakeOwner,
        condition: TemporalCondition,
        due_tick: ClockTick,
        telemetry: &mut TemporalTelemetry,
    ) -> Result<ScheduledTemporalWake, SignalError> {
        self.admit_scheduled_wake(owner, condition, due_tick, telemetry, false)
    }

    pub fn retire_wakes_for_owner(
        &mut self,
        owner: TemporalWakeOwner,
        reason: TemporalWakeRetirementReason,
        telemetry: &mut TemporalTelemetry,
    ) -> Result<TemporalWakeRetirementBatch, SignalError> {
        let Some(owned) = self.owner_frontier.get(&owner) else {
            return Ok(TemporalWakeRetirementBatch::new(owner, reason, Vec::new()));
        };
        let wake_ids = owned.values().copied().collect::<Vec<_>>();
        let unique_wake_ids = wake_ids.iter().copied().collect::<BTreeSet<_>>();
        if unique_wake_ids.len() != wake_ids.len() {
            return Err(SignalError::internal(format!(
                "owner frontier for {owner:?} contained duplicate active wake ids"
            )));
        }
        let mut retired = Vec::with_capacity(wake_ids.len());
        for wake_id in wake_ids {
            retired.push(self.retire_wake(wake_id, reason, telemetry)?);
        }
        Ok(TemporalWakeRetirementBatch::new(owner, reason, retired))
    }

    fn insert_scheduled_frontier_entry(&mut self, wake: &ScheduledTemporalWake) {
        self.scheduled_frontier
            .entry(wake.due_tick())
            .or_default()
            .insert(wake.ordinal(), wake.id());
    }

    fn remove_scheduled_frontier_entry(&mut self, wake: &ScheduledTemporalWake) {
        if let Some(bucket) = self.scheduled_frontier.get_mut(&wake.due_tick()) {
            bucket.remove(&wake.ordinal());
            if bucket.is_empty() {
                self.scheduled_frontier.remove(&wake.due_tick());
            }
        }
    }

    fn insert_owner_frontier_entry(
        &mut self,
        owner: TemporalWakeOwner,
        ordinal: WakeOrdinal,
        wake_id: TemporalWakeId,
    ) {
        self.owner_frontier
            .entry(owner)
            .or_default()
            .insert(ordinal, wake_id);
    }

    fn remove_owner_frontier_entry(
        &mut self,
        owner: TemporalWakeOwner,
        ordinal: WakeOrdinal,
        wake_id: TemporalWakeId,
    ) {
        if let Some(bucket) = self.owner_frontier.get_mut(&owner) {
            if bucket.get(&ordinal).copied() == Some(wake_id) {
                bucket.remove(&ordinal);
            }
            if bucket.is_empty() {
                self.owner_frontier.remove(&owner);
            }
        }
    }
}

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    fn retire_temporal_wake_if_owner_stale(
        &mut self,
        wake_id: TemporalWakeId,
    ) -> Result<Option<TemporalWakeOwner>, SignalError> {
        let Some(owner) = self.temporal.active_wake_owner(wake_id) else {
            return Ok(None);
        };
        if self.validate_temporal_wake_owner(owner).is_ok() {
            return Ok(None);
        }

        let _ = self.temporal.retire_wake(
            wake_id,
            TemporalWakeRetirementReason::Disposed,
            &mut self.telemetry.temporal,
        );
        Ok(Some(owner))
    }

    fn validate_temporal_wake_owner(&self, owner: TemporalWakeOwner) -> Result<(), SignalError> {
        match owner {
            TemporalWakeOwner::Manual => Ok(()),
            TemporalWakeOwner::Node(node) => {
                if self.graph.is_alive(node) {
                    Ok(())
                } else {
                    Err(SignalError::invalid_input(format!(
                        "cannot admit temporal wake for non-live node owner {node}"
                    )))
                }
            }
        }
    }

    fn ensure_active_temporal_wake_owner_live(
        &mut self,
        wake_id: TemporalWakeId,
        action: &str,
    ) -> Result<(), SignalError> {
        if let Some(owner) = self.retire_temporal_wake_if_owner_stale(wake_id)? {
            return Err(SignalError::invalid_input(format!(
                "cannot {action} temporal wake {} because owner {:?} is no longer live",
                wake_id.get(),
                owner
            )));
        }
        Ok(())
    }

    pub fn clock_basis(&self) -> RuntimeClockBasis {
        self.temporal.clock_basis()
    }

    pub fn validate_clock_advance(
        &self,
        request: ClockAdvanceRequest,
    ) -> Result<ValidatedClockAdvance, SignalError> {
        self.temporal.validate_clock_advance(request)
    }

    pub fn advance_clock(
        &mut self,
        request: ClockAdvanceRequest,
    ) -> Result<ValidatedClockAdvance, SignalError> {
        let validated = self.validate_clock_advance(request)?;
        self.temporal.apply_clock_advance(validated);
        Ok(validated)
    }

    pub fn schedule_temporal_wake(
        &mut self,
        condition: TemporalCondition,
        due_tick: ClockTick,
    ) -> Result<ScheduledTemporalWake, SignalError> {
        self.temporal
            .schedule_wake(condition, due_tick, &mut self.telemetry.temporal)
    }

    pub fn schedule_owned_temporal_wake(
        &mut self,
        owner: TemporalWakeOwner,
        condition: TemporalCondition,
        due_tick: ClockTick,
    ) -> Result<ScheduledTemporalWake, SignalError> {
        self.validate_temporal_wake_owner(owner)?;
        self.temporal
            .schedule_owned_wake(owner, condition, due_tick, &mut self.telemetry.temporal)
    }

    pub fn promote_temporal_wake_ready(
        &mut self,
        wake_id: TemporalWakeId,
    ) -> Result<ReadyTemporalWake, SignalError> {
        self.ensure_active_temporal_wake_owner_live(wake_id, "promote")?;
        self.temporal
            .promote_wake_ready(wake_id, &mut self.telemetry.temporal)
    }

    pub fn retire_temporal_wake(
        &mut self,
        wake_id: TemporalWakeId,
        reason: TemporalWakeRetirementReason,
    ) -> Result<RetiredTemporalWake, SignalError> {
        self.temporal
            .retire_wake(wake_id, reason, &mut self.telemetry.temporal)
    }

    pub fn retire_temporal_wakes_for_owner(
        &mut self,
        owner: TemporalWakeOwner,
        reason: TemporalWakeRetirementReason,
    ) -> Result<TemporalWakeRetirementBatch, SignalError> {
        self.temporal
            .retire_wakes_for_owner(owner, reason, &mut self.telemetry.temporal)
    }

    pub fn unregister_node(
        &mut self,
        node: crate::data::handle::NodeId,
    ) -> Result<TemporalWakeRetirementBatch, SignalError> {
        if !self.graph.is_alive(node) {
            return Err(SignalError::invalid_input(format!(
                "cannot unregister non-live node owner {node}"
            )));
        }

        self.graph.unregister_node(node)?;
        self.temporal.retire_wakes_for_owner(
            TemporalWakeOwner::Node(node),
            TemporalWakeRetirementReason::Disposed,
            &mut self.telemetry.temporal,
        )
    }

    pub fn replace_node_from_checkpoint_image(
        &mut self,
        node: crate::data::handle::NodeId,
        image: CheckpointNodeImage,
    ) -> Result<TemporalWakeRetirementBatch, SignalError> {
        if !self.graph.is_alive(node) {
            return Err(SignalError::invalid_input(format!(
                "cannot replace checkpoint image for non-live node owner {node}"
            )));
        }

        self.graph
            .replace_entry_from_checkpoint_image(node, image)?;
        self.temporal.retire_wakes_for_owner(
            TemporalWakeOwner::Node(node),
            TemporalWakeRetirementReason::Superseded,
            &mut self.telemetry.temporal,
        )
    }

    pub fn replace_node_evaluation_config(
        &mut self,
        node: crate::data::handle::NodeId,
        eval_config: NodeEvaluationConfig,
    ) -> Result<TemporalWakeRetirementBatch, SignalError> {
        if !self.graph.is_alive(node) {
            return Err(SignalError::invalid_input(format!(
                "cannot rewrite evaluation config for non-live node owner {node}"
            )));
        }

        self.graph.get_entry_mut(node)?.set_eval_config(eval_config);
        self.temporal.retire_wakes_for_owner(
            TemporalWakeOwner::Node(node),
            TemporalWakeRetirementReason::Superseded,
            &mut self.telemetry.temporal,
        )
    }

    pub fn temporal_wake_summary(&self) -> TemporalWakeSummary {
        self.temporal.wake_summary()
    }

    pub fn temporal_frontier_snapshot(&self) -> TemporalFrontierSnapshot {
        self.temporal.frontier_snapshot()
    }

    pub fn grant_temporal_previous_value_access(
        &mut self,
        wake_id: TemporalWakeId,
    ) -> Result<TemporalPreviousValueAccess, SignalError> {
        self.ensure_active_temporal_wake_owner_live(wake_id, "grant previous-value access")?;
        self.temporal
            .grant_previous_value_access(self.graph.current_branch().id, wake_id)
    }

    pub fn previous_temporal_value(
        &mut self,
        access: &TemporalPreviousValueAccess,
        node: crate::data::handle::NodeId,
    ) -> Result<TemporalPreviousValueReference, SignalError> {
        let current_branch = self.graph.current_branch();
        if access.branch_id() != current_branch.id {
            return Err(SignalError::invalid_input(format!(
                "temporal previous-value access belongs to branch {} but current branch is {}",
                access.branch_id().0,
                current_branch.id.0
            )));
        }
        let aspect_version = self.graph.node_aspect_version(node)?;
        let output_identity = self
            .graph
            .observe()
            .runtime_artifact_warm(node)?
            .and_then(|warm| warm.output_identity.clone());
        self.temporal.capture_previous_value_reference(
            access,
            node,
            aspect_version,
            output_identity,
            &mut self.telemetry.temporal,
        )
    }

    pub fn promote_due_temporal_wakes_ready(
        &mut self,
    ) -> Result<Vec<ReadyTemporalWake>, SignalError> {
        let mut ready = Vec::new();
        loop {
            let frontier = self.temporal.frontier_snapshot();
            let Some(next_due_tick) = frontier.next_due_tick() else {
                break;
            };
            if next_due_tick > self.temporal.clock_basis().current_tick() {
                break;
            }
            let Some(wake_id) = frontier.next_due_wake_id() else {
                return Err(SignalError::internal(
                    "temporal frontier reported due tick without a due wake id",
                ));
            };
            if self.retire_temporal_wake_if_owner_stale(wake_id)?.is_some() {
                continue;
            }
            ready.push(self.promote_temporal_wake_ready(wake_id)?);
        }
        self.telemetry.temporal.temporal_eligibility_lowering_count += ready.len() as u64;
        Ok(ready)
    }

    pub fn reschedule_temporal_wake(
        &mut self,
        wake_id: TemporalWakeId,
        due_tick: ClockTick,
    ) -> Result<TemporalWakeReschedule, SignalError> {
        self.ensure_active_temporal_wake_owner_live(wake_id, "reschedule")?;
        self.temporal
            .reschedule_wake(wake_id, due_tick, &mut self.telemetry.temporal)
    }

    pub fn regenerate_interval_wake(
        &mut self,
        wake_id: TemporalWakeId,
    ) -> Result<IntervalWakeRegeneration, SignalError> {
        self.ensure_active_temporal_wake_owner_live(wake_id, "regenerate")?;
        self.temporal
            .regenerate_interval_wake(wake_id, &mut self.telemetry.temporal)
    }
}
