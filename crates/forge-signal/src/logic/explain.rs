use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

use crate::data::aspect::{Aspect, AspectMask};
use crate::data::comparator::{
    ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    VersionComparatorPolicy,
};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::{EvaluationCondition, NodeState};
use crate::data::output::{
    ChangedRegion, MemoizedResultOrigin, OutputChange, OutputIdentity, PartitionMatchMode,
    PartitionSubscription,
};
use crate::data::trace::{CausalityMetadata, TraceSummary};

#[derive(Debug, Clone, PartialEq)]
pub enum MeaningfulChangeReason {
    ExactDifference,
    Tolerance { epsilon: u64 },
    OutputIdentity,
    CustomComparator { key: String },
    InheritedComparator,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConditionDecision {
    Deferred,
    RevertedClean,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpstreamCause {
    Changed {
        source: NodeId,
        aspect: Aspect,
        subscription: Option<PartitionSubscription>,
        cached_version: u64,
        current_version: u64,
        comparator: VersionComparatorPolicy,
        reason: MeaningfulChangeReason,
    },
    SkippedByComparator {
        source: NodeId,
        aspect: Aspect,
        subscription: Option<PartitionSubscription>,
        cached_version: u64,
        current_version: u64,
        comparator: VersionComparatorPolicy,
        reason: MeaningfulChangeReason,
    },
    ConditionDeferred {
        source: NodeId,
        aspect: Aspect,
        subscription: Option<PartitionSubscription>,
        cached_version: u64,
        current_version: u64,
        condition: EvaluationCondition,
        decision: ConditionDecision,
    },
    Clean {
        source: NodeId,
        aspect: Aspect,
        subscription: Option<PartitionSubscription>,
        cached_version: u64,
        current_version: u64,
    },
    MissingSnapshot {
        source: NodeId,
        aspect: Aspect,
        subscription: Option<PartitionSubscription>,
        current_version: Option<u64>,
    },
    DependencyRemoved {
        source: NodeId,
        aspect: Aspect,
        subscription: Option<PartitionSubscription>,
        cached_version: u64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct NodeExplanation {
    pub node: NodeId,
    pub state: NodeState,
    pub dirty_aspects: AspectMask,
    pub condition: EvaluationCondition,
    pub trace_summary: Option<TraceSummary>,
    pub output_identity: Option<OutputIdentity>,
    pub output_change: Option<OutputChange>,
    pub changed_regions: Vec<ChangedRegion>,
    pub propagation_suppressed: bool,
    pub memoized_origin: Option<MemoizedResultOrigin>,
    pub upstream: Vec<UpstreamCause>,
    pub causality: Option<CausalityMetadata>,
}

impl fmt::Display for NodeExplanation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Node {} state={:?} condition={:?}",
            self.node, self.state, self.condition
        )?;
        if !self.dirty_aspects.is_empty() {
            writeln!(f, "Dirty aspects: {:?}", self.dirty_aspects)?;
        }
        if let Some(trace) = &self.trace_summary {
            writeln!(
                f,
                "Trace: recomputed={} dependency_count={} meaningful_input_changes={} output_hash={}",
                trace.recomputed,
                trace.dependency_count,
                trace.meaningful_input_changes,
                trace.output_hash
            )?;
            writeln!(
                f,
                "Output: identity={:?} change={:?} propagation_suppressed={} memoized_origin={:?}",
                trace.output_identity,
                trace.output_change,
                trace.propagation_suppressed,
                trace.memoized_origin
            )?;
        }
        if let Some(causality) = &self.causality {
            writeln!(f, "Causality: {}", causality.kind)?;
        }
        if !self.changed_regions.is_empty() {
            writeln!(f, "Changed regions: {}", self.changed_regions.len())?;
        }
        for cause in &self.upstream {
            writeln!(f, "{}", format_upstream_cause(cause))?;
        }
        Ok(())
    }
}

fn format_upstream_cause(cause: &UpstreamCause) -> String {
    match cause {
        UpstreamCause::Changed {
            source,
            aspect,
            subscription,
            cached_version,
            current_version,
            ..
        } => format!(
            "  changed <- {} aspect {} scope {:?} ({} -> {})",
            source,
            aspect.index(),
            subscription,
            cached_version,
            current_version
        ),
        UpstreamCause::SkippedByComparator {
            source,
            aspect,
            subscription,
            cached_version,
            current_version,
            ..
        } => format!(
            "  skipped by comparator <- {} aspect {} scope {:?} ({} -> {})",
            source,
            aspect.index(),
            subscription,
            cached_version,
            current_version
        ),
        UpstreamCause::ConditionDeferred {
            source,
            aspect,
            subscription,
            cached_version,
            current_version,
            condition,
            decision,
        } => format!(
            "  condition {:?}/{:?} <- {} aspect {} scope {:?} ({} -> {})",
            condition,
            decision,
            source,
            aspect.index(),
            subscription,
            cached_version,
            current_version
        ),
        UpstreamCause::Clean {
            source,
            aspect,
            subscription,
            cached_version,
            current_version,
        } => format!(
            "  clean <- {} aspect {} scope {:?} ({} == {})",
            source,
            aspect.index(),
            subscription,
            cached_version,
            current_version
        ),
        UpstreamCause::MissingSnapshot {
            source,
            aspect,
            subscription,
            current_version,
        } => format!(
            "  missing snapshot <- {} aspect {} scope {:?} current={:?}",
            source,
            aspect.index(),
            subscription,
            current_version
        ),
        UpstreamCause::DependencyRemoved {
            source,
            aspect,
            subscription,
            cached_version,
        } => format!(
            "  dependency removed <- {} aspect {} scope {:?} cached={}",
            source,
            aspect.index(),
            subscription,
            cached_version
        ),
    }
}

fn reason_for_policy(policy: &VersionComparatorPolicy, explicit: bool) -> MeaningfulChangeReason {
    match policy {
        VersionComparatorPolicy::Exact => {
            if explicit {
                MeaningfulChangeReason::ExactDifference
            } else {
                MeaningfulChangeReason::InheritedComparator
            }
        }
        VersionComparatorPolicy::Tolerance { epsilon } => {
            MeaningfulChangeReason::Tolerance { epsilon: *epsilon }
        }
        VersionComparatorPolicy::OutputIdentity => MeaningfulChangeReason::OutputIdentity,
        VersionComparatorPolicy::Custom { key } => MeaningfulChangeReason::CustomComparator {
            key: key.clone(),
        },
    }
}

fn classify_condition_decision(
    graph: &SignalGraph,
    node: NodeId,
    condition: &EvaluationCondition,
) -> Option<ConditionDecision> {
    let entry = graph.get_entry(node).ok()?;
    let dirty_aspects = entry.get_dirty_aspects();
    let max_delta = max_dependency_delta(graph, node).ok()?;

    match condition {
        EvaluationCondition::AspectFilter(mask) if !dirty_aspects.is_empty() && !dirty_aspects.intersects(*mask) => {
            Some(ConditionDecision::Deferred)
        }
        EvaluationCondition::OnDemand => Some(ConditionDecision::Deferred),
        EvaluationCondition::DeltaThreshold(threshold)
            if !dirty_aspects.is_empty() && (max_delta as f64) <= *threshold =>
        {
            Some(ConditionDecision::RevertedClean)
        }
        EvaluationCondition::Debounce(_) => Some(ConditionDecision::Deferred),
        EvaluationCondition::Custom(_) => Some(ConditionDecision::Deferred),
        _ => None,
    }
}

fn max_dependency_delta(graph: &SignalGraph, node: NodeId) -> Result<u64, SignalError> {
    let mut max_delta = 0;
    for (source, aspect, cached_version, _) in graph.get_entry(node)?.get_dep_snapshot().entries() {
        if !graph.is_alive(*source) {
            continue;
        }
        let current_version = graph.get_entry(*source)?.get_aspect_version().get(*aspect);
        max_delta = max_delta.max(current_version.abs_diff(*cached_version));
    }
    Ok(max_delta)
}

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
    let current_dependencies = entry.get_dependencies();
    let snapshot_entries = entry.get_dep_snapshot().entries();

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
                graph.get_entry(dependency.source())?
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

