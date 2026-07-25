use worth_store_physical_backend::BackendQueueExecutionBackpressure;

use super::completion::QueueBackendCompletionEvidence;
use super::outcome::{
    ExecutedQueueEvidence, QueueExecutionBackpressured, QueueExecutionDenied,
    QueueExecutionOutcome, QueueExecutionViolation, QueueExecutionViolationCause,
};
use super::{
    QueueBackpressureCause, QueueExecutionCounterBasis, QueueExecutionCounterSnapshot,
    QueueExecutionObservation, QueueExecutionReadyPlan, QueueExecutionUnitCounts,
    QueueGroupedReadyPlans, QueueReadAheadBasis, QueueWriteBackBasis,
};

pub(crate) fn execute_ready_queue_plan_with_backend_completion(
    plan: QueueExecutionReadyPlan,
    completion: QueueBackendCompletionEvidence,
) -> QueueExecutionOutcome {
    if completion.posture().profile() != plan.backend_profile()
        || completion.posture().evidence_class() != plan.backend_evidence_class()
        || completion.binding() != &plan.backend_completion_binding()
        || completion.grouped_writes() > 0
    {
        return QueueExecutionOutcome::Violation(QueueExecutionViolation {
            counters: QueueExecutionCounterSnapshot::violation_from_completion(
                QueueExecutionUnitCounts::all_admitted(submitted_units(plan.admitted_budget())),
                &completion,
                completion.grouped_writes(),
                completion.backpressure().map(map_backend_backpressure),
            ),
            plan: plan.execute_proof(),
            secondary_plan: None,
            cause: QueueExecutionViolationCause::BackendContradictedWitness,
        });
    }
    execute_ready_queue_plan_with_authoritative_group_count(plan, None, completion, 0)
}

pub(crate) fn execute_grouped_ready_queue_plans_with_backend_completion(
    grouped: QueueGroupedReadyPlans,
    completion: QueueBackendCompletionEvidence,
) -> QueueExecutionOutcome {
    let expected_binding = grouped.backend_completion_binding();
    let (plan, secondary_plan, grouped_writes) = grouped.into_execution_pair();
    if completion.binding() != &expected_binding || completion.grouped_writes() != grouped_writes {
        let submitted = submitted_units(plan.admitted_budget())
            + submitted_units(secondary_plan.admitted_budget());
        return QueueExecutionOutcome::Violation(QueueExecutionViolation {
            counters: QueueExecutionCounterSnapshot::violation_from_completion(
                QueueExecutionUnitCounts::all_admitted(submitted),
                &completion,
                completion.grouped_writes(),
                completion.backpressure().map(map_backend_backpressure),
            ),
            plan: plan.execute_proof(),
            secondary_plan: Some(secondary_plan.execute_proof()),
            cause: QueueExecutionViolationCause::BackendContradictedWitness,
        });
    }
    execute_ready_queue_plan_with_authoritative_group_count(
        plan,
        Some(secondary_plan),
        completion,
        grouped_writes,
    )
}

fn execute_ready_queue_plan_with_authoritative_group_count(
    plan: QueueExecutionReadyPlan,
    secondary_plan: Option<QueueExecutionReadyPlan>,
    completion: QueueBackendCompletionEvidence,
    grouped_writes: u32,
) -> QueueExecutionOutcome {
    let secondary_budget = secondary_plan
        .as_ref()
        .map(|plan| plan.admitted_budget())
        .unwrap_or_default();
    let submitted = submitted_units(plan.admitted_budget()) + submitted_units(secondary_budget);
    if completion.posture().profile() != plan.backend_profile()
        || completion.posture().evidence_class() != plan.backend_evidence_class()
        || completion.binding() != &plan.backend_completion_binding() && secondary_plan.is_none()
    {
        return QueueExecutionOutcome::Violation(QueueExecutionViolation {
            counters: QueueExecutionCounterSnapshot::violation_from_completion(
                QueueExecutionUnitCounts::all_admitted(submitted),
                &completion,
                completion.grouped_writes(),
                completion.backpressure().map(map_backend_backpressure),
            ),
            plan: plan.execute_proof(),
            secondary_plan: secondary_plan.map(QueueExecutionReadyPlan::execute_proof),
            cause: QueueExecutionViolationCause::BackendContradictedWitness,
        });
    }
    let mut lowered = QueueExecutionObservation {
        queue_depth_sample: completion.queue_depth_sample(),
        grouped_writes,
        read_ahead_units: completion.read_ahead_units(),
        read_ahead_scope: completion.read_ahead_scope(),
        write_back_units: completion.write_back_units(),
        write_back_scope: completion.write_back_scope(),
        mechanical_retries: completion.mechanical_retries(),
        partial_read_events: completion.partial_read_events(),
        short_write_events: completion.short_write_events(),
        backpressure_cause: completion.backpressure().map(map_backend_backpressure),
        foreground_wait_events: completion.foreground_wait_events(),
        attempted_work_class: None,
        attempted_durability_class: None,
        attempted_security_scope_identity: None,
    };
    if completion.backpressure() == Some(BackendQueueExecutionBackpressure::ReadAheadDenied) {
        lowered.read_ahead_units = lowered.read_ahead_units.max(1);
    }
    execute_queue_batch(plan, secondary_plan, lowered)
}

