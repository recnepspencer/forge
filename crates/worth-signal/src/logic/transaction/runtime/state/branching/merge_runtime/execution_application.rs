use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::logic::transaction::runtime::{
    BranchMergeConflictKind, BranchMergePlan, NodeReconciliationDecision, NodeReconciliationShape,
};

use super::super::super::merge::{adopt_source_node_into_target, remap_dependency_snapshot};
use super::execution_preparation::PreparedMergeExecution;

pub(super) fn apply_governed_merge<D, I, T>(
    prepared: &mut PreparedMergeExecution<D, I, T>,
    plan: &BranchMergePlan,
) -> Result<(), SignalError>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    for (core, policy) in plan.adoption_core().iter().zip(plan.adoption_policy()) {
        let (materialized, remaps) = adopt_source_node_into_target(
            &mut prepared.target_state,
            prepared.source_state.graph(),
            core,
            policy,
            &prepared.node_map,
        )?;
        prepared
            .node_map
            .insert(core.source_node, materialized.target_node);
        prepared.touched.insert(materialized.target_node);
        prepared
            .repaired_sources
            .extend(remaps.iter().map(|record| record.target_dependency));
        prepared.dependency_remaps.extend(remaps);
    }

    for node_plan in plan.node_plan() {
        match node_plan.shape() {
            NodeReconciliationShape::ExistingTargetNode { target_node } => {
                let source_image = prepared
                    .source_state
                    .graph()
                    .node_checkpoint_image(node_plan.source_node())?;
                let mut replacement = source_image.clone();
                let (dependencies_id, dep_snapshot_id) = prepared
                    .target_state
                    .graph()
                    .node_dependency_ids(target_node)?;
                replacement.set_dependencies_id(dependencies_id);
                replacement.set_subscribers_id(
                    prepared
                        .target_state
                        .graph()
                        .node_subscribers_id(target_node)?,
                );
                replacement.set_dep_snapshot_id(dep_snapshot_id);
                if matches!(
                    node_plan.decision(),
                    NodeReconciliationDecision::AdoptSourceAuthority
                ) {
                    prepared.repaired_sources.extend(
                        prepared
                            .target_state
                            .graph()
                            .dependencies_of(target_node)?
                            .iter()
                            .map(|edge| edge.source()),
                    );
                    let mapped_edges = prepared
                        .source_state
                        .graph()
                        .dependencies_of(node_plan.source_node())?
                        .iter()
                        .map(|edge| {
                            let mapped =
                                prepared.node_map.resolve(edge.source()).ok_or_else(|| {
                                    SignalError::invalid_input(format!(
                                        "merge plan has unresolved dependency remap {} for node {}",
                                        edge.source(),
                                        node_plan.source_node()
                                    ))
                                })?;
                            Ok(match edge.scope_ref().cloned() {
                                Some(scope) => DependencyEdge::with_partition_scope(
                                    mapped,
                                    edge.aspect(),
                                    scope,
                                ),
                                None => DependencyEdge::new(mapped, edge.aspect()),
                            })
                        })
                        .collect::<Result<Vec<_>, SignalError>>()?;
                    prepared
                        .repaired_sources
                        .extend(mapped_edges.iter().map(|edge| edge.source()));
                    let remapped_snapshot = remap_dependency_snapshot(
                        node_plan.source_node(),
                        prepared
                            .source_state
                            .graph()
                            .get_dep_snapshot(node_plan.source_node())?,
                        &prepared.node_map,
                    )?;
                    prepared
                        .target_state
                        .replace_node_from_checkpoint_image(target_node, replacement)?;
                    prepared
                        .target_state
                        .graph_mut()
                        .set_dependencies(target_node, mapped_edges)?;
                    prepared
                        .target_state
                        .graph_mut()
                        .set_dep_snapshot(target_node, remapped_snapshot)?;
                    prepared.touched.insert(target_node);
                } else if matches!(
                    node_plan.decision(),
                    NodeReconciliationDecision::MarkEquivalentUnchanged
                ) && node_plan
                    .resolved_conflict_kinds()
                    .iter()
                    .any(|kind| matches!(kind, BranchMergeConflictKind::DependencySnapshotMismatch))
                {
                    let remapped_snapshot = remap_dependency_snapshot(
                        node_plan.source_node(),
                        prepared
                            .source_state
                            .graph()
                            .get_dep_snapshot(node_plan.source_node())?,
                        &prepared.node_map,
                    )?;
                    prepared
                        .target_state
                        .graph_mut()
                        .set_dep_snapshot(target_node, remapped_snapshot)?;
                    prepared.touched.insert(target_node);
                }
            }
            NodeReconciliationShape::SourceOnlyIntroduction => {
                if let Some(mapped) = prepared.node_map.resolve(node_plan.source_node()) {
                    prepared.touched.insert(mapped);
                }
            }
        }
    }
    Ok(())
}