        let policy = comparator_resolver
            .policy_for_node(node, entry.get_eval_config().comparator.as_ref());
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
            VersionComparatorPolicy::OutputIdentity | VersionComparatorPolicy::Custom { .. } => upstream.push(UpstreamCause::Changed {
                source: dependency.source(),
                aspect: dependency.aspect(),
                subscription: subscription.clone(),
                cached_version,
                current_version,
                comparator: policy.clone(),
                reason: reason_for_policy(&policy, explicit_comparator),
            }),
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
            (source.index(), source.generation(), aspect.index(), scope_key)
        }
    });

    Ok(NodeExplanation {
        node,
        state,
        dirty_aspects,
        condition,
        trace_summary,
        output_identity,
        output_change,
        changed_regions,
        propagation_suppressed,
        memoized_origin,
        upstream,
        causality,
    })
}

fn partition_scope_untouched(
    trace_summary: Option<&TraceSummary>,
    scope: &PartitionSubscription,
) -> bool {
    let Some(trace_summary) = trace_summary else {
        return false;
    };
    if trace_summary.output_change == OutputChange::Unchanged {
        return true;
    }
    if trace_summary.changed_regions.is_empty() {
        return false;
    }
    !trace_summary
        .changed_regions
        .iter()
        .any(|region| partition_subscription_matches(scope, region))
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

pub fn explain(graph: &SignalGraph, node: NodeId) -> Result<NodeExplanation, SignalError> {
    let resolver = DefaultComparatorPolicyResolver {
        fallback: VersionComparatorPolicy::Exact,
        custom: DefaultComparatorResolver,
    };
    explain_with_policy_resolver(graph, node, &resolver)
}

pub fn dependency_chain_to(
    graph: &SignalGraph,
    root: NodeId,
    target: NodeId,
) -> Result<Option<Vec<NodeId>>, SignalError> {
    graph.get_entry(root)?;
    graph.get_entry(target)?;

    if root == target {
        return Ok(Some(vec![root]));
    }

    let mut queue = VecDeque::from([root]);
    let mut visited = BTreeSet::from([root]);
    let mut previous = BTreeMap::<NodeId, NodeId>::new();

    while let Some(current) = queue.pop_front() {
        let mut subscribers = graph.get_entry(current)?.get_subscribers().to_vec();
        subscribers.sort();
        for subscriber in subscribers {
            if !visited.insert(subscriber) {
                continue;
            }
            previous.insert(subscriber, current);
            if subscriber == target {
                let mut path = vec![target];
                let mut cursor = target;
                while let Some(parent) = previous.get(&cursor).copied() {
                    path.push(parent);
                    if parent == root {
                        path.reverse();
                        return Ok(Some(path));
                    }
                    cursor = parent;
                }
            }
            queue.push_back(subscriber);
        }
    }

    Ok(None)
}
