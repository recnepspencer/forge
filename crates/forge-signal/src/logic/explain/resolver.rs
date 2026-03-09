use std::collections::{BTreeMap, BTreeSet};

use crate::data::comparator::{
    ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    VersionComparatorPolicy,
};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;

use super::analysis::{classify_condition_decision, partition_scope_untouched};
use super::types::{reason_for_policy, NodeExplanation, UpstreamCause};

pub fn explain_with_policy_resolver(
    graph: &SignalGraph,
    node: NodeId,
    comparator_resolver: &impl ComparatorPolicyResolver,
) -> Result<NodeExplanation, SignalError> {
    let entry = graph.get_entry(node)?;
    let state = *entry.get_state();
    let dirty_aspects = entry.get_dirty_aspects();
    let condition = entry.get_eval_config().condition.clone();
    let trace_summary = entry.get_trace_summary().cloned();
    let output_identity = trace_summary
        .as_ref()
        .and_then(|trace| trace.output_identity.clone());
    let execution_record_id = trace_summary
        .as_ref()
        .and_then(|trace| trace.execution_record_id);
    let semantic_segment_id = trace_summary
        .as_ref()
        .and_then(|trace| trace.semantic_segment_id);
    let output_change = trace_summary.as_ref().map(|trace| trace.output_change);
    let changed_regions = trace_summary
        .as_ref()
        .map(|trace| trace.changed_regions.clone())
        .unwrap_or_default();
    let propagation_suppressed = trace_summary
        .as_ref()
        .map(|trace| trace.propagation_suppressed)
        .unwrap_or(false);
    let memoized_origin = trace_summary.as_ref().map(|trace| trace.memoized_origin);
    let causality = entry.get_causality().cloned();
    let explicit_comparator = entry.get_eval_config().comparator.is_some();
    let condition_decision = classify_condition_decision(graph, node, &condition);

    let mut upstream = Vec::new();
    let current_dependencies = graph.dependencies_of(node)?;
    let snapshot_entries = graph.get_dep_snapshot(node)?.entries();

    let mut snapshot_by_dependency = BTreeMap::new();
    for (source, aspect, cached_version, scope) in snapshot_entries.iter().cloned() {
        snapshot_by_dependency.insert((source, aspect.index(), scope.clone()), cached_version);
    }
    let current_dependency_set: BTreeSet<_> = current_dependencies
        .iter()
        .map(|dependency| {
            (
                dependency.source(),
                dependency.aspect().index(),
                dependency.scope_ref().cloned(),
            )
        })
        .collect();

    for dependency in current_dependencies {
        let subscription = dependency.scope_ref().cloned();
        let key = (
            dependency.source(),
            dependency.aspect().index(),
            subscription.clone(),
        );
        let current_version = if graph.is_alive(dependency.source()) {
            Some(
                graph
                    .get_entry(dependency.source())?
                    .get_aspect_version()
                    .get(dependency.aspect()),
            )
        } else {
            None
        };

        let Some(cached_version) = snapshot_by_dependency.get(&key).copied() else {
            upstream.push(UpstreamCause::MissingSnapshot {
                source: dependency.source(),
                aspect: dependency.aspect(),
                subscription,
                current_version,
            });
            continue;
        };

        let Some(current_version) = current_version else {
            upstream.push(UpstreamCause::DependencyRemoved {
                source: dependency.source(),
                aspect: dependency.aspect(),
                subscription,
                cached_version,
            });
            continue;
        };

        if current_version == cached_version {
            upstream.push(UpstreamCause::Clean {
                source: dependency.source(),
                aspect: dependency.aspect(),
                subscription,
                cached_version,
                current_version,
            });
            continue;
        }

        if let Some(scope) = subscription.as_ref() {
            let source_trace = graph.get_entry(dependency.source())?.get_trace_summary();
            if partition_scope_untouched(source_trace, scope) {
                upstream.push(UpstreamCause::Clean {
                    source: dependency.source(),
                    aspect: dependency.aspect(),
                    subscription,
                    cached_version,
                    current_version,
                });
                continue;
            }
        }

        if let Some(decision) = condition_decision {
            upstream.push(UpstreamCause::ConditionDeferred {
                source: dependency.source(),
                aspect: dependency.aspect(),
                subscription,
                cached_version,
                current_version,
                condition: condition.clone(),
                decision,
            });
            continue;
        }

        let policy =
            comparator_resolver.policy_for_node(node, entry.get_eval_config().comparator.as_ref());
        match &policy {
            VersionComparatorPolicy::Exact => upstream.push(UpstreamCause::Changed {
                source: dependency.source(),
                aspect: dependency.aspect(),
                subscription: subscription.clone(),
                cached_version,
                current_version,
                comparator: policy.clone(),
                reason: reason_for_policy(&policy, explicit_comparator),
            }),
            VersionComparatorPolicy::Tolerance { epsilon } => {
                if current_version.abs_diff(cached_version) > *epsilon {
                    upstream.push(UpstreamCause::Changed {
                        source: dependency.source(),
                        aspect: dependency.aspect(),
                        subscription: subscription.clone(),
                        cached_version,
                        current_version,
                        comparator: policy.clone(),
                        reason: reason_for_policy(&policy, explicit_comparator),
                    });
                } else {
                    upstream.push(UpstreamCause::SkippedByComparator {
                        source: dependency.source(),
                        aspect: dependency.aspect(),
                        subscription: subscription.clone(),
                        cached_version,
                        current_version,
                        comparator: policy.clone(),
                        reason: reason_for_policy(&policy, explicit_comparator),
                    });
                }
            }
            VersionComparatorPolicy::OutputIdentity | VersionComparatorPolicy::Custom { .. } => {
                upstream.push(UpstreamCause::Changed {
                    source: dependency.source(),
                    aspect: dependency.aspect(),
                    subscription: subscription.clone(),
                    cached_version,
                    current_version,
                    comparator: policy.clone(),
                    reason: reason_for_policy(&policy, explicit_comparator),
                })
            }
        }
    }

    for (source, aspect, cached_version, subscription) in snapshot_entries.iter().cloned() {
        if !current_dependency_set.contains(&(source, aspect.index(), subscription.clone())) {
            upstream.push(UpstreamCause::DependencyRemoved {
                source,
                aspect,
                subscription,
                cached_version,
            });
        }
    }

    upstream.sort_by_key(|cause| match cause {
        UpstreamCause::Changed { source, aspect, .. }
        | UpstreamCause::SkippedByComparator { source, aspect, .. }
        | UpstreamCause::ConditionDeferred { source, aspect, .. }
        | UpstreamCause::Clean { source, aspect, .. }
        | UpstreamCause::MissingSnapshot { source, aspect, .. }
        | UpstreamCause::DependencyRemoved { source, aspect, .. } => {
            let scope_key = match cause {
                UpstreamCause::Changed { subscription, .. }
                | UpstreamCause::SkippedByComparator { subscription, .. }
                | UpstreamCause::ConditionDeferred { subscription, .. }
                | UpstreamCause::Clean { subscription, .. }
                | UpstreamCause::MissingSnapshot { subscription, .. }
                | UpstreamCause::DependencyRemoved { subscription, .. } => subscription
                    .as_ref()
                    .map(|scope| {
                        (
                            scope.partition.0.clone(),
                            scope.detail.clone().unwrap_or_default(),
                            scope.match_mode as u8,
                        )
                    })
                    .unwrap_or_default(),
            };
            (
                source.index(),
                source.generation(),
                aspect.index(),
                scope_key,
            )
        }
    });

    Ok(NodeExplanation {
        node,
        state,
        dirty_aspects,
        condition,
        trace_summary,
        execution_record_id,
        semantic_segment_id,
        output_identity,
        output_change,
        changed_regions,
        propagation_suppressed,
        memoized_origin,
        upstream,
        causality,
    })
}

pub fn explain(graph: &SignalGraph, node: NodeId) -> Result<NodeExplanation, SignalError> {
    let resolver = DefaultComparatorPolicyResolver {
        fallback: VersionComparatorPolicy::Exact,
        custom: DefaultComparatorResolver,
    };
    explain_with_policy_resolver(graph, node, &resolver)
}
