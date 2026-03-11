use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{EvaluationCondition, NodeState};
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::prepared::{ExecutionSnapshot, PreparedDependencyCapture, PreparedEvaluation};

use super::super::plan_builder::partition_scope_untouched;
use super::super::types::{EvaluationPlan, ExecutionReport};

#[cfg(test)]
pub(super) fn empty_execution_report(plan: &EvaluationPlan) -> ExecutionReport {
    ExecutionReport {
        plan_summary: plan.summary.clone(),
        stage_count: plan.summary.stage_count,
        task_count: plan.summary.task_count,
        tasks_executed: 0,
        tasks_pruned: 0,
        tasks_validated_clean: 0,
        tasks_deferred_by_condition: 0,
        tasks_reverted_clean_by_condition: 0,
        tasks_satisfied_by_memoization: 0,
        tasks_with_suppressed_propagation: 0,
        execution_snapshots_built: 0,
        prepared_evaluations_produced: 0,
        prepared_evaluations_applied: 0,
        dependency_capture_updates: 0,
        execution_snapshot_nanos: 0,
        stage_precompute_nanos: 0,
        stage_apply_nanos: 0,
        semantic_finalize_nanos: 0,
        semantic_segment_count: 0,
        stages: Vec::new(),
    }
}

#[cfg(test)]
pub(super) struct TestPreparedTask {
    pub prepared: PreparedEvaluation,
    pub telemetry: TestPrecomputeTelemetry,
}

#[cfg(test)]
#[derive(Default)]
pub(super) struct TestPrecomputeTelemetry {
    nodes_evaluated: u64,
    condition_skip_count: u64,
    ondemand_deferred_count: u64,
    debounce_deferred_count: u64,
    partition_scope_revert_clean_count: u64,
}

#[cfg(test)]
impl TestPrecomputeTelemetry {
    pub fn accumulate(&mut self, other: &Self) {
        self.nodes_evaluated += other.nodes_evaluated;
        self.condition_skip_count += other.condition_skip_count;
        self.ondemand_deferred_count += other.ondemand_deferred_count;
        self.debounce_deferred_count += other.debounce_deferred_count;
        self.partition_scope_revert_clean_count += other.partition_scope_revert_clean_count;
    }
}

#[cfg(test)]
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

    if matches!(state, NodeState::MaybeStale) {
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
    match preview_condition_action(graph, node, request_mode, condition_resolver)? {
        TestConditionAction::Evaluate => {
            let view = snapshot.read_view(node);
            let prepared = precompute(node, &view)?;
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
            debounce,
        } => {
            telemetry.condition_skip_count += 1;
            telemetry.ondemand_deferred_count += u64::from(on_demand);
            telemetry.debounce_deferred_count += u64::from(debounce);
            Ok(TestPreparedTask {
                prepared: PreparedEvaluation::deferred_by_condition()
                    .with_dependencies(dependencies),
                telemetry,
            })
        }
    }
}

