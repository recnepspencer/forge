use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{EvaluationCondition, NodeState};
use crate::data::temporal::{
    ClockTick, DeferredTemporalEligibility, ReadyTemporalEligibility, TemporalCondition,
};
use crate::logic::evaluation::EvaluationRequestMode;
use crate::logic::prepared::PreparedDependencyCapture;

use super::super::plan_builder::partition_scope_untouched;

pub(super) enum TestConditionAction {
    Evaluate {
        temporal_ready: Option<ReadyTemporalEligibility>,
    },
    RevertClean,
    Defer {
        on_demand: bool,
        temporal: bool,
        temporal_deferred: Option<DeferredTemporalEligibility>,
    },
}

pub(super) fn preview_condition_action(
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
        required_context: graph.get_contract(node)?.semantics.required_context,
    };

    match &entry.get_eval_config().condition {
        EvaluationCondition::Always => Ok(TestConditionAction::Evaluate {
            temporal_ready: None,
        }),
        EvaluationCondition::AspectFilter(mask) => {
            if dirty_aspects.is_empty() || dirty_aspects.intersects(*mask) {
                Ok(TestConditionAction::Evaluate {
                    temporal_ready: None,
                })
            } else {
                Ok(TestConditionAction::Defer {
                    on_demand: false,
                    temporal: false,
                    temporal_deferred: None,
                })
            }
        }
        EvaluationCondition::OnDemand => match request_mode {
            EvaluationRequestMode::Default => Ok(TestConditionAction::Defer {
                on_demand: true,
                temporal: false,
                temporal_deferred: None,
            }),
            EvaluationRequestMode::ForceOnDemand => Ok(TestConditionAction::Evaluate {
                temporal_ready: None,
            }),
        },
        EvaluationCondition::DeltaThreshold(threshold) => {
            if !has_dependency_snapshot
                || dirty_aspects.is_empty()
                || (max_dependency_delta as f64) > *threshold
            {
                Ok(TestConditionAction::Evaluate {
                    temporal_ready: None,
                })
            } else {
                Ok(TestConditionAction::RevertClean)
            }
        }
        EvaluationCondition::Temporal(condition) => {
            let condition = condition.clone();
            if resolver.resolve_temporal(&condition, &ctx)? {
                Ok(TestConditionAction::Evaluate {
                    temporal_ready: Some(ReadyTemporalEligibility::runtime_clock_backed(
                        condition,
                        ClockTick::ZERO,
                    )),
                })
            } else {
                Ok(TestConditionAction::Defer {
                    on_demand: false,
                    temporal: matches!(condition, TemporalCondition::Debounce(_)),
                    temporal_deferred: Some(DeferredTemporalEligibility::runtime_clock_backed(
                        condition,
                        ClockTick::ZERO,
                    )),
                })
            }
        }
        EvaluationCondition::Custom(key) => {
            if resolver.resolve_custom(key, &ctx)? {
                Ok(TestConditionAction::Evaluate {
                    temporal_ready: None,
                })
            } else {
                Ok(TestConditionAction::Defer {
                    on_demand: false,
                    temporal: false,
                    temporal_deferred: None,
                })
            }
        }
        EvaluationCondition::Installed(_) => Err(SignalError::invalid_input(
            "installed conditional authority must execute through the owner-bound runtime path",
        )),
    }
}

pub(super) fn max_dependency_delta(graph: &SignalGraph, node: NodeId) -> Result<u64, SignalError> {
    let mut max_delta = 0;
    for snapshot_entry in graph.get_dep_snapshot(node)?.entries() {
        if !graph.is_alive(snapshot_entry.source) {
            continue;
        }
        let current_version = graph
            .get_entry(snapshot_entry.source)?
            .version_for_scope(snapshot_entry.aspect, snapshot_entry.scope.as_ref());
        max_delta = max_delta.max(current_version.abs_diff(snapshot_entry.cached_version));
    }
    Ok(max_delta)
}

pub(super) fn capture_current_dependencies(
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

pub(super) struct UpstreamPreview {
    pub(super) unchanged: bool,
    pub(super) partition_scope_revert_clean_count: u64,
}

pub(super) fn preview_upstream_state(
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
            .version_for_scope(snapshot_entry.aspect, snapshot_entry.scope.as_ref());
        if let Some(scope) = &snapshot_entry.scope {
            if current_version == snapshot_entry.cached_version {
                continue;
            }
            if partition_scope_untouched(
                graph
                    .get_entry(snapshot_entry.source)?
                    .get_runtime_artifact_state()
                    .map(|state| state.hot()),
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
