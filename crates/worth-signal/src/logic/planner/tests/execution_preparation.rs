use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;
use crate::data::proof::invalidation::revalidation::NodeInvalidationInput;
use crate::data::temporal::LoweredTemporalEligibility;
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::prepared::{ExecutionSnapshot, PreparedEvaluation};

use super::condition_preview::{
    capture_current_dependencies, preview_condition_action, preview_upstream_state,
    TestConditionAction,
};

pub(super) struct TestPreparedTask {
    pub(super) prepared: PreparedEvaluation,
    pub(super) telemetry: TestPrecomputeTelemetry,
}

#[derive(Default)]
pub(super) struct TestPrecomputeTelemetry {
    nodes_evaluated: u64,
    condition_skip_count: u64,
    ondemand_deferred_count: u64,
    debounce_deferred_count: u64,
    temporal_eligibility_lowering_count: u64,
    partition_scope_revert_clean_count: u64,
}

impl TestPrecomputeTelemetry {
    pub(super) fn accumulate(&mut self, other: &Self) {
        self.nodes_evaluated += other.nodes_evaluated;
        self.condition_skip_count += other.condition_skip_count;
        self.ondemand_deferred_count += other.ondemand_deferred_count;
        self.debounce_deferred_count += other.debounce_deferred_count;
        self.temporal_eligibility_lowering_count += other.temporal_eligibility_lowering_count;
        self.partition_scope_revert_clean_count += other.partition_scope_revert_clean_count;
    }
}

pub(super) fn prepare_test_precomputed_task<F>(
    snapshot: &ExecutionSnapshot<'_>,
    node: NodeId,
    precompute: &F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    condition_resolver: &mut impl crate::logic::evaluation::ConditionResolver,
    request_mode: EvaluationRequestMode,
) -> Result<TestPreparedTask, SignalError>
where
    F: Fn(
            NodeId,
            &crate::logic::prepared::ExecutionReadView<'_>,
        ) -> Result<PreparedEvaluation, SignalError>
        + Sync,
{
    let mut telemetry = TestPrecomputeTelemetry::default();
    let graph = snapshot.graph();
    let state = *graph.get_entry(node)?.get_state();
    let dependencies = capture_current_dependencies(graph, node)?;
    let invalidation = graph.node_invalidation_input(node)?;
    if matches!(invalidation, NodeInvalidationInput::Pending(_)) {
        return Ok(TestPreparedTask {
            prepared: PreparedEvaluation::deferred_by_invalidation()
                .with_dependencies(dependencies),
            telemetry,
        });
    }
    if matches!(invalidation, NodeInvalidationInput::ResolvedNoChange(_))
        && matches!(state, NodeState::MaybeStale)
        && !matches!(request_mode, EvaluationRequestMode::ForceOnDemand)
    {
        return Ok(TestPreparedTask {
            prepared: PreparedEvaluation::validated_clean().with_dependencies(dependencies),
            telemetry,
        });
    }
    let dirty_aspects = invalidation
        .resolved_dirty_aspects()
        .expect("pending invalidation returned after the pending branch");
    let structural_revalidation = structural_revalidation_posture(graph, node)?;

    if structural_revalidation.is_none() && matches!(state, NodeState::MaybeStale) {
        let preview = preview_upstream_state(graph, node, comparator_resolver)?;
        telemetry.partition_scope_revert_clean_count = preview.partition_scope_revert_clean_count;
        if preview.unchanged {
            return Ok(TestPreparedTask {
                prepared: PreparedEvaluation::validated_clean().with_dependencies(dependencies),
                telemetry,
            });
        }
    }

    telemetry.nodes_evaluated += 1;
    if matches!(structural_revalidation, Some(true)) {
        let view = snapshot.read_view(node);
        return Ok(TestPreparedTask {
            prepared: precompute(node, &view)?.with_dependencies(dependencies),
            telemetry,
        });
    }
    match preview_condition_action(graph, node, request_mode, dirty_aspects, condition_resolver)? {
        TestConditionAction::Evaluate { temporal_ready } => {
            if temporal_ready.is_some() {
                telemetry.temporal_eligibility_lowering_count += 1;
            }
            let view = snapshot.read_view(node);
            let mut prepared = precompute(node, &view)?;
            if let Some(temporal_ready) = temporal_ready {
                prepared = prepared
                    .with_temporal_eligibility(LoweredTemporalEligibility::Ready(temporal_ready));
            }
            Ok(TestPreparedTask {
                prepared,
                telemetry,
            })
        }
        TestConditionAction::RevertClean => {
            telemetry.condition_skip_count += 1;
            Ok(TestPreparedTask {
                prepared: PreparedEvaluation::reverted_clean_by_condition()
                    .with_dependencies(dependencies),
                telemetry,
            })
        }
        TestConditionAction::Defer {
            on_demand,
            temporal,
            temporal_deferred,
        } => {
            telemetry.condition_skip_count += 1;
            telemetry.ondemand_deferred_count += u64::from(on_demand);
            telemetry.debounce_deferred_count += u64::from(temporal);
            if temporal_deferred.is_some() {
                telemetry.temporal_eligibility_lowering_count += 1;
            }
            let mut prepared = PreparedEvaluation::deferred_by_condition();
            if let Some(temporal_deferred) = temporal_deferred {
                prepared = prepared.with_temporal_eligibility(
                    LoweredTemporalEligibility::Deferred(temporal_deferred),
                );
            }
            Ok(TestPreparedTask {
                prepared: prepared.with_dependencies(dependencies),
                telemetry,
            })
        }
    }
}

