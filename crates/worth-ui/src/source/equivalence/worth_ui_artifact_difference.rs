use crate::source::WorthUiArtifactNodeKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiArtifactDifference {
    ModuleCountMismatch {
        left_module_count: usize,
        right_module_count: usize,
    },
    ModuleOrderMismatch {
        module_index: usize,
        left_module_id: String,
        right_module_id: String,
    },
    ModuleNodeCountMismatch {
        module_id: String,
        left_node_count: usize,
        right_node_count: usize,
    },
    NodeKindMismatch {
        module_id: String,
        node_index: usize,
        left_kind: WorthUiArtifactNodeKind,
        right_kind: WorthUiArtifactNodeKind,
    },
    NodeSemanticMismatch {
        module_id: String,
        node_index: usize,
        node_kind: WorthUiArtifactNodeKind,
        left_semantic_basis: String,
        right_semantic_basis: String,
    },
}
