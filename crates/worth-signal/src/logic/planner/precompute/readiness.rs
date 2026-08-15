use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::proof::invalidation::progression::{
    InvalidationReadinessEpoch, InvalidationStageOrder,
};
use crate::data::proof::invalidation::revalidation::NodeInvalidationInput;
use crate::logic::invalidation::scheduling::{
    admit_current_readiness, lower_current_work, ReadyInvalidationQueue, ReadyQueueEntry,
};
use crate::logic::planner::EligibleTask;
use crate::logic::prepared::PreparedEvaluation;

use super::eligibility::PrevalidatedTask;

pub(super) fn attach_ready_invalidation(
    graph: &mut SignalGraph,
    tasks: &[EligibleTask],
    stage_index: u32,
    readiness_epoch: InvalidationReadinessEpoch,
    prevalidated: &mut [PrevalidatedTask],
) -> Result<(), SignalError> {
    let mut queue = ReadyInvalidationQueue::new();
    let mut duplicates = Vec::new();
    for (task_index, (task, posture)) in tasks.iter().zip(prevalidated.iter()).enumerate() {
        if !matches!(posture, PrevalidatedTask::NeedsCompute { .. }) {
            continue;
        }
        let NodeInvalidationInput::Resolved(input) = graph.node_invalidation_input(task.node)?
        else {
            continue;
        };
        let stage_order = InvalidationStageOrder {
            stage: stage_index,
            order: task_index as u32,
        };
        let ready = lower_and_admit(
            graph,
            task.node,
            input.clone(),
            readiness_epoch,
            stage_order,
        )?;
        if !queue.insert(graph, ReadyQueueEntry { task_index, ready })? {
            duplicates.push(task_index);
        }
        for _ in 0..graph.take_repeated_invalidation_admissions(task.node) {
            let ready = lower_and_admit(
                graph,
                task.node,
                input.clone(),
                readiness_epoch,
                stage_order,
            )?;
            if queue.insert(graph, ReadyQueueEntry { task_index, ready })? {
                return Err(SignalError::internal(
                    "repeated same-epoch admission did not merge with current ready work",
                ));
            }
        }
    }
    suppress_duplicate_tasks(graph, tasks, prevalidated, duplicates)?;
    assign_ready_work(graph, prevalidated, &mut queue)
}

fn lower_and_admit(
    graph: &SignalGraph,
    target: crate::data::handle::NodeId,
    input: crate::data::proof::invalidation::revalidation::CanonicalDependencyCauseSet,
    epoch: InvalidationReadinessEpoch,
    order: InvalidationStageOrder,
) -> Result<crate::data::proof::invalidation::progression::ReadyInvalidationBatch, SignalError> {
    let lowered = lower_current_work(graph, target, input, epoch, order)?;
    admit_current_readiness(graph, lowered, epoch, order)
}

fn suppress_duplicate_tasks(
    graph: &SignalGraph,
    tasks: &[EligibleTask],
    prevalidated: &mut [PrevalidatedTask],
    duplicates: Vec<usize>,
) -> Result<(), SignalError> {
    for task_index in duplicates {
        let dependencies = super::super::validation::capture_current_dependencies_without_refresh(
            graph,
            tasks[task_index].node,
        )?;
        prevalidated[task_index] = PrevalidatedTask::Prepared(
            PreparedEvaluation::validated_clean().with_dependencies(dependencies),
        );
    }
    Ok(())
}

fn assign_ready_work(
    graph: &mut SignalGraph,
    prevalidated: &mut [PrevalidatedTask],
    queue: &mut ReadyInvalidationQueue,
) -> Result<(), SignalError> {
    while let Some(entry) = queue.pop(graph)? {
        let PrevalidatedTask::NeedsCompute {
            ready_invalidation, ..
        } = &mut prevalidated[entry.task_index]
        else {
            return Err(SignalError::internal(
                "ready invalidation queue targeted a non-compute task",
            ));
        };
        *ready_invalidation = Some(entry.ready);
    }
    Ok(())
}