pub(super) fn prepare_test_task<F, O>(
    graph: &SignalGraph,
    node: NodeId,
    compute: &mut F,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    condition_resolver: &mut impl crate::logic::evaluation::ConditionResolver,
    request_mode: EvaluationRequestMode,
) -> Result<TestPreparedTask, SignalError>
where
    F: FnMut(NodeId, &SignalGraph) -> Result<O, SignalError>,
    O: crate::data::output::IntoNodeEvaluationResult,
{
    let mut telemetry = TestPrecomputeTelemetry::default();
    let state = *graph.get_entry(node)?.get_state();
    let dependencies = capture_current_dependencies(graph, node)?;
    let invalidation = graph.node_invalidation_input(node)?;
    if matches!(invalidation, NodeInvalidationInput::Pending(_)) {
        return Ok(TestPreparedTask {
            prepared: PreparedEvaluation::deferred_by_invalidation()
                .with_dependencies(dependencies),
            telemetry,
        });
    }
    if matches!(invalidation, NodeInvalidationInput::ResolvedNoChange(_))
        && matches!(state, NodeState::MaybeStale)
        && !matches!(request_mode, EvaluationRequestMode::ForceOnDemand)
    {
        return Ok(TestPreparedTask {
            prepared: PreparedEvaluation::validated_clean().with_dependencies(dependencies),
            telemetry,
        });
    }
    let dirty_aspects = invalidation
        .resolved_dirty_aspects()
        .expect("pending invalidation returned after the pending branch");
    let structural_revalidation = structural_revalidation_posture(graph, node)?;

    if structural_revalidation.is_none() && matches!(state, NodeState::MaybeStale) {
        let preview = preview_upstream_state(graph, node, comparator_resolver)?;
        telemetry.partition_scope_revert_clean_count = preview.partition_scope_revert_clean_count;
        if preview.unchanged {
            return Ok(TestPreparedTask {
                prepared: PreparedEvaluation::validated_clean().with_dependencies(dependencies),
                telemetry,
            });
        }
    }

    telemetry.nodes_evaluated += 1;
    if matches!(structural_revalidation, Some(true)) {
        let result = compute(node, graph)?.into_evaluation_result();
        return Ok(TestPreparedTask {
            prepared: PreparedEvaluation::from_result(result).with_dependencies(dependencies),
            telemetry,
        });
    }
    match preview_condition_action(graph, node, request_mode, dirty_aspects, condition_resolver)? {
        TestConditionAction::Evaluate { temporal_ready } => {
            if temporal_ready.is_some() {
                telemetry.temporal_eligibility_lowering_count += 1;
            }
            let result = compute(node, graph)?.into_evaluation_result();
            let mut prepared = PreparedEvaluation::from_result(result);
            if let Some(temporal_ready) = temporal_ready {
                prepared = prepared
                    .with_temporal_eligibility(LoweredTemporalEligibility::Ready(temporal_ready));
            }
            Ok(TestPreparedTask {
                prepared: prepared.with_dependencies(dependencies),
                telemetry,
            })
        }
        TestConditionAction::RevertClean => {
            telemetry.condition_skip_count += 1;
            Ok(TestPreparedTask {
                prepared: PreparedEvaluation::reverted_clean_by_condition()
                    .with_dependencies(dependencies),
                telemetry,
            })
        }
        TestConditionAction::Defer {
            on_demand,
            temporal,
            temporal_deferred,
        } => {
            telemetry.condition_skip_count += 1;
            telemetry.ondemand_deferred_count += u64::from(on_demand);
            telemetry.debounce_deferred_count += u64::from(temporal);
            if temporal_deferred.is_some() {
                telemetry.temporal_eligibility_lowering_count += 1;
            }
            let mut prepared = PreparedEvaluation::deferred_by_condition();
            if let Some(temporal_deferred) = temporal_deferred {
                prepared = prepared.with_temporal_eligibility(
                    LoweredTemporalEligibility::Deferred(temporal_deferred),
                );
            }
            Ok(TestPreparedTask {
                prepared: prepared.with_dependencies(dependencies),
                telemetry,
            })
        }
    }
}

fn structural_revalidation_posture(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<Option<bool>, SignalError> {
    let Some(pending) = graph.pending_dependency_revalidation(node)? else {
        return Ok(None);
    };
    if pending.dependency_revision() != graph.dependency_revision(node)? {
        return Err(SignalError::invalid_input(format!(
            "pending dependency revalidation for {node} belongs to a stale dependency revision"
        )));
    }
    Ok(pending
        .requires_structural_recompute()
        .then_some(pending.is_resolved()))
}

pub(super) fn apply_test_precompute_telemetry(
    graph: &mut SignalGraph,
    telemetry: &TestPrecomputeTelemetry,
) {
    graph.telemetry_mut().evaluation.nodes_evaluated += telemetry.nodes_evaluated;
    graph.telemetry_mut().evaluation.condition_skip_count += telemetry.condition_skip_count;
    graph.telemetry_mut().evaluation.ondemand_deferred_count += telemetry.ondemand_deferred_count;
    graph.telemetry_mut().evaluation.debounce_deferred_count += telemetry.debounce_deferred_count;
    graph
        .telemetry_mut()
        .temporal
        .temporal_eligibility_lowering_count += telemetry.temporal_eligibility_lowering_count;
    graph
        .telemetry_mut()
        .invalidation
        .partition_scope_revert_clean_count += telemetry.partition_scope_revert_clean_count;
}
