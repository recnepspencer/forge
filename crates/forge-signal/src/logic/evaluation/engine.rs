use crate::data::aspect::{AspectMask, AspectVersion};
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::dependency::DependencySnapshot;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::{
    ChangedRegion, KeyedComputation, MemoizedResultOrigin, NodeEvaluationResult, OutputChange,
    OutputIdentity, PartitionMatchMode, PartitionSubscription,
};
use crate::data::trace::TraceSummary;
use crate::logic::prepared::{
    PreparedDependencyCapture, PreparedEvaluation, PreparedEvaluationOrigin,
    PreparedEvaluationOutcome,
};

use super::condition::ConditionResolver;

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
    let old_dependencies = graph.dependencies_of(node)?.to_vec();
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

fn build_dep_snapshot(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<DependencySnapshot, SignalError> {
    let mut snapshot = DependencySnapshot::empty();
    for dep in graph.dependencies_of(node)? {
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
    let mut changes = 0_u32;
    for dependency in graph.dependencies_of(node)? {
        let cached = graph
            .get_dep_snapshot(node)?
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
        keyed_family: execution_metadata.and_then(|metadata| {
            metadata
                .keyed
                .as_ref()
                .map(|keyed| keyed.family.as_str().to_owned())
        }),
        keyed_key: execution_metadata.and_then(|metadata| {
            metadata
                .keyed
                .as_ref()
                .map(|keyed| keyed.key.as_str().to_owned())
        }),
        memoized_origin: execution_metadata
            .map(|metadata| metadata.memoized_origin)
            .unwrap_or(MemoizedResultOrigin::DirectCompute),
        labels: result.labels.clone(),
        execution_record_id: None,
    };

    {
        let entry = graph.get_entry_mut(node)?;
        entry.set_aspect_version(result.aspect_version);
        entry.set_trace_summary(Some(trace_summary));
        entry.set_state(NodeState::Clean);
        entry.set_dirty_aspects(AspectMask::EMPTY);
        entry.clear_dirty_partition_scopes();
    }
    graph.set_dep_snapshot(node, snapshot)?;

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
    identity.stable_hash()
}

fn suppress_downstream_if_identity_unchanged(
    graph: &mut SignalGraph,
    node: NodeId,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<u64, SignalError> {
    let mut suppressed = 0_u64;
    let mut stack: Vec<NodeId> = graph.subscribers_of(node)?.to_vec();
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
            stack.extend_from_slice(graph.subscribers_of(current)?);
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
    let snapshot = graph.get_dep_snapshot(node)?;
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
