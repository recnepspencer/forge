use std::time::Instant;

use crate::data::aspect::{AspectMask, AspectVersion};
use crate::data::comparator::{
    ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    VersionComparatorResolver,
};
use crate::data::dependency::DependencySnapshot;
use crate::data::error::SignalError;
use crate::data::graph::{ScratchLeaseKind, SignalGraph, TraversalScratch};
use crate::data::handle::NodeId;
use crate::data::node::{EvaluationCondition, NodeState};

use super::condition::{
    ConditionEvaluationContext, ConditionResolver, DefaultConditionResolver, EvaluationRequestMode,
};

/// Evaluate a node, recomputing only if necessary.
///
/// Uses an explicit stack to avoid recursion on deep graphs.
pub fn evaluate<F>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
{
    let mut comparator = DefaultComparatorResolver;
    let mut condition = DefaultConditionResolver;
    evaluate_with_resolvers(
        graph,
        node,
        compute,
        &mut comparator,
        &mut condition,
        EvaluationRequestMode::Default,
    )
}

/// Evaluate a node while forcing `OnDemand` conditions to execute.
pub fn evaluate_on_demand<F>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
{
    let mut comparator = DefaultComparatorResolver;
    let mut condition = DefaultConditionResolver;
    evaluate_with_resolvers(
        graph,
        node,
        compute,
        &mut comparator,
        &mut condition,
        EvaluationRequestMode::ForceOnDemand,
    )
}

/// Evaluate a node with a host-provided custom comparator resolver.
pub fn evaluate_with_resolver<F, R>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    resolver: &mut R,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
    R: VersionComparatorResolver,
{
    let mut condition = DefaultConditionResolver;
    evaluate_with_resolvers(
        graph,
        node,
        compute,
        resolver,
        &mut condition,
        EvaluationRequestMode::Default,
    )
}

/// Evaluate a node with host-provided comparator and condition resolvers.
pub fn evaluate_with_resolvers<F, R, C>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    comparator_resolver: &mut R,
    condition_resolver: &mut C,
    request_mode: EvaluationRequestMode,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
    R: VersionComparatorResolver,
    C: ConditionResolver,
{
    let mut policy = DefaultComparatorPolicyResolver {
        fallback: crate::data::comparator::VersionComparatorPolicy::Exact,
        custom: comparator_resolver,
    };
    evaluate_with_policy_and_condition_resolvers(
        graph,
        node,
        compute,
        &mut policy,
        condition_resolver,
        request_mode,
    )
}

/// Evaluate a node with explicit comparator policy resolution.
pub fn evaluate_with_policy_resolver<F, R>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    resolver: &mut R,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
    R: ComparatorPolicyResolver,
{
    let mut condition = DefaultConditionResolver;
    evaluate_with_policy_and_condition_resolvers(
        graph,
        node,
        compute,
        resolver,
        &mut condition,
        EvaluationRequestMode::Default,
    )
}

/// Evaluate a node with explicit comparator policy and condition resolution.
pub fn evaluate_with_policy_and_condition_resolvers<F, R, C>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    comparator_resolver: &mut R,
    condition_resolver: &mut C,
    request_mode: EvaluationRequestMode,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
    R: ComparatorPolicyResolver,
    C: ConditionResolver,
{
    let eval_start = Instant::now();
    graph.telemetry_mut().evaluation_calls += 1;
    let mut scratch = graph.acquire_scratch(ScratchLeaseKind::Evaluation)?;
    let len = graph.arena_capacity();
    scratch.visited.next_pass(len);
    scratch.active.next_pass(len);
    scratch.eval_tasks.clear();
    scratch.eval_tasks.push((node, false));
    graph.telemetry_mut().evaluation_stack_peak =
        graph.telemetry().evaluation_stack_peak.max(scratch.eval_tasks.len() as u64);

    let result = loop {
        let Some((current, recompute)) = scratch.eval_tasks.pop() else {
            break Ok(());
        };
        match recompute {
            true => {
                scratch.active.clear(current.index() as usize);
                scratch.visited.mark(current.index() as usize);
                if let Err(err) = process_recompute_task(
                    graph,
                    current,
                    compute,
                    comparator_resolver,
                    condition_resolver,
                    request_mode,
                ) {
                    break Err(err);
                }
            }
            false => {
                if let Err(err) = process_evaluate_task(
                    graph,
                    current,
                    &mut scratch,
                    comparator_resolver,
                    condition_resolver,
                    request_mode,
                ) {
                    break Err(err);
                }
            }
        }
    };

    graph.restore_scratch(ScratchLeaseKind::Evaluation, scratch)?;
    graph.telemetry_mut().evaluation_nanos += eval_start.elapsed().as_nanos();
    result
}

