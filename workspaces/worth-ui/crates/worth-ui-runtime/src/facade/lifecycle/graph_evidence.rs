use crate::declaration::UiDeclarationArtifact;
use crate::graph::{
    UiGraphAspectEvidenceIndexes, UiGraphNodeEvidenceIndex, UiGraphSnapshot,
};

use super::bootstrap::WorthUiFacadeLifecycleBootstrap;

pub(crate) struct GraphEvidenceIndexes {
    pub node: UiGraphNodeEvidenceIndex,
    pub aspect: UiGraphAspectEvidenceIndexes,
}

pub(crate) fn build_graph_evidence_indexes(
    declaration_artifacts: &[UiDeclarationArtifact],
    graph_snapshot: &UiGraphSnapshot,
    lifecycle: &WorthUiFacadeLifecycleBootstrap,
) -> GraphEvidenceIndexes {
    let node = UiGraphNodeEvidenceIndex::rebuild(declaration_artifacts, graph_snapshot);
    lifecycle.record_graph_node_evidence_index_rebuild();
    let aspect = UiGraphAspectEvidenceIndexes::rebuild(graph_snapshot, &node);
    lifecycle.record_graph_aspect_evidence_index_rebuild();
    GraphEvidenceIndexes { node, aspect }
}