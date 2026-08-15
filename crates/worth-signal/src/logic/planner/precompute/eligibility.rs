use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::EvaluationCondition;
use crate::data::proof::invalidation::revalidation::NodeInvalidationInput;
use crate::data::temporal::{
    DeferredTemporalEligibility, LoweredTemporalEligibility, ReadyTemporalEligibility,
    TemporalCondition,
};
use crate::logic::evaluation::{
    ConditionEvaluationContext, ConditionResolver, DefaultConditionResolver,
};
use crate::logic::prepared::PreparedEvaluation;

use super::super::types::EligibleTask;
use super::super::validation::capture_current_dependencies_without_refresh;
use super::temporal::TemporalLoweringContext;

#[derive(Debug)]
pub(super) enum PrevalidatedTask {
    Prepared(PreparedEvaluation),
    NeedsCompute {
        temporal_ready: Option<ReadyTemporalEligibility>,
        ready_invalidation:
            Option<crate::data::proof::invalidation::progression::ReadyInvalidationBatch>,
    },
}

pub(super) fn prevalidate_stage_tasks(
    graph: &mut SignalGraph,
    tasks: &[EligibleTask],
    stage_index: u32,
    readiness_epoch: crate::data::proof::invalidation::progression::InvalidationReadinessEpoch,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    temporal_lowering: &TemporalLoweringContext,
) -> Result<Vec<PrevalidatedTask>, SignalError> {
    let mut prevalidated = Vec::with_capacity(tasks.len());
    for task in tasks {
        let prepared = if let Some(prepared) = prepare_invalidation_outcome(graph, task)? {
            prepared
        } else if let Some(prepared) =
            prepare_condition_outcome_if_blocked(graph, task, temporal_lowering)?
        {
            prepared
        } else {
            prepare_validated_clean_if_unchanged(graph, task, comparator_resolver)?.unwrap_or(
                PrevalidatedTask::NeedsCompute {
                    temporal_ready: None,
                    ready_invalidation: None,
                },
            )
        };
        prevalidated.push(prepared);
    }
    super::readiness::attach_ready_invalidation(
        graph,
        tasks,
        stage_index,
        readiness_epoch,
        &mut prevalidated,
    )?;
    Ok(prevalidated)
}

fn prepare_invalidation_outcome(
    graph: &SignalGraph,
    task: &EligibleTask,
) -> Result<Option<PrevalidatedTask>, SignalError> {
    match graph.node_invalidation_input(task.node)? {
        NodeInvalidationInput::Pending(_) => {
            let dependencies = capture_current_dependencies_without_refresh(graph, task.node)?;
            Ok(Some(PrevalidatedTask::Prepared(
                PreparedEvaluation::deferred_by_invalidation().with_dependencies(dependencies),
            )))
        }
        NodeInvalidationInput::Resolved(_) => {
            let structural = graph
                .pending_dependency_revalidation(task.node)?
                .is_some_and(|pending| pending.requires_structural_recompute());
            Ok(structural.then_some(PrevalidatedTask::NeedsCompute {
                temporal_ready: None,
                ready_invalidation: None,
            }))
        }
        NodeInvalidationInput::ResolvedNoChange(_) => {
            let validates_clean = matches!(
                task.admission.node_state_at_admission,
                Some(
                    crate::data::node::NodeState::MaybeStale | crate::data::node::NodeState::Clean
                )
            ) && !matches!(
                task.request_mode,
                crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand
            );
            if !validates_clean {
                return Ok(None);
            }
            let dependencies = capture_current_dependencies_without_refresh(graph, task.node)?;
            Ok(Some(PrevalidatedTask::Prepared(
                PreparedEvaluation::validated_clean().with_dependencies(dependencies),
            )))
        }
    }
}

fn prepare_condition_outcome_if_blocked(
    graph: &mut SignalGraph,
    task: &EligibleTask,
    temporal_lowering: &TemporalLoweringContext,
) -> Result<Option<PrevalidatedTask>, SignalError> {
    let invalidation = graph.node_invalidation_input(task.node)?;
    let Some(dirty_aspects) = invalidation.resolved_dirty_aspects() else {
        let dependencies = capture_current_dependencies_without_refresh(graph, task.node)?;
        return Ok(Some(PrevalidatedTask::Prepared(
            PreparedEvaluation::deferred_by_invalidation().with_dependencies(dependencies),
        )));
    };
    let required_context = graph.get_contract(task.node)?.semantics.required_context;
    let max_dependency_delta = max_dependency_delta(graph, task.node)?;
    let ctx = ConditionEvaluationContext {
        node: task.node,
        request_mode: task.request_mode,
        dirty_aspects,
        max_dependency_delta,
        required_context,
    };
    let has_dependency_snapshot = !graph.get_dep_snapshot(task.node)?.entries().is_empty();
    let mut default_resolver = DefaultConditionResolver;

    match graph.node_eval_config(task.node)?.condition.clone() {
        EvaluationCondition::Always | EvaluationCondition::OnDemand => Ok(None),
        EvaluationCondition::AspectFilter(mask) => {
            if !has_dependency_snapshot
                || dirty_aspects.is_empty()
                || dirty_aspects.intersects(mask)
            {
                Ok(None)
            } else {
                prepare_condition_blocked_result(
                    graph,
                    task.node,
                    PreparedEvaluation::deferred_by_condition(),
                )
            }
        }
        EvaluationCondition::DeltaThreshold(threshold) => {
            if !has_dependency_snapshot
                || dirty_aspects.is_empty()
                || (max_dependency_delta as f64) > threshold
            {
                Ok(None)
            } else {
                prepare_condition_blocked_result(
                    graph,
                    task.node,
                    PreparedEvaluation::reverted_clean_by_condition(),
                )
            }
        }
        EvaluationCondition::Temporal(condition) => {
            graph
                .telemetry_mut()
                .temporal
                .temporal_eligibility_lowering_count += 1;
            lower_temporal_condition(graph, task.node, condition, &ctx, temporal_lowering)
        }
        EvaluationCondition::Custom(key) => {
            if default_resolver.resolve_custom(&key, &ctx)? {
                Ok(None)
            } else {
                prepare_condition_blocked_result(
                    graph,
                    task.node,
                    PreparedEvaluation::deferred_by_condition(),
                )
            }
        }
        EvaluationCondition::Installed(_) => Err(SignalError::invalid_input(
            "installed conditions require the owner-bound conditional execution entry point",
        )),
    }
}

