use crate::data::aspect::{AspectVersion, AspectMask};
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::dependency::DependencySnapshot;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::output::{MemoizedResultOrigin, NodeEvaluationResult, OutputChange};
use crate::data::trace::TraceSummary;

use super::metadata::EvaluationExecutionMetadata;
use super::suppression::suppress_downstream_if_identity_unchanged;

pub fn apply_evaluation_result_with_policy_and_condition(
    graph: &mut SignalGraph,
    node: NodeId,
    result: NodeEvaluationResult,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    _condition_resolver: &mut impl super::super::condition::ConditionResolver,
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

pub(super) fn apply_evaluation_result_with_policy(
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

fn trace_identity_hash(identity: &crate::data::output::OutputIdentity) -> u128 {
    identity.stable_hash()
}

fn trace_output_hash(version: AspectVersion) -> u128 {
    let mut hash = 0xcbf29ce484222325_u128;
    for slot in version.slots() {
        hash ^= *slot as u128;
        hash = hash.wrapping_mul(0x100000001b3_u128);
    }
    hash
}
