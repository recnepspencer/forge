use crate::data::resource::descriptor::ResourceDescriptorId;
use crate::data::resource::lifecycle::ResourceLifecycleTransition;
use serde::{Deserialize, Serialize};

use super::super::performance::ResourceBoundaryPerformanceEnvelope;
use super::lifecycle::ResourceLifecycleSummary;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceDeclarationReport {
    descriptor_id: ResourceDescriptorId,
    lifecycle: ResourceLifecycleSummary,
    transition: ResourceLifecycleTransition,
    performance: ResourceBoundaryPerformanceEnvelope,
}

impl ResourceDeclarationReport {
    pub(crate) fn new(
        descriptor_id: ResourceDescriptorId,
        lifecycle: ResourceLifecycleSummary,
        transition: ResourceLifecycleTransition,
        performance: ResourceBoundaryPerformanceEnvelope,
    ) -> Self {
        Self {
            descriptor_id,
            lifecycle,
            transition,
            performance,
        }
    }

    pub fn descriptor_id(self) -> ResourceDescriptorId {
        self.descriptor_id
    }

    pub fn lifecycle(self) -> ResourceLifecycleSummary {
        self.lifecycle
    }

    pub fn transition(self) -> ResourceLifecycleTransition {
        self.transition
    }

    pub fn performance(self) -> ResourceBoundaryPerformanceEnvelope {
        self.performance
    }
}