fn process_evaluate_task(
    graph: &mut SignalGraph,
    current: NodeId,
    scratch: &mut TraversalScratch,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    condition_resolver: &mut impl ConditionResolver,
    request_mode: EvaluationRequestMode,
) -> Result<(), SignalError> {
    if !graph.is_alive(current) {
        return Ok(());
    }

    if scratch.visited.is_marked(current.index() as usize) {
        return Ok(());
    }

    if scratch.active.is_marked(current.index() as usize) {
        return Err(circular_reference_error(current));
    }

    let state = *graph.get_entry(current)?.get_state();

    match state {
        NodeState::Clean => {
            scratch.visited.mark(current.index() as usize);
            Ok(())
        }
        NodeState::MaybeStale => {
            let upstream_unchanged =
                check_upstream_unchanged(graph, current, comparator_resolver)?;
            if upstream_unchanged {
                revert_to_clean(graph, current)?;
                scratch.visited.mark(current.index() as usize);
                Ok(())
            } else {
                scratch.active.mark(current.index() as usize);
                push_deps_then_recompute(
                    graph,
                    current,
                    scratch,
                    comparator_resolver,
                    condition_resolver,
                    request_mode,
                )
            }
        }
        NodeState::Dirty => {
            scratch.active.mark(current.index() as usize);
            push_deps_then_recompute(
                graph,
                current,
                scratch,
                comparator_resolver,
                condition_resolver,
                request_mode,
            )
        }
    }
}