fn execute_queue_batch(
    plan: QueueExecutionReadyPlan,
    secondary_plan: Option<QueueExecutionReadyPlan>,
    observation: QueueExecutionObservation,
) -> QueueExecutionOutcome {
    let primary_budget = plan.admitted_budget();
    let secondary_budget = secondary_plan
        .as_ref()
        .map(|plan| plan.admitted_budget())
        .unwrap_or_default();
    let submitted = submitted_units(primary_budget) + submitted_units(secondary_budget);
    let read_ahead_limit =
        primary_budget.read_ahead_window() + secondary_budget.read_ahead_window();
    let write_back_limit =
        primary_budget.write_back_window() + secondary_budget.write_back_window();
    let queue_slot_limit = primary_budget.queue_slots() + secondary_budget.queue_slots();
    let read_ahead_basis =
        QueueReadAheadBasis::from_grouping(plan.grouping_basis(), read_ahead_limit);
    if observation.read_ahead_units > 0
        && !read_ahead_observation_admitted(read_ahead_basis, observation)
    {
        return denied_batch(
            plan,
            secondary_plan,
            observation.read_ahead_units,
            0,
            observation,
            QueueBackpressureCause::ReadAheadDenied,
        );
    }
    let write_back_basis =
        QueueWriteBackBasis::from_grouping(plan.grouping_basis(), write_back_limit);
    if observation.write_back_units > 0
        && !write_back_observation_admitted(write_back_basis, plan.grouping_basis(), observation)
    {
        return backpressured_batch(
            plan,
            secondary_plan,
            submitted,
            observation,
            QueueBackpressureCause::WriteBackWindowSaturated,
        );
    }
    if observation.read_ahead_units > read_ahead_limit {
        return denied_batch(
            plan,
            secondary_plan,
            observation.read_ahead_units,
            read_ahead_limit,
            observation,
            QueueBackpressureCause::ReadAheadDenied,
        );
    }
    if observation.write_back_units > write_back_limit {
        return backpressured_batch(
            plan,
            secondary_plan,
            submitted,
            observation,
            QueueBackpressureCause::WriteBackWindowSaturated,
        );
    }
    if queue_slot_limit > 0 && u64::from(observation.queue_depth_sample) > queue_slot_limit {
        return backpressured_batch(
            plan,
            secondary_plan,
            submitted,
            observation,
            QueueBackpressureCause::QueueDepthSaturated,
        );
    }
    if observation.foreground_wait_events > queue_slot_limit {
        return backpressured_batch(
            plan,
            secondary_plan,
            submitted,
            observation,
            QueueBackpressureCause::BackendTemporarilySaturated,
        );
    }
    if let Some(cause) = observation.backpressure_cause {
        return backpressured_batch(plan, secondary_plan, submitted, observation, cause);
    }
    QueueExecutionOutcome::Executed(ExecutedQueueEvidence {
        counters: QueueExecutionCounterSnapshot::executed(
            QueueExecutionCounterBasis::from_observation(
                QueueExecutionUnitCounts::all_admitted(submitted),
                observation,
            ),
        ),
        plan: plan.execute_proof(),
        secondary_plan: secondary_plan.map(QueueExecutionReadyPlan::execute_proof),
    })
}

