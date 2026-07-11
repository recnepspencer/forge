use forge_store_contracts::{QueueProducerKind, QueueProducerResourceShape};
use forge_store_security::{StoreAuthenticityRequirement, StoreKeyScope, StoreTenantScope};

use crate::WalSecurityMetadataCarrier;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WalQueueExecutionKind {
    CommitRecord,
    CheckpointRecord,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WalQueueGroupingScope {
    tenant_scope: StoreTenantScope,
    key_scope: StoreKeyScope,
    authenticity_requirement: StoreAuthenticityRequirement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WalQueueExecutionDeclaration {
    kind: WalQueueExecutionKind,
    flush_epoch: u64,
    resource_shape: QueueProducerResourceShape,
    grouping_scope: WalQueueGroupingScope,
}

impl WalQueueGroupingScope {
    pub const fn from_security_metadata(security_metadata: WalSecurityMetadataCarrier) -> Self {
        let metadata = security_metadata.physical_metadata();
        Self {
            tenant_scope: metadata.tenant_scope(),
            key_scope: metadata.key_scope(),
            authenticity_requirement: metadata.authenticity_requirement(),
        }
    }

    pub const fn tenant_scope(self) -> StoreTenantScope {
        self.tenant_scope
    }

    pub const fn key_scope(self) -> StoreKeyScope {
        self.key_scope
    }

    pub const fn authenticity_requirement(self) -> StoreAuthenticityRequirement {
        self.authenticity_requirement
    }
}

impl WalQueueExecutionDeclaration {
    pub const fn commit_record(
        flush_epoch: u64,
        resource_shape: QueueProducerResourceShape,
        grouping_scope: WalQueueGroupingScope,
    ) -> Self {
        Self {
            kind: WalQueueExecutionKind::CommitRecord,
            flush_epoch,
            resource_shape,
            grouping_scope,
        }
    }

    pub const fn checkpoint_record(
        flush_epoch: u64,
        resource_shape: QueueProducerResourceShape,
        grouping_scope: WalQueueGroupingScope,
    ) -> Self {
        Self {
            kind: WalQueueExecutionKind::CheckpointRecord,
            flush_epoch,
            resource_shape,
            grouping_scope,
        }
    }

    pub const fn kind(self) -> WalQueueExecutionKind {
        self.kind
    }

    pub const fn flush_epoch(self) -> u64 {
        self.flush_epoch
    }

    pub const fn producer_kind(self) -> QueueProducerKind {
        match self.kind {
            WalQueueExecutionKind::CommitRecord => QueueProducerKind::WalCommitRecord,
            WalQueueExecutionKind::CheckpointRecord => QueueProducerKind::WalCheckpointRecord,
        }
    }

    pub const fn resource_shape(self) -> QueueProducerResourceShape {
        self.resource_shape
    }

    pub const fn grouping_scope(self) -> WalQueueGroupingScope {
        self.grouping_scope
    }
}
