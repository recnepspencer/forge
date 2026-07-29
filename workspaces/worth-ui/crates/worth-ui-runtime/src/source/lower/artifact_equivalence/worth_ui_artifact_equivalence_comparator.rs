use crate::source::{
    WorthUiArtifact, WorthUiArtifactDifference, WorthUiArtifactDigestor,
    WorthUiArtifactEquivalence, WorthUiArtifactEquivalenceBasis, WorthUiArtifactEquivalenceDenial,
    WorthUiArtifactEquivalenceMetrics, WorthUiArtifactNode, WorthUiArtifactSemanticDelta,
};

use super::worth_ui_artifact_semantic_basis::node_semantic_basis;

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiArtifactEquivalenceComparator;

impl WorthUiArtifactEquivalenceComparator {
    #[cfg(test)]
    pub(crate) fn compare(
        left: &WorthUiArtifact,
        right: &WorthUiArtifact,
        basis: WorthUiArtifactEquivalenceBasis,
    ) -> WorthUiArtifactEquivalence {
        Self::compare_bounded(left, right, basis, usize::MAX)
            .expect("unbounded artifact comparison cannot exhaust structural entries")
    }

    pub(crate) fn compare_bounded(
        left: &WorthUiArtifact,
        right: &WorthUiArtifact,
        basis: WorthUiArtifactEquivalenceBasis,
        structural_entry_limit: usize,
    ) -> Result<WorthUiArtifactEquivalence, WorthUiArtifactEquivalenceDenial> {
        let left_digest = WorthUiArtifactDigestor::digest(left, basis);
        let right_digest = WorthUiArtifactDigestor::digest(right, basis);
        let mut metrics = WorthUiArtifactEquivalenceMetrics::default();
        let differences = all_differences(left, right, structural_entry_limit, &mut metrics)?;

        Ok(WorthUiArtifactEquivalence::new(
            basis,
            left_digest,
            right_digest,
            differences,
            metrics,
        ))
    }
}

fn all_differences(
    left: &WorthUiArtifact,
    right: &WorthUiArtifact,
    structural_entry_limit: usize,
    metrics: &mut WorthUiArtifactEquivalenceMetrics,
) -> Result<Vec<WorthUiArtifactDifference>, WorthUiArtifactEquivalenceDenial> {
    let mut differences = Vec::new();
    if left.module_ids().len() != right.module_ids().len() {
        differences.push(WorthUiArtifactDifference::ModuleCount {
            left_module_count: left.module_ids().len(),
            right_module_count: right.module_ids().len(),
        });
    }

    for (module_index, (left_module_id, right_module_id)) in left
        .module_ids()
        .iter()
        .zip(right.module_ids().iter())
        .enumerate()
    {
        if left_module_id != right_module_id {
            differences.push(WorthUiArtifactDifference::ModuleOrder {
                module_index,
                left_module_id: left_module_id.as_str().to_owned(),
                right_module_id: right_module_id.as_str().to_owned(),
            });
        }
    }

    let module_ids = left
        .module_ids()
        .iter()
        .chain(right.module_ids().iter())
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    for module_id in module_ids {
        metrics.record_module_compared();
        enforce_structural_limit(*metrics, structural_entry_limit)?;
        match (left.module(&module_id), right.module(&module_id)) {
            (None, Some(_)) => differences.push(WorthUiArtifactDifference::ModuleCreated {
                module_id: module_id.as_str().to_owned(),
            }),
            (Some(_), None) => differences.push(WorthUiArtifactDifference::ModuleRetired {
                module_id: module_id.as_str().to_owned(),
            }),
            (Some(left_module), Some(right_module))
                if left_module.nodes().len() != right_module.nodes().len() =>
            {
                differences.push(WorthUiArtifactDifference::ModuleNodeCount {
                    module_id: module_id.as_str().to_owned(),
                    left_node_count: left_module.nodes().len(),
                    right_node_count: right_module.nodes().len(),
                });
            }
            _ => {}
        }
    }

    let left_nodes = nodes_by_identity(left);
    let right_nodes = nodes_by_identity(right);
    let identities = left_nodes
        .keys()
        .chain(right_nodes.keys())
        .cloned()
        .collect::<std::collections::BTreeSet<_>>();
    for identity in identities {
        metrics.record_node_compared();
        enforce_structural_limit(*metrics, structural_entry_limit)?;
        match (left_nodes.get(&identity), right_nodes.get(&identity)) {
            (None, Some(right_node)) => {
                differences.push(WorthUiArtifactDifference::NodeCreated {
                    node_identity: identity,
                    candidate_authored_provenance_digest: right_node.authored_provenance_digest(),
                    module_id: right_node.handle().module_id().as_str().to_owned(),
                    node_index: right_node.handle().node_index(),
                    node_kind: right_node.handle().kind(),
                });
            }
            (Some(left_node), None) => {
                differences.push(WorthUiArtifactDifference::NodeRetired {
                    node_identity: identity,
                    active_authored_provenance_digest: left_node.authored_provenance_digest(),
                    module_id: left_node.handle().module_id().as_str().to_owned(),
                    node_index: left_node.handle().node_index(),
                    node_kind: left_node.handle().kind(),
                });
            }
            (Some(left_node), Some(right_node)) => {
                compare_matched_node(identity, left_node, right_node, metrics, &mut differences)
            }
            (None, None) => unreachable!("identity union contains one artifact node"),
        }
    }

    Ok(differences)
}

