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
use crate::data::output::{
    ChangedRegion, IntoNodeEvaluationResult, KeyedComputation, MemoizedResultOrigin,
    NodeEvaluationResult, OutputChange, OutputIdentity, PartitionMatchMode, PartitionSubscription,
};
use crate::data::trace::TraceSummary;
use crate::logic::planner::{
    build_evaluation_plan_with_policy_resolver, execute_plan_with_policy_and_condition,
    StageExecutor,
};
use crate::logic::prepared::{
    PreparedDependencyCapture, PreparedEvaluation, PreparedEvaluationOrigin,
    PreparedEvaluationOutcome,
};

use super::condition::{
    ConditionEvaluationContext, ConditionResolver, DefaultConditionResolver, EvaluationRequestMode,
};

#[derive(Debug, Clone)]
pub struct EvaluationExecutionMetadata {
    pub keyed: Option<KeyedComputation>,
    pub memoized_origin: MemoizedResultOrigin,
}

impl EvaluationExecutionMetadata {
    pub fn from_keyed(
        computation: &KeyedComputation,
        memoized_origin: MemoizedResultOrigin,
    ) -> Self {
        Self {
            keyed: Some(computation.clone()),
            memoized_origin,
        }
    }
}

/// Evaluate a node, recomputing only if necessary.
///
/// Uses an explicit stack to avoid recursion on deep graphs.
pub fn evaluate<F, O>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
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
pub fn evaluate_on_demand<F, O>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
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
pub fn evaluate_with_resolver<F, O, R>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    resolver: &mut R,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
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
pub fn evaluate_with_resolvers<F, O, R, C>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    comparator_resolver: &mut R,
    condition_resolver: &mut C,
    request_mode: EvaluationRequestMode,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
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
pub fn evaluate_with_policy_resolver<F, O, R>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    resolver: &mut R,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
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
pub fn evaluate_with_policy_and_condition_resolvers<F, O, R, C>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    comparator_resolver: &mut R,
    condition_resolver: &mut C,
    request_mode: EvaluationRequestMode,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
    R: ComparatorPolicyResolver,
    C: ConditionResolver,
{
    evaluate_with_policy_and_condition_resolvers_and_metadata(
        graph,
        node,
        compute,
        comparator_resolver,
        condition_resolver,
        request_mode,
        None,
    )
}

pub fn evaluate_with_policy_and_condition_resolvers_and_metadata<F, O, R, C>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    comparator_resolver: &mut R,
    condition_resolver: &mut C,
    request_mode: EvaluationRequestMode,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
    R: ComparatorPolicyResolver,
    C: ConditionResolver,
{
    let plan = build_evaluation_plan_with_policy_resolver(
        graph,
        &[node],
        request_mode,
        comparator_resolver,
    )?;
    execute_plan_with_policy_and_condition(
        graph,
        &plan,
        compute,
        comparator_resolver,
        condition_resolver,
        StageExecutor::Serial,
        execution_metadata,
    )?;
    Ok(())
}

pub(crate) fn evaluate_direct_with_policy_and_condition_resolvers_and_metadata<F, O, R, C>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    comparator_resolver: &mut R,
    condition_resolver: &mut C,
    request_mode: EvaluationRequestMode,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
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
    graph.telemetry_mut().evaluation_stack_peak = graph
        .telemetry()
        .evaluation_stack_peak
        .max(scratch.eval_tasks.len() as u64);

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
                    execution_metadata,
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

pub(crate) fn apply_prepared_evaluation_with_policy(
    graph: &mut SignalGraph,
    node: NodeId,
    prepared: PreparedEvaluation,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
) -> Result<u32, SignalError> {
    let dependency_updates = apply_prepared_dependencies(graph, node, &prepared.dependencies)?;
    match prepared.outcome {
        PreparedEvaluationOutcome::Evaluate => {
            if let Some(causality) = prepared.trace_data.causality.clone() {
                graph.get_entry_mut(node)?.set_causality(Some(causality));
            }
            let mut result = prepared.result;
            result.labels.extend(prepared.trace_data.labels);
            let metadata = match (execution_metadata, prepared.origin) {
                (Some(metadata), _) => Some(metadata),
                (None, PreparedEvaluationOrigin::MemoizedReuse) => {
                    let synthesized = EvaluationExecutionMetadata {
                        keyed: None,
                        memoized_origin: MemoizedResultOrigin::MemoizedFromCache,
                    };
                    return apply_prepared_with_synthesized_metadata(
                        graph,
                        node,
                        result,
                        comparator_resolver,
                        synthesized,
                        dependency_updates,
                    );
                }
                _ => None,
            };
            apply_evaluation_result_with_policy(
                graph,
                node,
                result,
                comparator_resolver,
                metadata,
                !matches!(prepared.origin, PreparedEvaluationOrigin::MemoizedReuse),
            )?;
        }
        PreparedEvaluationOutcome::ValidatedClean => {
            revert_to_clean(graph, node)?;
        }
        PreparedEvaluationOutcome::DeferredByCondition => {
            defer_due_to_condition(graph, node)?;
        }
        PreparedEvaluationOutcome::RevertedCleanByCondition => {
            revert_to_clean_due_to_condition(graph, node)?;
        }
    }
    Ok(dependency_updates)
}

