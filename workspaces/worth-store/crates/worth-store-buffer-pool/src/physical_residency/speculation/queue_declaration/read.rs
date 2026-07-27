use worth_store_contracts::{QueueProducerKind, QueueProducerResourceShape};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

use super::BufferPoolQueueDeclarationContext;
use crate::{
    BufferPoolQueueGroupingScope, PhysicalResidencyIncarnation, PrefetchResidencyGrant,
    ReadAheadFrameGrant,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BufferPoolReadQueueExecutionKind {
    Prefetch,
    ReadAhead,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BufferPoolReadQueueExecutionDeclaration {
    kind: BufferPoolReadQueueExecutionKind,
    store: StableStoreIdentity,
    pool: PhysicalResidencyIncarnation,
    frame: RecordFrameCoordinate,
    grouping_scope: BufferPoolQueueGroupingScope,
    resource_shape: QueueProducerResourceShape,
    flush_epoch: u64,
}

impl BufferPoolReadQueueExecutionDeclaration {
    pub fn prefetch(
        grant: &PrefetchResidencyGrant,
        context: BufferPoolQueueDeclarationContext,
    ) -> Self {
        Self::from_grant(grant, context)
    }

    pub fn read_ahead(
        grant: &ReadAheadFrameGrant<'_, '_>,
        context: BufferPoolQueueDeclarationContext,
    ) -> Self {
        Self::from_grant(grant, context)
    }

    fn from_grant<Grant: BufferPoolReadQueueGrant + ?Sized>(
        grant: &Grant,
        context: BufferPoolQueueDeclarationContext,
    ) -> Self {
        Self {
            kind: Grant::KIND,
            store: grant.store_identity(),
            pool: grant.pool_incarnation(),
            frame: grant.coordinate(),
            grouping_scope: context.grouping_scope(),
            resource_shape: context.resource_shape(),
            flush_epoch: context.flush_epoch(),
        }
    }

    pub const fn kind(self) -> BufferPoolReadQueueExecutionKind {
        self.kind
    }

    pub const fn producer_kind(self) -> QueueProducerKind {
        QueueProducerKind::BufferPoolReadAhead
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

trait BufferPoolReadQueueGrant {
    const KIND: BufferPoolReadQueueExecutionKind;

    fn store_identity(&self) -> StableStoreIdentity;
    fn pool_incarnation(&self) -> PhysicalResidencyIncarnation;
    fn coordinate(&self) -> RecordFrameCoordinate;
}

impl BufferPoolReadQueueGrant for PrefetchResidencyGrant {
    const KIND: BufferPoolReadQueueExecutionKind = BufferPoolReadQueueExecutionKind::Prefetch;

    fn store_identity(&self) -> StableStoreIdentity {
        self.store_identity()
    }

    fn pool_incarnation(&self) -> PhysicalResidencyIncarnation {
        self.pool_incarnation()
    }

    fn coordinate(&self) -> RecordFrameCoordinate {
        self.frame().coordinate()
    }
}

impl BufferPoolReadQueueGrant for ReadAheadFrameGrant<'_, '_> {
    const KIND: BufferPoolReadQueueExecutionKind = BufferPoolReadQueueExecutionKind::ReadAhead;

    fn store_identity(&self) -> StableStoreIdentity {
        self.store_identity()
    }

    fn pool_incarnation(&self) -> PhysicalResidencyIncarnation {
        self.pool_incarnation()
    }

    fn coordinate(&self) -> RecordFrameCoordinate {
        self.frame().coordinate()
    }
}
