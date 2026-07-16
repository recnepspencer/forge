use worth_store_physical_backend::StorageBoundaryExecutionIdentity;

use super::{PhysicalPublicationDenial, ReadCopyUpdateRootPublication};

#[derive(Debug)]
pub struct PhysicalRootPublicationAttempt {
    outcome: Result<ReadCopyUpdateRootPublication, PhysicalPublicationDenial>,
    storage_boundary_execution: Option<StorageBoundaryExecutionIdentity>,
}

impl PhysicalRootPublicationAttempt {
    #[cfg(any(test, feature = "certification-authority"))]
    pub(super) fn from_outcome(
        outcome: Result<ReadCopyUpdateRootPublication, PhysicalPublicationDenial>,
        storage_boundary_execution: Option<StorageBoundaryExecutionIdentity>,
    ) -> Self {
        Self {
            outcome,
            storage_boundary_execution,
        }
    }

    pub const fn publication(&self) -> Option<&ReadCopyUpdateRootPublication> {
        match &self.outcome {
            Ok(publication) => Some(publication),
            Err(_) => None,
        }
    }

    pub const fn denial(&self) -> Option<PhysicalPublicationDenial> {
        match &self.outcome {
            Ok(_) => None,
            Err(denial) => Some(*denial),
        }
    }

    pub const fn storage_boundary_execution_identity(
        &self,
    ) -> Option<StorageBoundaryExecutionIdentity> {
        self.storage_boundary_execution
    }
}
