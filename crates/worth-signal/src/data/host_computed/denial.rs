use serde::{Deserialize, Serialize};

use crate::data::dependency::DependencyEdge;
use crate::data::handle::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HostComputedDenialClass {
    SelfRead,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeniedHostComputedReadSet {
    node: NodeId,
    class: HostComputedDenialClass,
    dependency: DependencyEdge,
}

impl DeniedHostComputedReadSet {
    pub(crate) fn new(
        node: NodeId,
        class: HostComputedDenialClass,
        dependency: DependencyEdge,
    ) -> Self {
        Self {
            node,
            class,
            dependency,
        }
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn class(&self) -> HostComputedDenialClass {
        self.class
    }

    pub fn dependency(&self) -> &DependencyEdge {
        &self.dependency
    }
}
