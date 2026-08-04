use crate::data::resource::lifecycle::{
    ResourceLifecycleClass, ResourceLifecycleOrdinal, ResourceOutputContinuity,
};
use crate::data::resource::request::ResourceNodeId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceLifecycleSummary {
    node: ResourceNodeId,
    lifecycle: ResourceLifecycleClass,
    output_continuity: ResourceOutputContinuity,
    lifecycle_ordinal: ResourceLifecycleOrdinal,
}

impl ResourceLifecycleSummary {
    pub(crate) fn new(
        node: ResourceNodeId,
        lifecycle: ResourceLifecycleClass,
        output_continuity: ResourceOutputContinuity,
        lifecycle_ordinal: ResourceLifecycleOrdinal,
    ) -> Self {
        Self {
            node,
            lifecycle,
            output_continuity,
            lifecycle_ordinal,
        }
    }

    pub fn node(self) -> ResourceNodeId {
        self.node
    }

    pub fn lifecycle(self) -> ResourceLifecycleClass {
        self.lifecycle
    }

    pub fn output_continuity(self) -> ResourceOutputContinuity {
        self.output_continuity
    }

    pub fn lifecycle_ordinal(self) -> ResourceLifecycleOrdinal {
        self.lifecycle_ordinal
    }
}
