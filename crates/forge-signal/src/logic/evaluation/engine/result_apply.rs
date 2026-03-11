use crate::data::aspect::AspectVersion;
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::core_profile::StableHashValue;
use crate::data::dependency::{DependencySnapshot, DependencySnapshotEntry};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
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
    let previous_continuity_token = previous_trace
        .as_ref()
        .and_then(|trace| trace.continuity_token.clone());
    let comparator = {
        let entry = graph.get_entry(node)?;
        comparator_resolver.policy_for_node(node, entry.get_eval_config().comparator.as_ref())
    };

    let output_identity_unchanged = matches!(
        (&previous_output_identity, &result.output_identity),
        (Some(previous), Some(current)) if previous == current
    );
    let continuity_token_unchanged = matches!(
        (&previous_continuity_token, &result.continuity_token),
        (Some(previous), Some(current)) if previous == current
    );
    let propagation_suppressed = matches!(
        comparator,
        crate::data::comparator::VersionComparatorPolicy::OutputIdentity
    ) && output_identity_unchanged;
    let output_change = normalize_output_change(
        result.output_change,
        output_identity_unchanged,
        continuity_token_unchanged,
        result.output_identity.is_some(),
        result.continuity_token.is_some(),
    );
    let meaningful_input_changes = count_meaningful_input_changes(graph, node)?;
    let snapshot = build_dep_snapshot(graph, node)?;
    let changed_regions = canonical_changed_regions(&result.changed_regions);
    let mut labels = result.labels.clone();
    labels.sort();
    labels.dedup();
    let changed_partition_count = count_changed_partitions(&changed_regions);
    let trace_summary = TraceSummary {
        output_hash: result
            .output_identity
            .as_ref()
            .map(trace_identity_hash)
            .unwrap_or_else(|| trace_output_hash(result.aspect_version)),
        output_identity: result.output_identity.clone(),
        continuity_token: result.continuity_token.clone(),
        output_change,
        recomputed,
        dependency_count: snapshot.entries().len() as u32,
        meaningful_input_changes,
        changed_partition_count,
        propagation_suppressed,
        changed_regions,
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
        labels,
        execution_record_id: None,
        semantic_segment_id: None,
        lineage_artifact_id: None,
    };

    {
        let entry = graph.get_entry_mut(node)?;
        entry.set_aspect_version(result.aspect_version);
        entry.set_trace_summary(Some(trace_summary));
        entry.transition_clean();
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
    let dependencies = graph.dependencies_of(node)?;
    let snapshot_entries = graph.get_dep_snapshot(node)?.entries();
    let mut dep_index = 0usize;
    let mut snapshot_index = 0usize;
    let mut changes = 0_u32;
    while dep_index < dependencies.len() && snapshot_index < snapshot_entries.len() {
        let dependency = &dependencies[dep_index];
        let snapshot = &snapshot_entries[snapshot_index];
        match compare_dependency_to_snapshot(dependency, snapshot) {
            std::cmp::Ordering::Less => dep_index += 1,
            std::cmp::Ordering::Greater => snapshot_index += 1,
            std::cmp::Ordering::Equal => {
                let cached = snapshot.cached_version;
                if !graph.is_alive(dependency.source()) {
                    changes += 1;
                } else {
                    let current = graph
                        .get_entry(dependency.source())?
                        .get_aspect_version()
                        .get(dependency.aspect());
                    if current != cached {
                        changes += 1;
                    }
                }
                dep_index += 1;
                snapshot_index += 1;
            }
        }
    }
    Ok(changes)
}

fn normalize_output_change(
    declared: OutputChange,
    output_identity_unchanged: bool,
    _continuity_token_unchanged: bool,
    has_output_identity: bool,
    _has_continuity_token: bool,
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

fn canonical_changed_regions(
    changed_regions: &[crate::data::output::ChangedRegion],
) -> Vec<crate::data::output::ChangedRegion> {
    if changed_regions.len() <= 1
        || changed_regions
            .windows(2)
            .all(|window| window[0] < window[1])
    {
        return changed_regions.to_vec();
    }

    let mut canonical = changed_regions.to_vec();
    canonical.sort();
    canonical.dedup();
    canonical
}

fn compare_dependency_to_snapshot(
    dependency: &crate::data::dependency::DependencyEdge,
    snapshot: &DependencySnapshotEntry,
) -> std::cmp::Ordering {
    dependency.sort_key().cmp(&snapshot.sort_key())
}

fn trace_identity_hash(identity: &crate::data::output::OutputIdentity) -> StableHashValue {
    identity.stable_hash()
}

fn trace_output_hash(version: AspectVersion) -> StableHashValue {
    let mut hash = 0xcbf29ce484222325_u128;
    for slot in version.slots() {
        hash ^= *slot as u128;
        hash = hash.wrapping_mul(0x100000001b3_u128);
    }
    hash as StableHashValue
}
