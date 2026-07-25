use worth_store_physical_backend::{
    preserve_secure_io_for_backend_completion, BackendQueueExecutionCompletion,
};

use crate::IoSchedulerBackendCapabilityRequirement;

use super::adjudication::{
    execute_grouped_ready_queue_plans_with_backend_completion,
    execute_ready_queue_plan_with_backend_completion, map_backend_backpressure, submitted_units,
};
use super::completion::{QueueBackendCompletionAuthority, QueueBackendCompletionEvidence};
use super::{
    QueueExecutionCounterSnapshot, QueueExecutionOutcome, QueueExecutionReadyPlan,
    QueueExecutionUnitCounts, QueueExecutionViolation, QueueExecutionViolationCause,
    QueueGroupedReadyPlans,
};

pub fn execute_ready_queue_plan(
    plan: QueueExecutionReadyPlan,
    completion: BackendQueueExecutionCompletion,
) -> QueueExecutionOutcome {
    if secure_frame_backend_completion_is_invalid(&plan, completion) {
        return QueueExecutionOutcome::Violation(QueueExecutionViolation {
            cause: QueueExecutionViolationCause::BackendContradictedWitness,
            counters: completion_violation_counters(
                QueueExecutionUnitCounts::all_admitted(submitted_units(plan.admitted_budget())),
                completion,
            ),
            plan: plan.execute_proof(),
            secondary_plan: None,
        });
    }
    let authority = QueueBackendCompletionAuthority::for_ready_plan(&plan, completion);
    let Ok(authority) = authority else {
        return QueueExecutionOutcome::Violation(QueueExecutionViolation {
            cause: QueueExecutionViolationCause::BackendContradictedWitness,
            counters: completion_violation_counters(
                QueueExecutionUnitCounts::all_admitted(submitted_units(plan.admitted_budget())),
                completion,
            ),
            plan: plan.execute_proof(),
            secondary_plan: None,
        });
    };
    let completion = QueueBackendCompletionEvidence::from_backend_completion(authority, completion);
    execute_ready_queue_plan_with_backend_completion(plan, completion)
}

pub fn execute_grouped_ready_queue_plans(
    grouped: QueueGroupedReadyPlans,
    completion: BackendQueueExecutionCompletion,
) -> QueueExecutionOutcome {
    if secure_frame_backend_completion_is_invalid(grouped.first(), completion) {
        let first = grouped.first();
        let second = grouped.second();
        let submitted =
            submitted_units(first.admitted_budget()) + submitted_units(second.admitted_budget());
        let (plan, secondary_plan, _) = grouped.into_execution_pair();
        return QueueExecutionOutcome::Violation(QueueExecutionViolation {
            cause: QueueExecutionViolationCause::BackendContradictedWitness,
            counters: completion_violation_counters(
                QueueExecutionUnitCounts::all_admitted(submitted),
                completion,
            ),
            plan: plan.execute_proof(),
            secondary_plan: Some(secondary_plan.execute_proof()),
        });
    }
    let authority = QueueBackendCompletionAuthority::for_grouped_plans(&grouped, completion);
    let Ok(authority) = authority else {
        let first = grouped.first();
        let second = grouped.second();
        let submitted =
            submitted_units(first.admitted_budget()) + submitted_units(second.admitted_budget());
        let (plan, secondary_plan, _) = grouped.into_execution_pair();
        return QueueExecutionOutcome::Violation(QueueExecutionViolation {
            cause: QueueExecutionViolationCause::BackendContradictedWitness,
            counters: completion_violation_counters(
                QueueExecutionUnitCounts::all_admitted(submitted),
                completion,
            ),
            plan: plan.execute_proof(),
            secondary_plan: Some(secondary_plan.execute_proof()),
        });
    };
    let completion = QueueBackendCompletionEvidence::from_backend_completion(authority, completion);
    execute_grouped_ready_queue_plans_with_backend_completion(grouped, completion)
}

fn secure_frame_backend_completion_is_invalid(
    plan: &QueueExecutionReadyPlan,
    completion: BackendQueueExecutionCompletion,
) -> bool {
    plan.work().backend_requirement() == IoSchedulerBackendCapabilityRequirement::SecureFrameIo
        && preserve_secure_io_for_backend_completion(completion).is_err()
}

fn completion_violation_counters(
    units: QueueExecutionUnitCounts,
    completion: BackendQueueExecutionCompletion,
) -> QueueExecutionCounterSnapshot {
    QueueExecutionCounterSnapshot::violation_from_backend_completion(
        units,
        completion,
        completion.backpressure().map(map_backend_backpressure),
    )
}
