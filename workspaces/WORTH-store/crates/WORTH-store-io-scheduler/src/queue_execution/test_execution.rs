use super::execution::submitted_units;
use super::{
    ExecutedQueueEvidence, QueueBackpressureCause, QueueExecutionBackpressured,
    QueueExecutionCounterSnapshot, QueueExecutionDenied, QueueExecutionObservation,
    QueueExecutionOutcome, QueueExecutionReadyPlan, QueueExecutionViolation,
    QueueExecutionViolationCause, QueueReadAheadBasis, QueueWriteBackBasis,
};

pub(crate) fn execute_admitted_queue_plan(
    plan: QueueExecutionReadyPlan,
    observation: QueueExecutionObservation,
) -> QueueExecutionOutcome {
    if observation_attempts_reclassification(&plan, observation) {
        return violation(plan, observation);
    }

    let budget = plan.admitted_budget();
    let submitted = submitted_units(budget);
    if read_ahead_crosses_admitted_scope(&plan, observation) {
        return denied(
            plan,
            observation.read_ahead_units,
            0,
            QueueBackpressureCause::ReadAheadDenied,
        );
    }
    if write_back_crosses_admitted_scope(&plan, observation) {
        return backpressured(
            plan,
            submitted,
            observation,
            QueueBackpressureCause::WriteBackWindowSaturated,
        );
    }
    if observation.read_ahead_units > budget.read_ahead_window() {
        return denied(
            plan,
            observation.read_ahead_units,
            budget.read_ahead_window(),
            QueueBackpressureCause::ReadAheadDenied,
        );
    }
    if observation.write_back_units > budget.write_back_window() {
        return backpressured(
            plan,
            submitted,
            observation,
            QueueBackpressureCause::WriteBackWindowSaturated,
        );
    }
    if budget.queue_slots() > 0 && u64::from(observation.queue_depth_sample) > budget.queue_slots()
    {
        return backpressured(
            plan,
            submitted,
            observation,
            QueueBackpressureCause::QueueDepthSaturated,
        );
    }
    if observation.foreground_wait_events > budget.queue_slots() {
        return backpressured(
            plan,
            submitted,
            observation,
            QueueBackpressureCause::BackendTemporarilySaturated,
        );
    }
    if let Some(cause) = observation.backpressure_cause {
        return backpressured(plan, submitted, observation, cause);
    }
    QueueExecutionOutcome::Executed(ExecutedQueueEvidence {
        counters: QueueExecutionCounterSnapshot::executed(
            submitted,
            submitted,
            observation.queue_depth_sample,
            observation.grouped_writes,
            observation.read_ahead_units,
            observation.write_back_units,
            observation.mechanical_retries,
            observation.partial_read_events,
            observation.short_write_events,
        ),
        plan: plan.execute_proof(),
        secondary_plan: None,
    })
}

fn observation_attempts_reclassification(
    plan: &QueueExecutionReadyPlan,
    observation: QueueExecutionObservation,
) -> bool {
    observation
        .attempted_work_class
        .is_some_and(|class| class != plan.work().class())
        || observation
            .attempted_durability_class
            .is_some_and(|class| class != plan.work().durability_class())
        || observation
            .attempted_security_scope_identity
            .is_some_and(|identity| identity != plan.work().security_scope_identity())
}

fn read_ahead_crosses_admitted_scope(
    plan: &QueueExecutionReadyPlan,
    observation: QueueExecutionObservation,
) -> bool {
    let Some(scope) = observation.read_ahead_scope else {
        return observation.read_ahead_units > 0;
    };
    observation.read_ahead_units > 0
        && !QueueReadAheadBasis::from_grouping(
            plan.grouping_basis(),
            plan.admitted_budget().read_ahead_window(),
        )
        .admits_scope(
            observation.read_ahead_units,
            scope.security_scope_identity(),
            scope.tenant_scope(),
            scope.key_scope(),
        )
}

fn write_back_crosses_admitted_scope(
    plan: &QueueExecutionReadyPlan,
    observation: QueueExecutionObservation,
) -> bool {
    let Some(scope) = observation.write_back_scope else {
        return observation.write_back_units > 0;
    };
    observation.write_back_units > 0
        && !QueueWriteBackBasis::from_grouping(
            plan.grouping_basis(),
            plan.admitted_budget().write_back_window(),
        )
        .admits_scope(
            observation.write_back_units,
            scope.security_scope_identity(),
            scope.tenant_scope(),
            scope.key_scope(),
            plan.grouping_basis().writeback_policy(),
        )
}

fn violation(
    plan: QueueExecutionReadyPlan,
    observation: QueueExecutionObservation,
) -> QueueExecutionOutcome {
    let submitted = submitted_units(plan.admitted_budget());
    QueueExecutionOutcome::Violation(QueueExecutionViolation {
        cause: QueueExecutionViolationCause::ExecutionReclassifiedWork,
        counters: QueueExecutionCounterSnapshot::violation_observed(
            submitted,
            submitted,
            observation.queue_depth_sample,
            observation.grouped_writes,
            observation.read_ahead_units,
            observation.write_back_units,
            observation.mechanical_retries,
            observation.partial_read_events,
            observation.short_write_events,
            observation.foreground_wait_events,
            observation.backpressure_cause,
        ),
        plan: plan.execute_proof(),
        secondary_plan: None,
    })
}

fn denied(
    plan: QueueExecutionReadyPlan,
    submitted: u64,
    admitted: u64,
    cause: QueueBackpressureCause,
) -> QueueExecutionOutcome {
    QueueExecutionOutcome::Denied(QueueExecutionDenied {
        counters: QueueExecutionCounterSnapshot::denied(
            submitted, admitted, 0, submitted, 0, 0, 0, 0, cause,
        ),
        cause,
        plan: plan.execute_proof(),
        secondary_plan: None,
    })
}

fn backpressured(
    plan: QueueExecutionReadyPlan,
    submitted: u64,
    observation: QueueExecutionObservation,
    cause: QueueBackpressureCause,
) -> QueueExecutionOutcome {
    QueueExecutionOutcome::Backpressured(QueueExecutionBackpressured {
        counters: QueueExecutionCounterSnapshot::backpressured(
            submitted,
            submitted,
            observation.queue_depth_sample,
            observation.grouped_writes,
            observation.read_ahead_units,
            observation.write_back_units,
            observation.mechanical_retries,
            observation.partial_read_events,
            observation.short_write_events,
            cause,
            observation.foreground_wait_events,
        ),
        cause,
        plan: plan.execute_proof(),
        secondary_plan: None,
    })
}
