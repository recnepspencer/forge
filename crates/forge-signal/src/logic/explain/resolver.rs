use std::collections::{BTreeMap, BTreeSet};

use crate::data::comparator::{
    ComparatorPolicyResolver, DefaultComparatorPolicyResolver, DefaultComparatorResolver,
    VersionComparatorPolicy,
};
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::PartitionSubscription;

use super::analysis::{classify_condition_decision, partition_scope_untouched};
use super::types::{
    reason_for_policy, CausalDisposition, CausalLink, NodeExplanation, RewiringDependency,
    RewiringSummary, ScopeProvenance, ScopeProvenanceKind, UpstreamCause,
};
use crate::diagnostics::policy::ArtifactMaterializationMode;

pub fn explain_with_policy_resolver(
    graph: &SignalGraph,
    node: NodeId,
    comparator_resolver: &impl ComparatorPolicyResolver,
) -> Result<NodeExplanation, SignalError> {
    let entry = graph.get_entry(node)?;
    if let Some(fact) = graph.explanation_fact(node) {
        let current_record = entry.historical_artifact_record(node);
        if fact.explanation.state == *entry.get_state()
            && fact.explanation.historical_artifact_record == current_record
        {
            return Ok(fact.explanation.clone());
        }
    }
    let state = *entry.get_state();
    let dirty_aspects = entry.get_dirty_aspects();
    let contract = graph.get_contract(node)?.clone();
    let condition = entry.get_eval_config().condition.clone();
    let historical_artifact_record = entry.historical_artifact_record(node);
    let trace_summary = historical_artifact_record
        .as_ref()
        .map(crate::data::trace::TraceSummary::from_record);
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
    let reuse_basis = trace_summary.as_ref().map(|trace| trace.reuse_basis);
    let reuse_certification = historical_artifact_record
        .as_ref()
        .and_then(|record| record.retained.as_ref())
        .and_then(|retained| retained.reuse_certification.clone());
    let causality = entry.get_causality().cloned();
    let explicit_comparator = entry.get_eval_config().comparator.is_some();
    let condition_decision = classify_condition_decision(graph, node, &condition);

    let mut upstream = Vec::new();
    let current_dependencies = graph.dependencies_of(node)?;
    let snapshot_entries = graph.get_dep_snapshot(node)?.entries();

    let mut snapshot_by_dependency = BTreeMap::new();
    for snapshot_entry in snapshot_entries.iter().cloned() {
        snapshot_by_dependency.insert(
            (
                snapshot_entry.source,
                snapshot_entry.aspect.index(),
                snapshot_entry.scope.clone(),
            ),
            snapshot_entry.cached_version,
        );
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
    let rewiring = rewiring_summary(snapshot_entries, current_dependencies);

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
                    .version_for_scope(dependency.aspect(), dependency.scope_ref()),
            )
        } else {
            None
        };

        let Some(cached_version) = snapshot_by_dependency.get(&key).copied() else {
            let cause = UpstreamCause::MissingSnapshot {
                source: dependency.source(),
                aspect: dependency.aspect(),
                subscription: subscription.clone(),
                current_version,
            };
            upstream.push(cause);
            continue;
        };

        let Some(current_version) = current_version else {
            let cause = UpstreamCause::DependencyRemoved {
                source: dependency.source(),
                aspect: dependency.aspect(),
                subscription: subscription.clone(),
                cached_version,
            };
            upstream.push(cause);
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
            let source_trace = graph
                .get_entry(dependency.source())?
                .get_runtime_artifact_state();
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

    for snapshot_entry in snapshot_entries.iter().cloned() {
        if !current_dependency_set.contains(&(
            snapshot_entry.source,
            snapshot_entry.aspect.index(),
            snapshot_entry.scope.clone(),
        )) {
            upstream.push(UpstreamCause::DependencyRemoved {
                source: snapshot_entry.source,
                aspect: snapshot_entry.aspect,
                subscription: snapshot_entry.scope,
                cached_version: snapshot_entry.cached_version,
            });
        }
    }

    canonicalize_upstream_causes(&mut upstream);

    Ok(NodeExplanation {
        node,
        materialization_mode: ArtifactMaterializationMode::Reconstructed,
        state,
        dirty_aspects,
        contract_reads: contract.semantics.reads,
        contract_produces: contract.semantics.produces,
        contract_partition_scope: contract.semantics.partition_scope.clone(),
        required_context: contract.semantics.required_context,
        condition,
        historical_artifact_record,
        execution_record_id,
        semantic_segment_id,
        output_identity,
        output_change,
        changed_regions,
        propagation_suppressed,
        memoized_origin,
        reuse_basis,
        reuse_certification,
        causal_links: upstream
            .iter()
            .map(|cause| build_causal_link_with_graph(graph, cause))
            .collect(),
        rewiring,
        upstream,
        causality,
    })
}

