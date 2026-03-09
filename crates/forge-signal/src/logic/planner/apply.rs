use std::collections::{BTreeMap, BTreeSet};

use crate::data::aspect::AspectMask;
use crate::data::comparator::{ComparatorPolicyResolver, VersionComparatorPolicy};
use crate::data::dependency::{DependencyEdge, DependencySnapshot};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{NodeEntry, NodeState};
use crate::data::trace::TraceSummary;
use crate::logic::prepared::{
    PreparedDependencyCapture, PreparedEvaluation, PreparedEvaluationOrigin,
    PreparedEvaluationOutcome,
};

use super::apply_groups::ApplyGroup;
use super::apply_trace::{build_trace_summary, execution_metadata_for};
use super::precompute::PreparedTaskPatch;

#[derive(Debug, Clone)]
pub(super) struct TaskPatch {
    pub task_index: usize,
    pub node: NodeId,
    pub prepared: PreparedEvaluation,
    pub before_state: NodeState,
    pub before_trace: Option<TraceSummary>,
    pub dependency_updates: u32,
    pub recomputed: bool,
    pub partition_aware: bool,
    pub current_dependencies: Vec<DependencyEdge>,
    pub next_dependencies: Vec<DependencyEdge>,
}

#[derive(Debug, Clone)]
pub(super) struct ConcurrentStagePatches {
    pub tasks: Vec<TaskPatch>,
    pub serial_fallbacks: Vec<PreparedTaskPatch>,
}

