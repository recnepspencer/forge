use super::super::{WorthUiCompositionNodeId, WorthUiCompositionRootId};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiCompositionGraphAccessRequest {
    MountedProductTree,
    OrderedChildren { parent_id: String },
    RootChildren,
    ParentOf { node_id: WorthUiCompositionNodeId },
    AncestorsOf { node_id: WorthUiCompositionNodeId },
    ParticipatingDescendants { parent_id: String },
    AffectedConsumersForChangedNode { node_id: WorthUiCompositionNodeId },
    AffectedConsumersForChangedEdge { edge_identity: String },
    AffectedConsumersForChangedPolicy { policy_identity: String },
}

impl WorthUiCompositionGraphAccessRequest {
    pub fn mounted_product_tree() -> Self {
        Self::MountedProductTree
    }

    pub fn ordered_children(parent_id: impl Into<String>) -> Self {
        Self::OrderedChildren {
            parent_id: parent_id.into(),
        }
    }

    pub fn root_children() -> Self {
        Self::RootChildren
    }

    pub fn parent_of(node_id: WorthUiCompositionNodeId) -> Self {
        Self::ParentOf { node_id }
    }

    pub fn ancestors_of(node_id: WorthUiCompositionNodeId) -> Self {
        Self::AncestorsOf { node_id }
    }

    pub fn participating_descendants(parent_id: impl Into<String>) -> Self {
        Self::ParticipatingDescendants {
            parent_id: parent_id.into(),
        }
    }

    pub fn affected_consumers_for_changed_node(node_id: WorthUiCompositionNodeId) -> Self {
        Self::AffectedConsumersForChangedNode { node_id }
    }

    pub fn affected_consumers_for_changed_edge(edge_identity: impl Into<String>) -> Self {
        Self::AffectedConsumersForChangedEdge {
            edge_identity: edge_identity.into(),
        }
    }

    pub fn affected_consumers_for_changed_policy(policy_identity: impl Into<String>) -> Self {
        Self::AffectedConsumersForChangedPolicy {
            policy_identity: policy_identity.into(),
        }
    }

    pub(super) fn token(&self) -> &'static str {
        match self {
            Self::MountedProductTree => "mounted_product_tree",
            Self::OrderedChildren { .. } => "ordered_children",
            Self::RootChildren => "root_children",
            Self::ParentOf { .. } => "parent_of",
            Self::AncestorsOf { .. } => "ancestors_of",
            Self::ParticipatingDescendants { .. } => "participating_descendants",
            Self::AffectedConsumersForChangedNode { .. } => "affected_consumers_for_changed_node",
            Self::AffectedConsumersForChangedEdge { .. } => "affected_consumers_for_changed_edge",
            Self::AffectedConsumersForChangedPolicy { .. } => {
                "affected_consumers_for_changed_policy"
            }
        }
    }

    pub(super) fn identity(&self, root_id: &WorthUiCompositionRootId) -> String {
        match self {
            Self::MountedProductTree => format!("{}:mounted_product_tree", root_id.as_str()),
            Self::OrderedChildren { parent_id } => format!("ordered_children:{parent_id}"),
            Self::RootChildren => format!("root_children:{}", root_id.as_str()),
            Self::ParentOf { node_id } => format!("parent_of:{}", node_id.as_str()),
            Self::AncestorsOf { node_id } => format!("ancestors_of:{}", node_id.as_str()),
            Self::ParticipatingDescendants { parent_id } => {
                format!("participating_descendants:{parent_id}")
            }
            Self::AffectedConsumersForChangedNode { node_id } => {
                format!("affected_consumers_for_changed_node:{}", node_id.as_str())
            }
            Self::AffectedConsumersForChangedEdge { edge_identity } => {
                format!("affected_consumers_for_changed_edge:{edge_identity}")
            }
            Self::AffectedConsumersForChangedPolicy { policy_identity } => {
                format!("affected_consumers_for_changed_policy:{policy_identity}")
            }
        }
    }
}
