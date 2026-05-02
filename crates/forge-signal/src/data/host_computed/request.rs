use serde::{Deserialize, Serialize};

use crate::data::dependency::DependencyEdge;
use crate::data::handle::NodeId;

use super::descriptor::HostComputedDescriptor;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostComputedEvaluationRequest {
    descriptor: HostComputedDescriptor,
    previous_dependencies: Vec<DependencyEdge>,
}

impl HostComputedEvaluationRequest {
    pub(crate) fn new(
        descriptor: HostComputedDescriptor,
        previous_dependencies: &[DependencyEdge],
    ) -> Self {
        Self {
            descriptor,
            previous_dependencies: previous_dependencies.to_vec(),
        }
    }

    pub fn descriptor(&self) -> &HostComputedDescriptor {
        &self.descriptor
    }

    pub fn node(&self) -> NodeId {
        self.descriptor.node()
    }

    pub fn previous_dependencies(&self) -> &[DependencyEdge] {
        &self.previous_dependencies
    }

    pub fn previous_dependency_count(&self) -> usize {
        self.previous_dependencies.len()
    }
}
