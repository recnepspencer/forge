use worth_store_contracts::{QueueProducerKind, QueueProducerResourceShape};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};
use worth_store_security::{
    StoreAuthenticityRequirement, StoreKeyScope, StoreSecurityScopeIdentity, StoreTenantScope,
};

use crate::{
    PhysicalFrameKey, PhysicalResidencyDenial, PhysicalResidencyIncarnation, PhysicalResidencyPool,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BufferPoolQueueExecutionKind {
    ReadAhead,
    WriteBack,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BufferPoolQueueGroupingScope {
    security_scope_identity: StoreSecurityScopeIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BufferPoolQueueExecutionDeclaration {
    kind: BufferPoolQueueExecutionKind,
    store: StableStoreIdentity,
    pool: PhysicalResidencyIncarnation,
    frame: RecordFrameCoordinate,
    grouping_scope: BufferPoolQueueGroupingScope,
    resource_shape: QueueProducerResourceShape,
    flush_epoch: u64,
}

impl BufferPoolQueueGroupingScope {
    pub const fn new(security_scope_identity: StoreSecurityScopeIdentity) -> Self {
        Self {
            security_scope_identity,
        }
    }

    pub const fn security_scope_identity(self) -> StoreSecurityScopeIdentity {
        self.security_scope_identity
    }

    pub const fn tenant_scope(self) -> StoreTenantScope {
        self.security_scope_identity.tenant_scope()
    }

    pub const fn key_scope(self) -> StoreKeyScope {
        self.security_scope_identity.key_scope()
    }

    pub const fn authenticity_requirement(self) -> StoreAuthenticityRequirement {
        self.security_scope_identity.authenticity_requirement()
    }
}

impl BufferPoolQueueExecutionDeclaration {
    pub fn read_ahead(
        pool: &PhysicalResidencyPool,
        frame: PhysicalFrameKey,
        grouping_scope: BufferPoolQueueGroupingScope,
        flush_epoch: u64,
        resource_shape: QueueProducerResourceShape,
    ) -> Result<Self, PhysicalResidencyDenial> {
        let (store, incarnation, coordinate) = pool.bind_queue_frame(frame)?;
        Ok(Self {
            kind: BufferPoolQueueExecutionKind::ReadAhead,
            store,
            pool: incarnation,
            frame: coordinate,
            grouping_scope,
            resource_shape,
            flush_epoch,
        })
    }

    pub fn write_back(
        pool: &PhysicalResidencyPool,
        frame: PhysicalFrameKey,
        grouping_scope: BufferPoolQueueGroupingScope,
        flush_epoch: u64,
        resource_shape: QueueProducerResourceShape,
    ) -> Result<Self, PhysicalResidencyDenial> {
        let (store, incarnation, coordinate) = pool.bind_queue_frame(frame)?;
        Ok(Self {
            kind: BufferPoolQueueExecutionKind::WriteBack,
            store,
            pool: incarnation,
            frame: coordinate,
            grouping_scope,
            resource_shape,
            flush_epoch,
        })
    }

    pub const fn kind(self) -> BufferPoolQueueExecutionKind {
        self.kind
    }

    pub const fn producer_kind(self) -> QueueProducerKind {
        match self.kind {
            BufferPoolQueueExecutionKind::ReadAhead => QueueProducerKind::BufferPoolReadAhead,
            BufferPoolQueueExecutionKind::WriteBack => QueueProducerKind::BufferPoolWriteBack,
        }
    }

    pub const fn store(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn pool(self) -> PhysicalResidencyIncarnation {
        self.pool
    }

    pub const fn frame(self) -> RecordFrameCoordinate {
        self.frame
    }

    pub const fn grouping_scope(self) -> BufferPoolQueueGroupingScope {
        self.grouping_scope
    }

    pub const fn resource_shape(self) -> QueueProducerResourceShape {
        self.resource_shape
    }

    pub const fn flush_epoch(self) -> u64 {
        self.flush_epoch
    }
}