fn process_recompute_task<F>(
    graph: &mut SignalGraph,
    current: NodeId,
    compute: &mut F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    condition_resolver: &mut impl ConditionResolver,
    request_mode: EvaluationRequestMode,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
{
    graph.telemetry_mut().nodes_evaluated += 1;
    if !graph.is_alive(current) {
        return Ok(());
    }

    let state = *graph.get_entry(current)?.get_state();
    if matches!(state, NodeState::Clean) {
        return Ok(());
    }

    if matches!(state, NodeState::MaybeStale) {
        let upstream_unchanged =
            check_upstream_unchanged(graph, current, comparator_resolver)?;
        if upstream_unchanged {
            return revert_to_clean(graph, current);
        }
    }

    match resolve_condition_action(
        graph,
        current,
        request_mode,
        condition_resolver,
    )? {
        ConditionAction::Evaluate => recompute_node(graph, current, compute),
        ConditionAction::RevertClean => revert_to_clean_due_to_condition(graph, current),
        ConditionAction::Defer => defer_due_to_condition(graph, current),
    }
}

fn push_deps_then_recompute(
    graph: &mut SignalGraph,
    current: NodeId,
    scratch: &mut TraversalScratch,
    _comparator_resolver: &mut impl ComparatorPolicyResolver,
    _condition_resolver: &mut impl ConditionResolver,
    _request_mode: EvaluationRequestMode,
) -> Result<(), SignalError> {
    let deps = graph.get_entry(current)?.get_dependencies();

    scratch.eval_tasks.push((current, true));
    for dep in deps.iter().rev() {
        scratch.eval_tasks.push((dep.source(), false));
    }
    graph.telemetry_mut().evaluation_stack_peak =
        graph.telemetry().evaluation_stack_peak.max(scratch.eval_tasks.len() as u64);
    Ok(())
}

fn resolve_condition_action(
    graph: &mut SignalGraph,
    node: NodeId,
    request_mode: EvaluationRequestMode,
    resolver: &mut impl ConditionResolver,
) -> Result<ConditionAction, SignalError> {
    let entry = graph.get_entry(node)?;
    let dirty_aspects = entry.get_dirty_aspects();
    let condition = &entry.get_eval_config().condition;
    let max_dependency_delta = max_dependency_delta(graph, node)?;

    let ctx = ConditionEvaluationContext {
        node,
        request_mode,
        dirty_aspects,
        max_dependency_delta,
    };

    let action = match condition {
        EvaluationCondition::Always => ConditionAction::Evaluate,
        EvaluationCondition::AspectFilter(mask) => {
            if dirty_aspects.is_empty() || dirty_aspects.intersects(*mask) {
                ConditionAction::Evaluate
            } else {
                graph.telemetry_mut().condition_skip_count += 1;
                ConditionAction::Defer
            }
        }
        EvaluationCondition::OnDemand => match request_mode {
            EvaluationRequestMode::Default => {
                graph.telemetry_mut().condition_skip_count += 1;
                graph.telemetry_mut().ondemand_deferred_count += 1;
                ConditionAction::Defer
            }
            EvaluationRequestMode::ForceOnDemand => ConditionAction::Evaluate,
        },
        EvaluationCondition::DeltaThreshold(threshold) => {
            if dirty_aspects.is_empty() || (max_dependency_delta as f64) > *threshold {
                ConditionAction::Evaluate
            } else {
                graph.telemetry_mut().condition_skip_count += 1;
                ConditionAction::RevertClean
            }
        }
        EvaluationCondition::Debounce(quiet_period_ms) => {
            if resolver.debounce_ready(*quiet_period_ms, &ctx)? {
                ConditionAction::Evaluate
            } else {
                graph.telemetry_mut().condition_skip_count += 1;
                graph.telemetry_mut().debounce_deferred_count += 1;
                ConditionAction::Defer
            }
        }
        EvaluationCondition::Custom(key) => {
            if resolver.resolve_custom(key, &ctx)? {
                ConditionAction::Evaluate
            } else {
                graph.telemetry_mut().condition_skip_count += 1;
                ConditionAction::Defer
            }
        }
    };

    Ok(action)
}

fn max_dependency_delta(graph: &SignalGraph, node: NodeId) -> Result<u64, SignalError> {
    let mut max_delta = 0;
    for (source, aspect, cached_version) in graph.get_entry(node)?.get_dep_snapshot().entries() {
        if !graph.is_alive(*source) {
            continue;
        }
        let current_version = graph.get_entry(*source)?.get_aspect_version().get(*aspect);
        max_delta = max_delta.max(current_version.abs_diff(*cached_version));
    }
    Ok(max_delta)
}

fn revert_to_clean(graph: &mut SignalGraph, node: NodeId) -> Result<(), SignalError> {
    graph.telemetry_mut().skipped_by_comparator += 1;
    let entry = graph.get_entry_mut(node)?;
    entry.set_state(NodeState::Clean);
    entry.set_dirty_aspects(AspectMask::EMPTY);
    Ok(())
}

fn revert_to_clean_due_to_condition(graph: &mut SignalGraph, node: NodeId) -> Result<(), SignalError> {
    let entry = graph.get_entry_mut(node)?;
    entry.set_state(NodeState::Clean);
    entry.set_dirty_aspects(AspectMask::EMPTY);
    Ok(())
}

fn defer_due_to_condition(graph: &mut SignalGraph, node: NodeId) -> Result<(), SignalError> {
    graph.get_entry_mut(node)?.set_state(NodeState::MaybeStale);
    Ok(())
}

fn check_upstream_unchanged(
    graph: &SignalGraph,
    node: NodeId,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<bool, SignalError> {
    let entry = graph.get_entry(node)?;
    let snapshot = entry.get_dep_snapshot();
    let node_cfg = entry.get_eval_config();
    let comparator = resolver.policy_for_node(node, node_cfg.comparator.as_ref());

    for (source, aspect, cached_version) in snapshot.entries() {
        if !graph.is_alive(*source) {
            return Ok(false);
        }
        let current_version = graph.get_entry(*source)?.get_aspect_version().get(*aspect);
        if comparator.has_meaningful_change(*aspect, *cached_version, current_version, resolver)? {
            return Ok(false);
        }
    }

    Ok(true)
}

fn recompute_node<F>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<AspectVersion, SignalError>,
{
    let new_version = compute(node, graph)?;
    let snapshot = build_dep_snapshot(graph, node)?;

    let entry = graph.get_entry_mut(node)?;
    entry.set_aspect_version(new_version);
    entry.set_dep_snapshot(snapshot);
    entry.set_state(NodeState::Clean);
    entry.set_dirty_aspects(AspectMask::EMPTY);
    graph.telemetry_mut().nodes_recomputed += 1;

    Ok(())
}

fn build_dep_snapshot(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<DependencySnapshot, SignalError> {
    let mut snapshot = DependencySnapshot::empty();
    for dep in graph.get_entry(node)?.get_dependencies() {
        let source = dep.source();
        let aspect = dep.aspect();
        if graph.is_alive(source) {
            let ver = graph.get_entry(source)?.get_aspect_version().get(aspect);
            snapshot.record(source, aspect, ver);
        }
    }
    Ok(snapshot)
}

enum ConditionAction {
    Evaluate,
    RevertClean,
    Defer,
}

fn circular_reference_error(node: NodeId) -> SignalError {
    SignalError::InvalidInput {
        message: format!("Circular reference detected at signal node: {}", node),
        context: None,
    }
}