fn rewiring_summary(
    snapshot_entries: &[crate::data::dependency::DependencySnapshotEntry],
    current_dependencies: &[DependencyEdge],
) -> Option<RewiringSummary> {
    let current = current_dependencies
        .iter()
        .map(|dependency| {
            (
                dependency.source(),
                dependency.aspect(),
                dependency.scope_ref().cloned(),
            )
        })
        .collect::<Vec<_>>();
    let snapshot = snapshot_entries
        .iter()
        .map(|entry| (entry.source, entry.aspect, entry.scope.clone()))
        .collect::<Vec<_>>();

    let mut added = current
        .iter()
        .filter(|candidate| !snapshot.contains(candidate))
        .map(|(source, aspect, subscription)| RewiringDependency {
            source: *source,
            aspect: *aspect,
            subscription: subscription.clone(),
        })
        .collect::<Vec<_>>();
    let mut removed = snapshot
        .iter()
        .filter(|candidate| !current.contains(candidate))
        .map(|(source, aspect, subscription)| RewiringDependency {
            source: *source,
            aspect: *aspect,
            subscription: subscription.clone(),
        })
        .collect::<Vec<_>>();

    if added.is_empty() && removed.is_empty() {
        None
    } else {
        canonicalize_rewiring_dependencies(&mut added);
        canonicalize_rewiring_dependencies(&mut removed);
        Some(RewiringSummary { added, removed })
    }
}

fn canonicalize_upstream_causes(upstream: &mut [UpstreamCause]) {
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
}

fn canonicalize_rewiring_dependencies(dependencies: &mut [RewiringDependency]) {
    dependencies.sort_by_key(rewiring_key);
}

