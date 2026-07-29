use crate::source::WorthUiArtifactNodeKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiArtifactSemanticDelta {
    SurfaceCommandSlotsChanged,
    SurfacePlacementClassChanged,
    SurfacePlacementAndCommandSlotsChanged,
    Other,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiArtifactDifference {
    ModuleCreated {
        module_id: String,
    },
    ModuleRetired {
        module_id: String,
    },
    ModuleCount {
        left_module_count: usize,
        right_module_count: usize,
    },
    ModuleOrder {
        module_index: usize,
        left_module_id: String,
        right_module_id: String,
    },
    ModuleNodeCount {
        module_id: String,
        left_node_count: usize,
        right_node_count: usize,
    },
    NodeCreated {
        node_identity: String,
        candidate_authored_provenance_digest: u64,
        module_id: String,
        node_index: usize,
        node_kind: WorthUiArtifactNodeKind,
    },
    NodeRetired {
        node_identity: String,
        active_authored_provenance_digest: u64,
        module_id: String,
        node_index: usize,
        node_kind: WorthUiArtifactNodeKind,
    },
    NodeMoved {
        node_identity: String,
        active_authored_provenance_digest: u64,
        candidate_authored_provenance_digest: u64,
        left_module_id: String,
        left_node_index: usize,
        right_module_id: String,
        right_node_index: usize,
    },
    NodeKind {
        node_identity: String,
        active_authored_provenance_digest: u64,
        candidate_authored_provenance_digest: u64,
        module_id: String,
        node_index: usize,
        left_kind: WorthUiArtifactNodeKind,
        right_kind: WorthUiArtifactNodeKind,
    },
    NodeSemantics {
        node_identity: String,
        active_authored_provenance_digest: u64,
        candidate_authored_provenance_digest: u64,
        module_id: String,
        node_index: usize,
        node_kind: WorthUiArtifactNodeKind,
        semantic_delta: WorthUiArtifactSemanticDelta,
        left_semantic_basis: String,
        right_semantic_basis: String,
    },
}