pub(super) fn prepare_stage_patches(
    graph: &mut SignalGraph,
    patches: Vec<PreparedTaskPatch>,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<ConcurrentStagePatches, SignalError> {
    let mut tasks = Vec::new();
    let mut serial_fallbacks = Vec::new();

    for patch in patches {
        if !can_materialize_concurrently(graph, &patch, comparator_resolver)? {
            serial_fallbacks.push(patch);
            continue;
        }

        let current_dependencies = graph.dependencies_of(patch.node)?.to_vec();
        let next_dependencies = capture_to_dependency_edges(graph, &patch.prepared.dependencies)?;
        let before_entry = graph.get_entry(patch.node)?.clone();

        let prepared = patch.prepared;
        let recomputed = matches!(prepared.outcome, PreparedEvaluationOutcome::Evaluate)
            && !matches!(prepared.origin, PreparedEvaluationOrigin::MemoizedReuse);
        let partition_aware = !prepared.result.changed_regions.is_empty();

        tasks.push(TaskPatch {
            task_index: patch.task_index,
            node: patch.node,
            prepared,
            before_state: *before_entry.get_state(),
            before_trace: before_entry.get_trace_summary().cloned(),
            dependency_updates: count_dependency_updates(&current_dependencies, &next_dependencies),
            recomputed,
            partition_aware,
            current_dependencies,
            next_dependencies,
        });
    }

    Ok(ConcurrentStagePatches {
        tasks,
        serial_fallbacks,
    })
}

pub(super) fn materialize_apply_group(
    graph: &mut SignalGraph,
    group: &ApplyGroup,
) -> Result<Vec<(NodeId, NodeEntry)>, SignalError> {
    let mut entry_updates: BTreeMap<NodeId, NodeEntry> = BTreeMap::new();
    let mut subscriber_sets: BTreeMap<NodeId, BTreeSet<NodeId>> = BTreeMap::new();

    for task in &group.tasks {
        stage_subscriber_updates(
            graph,
            task.node,
            &task.current_dependencies,
            &task.next_dependencies,
            &mut subscriber_sets,
        )?;
        let before_entry = graph.get_entry(task.node)?.clone();
        let snapshot = build_dep_snapshot_from_edges(graph, &task.next_dependencies)?;
        let snapshot_id = graph.store_dependency_snapshot(snapshot.clone());
        let dependencies_id = graph.store_dependency_edges(&task.next_dependencies);
        let mut next_entry = before_entry.clone();
        next_entry.set_dependencies_id(dependencies_id);
        next_entry.set_dep_snapshot_id(snapshot_id);
        apply_target_local_updates(
            graph,
            task.node,
            &mut next_entry,
            &task.prepared,
            &snapshot,
            task.before_trace.as_ref(),
        )?;
        entry_updates.insert(task.node, next_entry);
    }

    for (source, subscribers) in subscriber_sets {
        let mut entry = entry_updates
            .remove(&source)
            .unwrap_or_else(|| graph.get_entry(source).expect("validated source").clone());
        let subscribers: Vec<_> = subscribers.into_iter().collect();
        entry.set_subscribers_id(graph.store_subscribers(&subscribers));
        entry_updates.insert(source, entry);
    }

    let mut updates = entry_updates.into_iter().collect::<Vec<_>>();
    updates.sort_by_key(|(node, _)| (node.index(), node.generation()));
    Ok(updates)
}

fn can_materialize_concurrently(
    graph: &SignalGraph,
    patch: &PreparedTaskPatch,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<bool, SignalError> {
    if !matches!(patch.prepared.outcome, PreparedEvaluationOutcome::Evaluate) {
        return Ok(true);
    }
    let entry = graph.get_entry(patch.node)?;
    let comparator = comparator_resolver
        .policy_for_node(patch.node, entry.get_eval_config().comparator.as_ref());
    let previous_output_identity = entry
        .get_trace_summary()
        .and_then(|trace| trace.output_identity.clone());
    let output_identity_unchanged = matches!(
        (&previous_output_identity, &patch.prepared.result.output_identity),
        (Some(previous), Some(current)) if previous == current
    );
    Ok(
        !(matches!(comparator, VersionComparatorPolicy::OutputIdentity)
            && output_identity_unchanged),
    )
}

fn stage_subscriber_updates(
    graph: &SignalGraph,
    target: NodeId,
    current_dependencies: &[DependencyEdge],
    next_dependencies: &[DependencyEdge],
    subscriber_sets: &mut BTreeMap<NodeId, BTreeSet<NodeId>>,
) -> Result<(), SignalError> {
    let current_sources = current_dependencies
        .iter()
        .map(|edge| edge.source())
        .collect::<BTreeSet<_>>();
    let next_sources = next_dependencies
        .iter()
        .map(|edge| edge.source())
        .collect::<BTreeSet<_>>();
    for source in current_sources.difference(&next_sources) {
        subscriber_sets
            .entry(*source)
            .or_insert_with(|| current_subscribers(graph, *source))
            .remove(&target);
    }
    for source in next_sources.difference(&current_sources) {
        subscriber_sets
            .entry(*source)
            .or_insert_with(|| current_subscribers(graph, *source))
            .insert(target);
    }
    Ok(())
}

fn current_subscribers(graph: &SignalGraph, node: NodeId) -> BTreeSet<NodeId> {
    graph
        .subscribers_of(node)
        .map(|subscribers| subscribers.iter().copied().collect())
        .unwrap_or_default()
}

fn count_dependency_updates(
    current_dependencies: &[DependencyEdge],
    next_dependencies: &[DependencyEdge],
) -> u32 {
    let mut remaining_next = next_dependencies.to_vec();
    let mut removed = 0usize;
    for current in current_dependencies {
        if let Some(index) = remaining_next.iter().position(|next| next == current) {
            remaining_next.swap_remove(index);
        } else {
            removed += 1;
        }
    }
    (removed + remaining_next.len()) as u32
}

fn apply_target_local_updates(
    graph: &SignalGraph,
    node: NodeId,
    entry: &mut NodeEntry,
    prepared: &PreparedEvaluation,
    snapshot: &DependencySnapshot,
    previous_trace: Option<&TraceSummary>,
) -> Result<(), SignalError> {
    match prepared.outcome {
        PreparedEvaluationOutcome::Evaluate => {
            if let Some(causality) = prepared.trace_data.causality.clone() {
                entry.set_causality(Some(causality));
            }
            let mut result = prepared.result.clone();
            result.labels.extend(prepared.trace_data.labels.clone());
            let previous_output_identity =
                previous_trace.and_then(|trace| trace.output_identity.clone());
            let output_identity_unchanged = matches!(
                (&previous_output_identity, &result.output_identity),
                (Some(previous), Some(current)) if previous == current
            );
            entry.set_aspect_version(result.aspect_version);
            entry.set_trace_summary(Some(build_trace_summary(
                graph,
                node,
                &result,
                snapshot,
                output_identity_unchanged,
                execution_metadata_for(prepared),
                !matches!(prepared.origin, PreparedEvaluationOrigin::MemoizedReuse),
            )?));
            entry.set_state(NodeState::Clean);
            entry.set_dirty_aspects(AspectMask::EMPTY);
            entry.clear_dirty_partition_scopes();
        }
        PreparedEvaluationOutcome::ValidatedClean
        | PreparedEvaluationOutcome::RevertedCleanByCondition => {
            entry.set_state(NodeState::Clean);
            entry.set_dirty_aspects(AspectMask::EMPTY);
            entry.clear_dirty_partition_scopes();
        }
        PreparedEvaluationOutcome::DeferredByCondition => entry.set_state(NodeState::MaybeStale),
    }
    Ok(())
}

fn capture_to_dependency_edges(
    graph: &mut SignalGraph,
    capture: &PreparedDependencyCapture,
) -> Result<Vec<DependencyEdge>, SignalError> {
    let mut edges = Vec::with_capacity(capture.as_slice().len());
    for dependency in capture.as_slice() {
        let edge = match dependency.scope.clone() {
            Some(scope) => {
                let interned_scope = graph.partition_interner_mut().intern_subscription(&scope);
                DependencyEdge::with_scope(
                    dependency.source,
                    dependency.aspect,
                    scope,
                    interned_scope,
                )
            }
            None => DependencyEdge::new(dependency.source, dependency.aspect),
        };
        if !edges.contains(&edge) {
            edges.push(edge);
        }
    }
    Ok(edges)
}

fn build_dep_snapshot_from_edges(
    graph: &SignalGraph,
    dependencies: &[DependencyEdge],
) -> Result<DependencySnapshot, SignalError> {
    let mut snapshot = DependencySnapshot::empty();
    for dep in dependencies {
        if graph.is_alive(dep.source()) {
            let version = graph
                .get_entry(dep.source())?
                .get_aspect_version()
                .get(dep.aspect());
            snapshot.record(
                dep.source(),
                dep.aspect(),
                version,
                dep.scope_ref().cloned(),
            );
        }
    }
    Ok(snapshot)
}
