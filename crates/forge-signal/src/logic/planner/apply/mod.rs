#[cfg(feature = "parallel")]
pub(crate) mod groups;
#[cfg(feature = "parallel")]
pub(crate) mod full_parallel;
pub(crate) mod rewiring;
pub(crate) mod stage;

#[cfg(feature = "parallel")]
use crate::data::comparator::{ComparatorPolicyResolver, VersionComparatorPolicy};
#[cfg(feature = "parallel")]
use crate::data::dependency::DependencyEdge;
#[cfg(feature = "parallel")]
use crate::data::error::SignalError;
#[cfg(feature = "parallel")]
use crate::data::graph::SignalGraph;
#[cfg(feature = "parallel")]
use crate::data::handle::NodeId;
#[cfg(feature = "parallel")]
use crate::data::trace::TraceSummary;
#[cfg(feature = "parallel")]
use crate::logic::explain::{RewiringDependency, RewiringSummary};
#[cfg(feature = "parallel")]
use crate::logic::prepared::{PreparedEvaluation, PreparedEvaluationOrigin, PreparedEvaluationOutcome};

#[cfg(feature = "parallel")]
use super::precompute::PreparedTaskPatch;

#[cfg(feature = "parallel")]
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
    pub rewiring: Option<RewiringSummary>,
}

#[cfg(feature = "parallel")]
#[derive(Debug, Clone)]
pub(super) struct ConcurrentStagePatches {
    pub tasks: Vec<TaskPatch>,
    pub serial_fallbacks: Vec<PreparedTaskPatch>,
}

#[cfg(feature = "parallel")]
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

        let current_dependencies = graph.runtime_dependencies_of(patch.node)?.to_vec();
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
            rewiring: rewiring_summary_from_edges(&current_dependencies, &next_dependencies),
        });
    }

    Ok(ConcurrentStagePatches {
        tasks,
        serial_fallbacks,
    })
}

#[cfg(feature = "parallel")]
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

#[cfg(feature = "parallel")]
fn count_dependency_updates(
    current_dependencies: &[DependencyEdge],
    next_dependencies: &[DependencyEdge],
) -> u32 {
    let mut current_index = 0usize;
    let mut next_index = 0usize;
    let mut changes = 0u32;

    while current_index < current_dependencies.len() && next_index < next_dependencies.len() {
        match compare_dependency_edges(
            &current_dependencies[current_index],
            &next_dependencies[next_index],
        ) {
            std::cmp::Ordering::Less => {
                changes += 1;
                current_index += 1;
            }
            std::cmp::Ordering::Greater => {
                changes += 1;
                next_index += 1;
            }
            std::cmp::Ordering::Equal => {
                current_index += 1;
                next_index += 1;
            }
        }
    }

    changes
        + (current_dependencies.len() - current_index) as u32
        + (next_dependencies.len() - next_index) as u32
}

#[cfg(feature = "parallel")]
fn capture_to_dependency_edges(
    graph: &mut SignalGraph,
    capture: &PreparedDependencyCapture,
) -> Result<Vec<DependencyEdge>, SignalError> {
    let mut edges = Vec::with_capacity(capture.as_slice().len());
    for dependency in capture.as_slice() {
        let edge = graph.build_dependency_edge(
            dependency.source,
            dependency.aspect,
            dependency.scope.clone(),
        );
        if !edges.contains(&edge) {
            edges.push(edge);
        }
    }
    Ok(edges)
}

#[cfg(feature = "parallel")]
pub(super) fn rewiring_summary_from_edges(
    current_dependencies: &[DependencyEdge],
    next_dependencies: &[DependencyEdge],
) -> Option<RewiringSummary> {
    let mut added = next_dependencies
        .iter()
        .filter(|candidate| !current_dependencies.contains(candidate))
        .map(rewiring_dependency_from_edge)
        .collect::<Vec<_>>();
    let mut removed = current_dependencies
        .iter()
        .filter(|candidate| !next_dependencies.contains(candidate))
        .map(rewiring_dependency_from_edge)
        .collect::<Vec<_>>();

    if added.is_empty() && removed.is_empty() {
        None
    } else {
        added.sort_by_key(rewiring_dependency_key);
        removed.sort_by_key(rewiring_dependency_key);
        Some(RewiringSummary { added, removed })
    }
}

#[cfg(feature = "parallel")]
fn rewiring_dependency_from_edge(edge: &DependencyEdge) -> RewiringDependency {
    RewiringDependency {
        source: edge.source(),
        aspect: edge.aspect(),
        subscription: edge.scope_ref().cloned(),
    }
}

#[cfg(feature = "parallel")]
fn rewiring_dependency_key(dependency: &RewiringDependency) -> (u32, u32, usize, String, u8) {
    let scope = dependency.subscription.as_ref().map(|subscription| {
        (
            subscription.detail.clone().unwrap_or_default(),
            subscription.match_mode as u8,
        )
    });
    (
        dependency.source.index(),
        dependency.source.generation(),
        dependency.aspect.index(),
        scope
            .as_ref()
            .map(|(detail, _)| detail.clone())
            .unwrap_or_default(),
        scope.as_ref().map(|(_, mode)| *mode).unwrap_or_default(),
    )
}

#[cfg(feature = "parallel")]
fn compare_dependency_edges(left: &DependencyEdge, right: &DependencyEdge) -> std::cmp::Ordering {
    (
        left.source().index(),
        left.source().generation(),
        left.aspect().index(),
        left.scope_ref(),
    )
        .cmp(&(
            right.source().index(),
            right.source().generation(),
            right.aspect().index(),
            right.scope_ref(),
        ))
}