fn rewiring_key(dependency: &RewiringDependency) -> (u32, u32, usize, String, u8) {
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

fn build_causal_link_with_graph(graph: &SignalGraph, cause: &UpstreamCause) -> CausalLink {
    let scope = scope_provenance_for_cause(graph, cause);
    match cause {
        UpstreamCause::Changed {
            source,
            aspect,
            subscription,
            cached_version,
            current_version,
            comparator,
            reason,
        } => {
            let note = scope_note_for_changed(subscription, &scope);
            CausalLink {
                source: Some(*source),
                aspect: Some(*aspect),
                disposition: CausalDisposition::Semantic,
                kind: "Changed".to_string(),
                scope,
                cached_version: Some(*cached_version),
                current_version: Some(*current_version),
                comparator: Some(comparator.clone()),
                reason: Some(reason.clone()),
                note,
            }
        }
        UpstreamCause::SkippedByComparator {
            source,
            aspect,
            subscription: _,
            cached_version,
            current_version,
            comparator,
            reason,
        } => CausalLink {
            source: Some(*source),
            aspect: Some(*aspect),
            disposition: CausalDisposition::Suppressed,
            kind: "SkippedByComparator".to_string(),
            scope,
            cached_version: Some(*cached_version),
            current_version: Some(*current_version),
            comparator: Some(comparator.clone()),
            reason: Some(reason.clone()),
            note: None,
        },
        UpstreamCause::ConditionDeferred {
            source,
            aspect,
            subscription: _,
            cached_version,
            current_version,
            condition,
            decision,
        } => CausalLink {
            source: Some(*source),
            aspect: Some(*aspect),
            disposition: CausalDisposition::Ignored,
            kind: format!("ConditionDeferred::{condition:?}/{decision:?}"),
            scope,
            cached_version: Some(*cached_version),
            current_version: Some(*current_version),
            comparator: None,
            reason: None,
            note: None,
        },
        UpstreamCause::Clean {
            source,
            aspect,
            subscription,
            cached_version,
            current_version,
        } => CausalLink {
            source: Some(*source),
            aspect: Some(*aspect),
            disposition: CausalDisposition::Ignored,
            kind: if subscription.is_some() && cached_version != current_version {
                "ScopeUntouched".to_string()
            } else {
                "Clean".to_string()
            },
            scope,
            cached_version: Some(*cached_version),
            current_version: Some(*current_version),
            comparator: None,
            reason: None,
            note: if subscription.is_some() && cached_version != current_version {
                Some("partition-sensitive validation discarded the upstream change for this local scope".to_string())
            } else {
                None
            },
        },
        UpstreamCause::MissingSnapshot {
            source,
            aspect,
            subscription: _,
            current_version,
        } => CausalLink {
            source: Some(*source),
            aspect: Some(*aspect),
            disposition: CausalDisposition::Conservative,
            kind: "MissingSnapshot".to_string(),
            scope,
            cached_version: None,
            current_version: *current_version,
            comparator: None,
            reason: None,
            note: Some("missing dependency snapshot".to_string()),
        },
        UpstreamCause::DependencyRemoved {
            source,
            aspect,
            subscription: _,
            cached_version,
        } => CausalLink {
            source: Some(*source),
            aspect: Some(*aspect),
            disposition: CausalDisposition::Topology,
            kind: "DependencyRemoved".to_string(),
            scope,
            cached_version: Some(*cached_version),
            current_version: None,
            comparator: None,
            reason: None,
            note: Some("dependency rewired away from current topology".to_string()),
        },
    }
}

fn scope_provenance_for_cause(graph: &SignalGraph, cause: &UpstreamCause) -> ScopeProvenance {
    let (source, subscription, changed, missing_snapshot) = match cause {
        UpstreamCause::Changed {
            source,
            subscription,
            ..
        }
        | UpstreamCause::SkippedByComparator {
            source,
            subscription,
            ..
        }
        | UpstreamCause::ConditionDeferred {
            source,
            subscription,
            ..
        }
        | UpstreamCause::Clean {
            source,
            subscription,
            ..
        }
        | UpstreamCause::MissingSnapshot {
            source,
            subscription,
            ..
        }
        | UpstreamCause::DependencyRemoved {
            source,
            subscription,
            ..
        } => (
            *source,
            subscription.clone(),
            !matches!(
                cause,
                UpstreamCause::Clean { .. } | UpstreamCause::DependencyRemoved { .. }
            ),
            matches!(cause, UpstreamCause::MissingSnapshot { .. }),
        ),
    };

    let Some(validation_scope) = subscription else {
        return ScopeProvenance {
            source_scope: None,
            validation_scope: None,
            kind: ScopeProvenanceKind::None,
            note: None,
        };
    };

    if missing_snapshot {
        return ScopeProvenance {
            source_scope: Some(validation_scope.clone()),
            validation_scope: Some(validation_scope),
            kind: ScopeProvenanceKind::InsufficientEvidence,
            note: Some(
                "recomputed conservatively because dependency snapshot evidence was missing"
                    .to_string(),
            ),
        };
    }

    let source_scope = graph
        .get_entry(source)
        .ok()
        .and_then(|entry| entry.get_runtime_artifact_state())
        .and_then(|trace| {
            translated_source_scope(trace.changed_scopes.as_slice(), &validation_scope)
        });

    let (kind, note) = match (source_scope.as_ref(), changed) {
        (Some(source_scope), true) if *source_scope != validation_scope => (
            ScopeProvenanceKind::Translated,
            Some("upstream region evidence was translated into this node's validation scope".to_string()),
        ),
        (Some(_), true) => (
            ScopeProvenanceKind::Direct,
            None,
        ),
        (Some(_), false) => (
            ScopeProvenanceKind::Discarded,
            Some("scope evidence was considered local but untouched for this node".to_string()),
        ),
        (None, true) => (
            ScopeProvenanceKind::InsufficientEvidence,
            Some("partition-sensitive validation fell back because upstream region evidence was insufficient".to_string()),
        ),
        (None, false) => (
            ScopeProvenanceKind::Direct,
            None,
        ),
    };

    ScopeProvenance {
        source_scope: source_scope.or_else(|| Some(validation_scope.clone())),
        validation_scope: Some(validation_scope),
        kind,
        note,
    }
}

fn scope_note_for_changed(
    subscription: &Option<crate::data::output::PartitionSubscription>,
    scope: &ScopeProvenance,
) -> Option<String> {
    subscription.as_ref().map(|_| match scope.kind {
        ScopeProvenanceKind::Translated => {
            "partition-sensitive scope participated via translated upstream region evidence"
                .to_string()
        }
        _ => "partition-sensitive scope participated directly in this causal frontier".to_string(),
    })
}

fn translated_source_scope(
    changed_scopes: &[PartitionSubscription],
    validation_scope: &PartitionSubscription,
) -> Option<PartitionSubscription> {
    changed_scopes
        .iter()
        .find(|scope| scope.partition == validation_scope.partition)
        .cloned()
}

pub fn explain(graph: &SignalGraph, node: NodeId) -> Result<NodeExplanation, SignalError> {
    let resolver = DefaultComparatorPolicyResolver {
        fallback: VersionComparatorPolicy::Exact,
        custom: DefaultComparatorResolver,
    };
    explain_with_policy_resolver(graph, node, &resolver)
}
