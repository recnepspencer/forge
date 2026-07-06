use crate::declaration::stable_text_digest;
use crate::evidence::UiAllocationNeighborhood;
use crate::graph::UiGraphNodeIdentity;

use super::UiGraphTouchDescriptor;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiGraphTouchAllocationNeighborhood {
    touch_identity_digest: u64,
    graph_node_identity: UiGraphNodeIdentity,
    neighborhood: UiAllocationNeighborhood,
    identity_digest: u64,
}

impl UiGraphTouchAllocationNeighborhood {
    pub(crate) fn from_touch(
        touch: &UiGraphTouchDescriptor,
        neighborhood: &UiAllocationNeighborhood,
    ) -> Option<Self> {
        if touch.target().graph_node_identity() != neighborhood.root_graph_node_identity()
            || touch.world().world_profile().identity_digest()
                != neighborhood.world_identity_digest()
        {
            return None;
        }

        Some(Self::new(
            touch.identity_digest(),
            neighborhood.root_graph_node_identity(),
            neighborhood.clone(),
        ))
    }

    pub(crate) fn new(
        touch_identity_digest: u64,
        graph_node_identity: UiGraphNodeIdentity,
        neighborhood: UiAllocationNeighborhood,
    ) -> Self {
        let identity_digest = stable_text_digest("touch-allocation-neighborhood")
            ^ touch_identity_digest.rotate_left(7)
            ^ graph_node_identity.digest().rotate_left(13)
            ^ neighborhood.identity().identity_digest().rotate_left(19);

        Self {
            touch_identity_digest,
            graph_node_identity,
            neighborhood,
            identity_digest,
        }
    }

    pub fn touch_identity_digest(&self) -> u64 {
        self.touch_identity_digest
    }

    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn neighborhood(&self) -> &UiAllocationNeighborhood {
        &self.neighborhood
    }

    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
}
