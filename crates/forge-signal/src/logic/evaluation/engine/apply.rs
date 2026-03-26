use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::dependency::{
    CommittedSnapshotUpdate, DependencyEdge, DependencyInputScan, DependencySnapshot,
    ReplacementSnapshotUpdate, SnapshotDeltaRecord, StableShapeSnapshotBasis,
    VersionOnlySnapshotUpdate, VersionVector,
};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::output::{MemoizedResultOrigin, NodeEvaluationResult};
use crate::data::reuse::{
    ReuseBoundaryAuthority, ReuseBoundaryContext, ReuseCertificationRecord, ReuseOrigin,
};
use crate::logic::evaluation::{
    DiagnosticEnvelope, EffectDependencyInputs, EffectRuntimeMetadata, EvaluationEffect,
    EvaluationVerdict, OperationalEffect, PreparedApplyResult, SuppressionReason,
};
use crate::logic::prepared::PreparedKeyedContext;

use super::metadata::EvaluationExecutionMetadata;

pub(crate) fn apply_effect_with_policy_and_condition(
    graph: &mut SignalGraph,
    node: NodeId,
    result: NodeEvaluationResult,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
    verdict: EvaluationVerdict,
    recomputed: bool,
    reuse_boundary_authority: ReuseBoundaryAuthority,
    reuse_boundary_detail: Option<ReuseBoundaryContext>,
    keyed_context: Option<PreparedKeyedContext>,
    causality: Option<crate::data::trace::CausalityMetadata>,
    reuse_certification: Option<ReuseCertificationRecord>,
    dependency_inputs: Option<EffectDependencyInputs>,
    defer_snapshot_commit: bool,
) -> Result<PreparedApplyResult, SignalError> {
    let dependency_inputs = resolve_effect_dependency_inputs(graph, node, dependency_inputs)?;
    let effect = build_evaluation_effect(
        node,
        result,
        execution_metadata,
        verdict,
        recomputed,
        reuse_boundary_authority,
        reuse_boundary_detail,
        keyed_context,
        causality,
        reuse_certification,
        dependency_inputs,
    );
    let comparator = resolve_effect_comparator(graph, node, comparator_resolver)?;
    let (report, pending_snapshot) = graph.apply_effect(
        effect,
        comparator,
        comparator_resolver,
        defer_snapshot_commit,
    )?;
    Ok(PreparedApplyResult {
        dependency_updates: 0,
        report,
        pending_snapshot,
    })
}

fn resolve_effect_dependency_inputs(
    graph: &mut SignalGraph,
    node: NodeId,
    dependency_inputs: Option<EffectDependencyInputs>,
) -> Result<EffectDependencyInputs, SignalError> {
    match dependency_inputs {
        Some(inputs) if dependency_inputs_match_graph(graph, node, &inputs)? => {
            graph.telemetry_mut().execution.dependency_input_reuse_count += 1;
            Ok(inputs)
        }
        Some(_) => {
            graph
                .telemetry_mut()
                .execution
                .dependency_input_rebuild_count += 1;
            build_effect_dependency_inputs(graph, node)
        }
        None => build_effect_dependency_inputs(graph, node),
    }
}

fn dependency_inputs_match_graph(
    graph: &SignalGraph,
    node: NodeId,
    dependency_inputs: &EffectDependencyInputs,
) -> Result<bool, SignalError> {
    let entry = graph.get_entry(node)?;
    Ok(
        dependency_inputs.context.dependency_set_id == entry.get_dependencies_id()
            && dependency_inputs.context.dependency_snapshot_id == entry.get_dep_snapshot_id(),
    )
}

pub(crate) fn build_evaluation_effect(
    node: NodeId,
    result: NodeEvaluationResult,
    execution_metadata: Option<&EvaluationExecutionMetadata>,
    verdict: EvaluationVerdict,
    recomputed: bool,
    reuse_boundary_authority: ReuseBoundaryAuthority,
    reuse_boundary_detail: Option<ReuseBoundaryContext>,
    keyed_context: Option<PreparedKeyedContext>,
    causality: Option<crate::data::trace::CausalityMetadata>,
    reuse_certification: Option<ReuseCertificationRecord>,
    dependency_inputs: EffectDependencyInputs,
) -> EvaluationEffect {
    let memoized_origin = execution_metadata
        .map(|metadata| metadata.memoized_origin)
        .unwrap_or(MemoizedResultOrigin::DirectCompute);
    let reuse_basis = execution_metadata
        .map(|metadata| metadata.reuse_basis.clone())
        .unwrap_or_else(crate::data::reuse::ReuseBasis::fresh_compute);
    let reuse_origin = execution_metadata
        .map(|metadata| metadata.reuse_origin)
        .unwrap_or(ReuseOrigin::FreshCompute);
    EvaluationEffect {
        operational: OperationalEffect {
            node,
            verdict,
            aspect_version: result.aspect_version,
            output_change: result.output_change,
            reuse_basis,
            reuse_origin,
            reuse_boundary_authority,
            dependency_snapshot_update: dependency_inputs.dependency_snapshot_update,
            snapshot_delta: dependency_inputs.snapshot_delta,
            meaningful_input_changes: dependency_inputs.meaningful_input_changes,
        },
        diagnostics: DiagnosticEnvelope::from_parts(
            result.output_identity,
            result.continuity_token,
            result.changed_regions,
            result.labels,
        ),
        runtime_metadata: EffectRuntimeMetadata {
            memoized_origin,
            recomputed,
            keyed_context,
            causality,
            reuse_certification,
            reuse_boundary_detail,
        },
    }
}

