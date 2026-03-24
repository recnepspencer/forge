use crate::data::dependency::{DependencyEdge, DependencySnapshot};
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::trace::{
    ArtifactMergeAuthority, ArtifactWriteDelta, MergeAdoptability, RuntimeArtifactState,
};

use super::{
    AdoptedNodeMaterialization, ArtifactMergeComparable, CausalityCarryPolicy,
    DependencyFingerprint, DependencyRemapRecord, MergeNodeMap, RetainedArtifactCarryPolicy,
    RuntimeArtifactCarryPolicy, SourceNodeAdoptionCarryPolicy, SourceNodeAdoptionPlanCore,
    TargetNodeIdentityIntent,
};

pub(crate) fn merge_comparable(
    runtime: Option<&RuntimeArtifactState>,
) -> Option<ArtifactMergeComparable> {
    let runtime = runtime?;
    Some(ArtifactMergeComparable {
        output_identity: runtime.output_identity.clone(),
        continuity_token: runtime.continuity_token.clone(),
        reuse_basis: runtime.reuse_basis.clone(),
        dependency_fingerprint: DependencyFingerprint {
            dependency_count: runtime.dependency_count,
            meaningful_input_changes: runtime.meaningful_input_changes,
            output_hash: runtime.output_hash,
        },
        authority: runtime.merge_authority.clone(),
    })
}

pub(crate) fn adopt_source_node_into_target(
    target_graph: &mut SignalGraph,
    source_graph: &SignalGraph,
    core: &SourceNodeAdoptionPlanCore,
    carry_policy: &SourceNodeAdoptionCarryPolicy,
    node_map: &MergeNodeMap,
) -> Result<(AdoptedNodeMaterialization, Vec<DependencyRemapRecord>), SignalError> {
    let source_entry = source_graph.get_entry(core.source_node)?.clone();
    let target_node = match core.target_identity {
        TargetNodeIdentityIntent::ExistingMapping { mapped_target_node } => mapped_target_node,
        TargetNodeIdentityIntent::AllocateTargetNode => {
            let mut entry = source_entry.clone();
            entry.set_dependencies_id(crate::data::graph::DependencySetId::EMPTY);
            entry.set_subscribers_id(crate::data::graph::SubscriberSetId::EMPTY);
            entry.set_dep_snapshot_id(crate::data::dependency::DependencySnapshotId::EMPTY);
            apply_carry_policy(&mut entry, &source_entry, carry_policy, &core.authority);
            let target_node = target_graph.create_node_from_entry(entry);
            if let Some(mut runtime) = target_graph
                .get_entry(target_node)?
                .get_runtime_artifact_state()
                .cloned()
            {
                runtime.lineage_artifact_id = Some(
                    target_graph
                        .diagnostics_state_mut()
                        .allocate_lineage_artifact_id(),
                );
                target_graph
                    .get_entry_mut(target_node)?
                    .set_runtime_artifact_state(Some(runtime));
            }
            target_node
        }
    };

    let remapped_edges = remap_dependency_edges(
        target_graph,
        core.source_node,
        core.dependency_topology.dependencies.as_slice(),
        node_map,
    )?;
    let remapped_snapshot = remap_dependency_snapshot(
        core.source_node,
        &core.dependency_snapshot_ref.snapshot,
        node_map,
    )?;
    target_graph
        .get_entry_mut(target_node)?
        .set_eval_config(core.entry_contract.eval_config.clone());
    target_graph.set_dependencies(target_node, remapped_edges.clone())?;
    target_graph.set_dep_snapshot(target_node, remapped_snapshot)?;
    if matches!(
        core.target_identity,
        TargetNodeIdentityIntent::ExistingMapping { .. }
    ) {
        let mut entry = target_graph.get_entry(target_node)?.clone();
        apply_carry_policy(&mut entry, &source_entry, carry_policy, &core.authority);
        target_graph.replace_entry(target_node, entry)?;
        target_graph.set_dependencies(target_node, remapped_edges)?;
    }

    Ok((
        AdoptedNodeMaterialization {
            target_node,
            dependency_count: core.dependency_topology.dependencies.len(),
        },
        build_remap_records(
            core.source_node,
            &core.dependency_topology.dependencies,
            node_map,
        )?,
    ))
}