fn prepare_condition_blocked_result(
    graph: &mut SignalGraph,
    node: NodeId,
    prepared: PreparedEvaluation,
) -> Result<Option<PrevalidatedTask>, SignalError> {
    let dependencies = capture_current_dependencies_without_refresh(graph, node)?;
    Ok(Some(PrevalidatedTask::Prepared(
        prepared.with_dependencies(dependencies),
    )))
}

fn lower_temporal_condition(
    graph: &mut SignalGraph,
    node: NodeId,
    condition: TemporalCondition,
    ctx: &ConditionEvaluationContext,
    temporal_lowering: &TemporalLoweringContext,
) -> Result<Option<PrevalidatedTask>, SignalError> {
    if let Some(prevalidated) =
        lower_temporal_condition_from_runtime_clock(condition.clone(), temporal_lowering)
    {
        return match prevalidated {
            PrevalidatedTask::Prepared(prepared) => {
                prepare_condition_blocked_result(graph, node, prepared)
            }
            other => Ok(Some(other)),
        };
    }

    if let Some(ready) = temporal_lowering.ready_wake_for_node(node) {
        if ready.condition() == &condition {
            return Ok(Some(PrevalidatedTask::NeedsCompute {
                temporal_ready: Some(ReadyTemporalEligibility::runtime_wake_backed(
                    condition,
                    ready.id(),
                    ready.ready_ordinal(),
                    ready.ready_tick(),
                )),
                ready_invalidation: None,
            }));
        }
        return Err(SignalError::internal(format!(
            "ready temporal wake {} for node {} carried a descriptor different from the node declaration",
            ready.id().get(),
            node
        )));
    }

    let Some(authority_tick) = temporal_lowering.current_runtime_tick() else {
        return Err(SignalError::invalid_input(format!(
            "temporal condition for node {node} requires runtime-owned temporal lowering"
        )));
    };

    let _ = ctx;
    prepare_condition_blocked_result(
        graph,
        node,
        PreparedEvaluation::deferred_by_time(LoweredTemporalEligibility::Deferred(
            DeferredTemporalEligibility::runtime_wake_deferred(condition, authority_tick),
        )),
    )
}

fn lower_temporal_condition_from_runtime_clock(
    condition: TemporalCondition,
    temporal_lowering: &TemporalLoweringContext,
) -> Option<PrevalidatedTask> {
    match condition.clone() {
        TemporalCondition::AtOrAfter(at_or_after) => {
            let authority_tick = temporal_lowering.runtime_tick_for(at_or_after.clock_domain())?;
            if authority_tick >= at_or_after.tick() {
                Some(PrevalidatedTask::NeedsCompute {
                    temporal_ready: Some(ReadyTemporalEligibility::runtime_clock_backed(
                        condition,
                        authority_tick,
                    )),
                    ready_invalidation: None,
                })
            } else {
                Some(PrevalidatedTask::Prepared(
                    PreparedEvaluation::deferred_by_time(LoweredTemporalEligibility::Deferred(
                        DeferredTemporalEligibility::runtime_clock_backed(
                            condition,
                            authority_tick,
                        ),
                    )),
                ))
            }
        }
        _ => None,
    }
}

fn prepare_validated_clean_if_unchanged(
    graph: &mut SignalGraph,
    task: &EligibleTask,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<Option<PrevalidatedTask>, SignalError> {
    if matches!(
        graph.node_invalidation_input(task.node)?,
        NodeInvalidationInput::Resolved(ref causes) if causes.is_source_recompute()
    ) {
        return Ok(None);
    }
    if matches!(
        task.request_mode,
        crate::logic::evaluation::EvaluationRequestMode::ForceOnDemand
    ) {
        return Ok(None);
    }

    if !matches!(
        task.admission.node_state_at_admission,
        Some(crate::data::node::NodeState::MaybeStale)
    ) {
        return Ok(None);
    }

    if task.admission.dirty_partition_scopes_present {
        return Ok(None);
    }

    let preview =
        super::super::validation::preview_maybe_stale(graph, task.node, comparator_resolver)?;
    if !preview.unchanged {
        return Ok(None);
    }

    let dependencies = capture_current_dependencies_without_refresh(graph, task.node)?;
    Ok(Some(PrevalidatedTask::Prepared(
        PreparedEvaluation::validated_clean().with_dependencies(dependencies),
    )))
}

fn max_dependency_delta(graph: &SignalGraph, node: NodeId) -> Result<u64, SignalError> {
    let mut max_delta = 0;
    for snapshot_entry in graph.get_dep_snapshot(node)?.entries() {
        if !graph.is_alive(snapshot_entry.source) {
            continue;
        }
        let current_version = graph.node_version_for_scope(
            snapshot_entry.source,
            snapshot_entry.aspect,
            snapshot_entry.scope.as_ref(),
        )?;
        max_delta = max_delta.max(current_version.abs_diff(snapshot_entry.cached_version));
    }
    Ok(max_delta)
}