fn resolve_effect_comparator(
    graph: &SignalGraph,
    node: NodeId,
    comparator_resolver: &mut impl ComparatorPolicyResolver,
) -> Result<crate::data::comparator::VersionComparatorPolicy, SignalError> {
    let entry = graph.get_entry(node)?;
    Ok(comparator_resolver.policy_for_node(node, entry.get_eval_config().comparator.as_ref()))
}

pub(crate) fn collect_effect_dependency_inputs_iter<I>(
    graph: &mut SignalGraph,
    nodes: I,
) -> Result<Vec<EffectDependencyInputs>, SignalError>
where
    I: IntoIterator<Item = NodeId>,
{
    nodes
        .into_iter()
        .map(|node| build_effect_dependency_inputs(graph, node))
        .collect()
}

pub(crate) fn verdict_for_evaluated_result(
    graph: &SignalGraph,
    node: NodeId,
    result: &NodeEvaluationResult,
    meaningful_output_change: bool,
) -> Result<EvaluationVerdict, SignalError> {
    let previous_trace = graph.get_entry(node)?.get_runtime_artifact_state();
    let previous_output_identity = previous_trace.and_then(|trace| trace.output_identity.as_ref());
    let previous_continuity_token =
        previous_trace.and_then(|trace| trace.continuity_token.as_ref());

    let output_identity_unchanged = matches!(
        (previous_output_identity, result.output_identity.as_ref()),
        (Some(previous), Some(current)) if previous == current
    );
    let continuity_token_unchanged = matches!(
        (previous_continuity_token, result.continuity_token.as_ref()),
        (Some(previous), Some(current)) if previous == current
    );

    // Suppression precedence is part of the runtime contract:
    // 1. output identity continuity
    // 2. explicit continuity token continuity
    // 3. comparator-match suppression when no recompute occurred
    // 4. otherwise the result is authoritative recomputation
    let verdict = if meaningful_output_change {
        EvaluationVerdict::Recomputed
    } else if output_identity_unchanged {
        EvaluationVerdict::Suppressed {
            reason: SuppressionReason::OutputIdentityUnchanged,
        }
    } else if continuity_token_unchanged
        && previous_output_identity.is_none()
        && result.output_identity.is_none()
    {
        EvaluationVerdict::Suppressed {
            reason: SuppressionReason::ContinuityTokenUnchanged,
        }
    } else {
        EvaluationVerdict::Suppressed {
            reason: SuppressionReason::ComparatorMatch,
        }
    };

    Ok(verdict)
}

fn build_effect_dependency_inputs(
    graph: &mut SignalGraph,
    node: NodeId,
) -> Result<EffectDependencyInputs, SignalError> {
    let entry = graph.get_entry(node)?;
    let context = crate::logic::evaluation::DependencyInputContext {
        dependency_set_id: entry.get_dependencies_id(),
        dependency_snapshot_id: entry.get_dep_snapshot_id(),
    };
    graph.refresh_runtime_dependencies_of(node)?;
    let dependencies = graph.current_runtime_dependencies_of(node)?.to_vec();
    build_effect_dependency_inputs_for_dependencies(graph, node, context, dependencies.as_slice())
}

