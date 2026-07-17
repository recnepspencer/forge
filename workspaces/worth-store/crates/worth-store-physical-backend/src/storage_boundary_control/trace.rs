use crate::ProductionStorageBoundarySeam;

use super::{StorageBoundaryExecutionIdentity, StorageBoundaryFault};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StorageBoundaryTrace {
    execution_identity: Option<StorageBoundaryExecutionIdentity>,
    reached: Vec<ProductionStorageBoundarySeam>,
    injected: Vec<(ProductionStorageBoundarySeam, StorageBoundaryFault)>,
}

impl StorageBoundaryTrace {
    pub(super) fn for_execution(execution_identity: StorageBoundaryExecutionIdentity) -> Self {
        Self {
            execution_identity: Some(execution_identity),
            reached: Vec::new(),
            injected: Vec::new(),
        }
    }

    pub(super) fn record_reached(&mut self, seam: ProductionStorageBoundarySeam) {
        self.reached.push(seam);
    }

    pub(super) fn record_injected(
        &mut self,
        seam: ProductionStorageBoundarySeam,
        fault: StorageBoundaryFault,
    ) {
        self.injected.push((seam, fault));
    }

    pub fn reached(&self) -> &[ProductionStorageBoundarySeam] {
        &self.reached
    }

    pub fn injected(&self) -> &[(ProductionStorageBoundarySeam, StorageBoundaryFault)] {
        &self.injected
    }

    pub const fn execution_identity(&self) -> Option<StorageBoundaryExecutionIdentity> {
        self.execution_identity
    }
}
