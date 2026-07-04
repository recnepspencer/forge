use crate::facade::WorthUiApp;
use crate::graph::{UiGraphAspectEvidenceIndexes, UiGraphNodeEvidenceIndex, UiGraphSnapshot};

impl WorthUiApp {
    pub(crate) fn build_graph_node_evidence_index(
        declaration_artifacts: &[crate::declaration::UiDeclarationArtifact],
        graph_snapshot: &UiGraphSnapshot,
        lifecycle: &crate::facade::runtime_bridge::WorthUiFacadeLifecycleBootstrap,
    ) -> UiGraphNodeEvidenceIndex {
        let graph_node_evidence_index =
            UiGraphNodeEvidenceIndex::rebuild(declaration_artifacts, graph_snapshot);
        lifecycle.record_graph_node_evidence_index_rebuild();
        graph_node_evidence_index
    }

    pub(crate) fn build_graph_aspect_evidence_indexes(
        graph_snapshot: &UiGraphSnapshot,
        graph_node_evidence_index: &UiGraphNodeEvidenceIndex,
        lifecycle: &crate::facade::runtime_bridge::WorthUiFacadeLifecycleBootstrap,
    ) -> UiGraphAspectEvidenceIndexes {
        let graph_aspect_evidence_indexes =
            UiGraphAspectEvidenceIndexes::rebuild(graph_snapshot, graph_node_evidence_index);
        lifecycle.record_graph_aspect_evidence_index_rebuild();
        graph_aspect_evidence_indexes
    }
}
