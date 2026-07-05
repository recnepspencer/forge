use forge_store_contracts::{S6QueueProducerKind, S6QueueProducerResourceShape};
use forge_store_security::{StoreAuthenticityRequirement, StoreKeyScope, StoreTenantScope};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BufferPoolQueueExecutionKind {
    ReadAhead,
    WriteBack,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BufferPoolQueueGroupingScope {
    tenant_scope: StoreTenantScope,
    key_scope: StoreKeyScope,
    authenticity_requirement: StoreAuthenticityRequirement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BufferPoolQueueExecutionDeclaration {
    kind: BufferPoolQueueExecutionKind,
    resource_shape: S6QueueProducerResourceShape,
    flush_epoch: u64,
}

impl BufferPoolQueueGroupingScope {
    pub const fn new(
        tenant_scope: StoreTenantScope,
        key_scope: StoreKeyScope,
        authenticity_requirement: StoreAuthenticityRequirement,
    ) -> Self {
        Self {
            tenant_scope,
            key_scope,
            authenticity_requirement,
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

impl BufferPoolQueueExecutionDeclaration {
    pub const fn read_ahead(
        flush_epoch: u64,
        resource_shape: S6QueueProducerResourceShape,
    ) -> Self {
        Self {
            kind: BufferPoolQueueExecutionKind::ReadAhead,
            resource_shape,
            flush_epoch,
        }
    }

    pub const fn write_back(
        flush_epoch: u64,
        resource_shape: S6QueueProducerResourceShape,
    ) -> Self {
        Self {
            kind: BufferPoolQueueExecutionKind::WriteBack,
            resource_shape,
            flush_epoch,
        }
    }

    pub const fn kind(self) -> BufferPoolQueueExecutionKind {
        self.kind
    }

    pub const fn producer_kind(self) -> S6QueueProducerKind {
        match self.kind {
            BufferPoolQueueExecutionKind::ReadAhead => S6QueueProducerKind::BufferPoolReadAhead,
            BufferPoolQueueExecutionKind::WriteBack => S6QueueProducerKind::BufferPoolWriteBack,
        }
    }

    pub const fn resource_shape(self) -> S6QueueProducerResourceShape {
        self.resource_shape
    }

    pub const fn flush_epoch(self) -> u64 {
        self.flush_epoch
    }
}