fn apply_carry_policy(
    entry: &mut crate::data::node::NodeEntry,
    source_entry: &crate::data::node::NodeEntry,
    carry_policy: &SourceNodeAdoptionCarryPolicy,
    authority: &ArtifactMergeAuthority,
) {
    let runtime = match carry_policy.runtime_artifact {
        RuntimeArtifactCarryPolicy::CarryMergeAdoptable
            if matches!(authority.adoptability, MergeAdoptability::Adoptable) =>
        {
            source_entry.get_runtime_artifact_state().cloned()
        }
        RuntimeArtifactCarryPolicy::CarryMergeAdoptable
        | RuntimeArtifactCarryPolicy::RebuildAfterAdoption
        | RuntimeArtifactCarryPolicy::DoNotCarry => None,
    };

    let retained = match carry_policy.retained_artifact {
        RetainedArtifactCarryPolicy::CarryIfPolicyAllows => {
            source_entry.retained_diagnostic_artifact().cloned()
        }
        RetainedArtifactCarryPolicy::ReconstructIfNeeded | RetainedArtifactCarryPolicy::Drop => {
            None
        }
    };

    entry.apply_artifact_write_delta(ArtifactWriteDelta { runtime, retained });

    match carry_policy.causality {
        CausalityCarryPolicy::CarryIfPolicyAllows => {
            entry.set_causality(source_entry.get_causality().cloned());
        }
        CausalityCarryPolicy::Drop => entry.set_causality(None),
    }
}

fn remap_dependency_edges(
    _target_graph: &SignalGraph,
    source_node: NodeId,
    dependencies: &[DependencyEdge],
    node_map: &MergeNodeMap,
) -> Result<Vec<DependencyEdge>, SignalError> {
    let mut remapped = Vec::with_capacity(dependencies.len());
    for edge in dependencies {
        let mapped = node_map.resolve(edge.source()).ok_or_else(|| {
            SignalError::invalid_input(format!(
                "merge adoption for node {} has unresolved dependency remap {}",
                source_node,
                edge.source()
            ))
        })?;
        let rebuilt = match edge.scope_ref().cloned() {
            Some(scope) => DependencyEdge::with_partition_scope(mapped, edge.aspect(), scope),
            None => DependencyEdge::new(mapped, edge.aspect()),
        };
        remapped.push(rebuilt);
    }
    Ok(remapped)
}

pub(crate) fn remap_dependency_snapshot(
    source_node: NodeId,
    snapshot: &DependencySnapshot,
    node_map: &MergeNodeMap,
) -> Result<DependencySnapshot, SignalError> {
    let mut remapped = DependencySnapshot::empty();
    for entry in snapshot.entries() {
        let mapped = node_map.resolve(entry.source).ok_or_else(|| {
            SignalError::invalid_input(format!(
                "merge adoption snapshot for node {} has unresolved dependency remap {}",
                source_node, entry.source
            ))
        })?;
        remapped.record(
            mapped,
            entry.aspect,
            entry.cached_version,
            entry.scope.clone(),
        );
    }
    Ok(remapped.canonicalize_unordered())
}

fn build_remap_records(
    source_node: NodeId,
    dependencies: &[DependencyEdge],
    node_map: &MergeNodeMap,
) -> Result<Vec<DependencyRemapRecord>, SignalError> {
    let mut records = Vec::with_capacity(dependencies.len());
    for dependency in dependencies {
        let target_dependency = node_map.resolve(dependency.source()).ok_or_else(|| {
            SignalError::invalid_input(format!(
                "merge adoption for node {} has unresolved dependency remap {}",
                source_node,
                dependency.source()
            ))
        })?;
        records.push(DependencyRemapRecord {
            source_node,
            source_dependency: dependency.source(),
            target_dependency,
        });
    }
    Ok(records)
}
