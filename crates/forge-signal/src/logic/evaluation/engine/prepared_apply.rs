use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{MemoizedResultOrigin, NodeEvaluationResult};
use crate::logic::prepared::{
    PreparedDependencyCapture, PreparedEvaluation, PreparedEvaluationOrigin,
    PreparedEvaluationOutcome,
};

use super::metadata::EvaluationExecutionMetadata;
use super::result_apply::apply_evaluation_result_with_policy;

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
    let desired = capture
        .as_slice()
        .iter()
        .map(|dependency| {
            graph.build_dependency_edge(
                dependency.source,
                dependency.aspect,
                dependency.scope.clone(),
            )
        })
        .collect::<Vec<_>>();
    let report = graph.reconcile_dependencies(node, &desired)?;
    Ok(report.update_count())
}

pub(super) fn revert_to_clean(graph: &mut SignalGraph, node: NodeId) -> Result<(), SignalError> {
    graph.telemetry_mut().skipped_by_comparator += 1;
    graph.get_entry_mut(node)?.transition_clean();
    Ok(())
}

fn revert_to_clean_due_to_condition(
    graph: &mut SignalGraph,
    node: NodeId,
) -> Result<(), SignalError> {
    graph.get_entry_mut(node)?.transition_clean();
    Ok(())
}

fn defer_due_to_condition(graph: &mut SignalGraph, node: NodeId) -> Result<(), SignalError> {
    graph
        .get_entry_mut(node)?
        .set_state(crate::data::node::NodeState::MaybeStale);
    Ok(())
}