pub(crate) fn build_effect_dependency_inputs_for_dependencies(
    graph: &mut SignalGraph,
    node: NodeId,
    context: crate::logic::evaluation::DependencyInputContext,
    dependencies: &[DependencyEdge],
) -> Result<EffectDependencyInputs, SignalError> {
    let shape_handle_lookup_start = std::time::Instant::now();
    let previous_shape_handle =
        graph.dependency_snapshot_shape_handle(context.dependency_snapshot_id);
    let shape_handle_lookup_nanos = shape_handle_lookup_start.elapsed().as_nanos();
    let previous_snapshot_fetch_start = std::time::Instant::now();
    let previous_snapshot = graph.get_dep_snapshot(node)?;
    let previous_snapshot_fetch_nanos = previous_snapshot_fetch_start.elapsed().as_nanos();
    let (stable_shape_proved, inputs) = {
        let previous_entries = previous_snapshot.entries();
        let mut previous_index = 0usize;
        let mut shape_stable = dependencies.len() == previous_entries.len();
        let mut changes = 0_u32;
        let mut stable_shape_versions = Vec::with_capacity(dependencies.len());
        let version_scan_start = std::time::Instant::now();
        for dep in dependencies {
            let source = dep.source();
            let aspect = dep.aspect();
            let Some(previous_entry) = previous_entries.get(previous_index) else {
                shape_stable = false;
                break;
            };
            if !graph.is_alive(source) {
                shape_stable = false;
                break;
            }

            let entry = graph.get_entry(source)?;
            let version = entry.version_for_scope(aspect, dep.scope_ref());
            stable_shape_versions.push(version);
            if previous_entry.sort_key() != dep.sort_key() {
                shape_stable = false;
                break;
            }
            if previous_entry.cached_version != version {
                changes += 1;
            }
            previous_index += 1;
        }
        let version_scan_nanos = version_scan_start.elapsed().as_nanos();

        if shape_stable && previous_index == previous_entries.len() {
            let stable_proof_start = std::time::Instant::now();
            let scan = DependencyInputScan::stable_shape(
                node,
                context.dependency_snapshot_id,
                previous_entries.len(),
                stable_shape_versions.len(),
                stable_shape_versions,
            );
            let basis =
                StableShapeSnapshotBasis::prove(&scan, previous_shape_handle).ok_or_else(|| {
                    SignalError::internal("stable-shape dependency scan failed to produce a proof")
                })?;
            let versions = VersionVector::from_scan(&basis, &scan);
            let stable_proof_nanos = stable_proof_start.elapsed().as_nanos();
            let version_delta_start = std::time::Instant::now();
            let snapshot_delta = SnapshotDeltaRecord::for_version_update(
                node,
                previous_snapshot,
                scan.stable_shape_versions(),
            );
            let dependency_snapshot_update = CommittedSnapshotUpdate::VersionOnly(
                VersionOnlySnapshotUpdate::from_basis_and_versions(basis, versions),
            );
            let version_delta_nanos = version_delta_start.elapsed().as_nanos();
            {
                let execution = &mut graph.telemetry_mut().execution;
                execution.dependency_input_shape_handle_lookup_nanos += shape_handle_lookup_nanos;
                execution.dependency_input_previous_snapshot_fetch_nanos +=
                    previous_snapshot_fetch_nanos;
                execution.dependency_input_version_scan_nanos += version_scan_nanos;
                execution.dependency_input_stable_proof_nanos += stable_proof_nanos;
                execution.dependency_input_version_delta_nanos += version_delta_nanos;
                execution.dependency_input_stable_shape_count += 1;
            }
            (
                true,
                EffectDependencyInputs {
                    context,
                    snapshot_delta,
                    dependency_snapshot_update,
                    meaningful_input_changes: changes,
                },
            )
        } else {
            // `runtime_dependencies_of(node)` must preserve canonical dependency order
            // by `DependencyEdge::sort_key()`. Snapshot reuse and delta detection rely
            // on stable ordering between the current dependency view and the prior
            // snapshot entries.
            let mut snapshot = DependencySnapshot::empty();
            let snapshot_entries = previous_entries;
            let mut snapshot_index = 0usize;
            changes = 0_u32;

            let replacement_build_start = std::time::Instant::now();
            for dep in dependencies {
                let source = dep.source();
                let aspect = dep.aspect();
                if graph.is_alive(source) {
                    let entry = graph.get_entry(source)?;
                    let ver = entry.version_for_scope(aspect, dep.scope_ref());
                    snapshot.record(source, aspect, ver, dep.scope_ref().cloned());
                    while snapshot_index < snapshot_entries.len()
                        && snapshot_entries[snapshot_index].sort_key() < dep.sort_key()
                    {
                        snapshot_index += 1;
                    }
                    if snapshot_index < snapshot_entries.len()
                        && snapshot_entries[snapshot_index].sort_key() == dep.sort_key()
                    {
                        if snapshot_entries[snapshot_index].cached_version != ver {
                            changes += 1;
                        }
                        snapshot_index += 1;
                    }
                } else {
                    changes += 1;
                }
            }

            let replacement_snapshot =
                crate::data::dependency::SharedDependencySnapshot::new(snapshot.clone());
            let snapshot_delta =
                SnapshotDeltaRecord::between(node, previous_snapshot, &replacement_snapshot);
            let dependency_snapshot_update =
                CommittedSnapshotUpdate::Replace(ReplacementSnapshotUpdate::from_snapshot(
                    snapshot,
                    graph.dependency_snapshot_shapes_mut(),
                ));
            let replacement_build_nanos = replacement_build_start.elapsed().as_nanos();
            {
                let execution = &mut graph.telemetry_mut().execution;
                execution.dependency_input_shape_handle_lookup_nanos += shape_handle_lookup_nanos;
                execution.dependency_input_previous_snapshot_fetch_nanos +=
                    previous_snapshot_fetch_nanos;
                execution.dependency_input_version_scan_nanos += version_scan_nanos;
                execution.dependency_input_replacement_build_nanos += replacement_build_nanos;
                execution.dependency_input_replacement_count += 1;
            }
            (
                false,
                EffectDependencyInputs {
                    context,
                    snapshot_delta,
                    dependency_snapshot_update,
                    meaningful_input_changes: changes,
                },
            )
        }
    };
    if stable_shape_proved {
        graph
            .telemetry_mut()
            .storage
            .stable_shape_snapshot_proof_count += 1;
    } else {
        graph
            .telemetry_mut()
            .storage
            .stable_shape_snapshot_proof_failure_count += 1;
    }
    Ok(inputs)
}