fn apply_prepared_with_synthesized_metadata(
    graph: &mut SignalGraph,
    node: NodeId,
    result: NodeEvaluationResult,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    metadata: EvaluationExecutionMetadata,
    dependency_updates: u32,
) -> Result<u32, SignalError> {
    apply_evaluation_result_with_policy(
        graph,
        node,
        result,
        comparator_resolver,
        Some(&metadata),
        false,
    )?;
    Ok(dependency_updates)
}

fn apply_prepared_dependencies(
    graph: &mut SignalGraph,
    node: NodeId,
    capture: &PreparedDependencyCapture,
) -> Result<u32, SignalError> {
    let old_dependencies = graph.get_entry(node)?.get_dependencies().to_vec();
    let mut updates = 0_u32;

    for dependency in &old_dependencies {
        let still_present = capture.as_slice().iter().any(|captured| {
            captured.source == dependency.source()
                && captured.aspect == dependency.aspect()
                && captured.scope == dependency.scope_ref().cloned()
        });
        if !still_present {
            let removed = graph.disconnect_dependency_edge(node, dependency.clone())?;
            updates += u32::from(removed);
        }
    }

    for dependency in capture.as_slice() {
        let inserted = graph.connect_dependency_capture(
            node,
            dependency.source,
            dependency.aspect,
            dependency.scope.clone(),
        )?;
        updates += u32::from(inserted);
    }

    Ok(updates)
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
            let upstream_unchanged = check_upstream_unchanged(graph, current, comparator_resolver)?;
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

fn process_recompute_task<F, O>(
    graph: &mut SignalGraph,
    current: NodeId,
    compute: &mut F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    condition_resolver: &mut impl ConditionResolver,
    request_mode: EvaluationRequestMode,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
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
        let upstream_unchanged = check_upstream_unchanged(graph, current, comparator_resolver)?;
        if upstream_unchanged {
            return revert_to_clean(graph, current);
        }
    }

    match resolve_condition_action(graph, current, request_mode, condition_resolver)? {
        ConditionAction::Evaluate => recompute_node(
            graph,
            current,
            compute,
            comparator_resolver,
            execution_metadata,
        ),
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
    graph.telemetry_mut().evaluation_stack_peak = graph
        .telemetry()
        .evaluation_stack_peak
        .max(scratch.eval_tasks.len() as u64);
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
    for (source, aspect, cached_version, _) in graph.get_entry(node)?.get_dep_snapshot().entries() {
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
    entry.clear_dirty_partition_scopes();
    Ok(())
}

fn revert_to_clean_due_to_condition(
    graph: &mut SignalGraph,
    node: NodeId,
) -> Result<(), SignalError> {
    let entry = graph.get_entry_mut(node)?;
    entry.set_state(NodeState::Clean);
    entry.set_dirty_aspects(AspectMask::EMPTY);
    entry.clear_dirty_partition_scopes();
    Ok(())
}

fn defer_due_to_condition(graph: &mut SignalGraph, node: NodeId) -> Result<(), SignalError> {
    graph.get_entry_mut(node)?.set_state(NodeState::MaybeStale);
    Ok(())
}

fn check_upstream_unchanged(
    graph: &mut SignalGraph,
    node: NodeId,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<bool, SignalError> {
    let (snapshot_entries, node_comparator) = {
        let entry = graph.get_entry(node)?;
        (
            entry.get_dep_snapshot().entries().to_vec(),
            entry.get_eval_config().comparator.clone(),
        )
    };
    let comparator = resolver.policy_for_node(node, node_comparator.as_ref());

    for (source, aspect, cached_version, scope) in &snapshot_entries {
        if !graph.is_alive(*source) {
            return Ok(false);
        }
        if !matches!(graph.get_entry(*source)?.get_state(), NodeState::Clean) {
            return Ok(false);
        }
        let current_version = graph.get_entry(*source)?.get_aspect_version().get(*aspect);
        if let Some(scope) = scope {
            if current_version == *cached_version {
                continue;
            }
            if partition_scope_untouched(graph.get_entry(*source)?.get_trace_summary(), scope) {
                graph.telemetry_mut().partition_scope_revert_clean_count += 1;
                continue;
            }
            return Ok(false);
        }
        if comparator.has_meaningful_change(*aspect, *cached_version, current_version, resolver)? {
            return Ok(false);
        }
    }

    Ok(true)
}

fn recompute_node<F, O>(
    graph: &mut SignalGraph,
    node: NodeId,
    compute: &mut F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
) -> Result<(), SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: IntoNodeEvaluationResult,
{
    let result = compute(node, graph)?.into_evaluation_result();
    apply_evaluation_result_with_policy(
        graph,
        node,
        result,
        comparator_resolver,
        execution_metadata,
        true,
    )
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
            snapshot.record(source, aspect, ver, dep.scope_ref().cloned());
        }
    }
    Ok(snapshot)
}

fn count_meaningful_input_changes(graph: &SignalGraph, node: NodeId) -> Result<u32, SignalError> {
    let entry = graph.get_entry(node)?;
    let mut changes = 0_u32;
    for dependency in entry.get_dependencies() {
        let cached = entry
            .get_dep_snapshot()
            .entries()
            .iter()
            .find(|(source, aspect, _, scope)| {
                *source == dependency.source()
                    && *aspect == dependency.aspect()
                    && *scope == dependency.scope_ref().cloned()
            })
            .map(|(_, _, version, _)| *version);
        let Some(cached) = cached else {
            continue;
        };
        if !graph.is_alive(dependency.source()) {
            changes += 1;
            continue;
        }
        let current = graph
            .get_entry(dependency.source())?
            .get_aspect_version()
            .get(dependency.aspect());
        if current != cached {
            changes += 1;
        }
    }
    Ok(changes)
}

fn trace_output_hash(version: AspectVersion) -> u128 {
    let mut hash = 0xcbf29ce484222325_u128;
    for slot in version.slots() {
        hash ^= *slot as u128;
        hash = hash.wrapping_mul(0x100000001b3_u128);
    }
    hash
}

pub fn apply_evaluation_result_with_policy_and_condition(
    graph: &mut SignalGraph,
    node: NodeId,
    result: NodeEvaluationResult,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    _condition_resolver: &mut impl ConditionResolver,
    execution_metadata: &EvaluationExecutionMetadata,
    recomputed: bool,
) -> Result<(), SignalError> {
    apply_evaluation_result_with_policy(
        graph,
        node,
        result,
        comparator_resolver,
        Some(execution_metadata),
        recomputed,
    )
}

fn apply_evaluation_result_with_policy(
    graph: &mut SignalGraph,
    node: NodeId,
    result: NodeEvaluationResult,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
    recomputed: bool,
) -> Result<(), SignalError> {
    let previous_trace = graph.get_entry(node)?.get_trace_summary().cloned();
    let previous_output_identity = previous_trace
        .as_ref()
        .and_then(|trace| trace.output_identity.clone());
    let comparator = {
        let entry = graph.get_entry(node)?;
        comparator_resolver.policy_for_node(node, entry.get_eval_config().comparator.as_ref())
    };

    let output_identity_unchanged = matches!(
        (&previous_output_identity, &result.output_identity),
        (Some(previous), Some(current)) if previous == current
    );
    let propagation_suppressed = matches!(
        comparator,
        crate::data::comparator::VersionComparatorPolicy::OutputIdentity
    ) && output_identity_unchanged;
    let output_change = normalize_output_change(
        result.output_change,
        output_identity_unchanged,
        result.output_identity.is_some(),
    );
    let meaningful_input_changes = count_meaningful_input_changes(graph, node)?;
    let snapshot = build_dep_snapshot(graph, node)?;
    let changed_partition_count = count_changed_partitions(&result.changed_regions);
    let trace_summary = TraceSummary {
        output_hash: result
            .output_identity
            .as_ref()
            .map(trace_identity_hash)
            .unwrap_or_else(|| trace_output_hash(result.aspect_version)),
        output_identity: result.output_identity.clone(),
        output_change,
        recomputed,
        dependency_count: snapshot.entries().len() as u32,
        meaningful_input_changes,
        changed_partition_count,
        propagation_suppressed,
        changed_regions: result.changed_regions.clone(),
        keyed_family: execution_metadata
            .and_then(|metadata| metadata.keyed.as_ref().map(|keyed| keyed.family.0.clone())),
        keyed_key: execution_metadata
            .and_then(|metadata| metadata.keyed.as_ref().map(|keyed| keyed.key.0.clone())),
        memoized_origin: execution_metadata
            .map(|metadata| metadata.memoized_origin)
            .unwrap_or(MemoizedResultOrigin::DirectCompute),
        labels: result.labels.clone(),
        execution_record_id: None,
    };

    {
        let entry = graph.get_entry_mut(node)?;
        entry.set_aspect_version(result.aspect_version);
        entry.set_dep_snapshot(snapshot);
        entry.set_trace_summary(Some(trace_summary));
        entry.set_state(NodeState::Clean);
        entry.set_dirty_aspects(AspectMask::EMPTY);
        entry.clear_dirty_partition_scopes();
    }

    if recomputed {
        graph.telemetry_mut().nodes_recomputed += 1;
    }
    if propagation_suppressed {
        graph.telemetry_mut().output_identity_unchanged_count += 1;
        let suppressed =
            suppress_downstream_if_identity_unchanged(graph, node, comparator_resolver)?;
        graph.telemetry_mut().suppressed_downstream_propagations += suppressed;
    }
    if !result.changed_regions.is_empty() {
        graph.telemetry_mut().partition_aware_recomputations += 1;
    }

    Ok(())
}

fn normalize_output_change(
    declared: OutputChange,
    output_identity_unchanged: bool,
    has_output_identity: bool,
) -> OutputChange {
    if has_output_identity && output_identity_unchanged {
        OutputChange::Unchanged
    } else {
        declared
    }
}

fn count_changed_partitions(changed_regions: &[crate::data::output::ChangedRegion]) -> u32 {
    let mut partitions = std::collections::BTreeSet::new();
    for region in changed_regions {
        partitions.insert(region.partition.clone());
    }
    partitions.len() as u32
}

fn trace_identity_hash(identity: &OutputIdentity) -> u128 {
    let mut hash = 0xcbf29ce484222325_u128;
    for byte in identity.0.as_bytes() {
        hash ^= *byte as u128;
        hash = hash.wrapping_mul(0x100000001b3_u128);
    }
    hash
}

fn suppress_downstream_if_identity_unchanged(
    graph: &mut SignalGraph,
    node: NodeId,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<u64, SignalError> {
    let mut suppressed = 0_u64;
    let mut stack: Vec<NodeId> = graph.get_entry(node)?.get_subscribers().to_vec();
    while let Some(current) = stack.pop() {
        if !graph.is_alive(current) {
            continue;
        }
        if matches!(graph.get_entry(current)?.get_state(), NodeState::Clean) {
            continue;
        }
        if check_upstream_unchanged_ignoring_source(graph, current, node, comparator_resolver)? {
            revert_to_clean(graph, current)?;
            suppressed += 1;
            stack.extend_from_slice(graph.get_entry(current)?.get_subscribers());
        }
    }
    Ok(suppressed)
}

fn check_upstream_unchanged_ignoring_source(
    graph: &SignalGraph,
    node: NodeId,
    ignored_source: NodeId,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<bool, SignalError> {
    let entry = graph.get_entry(node)?;
    let snapshot = entry.get_dep_snapshot();
    let node_cfg = entry.get_eval_config();
    let comparator = resolver.policy_for_node(node, node_cfg.comparator.as_ref());

    for (source, aspect, cached_version, scope) in snapshot.entries() {
        if *source == ignored_source {
            if let Some(scope) = scope {
                if !matches!(graph.get_entry(*source)?.get_state(), NodeState::Clean) {
                    return Ok(false);
                }
                if partition_scope_touched(graph.get_entry(*source)?.get_trace_summary(), scope) {
                    return Ok(false);
                }
            }
            continue;
        }
        if !graph.is_alive(*source) {
            return Ok(false);
        }
        if !matches!(graph.get_entry(*source)?.get_state(), NodeState::Clean) {
            return Ok(false);
        }
        let current_version = graph.get_entry(*source)?.get_aspect_version().get(*aspect);
        if let Some(scope) = scope {
            if current_version == *cached_version {
                continue;
            }
            if partition_scope_untouched(graph.get_entry(*source)?.get_trace_summary(), scope) {
                continue;
            }
            return Ok(false);
        }
        if comparator.has_meaningful_change(*aspect, *cached_version, current_version, resolver)? {
            return Ok(false);
        }
    }

    Ok(true)
}

fn partition_scope_touched(
    trace_summary: Option<&TraceSummary>,
    scope: &PartitionSubscription,
) -> bool {
    let Some(trace_summary) = trace_summary else {
        return false;
    };
    if trace_summary.output_change == OutputChange::Unchanged {
        return false;
    }
    if trace_summary.changed_regions.is_empty() {
        return true;
    }
    trace_summary
        .changed_regions
        .iter()
        .any(|region| partition_subscription_matches(scope, region))
}

fn partition_scope_untouched(
    trace_summary: Option<&TraceSummary>,
    scope: &PartitionSubscription,
) -> bool {
    !partition_scope_touched(trace_summary, scope)
}

fn partition_subscription_matches(
    subscription: &PartitionSubscription,
    region: &ChangedRegion,
) -> bool {
    if subscription.partition != region.partition {
        return false;
    }
    match subscription.match_mode {
        PartitionMatchMode::WholePartition => true,
        PartitionMatchMode::PartitionAndDetail => subscription.detail == region.detail,
    }
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