fn denied_batch(
    plan: QueueExecutionReadyPlan,
    secondary_plan: Option<QueueExecutionReadyPlan>,
    submitted_units: u64,
    admitted_units: u64,
    observation: QueueExecutionObservation,
    cause: QueueBackpressureCause,
) -> QueueExecutionOutcome {
    QueueExecutionOutcome::Denied(QueueExecutionDenied {
        counters: QueueExecutionCounterSnapshot::denied(
            QueueExecutionCounterBasis::from_observation(
                QueueExecutionUnitCounts {
                    submitted: submitted_units,
                    admitted: admitted_units,
                },
                observation,
            ),
            cause,
        ),
        cause,
        plan: plan.execute_proof(),
        secondary_plan: secondary_plan.map(QueueExecutionReadyPlan::execute_proof),
    })
}

fn backpressured_batch(
    plan: QueueExecutionReadyPlan,
    secondary_plan: Option<QueueExecutionReadyPlan>,
    submitted: u64,
    observation: QueueExecutionObservation,
    cause: QueueBackpressureCause,
) -> QueueExecutionOutcome {
    QueueExecutionOutcome::Backpressured(QueueExecutionBackpressured {
        counters: QueueExecutionCounterSnapshot::backpressured(
            QueueExecutionCounterBasis::from_observation(
                QueueExecutionUnitCounts::all_admitted(submitted),
                observation,
            ),
            cause,
        ),
        cause,
        plan: plan.execute_proof(),
        secondary_plan: secondary_plan.map(QueueExecutionReadyPlan::execute_proof),
    })
}

fn read_ahead_observation_admitted(
    basis: QueueReadAheadBasis,
    observation: QueueExecutionObservation,
) -> bool {
    let Some(scope) = observation.read_ahead_scope else {
        return false;
    };
    basis.admits_scope(
        observation.read_ahead_units,
        scope.security_scope_identity(),
        scope.tenant_scope(),
        scope.key_scope(),
    )
}

fn write_back_observation_admitted(
    basis: QueueWriteBackBasis,
    grouping: &super::QueueGroupingBasis,
    observation: QueueExecutionObservation,
) -> bool {
    let Some(scope) = observation.write_back_scope else {
        return false;
    };
    basis.admits_scope(
        observation.write_back_units,
        scope.security_scope_identity(),
        scope.tenant_scope(),
        scope.key_scope(),
        grouping.writeback_policy(),
    )
}

pub(crate) const fn map_backend_backpressure(
    cause: BackendQueueExecutionBackpressure,
) -> QueueBackpressureCause {
    match cause {
        BackendQueueExecutionBackpressure::QueueDepthSaturated => {
            QueueBackpressureCause::QueueDepthSaturated
        }
        BackendQueueExecutionBackpressure::BandwidthSaturated => {
            QueueBackpressureCause::BandwidthSaturated
        }
        BackendQueueExecutionBackpressure::FlushDelayed => QueueBackpressureCause::FlushDelayed,
        BackendQueueExecutionBackpressure::WriteBackWindowSaturated => {
            QueueBackpressureCause::WriteBackWindowSaturated
        }
        BackendQueueExecutionBackpressure::ReadAheadDenied => {
            QueueBackpressureCause::ReadAheadDenied
        }
        BackendQueueExecutionBackpressure::BackgroundYielded => {
            QueueBackpressureCause::BackgroundYielded
        }
        BackendQueueExecutionBackpressure::BackendTemporarilySaturated => {
            QueueBackpressureCause::BackendTemporarilySaturated
        }
    }
}

pub(crate) const fn submitted_units(budget: crate::BackgroundResourceBudget) -> u64 {
    budget
        .queue_slots()
        .saturating_add(budget.bandwidth_tokens())
        .saturating_add(budget.flush_permits())
        .saturating_add(budget.sync_debt())
        .saturating_add(budget.read_ahead_window())
        .saturating_add(budget.write_back_window())
        .saturating_add(budget.dirty_page_budget())
        .saturating_add(budget.worker_permits())
        .saturating_add(budget.cache_residency_hints())
        .saturating_add(budget.reclaim_permits())
}