#[cfg(test)]
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

    if matches!(state, NodeState::MaybeStale) {
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
    match preview_condition_action(graph, node, request_mode, condition_resolver)? {
        TestConditionAction::Evaluate => {
            let result = compute(node, graph)?.into_evaluation_result();
            Ok(TestPreparedTask {
                prepared: PreparedEvaluation::from_result(result).with_dependencies(dependencies),
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
            debounce,
        } => {
            telemetry.condition_skip_count += 1;
            telemetry.ondemand_deferred_count += u64::from(on_demand);
            telemetry.debounce_deferred_count += u64::from(debounce);
            Ok(TestPreparedTask {
                prepared: PreparedEvaluation::deferred_by_condition()
                    .with_dependencies(dependencies),
                telemetry,
            })
        }
    }
}

#[cfg(test)]
pub(super) fn apply_test_precompute_telemetry(
    graph: &mut SignalGraph,
    telemetry: &TestPrecomputeTelemetry,
) {
    graph.telemetry_mut().nodes_evaluated += telemetry.nodes_evaluated;
    graph.telemetry_mut().condition_skip_count += telemetry.condition_skip_count;
    graph.telemetry_mut().ondemand_deferred_count += telemetry.ondemand_deferred_count;
    graph.telemetry_mut().debounce_deferred_count += telemetry.debounce_deferred_count;
    graph.telemetry_mut().partition_scope_revert_clean_count +=
        telemetry.partition_scope_revert_clean_count;
}

#[cfg(test)]
enum TestConditionAction {
    Evaluate,
    RevertClean,
    Defer { on_demand: bool, debounce: bool },
}

#[cfg(test)]
fn preview_condition_action(
    graph: &SignalGraph,
    node: NodeId,
    request_mode: EvaluationRequestMode,
    resolver: &mut impl crate::logic::evaluation::ConditionResolver,
) -> Result<TestConditionAction, SignalError> {
    let entry = graph.get_entry(node)?;
    let dirty_aspects = entry.get_dirty_aspects();
    let has_dependency_snapshot = !graph.get_dep_snapshot(node)?.entries().is_empty();
    let max_dependency_delta = max_dependency_delta(graph, node)?;
    let ctx = crate::logic::evaluation::ConditionEvaluationContext {
        node,
        request_mode,
        dirty_aspects,
        max_dependency_delta,
    };

    match &entry.get_eval_config().condition {
        EvaluationCondition::Always => Ok(TestConditionAction::Evaluate),
        EvaluationCondition::AspectFilter(mask) => {
            if dirty_aspects.is_empty() || dirty_aspects.intersects(*mask) {
                Ok(TestConditionAction::Evaluate)
            } else {
                Ok(TestConditionAction::Defer {
                    on_demand: false,
                    debounce: false,
                })
            }
        }
        EvaluationCondition::OnDemand => match request_mode {
            EvaluationRequestMode::Default => Ok(TestConditionAction::Defer {
                on_demand: true,
                debounce: false,
            }),
            EvaluationRequestMode::ForceOnDemand => Ok(TestConditionAction::Evaluate),
        },
        EvaluationCondition::DeltaThreshold(threshold) => {
            if !has_dependency_snapshot
                || dirty_aspects.is_empty()
                || (max_dependency_delta as f64) > *threshold
            {
                Ok(TestConditionAction::Evaluate)
            } else {
                Ok(TestConditionAction::RevertClean)
            }
        }
        EvaluationCondition::Debounce(quiet_period_ms) => {
            if resolver.debounce_ready(*quiet_period_ms, &ctx)? {
                Ok(TestConditionAction::Evaluate)
            } else {
                Ok(TestConditionAction::Defer {
                    on_demand: false,
                    debounce: true,
                })
            }
        }
        EvaluationCondition::Custom(key) => {
            if resolver.resolve_custom(key, &ctx)? {
                Ok(TestConditionAction::Evaluate)
            } else {
                Ok(TestConditionAction::Defer {
                    on_demand: false,
                    debounce: false,
                })
            }
        }
    }
}

#[cfg(test)]
fn max_dependency_delta(graph: &SignalGraph, node: NodeId) -> Result<u64, SignalError> {
    let mut max_delta = 0;
    for snapshot_entry in graph.get_dep_snapshot(node)?.entries() {
        if !graph.is_alive(snapshot_entry.source) {
            continue;
        }
        let current_version = graph
            .get_entry(snapshot_entry.source)?
            .get_aspect_version()
            .get(snapshot_entry.aspect);
        max_delta = max_delta.max(current_version.abs_diff(snapshot_entry.cached_version));
    }
    Ok(max_delta)
}

#[cfg(test)]
fn capture_current_dependencies(
    graph: &SignalGraph,
    node: NodeId,
) -> Result<PreparedDependencyCapture, SignalError> {
    let mut capture = PreparedDependencyCapture::new();
    for dependency in graph.dependencies_of(node)? {
        capture.record(
            dependency.source(),
            dependency.aspect(),
            dependency.scope_ref().cloned(),
        );
    }
    Ok(capture.into_sorted_unique())
}

#[cfg(test)]
struct UpstreamPreview {
    unchanged: bool,
    partition_scope_revert_clean_count: u64,
}

#[cfg(test)]
fn preview_upstream_state(
    graph: &SignalGraph,
    node: NodeId,
    resolver: &mut impl ComparatorPolicyResolver,
) -> Result<UpstreamPreview, SignalError> {
    let entry = graph.get_entry(node)?;
    let snapshot = graph.get_dep_snapshot(node)?;
    let node_cfg = entry.get_eval_config();
    let comparator = resolver.policy_for_node(node, node_cfg.comparator.as_ref());
    let mut partition_scope_revert_clean_count = 0;

    for snapshot_entry in snapshot.entries() {
        if !graph.is_alive(snapshot_entry.source) {
            return Ok(UpstreamPreview {
                unchanged: false,
                partition_scope_revert_clean_count,
            });
        }
        if !matches!(
            graph.get_entry(snapshot_entry.source)?.get_state(),
            NodeState::Clean
        ) {
            return Ok(UpstreamPreview {
                unchanged: false,
                partition_scope_revert_clean_count,
            });
        }
        let current_version = graph
            .get_entry(snapshot_entry.source)?
            .get_aspect_version()
            .get(snapshot_entry.aspect);
        if let Some(scope) = &snapshot_entry.scope {
            if current_version == snapshot_entry.cached_version {
                continue;
            }
            if partition_scope_untouched(
                graph.get_entry(snapshot_entry.source)?.get_trace_summary(),
                scope,
            ) {
                partition_scope_revert_clean_count += 1;
                continue;
            }
            return Ok(UpstreamPreview {
                unchanged: false,
                partition_scope_revert_clean_count,
            });
        }
        if comparator.has_meaningful_change(
            snapshot_entry.aspect,
            snapshot_entry.cached_version,
            current_version,
            resolver,
        )? {
            return Ok(UpstreamPreview {
                unchanged: false,
                partition_scope_revert_clean_count,
            });
        }
    }

    Ok(UpstreamPreview {
        unchanged: true,
        partition_scope_revert_clean_count,
    })
}
