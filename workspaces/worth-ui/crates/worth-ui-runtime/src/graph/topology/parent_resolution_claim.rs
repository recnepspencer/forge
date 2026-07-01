use crate::declaration::stable_text_digest;
use crate::graph::UiGraphNodeIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiGraphParentResolutionClaim {
    RootPage,
    ContainedByRootPage,
}

impl UiGraphParentResolutionClaim {
    pub(crate) fn resolve_parent_node_identity(
        &self,
        root_page_identity: UiGraphNodeIdentity,
    ) -> Option<UiGraphNodeIdentity> {
        match self {
            Self::RootPage => None,
            Self::ContainedByRootPage => Some(root_page_identity),
        }
    }

    pub(crate) fn resolve_page_membership(
        &self,
        node_identity: UiGraphNodeIdentity,
        root_page_identity: UiGraphNodeIdentity,
    ) -> UiGraphNodeIdentity {
        match self {
            Self::RootPage => node_identity,
            Self::ContainedByRootPage => root_page_identity,
        }
    }

    pub(crate) fn identity_digest(&self) -> u64 {
        match self {
            Self::RootPage => stable_text_digest("graph-topology:parent-resolution:root-page"),
            Self::ContainedByRootPage => {
                stable_text_digest("graph-topology:parent-resolution:contained-by-root-page")
            }
        }
    }
}
