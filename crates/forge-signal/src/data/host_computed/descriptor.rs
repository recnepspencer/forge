use serde::{Deserialize, Serialize};

use crate::data::handle::NodeId;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct HostComputedDescriptorId(u64);

impl HostComputedDescriptorId {
    pub(crate) fn new(value: u64) -> Self {
        Self(value)
    }

    pub fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum HostComputedApiFamily {
    #[default]
    CorePreparedEvaluation,
    EasyClosure,
    OpaqueHostAdapter,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostComputedDescriptor {
    descriptor_id: HostComputedDescriptorId,
    node: NodeId,
    api_family: HostComputedApiFamily,
}

impl HostComputedDescriptor {
    pub(crate) fn new(
        descriptor_id: HostComputedDescriptorId,
        node: NodeId,
        api_family: HostComputedApiFamily,
    ) -> Self {
        Self {
            descriptor_id,
            node,
            api_family,
        }
    }

    pub(crate) fn for_node(node: NodeId, api_family: HostComputedApiFamily) -> Self {
        let descriptor_id =
            HostComputedDescriptorId::new(((node.index() as u64) << 32) | node.generation() as u64);
        Self::new(descriptor_id, node, api_family)
    }

    pub fn descriptor_id(&self) -> HostComputedDescriptorId {
        self.descriptor_id
    }

    pub fn node(&self) -> NodeId {
        self.node
    }

    pub fn api_family(&self) -> HostComputedApiFamily {
        self.api_family
    }
}
