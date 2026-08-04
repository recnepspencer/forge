use worth_store_physical_backend::{
    ArtifactTreeFailure, MediaOperationIdentity, MediaOperationRole,
};

use crate::physical_runtime::work::PhysicalCheckpointRecoveryAction;

pub struct CompletedPhysicalCheckpointAction {
    action: PhysicalCheckpointRecoveryAction,
    operation: MediaOperationIdentity,
    role: MediaOperationRole,
    completed_bytes: u64,
}

pub(in crate::physical_runtime) struct IndeterminatePhysicalCheckpointAction {
    action: PhysicalCheckpointRecoveryAction,
    operation: MediaOperationIdentity,
    role: MediaOperationRole,
    completed_bytes: u64,
    failure: ArtifactTreeFailure,
}

impl CompletedPhysicalCheckpointAction {
    pub(in crate::physical_runtime) const fn new(
        action: PhysicalCheckpointRecoveryAction,
        operation: MediaOperationIdentity,
        role: MediaOperationRole,
        completed_bytes: u64,
    ) -> Self {
        Self {
            action,
            operation,
            role,
            completed_bytes,
        }
    }

    pub const fn action(&self) -> PhysicalCheckpointRecoveryAction {
        self.action
    }

    pub const fn operation(&self) -> MediaOperationIdentity {
        self.operation
    }

    pub const fn role(&self) -> MediaOperationRole {
        self.role
    }

    pub const fn completed_bytes(&self) -> u64 {
        self.completed_bytes
    }
}

impl IndeterminatePhysicalCheckpointAction {
    pub(in crate::physical_runtime) const fn new(
        action: PhysicalCheckpointRecoveryAction,
        operation: MediaOperationIdentity,
        role: MediaOperationRole,
        completed_bytes: u64,
        failure: ArtifactTreeFailure,
    ) -> Self {
        Self {
            action,
            operation,
            role,
            completed_bytes,
            failure,
        }
    }

    pub(in crate::physical_runtime) const fn action(&self) -> PhysicalCheckpointRecoveryAction {
        self.action
    }

    pub(in crate::physical_runtime) const fn operation(&self) -> MediaOperationIdentity {
        self.operation
    }

    pub(in crate::physical_runtime) const fn role(&self) -> MediaOperationRole {
        self.role
    }

    pub(in crate::physical_runtime) const fn completed_bytes(&self) -> u64 {
        self.completed_bytes
    }

    pub(in crate::physical_runtime) const fn failure(&self) -> ArtifactTreeFailure {
        self.failure
    }
}