fn nodes_by_identity(
    artifact: &WorthUiArtifact,
) -> std::collections::BTreeMap<String, &WorthUiArtifactNode> {
    artifact
        .identity_handles()
        .filter_map(|(identity, _)| {
            artifact
                .node_for_identity_basis(identity)
                .map(|node| (identity.to_owned(), node))
        })
        .collect()
}

fn compare_matched_node(
    identity: String,
    left_node: &WorthUiArtifactNode,
    right_node: &WorthUiArtifactNode,
    metrics: &mut WorthUiArtifactEquivalenceMetrics,
    differences: &mut Vec<WorthUiArtifactDifference>,
) {
    let left_handle = left_node.handle();
    let right_handle = right_node.handle();
    if left_handle.module_id() != right_handle.module_id()
        || left_handle.node_index() != right_handle.node_index()
    {
        differences.push(WorthUiArtifactDifference::NodeMoved {
            node_identity: identity.clone(),
            active_authored_provenance_digest: left_node.authored_provenance_digest(),
            candidate_authored_provenance_digest: right_node.authored_provenance_digest(),
            left_module_id: left_handle.module_id().as_str().to_owned(),
            left_node_index: left_handle.node_index(),
            right_module_id: right_handle.module_id().as_str().to_owned(),
            right_node_index: right_handle.node_index(),
        });
    }

    let left_kind = left_handle.kind();
    let right_kind = right_handle.kind();
    if left_kind != right_kind {
        differences.push(WorthUiArtifactDifference::NodeKind {
            node_identity: identity,
            active_authored_provenance_digest: left_node.authored_provenance_digest(),
            candidate_authored_provenance_digest: right_node.authored_provenance_digest(),
            module_id: right_handle.module_id().as_str().to_owned(),
            node_index: right_handle.node_index(),
            left_kind,
            right_kind,
        });
        return;
    }

    metrics.record_semantic_payload_compared();
    let left_basis = node_semantic_basis(left_node);
    let right_basis = node_semantic_basis(right_node);
    if left_basis != right_basis {
        differences.push(WorthUiArtifactDifference::NodeSemantics {
            node_identity: identity,
            active_authored_provenance_digest: left_node.authored_provenance_digest(),
            candidate_authored_provenance_digest: right_node.authored_provenance_digest(),
            module_id: right_handle.module_id().as_str().to_owned(),
            node_index: right_handle.node_index(),
            node_kind: left_kind,
            semantic_delta: semantic_delta(left_node, right_node),
            left_semantic_basis: left_basis,
            right_semantic_basis: right_basis,
        });
    }
}

fn enforce_structural_limit(
    metrics: WorthUiArtifactEquivalenceMetrics,
    structural_entry_limit: usize,
) -> Result<(), WorthUiArtifactEquivalenceDenial> {
    let observed = metrics.structural_entries_compared();
    if observed > structural_entry_limit {
        Err(
            WorthUiArtifactEquivalenceDenial::structural_capacity_exceeded(
                structural_entry_limit,
                observed,
            ),
        )
    } else {
        Ok(())
    }
}

fn semantic_delta(
    left_node: &WorthUiArtifactNode,
    right_node: &WorthUiArtifactNode,
) -> WorthUiArtifactSemanticDelta {
    match (left_node, right_node) {
        (WorthUiArtifactNode::Surface(left), WorthUiArtifactNode::Surface(right)) => {
            let placement_changed =
                left.descriptor().placement_class() != right.descriptor().placement_class();
            let command_slots_changed =
                left.descriptor().command_slots() != right.descriptor().command_slots();

            if placement_changed && command_slots_changed {
                WorthUiArtifactSemanticDelta::SurfacePlacementAndCommandSlotsChanged
            } else if placement_changed {
                WorthUiArtifactSemanticDelta::SurfacePlacementClassChanged
            } else if command_slots_changed {
                WorthUiArtifactSemanticDelta::SurfaceCommandSlotsChanged
            } else {
                WorthUiArtifactSemanticDelta::Other
            }
        }
        _ => WorthUiArtifactSemanticDelta::Other,
    }
}
