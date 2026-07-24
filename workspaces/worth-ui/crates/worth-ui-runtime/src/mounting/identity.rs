use crate::graph::{UiGraphNodeIdentity, UiRepeatedInstanceBasis};
use worth_ui_host_contract::{UiMountIncarnation, UiSemanticSurfaceIdentity};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UiMountedGraphWorldIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMountedGraphNodeHandle {
    world_identity: UiMountedGraphWorldIdentity,
    graph_node_identity: UiGraphNodeIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiMountedIdentityBasis {
    graph_node_identity: UiGraphNodeIdentity,
    repeated_instance_basis: UiRepeatedInstanceBasis,
    semantic_surface_identity: UiSemanticSurfaceIdentity,
    mount_incarnation: UiMountIncarnation,
}

impl UiMountedIdentityBasis {
    pub(crate) fn new(
        graph_node_identity: UiGraphNodeIdentity,
        repeated_instance_basis: UiRepeatedInstanceBasis,
        semantic_surface_identity: UiSemanticSurfaceIdentity,
        mount_incarnation: UiMountIncarnation,
    ) -> Self {
        Self {
            graph_node_identity,
            repeated_instance_basis,
            semantic_surface_identity,
            mount_incarnation,
        }
    }

    pub fn graph_node_identity(&self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }

    pub fn repeated_instance_basis(&self) -> &UiRepeatedInstanceBasis {
        &self.repeated_instance_basis
    }

    pub fn semantic_surface_identity(&self) -> UiSemanticSurfaceIdentity {
        self.semantic_surface_identity
    }

    pub fn mount_incarnation(&self) -> UiMountIncarnation {
        self.mount_incarnation
    }
}

impl UiMountedGraphWorldIdentity {
    pub(crate) const fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn diagnostic_value(self) -> u64 {
        self.0
    }
}

impl UiMountedGraphNodeHandle {
    pub(crate) const fn new(
        world_identity: UiMountedGraphWorldIdentity,
        graph_node_identity: UiGraphNodeIdentity,
    ) -> Self {
        Self {
            world_identity,
            graph_node_identity,
        }
    }

    pub fn world_identity(self) -> UiMountedGraphWorldIdentity {
        self.world_identity
    }

    pub fn graph_node_identity(self) -> UiGraphNodeIdentity {
        self.graph_node_identity
    }
}
