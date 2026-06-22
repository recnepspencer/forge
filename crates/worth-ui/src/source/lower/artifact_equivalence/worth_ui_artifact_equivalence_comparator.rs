use crate::source::{
    WorthUiArtifact, WorthUiArtifactDifference, WorthUiArtifactDigestor,
    WorthUiArtifactEquivalence, WorthUiArtifactEquivalenceBasis, WorthUiArtifactEquivalenceMetrics,
    WorthUiArtifactNode, WorthUiArtifactSemanticDelta,
};

use super::worth_ui_artifact_semantic_basis::node_semantic_basis;

#[derive(Clone, Debug, Default)]
pub(crate) struct WorthUiArtifactEquivalenceComparator;

impl WorthUiArtifactEquivalenceComparator {
    pub(crate) fn compare(
        left: &WorthUiArtifact,
        right: &WorthUiArtifact,
        basis: WorthUiArtifactEquivalenceBasis,
    ) -> WorthUiArtifactEquivalence {
        let left_digest = WorthUiArtifactDigestor::digest(left, basis);
        let right_digest = WorthUiArtifactDigestor::digest(right, basis);
        let mut metrics = WorthUiArtifactEquivalenceMetrics::default();
        let first_difference = first_difference(left, right, &mut metrics);

        WorthUiArtifactEquivalence::new(basis, left_digest, right_digest, first_difference, metrics)
    }
}

fn first_difference(
    left: &WorthUiArtifact,
    right: &WorthUiArtifact,
    metrics: &mut WorthUiArtifactEquivalenceMetrics,
) -> Option<WorthUiArtifactDifference> {
    if left.module_ids().len() != right.module_ids().len() {
        return Some(WorthUiArtifactDifference::ModuleCountMismatch {
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
            return Some(WorthUiArtifactDifference::ModuleOrderMismatch {
                module_index,
                left_module_id: left_module_id.as_str().to_owned(),
                right_module_id: right_module_id.as_str().to_owned(),
            });
        }

        metrics.record_module_compared();
        let left_module = left.module(left_module_id).expect("left artifact module");
        let right_module = right
            .module(right_module_id)
            .expect("right artifact module");
        if left_module.nodes().len() != right_module.nodes().len() {
            return Some(WorthUiArtifactDifference::ModuleNodeCountMismatch {
                module_id: left_module_id.as_str().to_owned(),
                left_node_count: left_module.nodes().len(),
                right_node_count: right_module.nodes().len(),
            });
        }

        for (node_index, (left_node, right_node)) in left_module
            .nodes()
            .iter()
            .zip(right_module.nodes().iter())
            .enumerate()
        {
            metrics.record_node_compared();
            let left_kind = left_node.handle().kind();
            let right_kind = right_node.handle().kind();
            if left_kind != right_kind {
                return Some(WorthUiArtifactDifference::NodeKindMismatch {
                    module_id: left_module_id.as_str().to_owned(),
                    node_index,
                    left_kind,
                    right_kind,
                });
            }

            metrics.record_semantic_payload_compared();
            let left_basis = node_semantic_basis(left_node);
            let right_basis = node_semantic_basis(right_node);
            if left_basis != right_basis {
                return Some(WorthUiArtifactDifference::NodeSemanticMismatch {
                    module_id: left_module_id.as_str().to_owned(),
                    node_index,
                    node_kind: left_kind,
                    semantic_delta: semantic_delta(left_node, right_node),
                    left_semantic_basis: left_basis,
                    right_semantic_basis: right_basis,
                });
            }
        }
    }

    None
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
