use std::collections::BTreeSet;

use crate::data::error::SignalError;
use crate::data::node::{CheckpointNodeImage, NodeEvaluationConfig};
use crate::data::temporal::{
    ReadyTemporalWake, TemporalCondition, TemporalWakeId, TemporalWakeOwner,
    TemporalWakeRetirementBatch, TemporalWakeRetirementReason,
};
use crate::logic::planner::ExecutionReport;

use super::super::runtime_state::SignalRuntime;

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

    pub(super) fn ensure_active_temporal_wake_owner_live(
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

    pub fn retire_consumed_temporal_wakes_from_report(
        &mut self,
        report: &ExecutionReport,
    ) -> Result<(), SignalError> {
        let mut consumed = BTreeSet::new();
        for stage in &report.stages {
            for task in &stage.task_records {
                let Some(crate::data::temporal::LoweredTemporalEligibility::Ready(ready)) =
                    task.temporal_eligibility.as_ref()
                else {
                    continue;
                };
                if let Some(wake_id) = ready.wake_id() {
                    consumed.insert(wake_id);
                }
            }
        }
        for wake_id in consumed {
            let Some(owner) = self.temporal.active_wake_owner(wake_id) else {
                continue;
            };
            match self.temporal.ready_wake_for_owner(owner) {
                Some(ready) if matches!(ready.condition(), TemporalCondition::Interval(_)) => {
                    self.regenerate_interval_wake(wake_id)?;
                }
                Some(_) => {
                    self.retire_temporal_wake(wake_id, TemporalWakeRetirementReason::Consumed)?;
                }
                None => {}
            }
        }
        Ok(())
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
    ) -> Result<crate::data::temporal::RetiredTemporalWake, SignalError> {
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

    pub fn promote_due_temporal_wakes_ready(
        &mut self,
    ) -> Result<Vec<ReadyTemporalWake>, SignalError> {
        let mut ready = Vec::new();
        self.telemetry.temporal.temporal_broad_scan_denial_count += 1;
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

    pub fn promote_due_temporal_wakes_ready_with_summary(
        &mut self,
    ) -> Result<crate::data::temporal::TemporalReadyPromotionSummary, SignalError> {
        let frontier_before = self.temporal.frontier_snapshot();
        let broad_scan_denial_before = self.telemetry.temporal.temporal_broad_scan_denial_count;
        let ready_wakes = self.promote_due_temporal_wakes_ready()?;
        let broad_scan_denial_after = self.telemetry.temporal.temporal_broad_scan_denial_count;
        let frontier_after = self.temporal.frontier_snapshot();
        Ok(crate::data::temporal::TemporalReadyPromotionSummary::new(
            frontier_before,
            frontier_after,
            ready_wakes,
            broad_scan_denial_after.saturating_sub(broad_scan_denial_before),
        ))
    }

    pub fn reschedule_temporal_wake(
        &mut self,
        wake_id: TemporalWakeId,
        due_tick: crate::data::temporal::ClockTick,
    ) -> Result<crate::data::temporal::TemporalWakeReschedule, SignalError> {
        self.ensure_active_temporal_wake_owner_live(wake_id, "reschedule")?;
        self.temporal
            .reschedule_wake(wake_id, due_tick, &mut self.telemetry.temporal)
    }

    pub fn regenerate_interval_wake(
        &mut self,
        wake_id: TemporalWakeId,
    ) -> Result<crate::data::temporal::IntervalWakeRegeneration, SignalError> {
        self.ensure_active_temporal_wake_owner_live(wake_id, "regenerate")?;
        self.temporal
            .regenerate_interval_wake(wake_id, &mut self.telemetry.temporal)
    }
}
