use worth_store_contracts::{QueueProducerKind, QueueProducerResourceShape};
use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

use super::BufferPoolQueueDeclarationContext;
use crate::{
    BufferPoolQueueGroupingScope, PhysicalResidencyDenial, PhysicalResidencyIncarnation,
    PhysicalWritebackClaim, PhysicalWritebackRangePosture,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BufferPoolQueueWriteDurability {
    BufferedWrite,
    FileDataSynchronization,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BufferPoolWritebackQueueExecutionDeclaration {
    store: StableStoreIdentity,
    pool: PhysicalResidencyIncarnation,
    frame: RecordFrameCoordinate,
    grouping_scope: BufferPoolQueueGroupingScope,
    resource_shape: QueueProducerResourceShape,
    flush_epoch: u64,
    durability: BufferPoolQueueWriteDurability,
    range_posture: PhysicalWritebackRangePosture,
}

impl BufferPoolWritebackQueueExecutionDeclaration {
    pub fn for_claim(
        claim: &PhysicalWritebackClaim,
        context: BufferPoolQueueDeclarationContext,
        durability: BufferPoolQueueWriteDurability,
    ) -> Result<Self, PhysicalResidencyDenial> {
        let [frame] = claim.frames() else {
            return Err(PhysicalResidencyDenial::WriteBackExceedsDirtyPosture);
        };
        let range_posture = claim
            .range_posture(0)
            .ok_or(PhysicalResidencyDenial::WriteBackReceiptMismatch)?;
        Ok(Self {
            store: claim.store_identity(),
            pool: claim.pool_incarnation(),
            frame: frame.coordinate(),
            grouping_scope: context.grouping_scope(),
            resource_shape: context.resource_shape(),
            flush_epoch: context.flush_epoch(),
            durability,
            range_posture,
        })
    }

    pub const fn producer_kind(self) -> QueueProducerKind {
        QueueProducerKind::BufferPoolWriteBack
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

    pub const fn durability(self) -> BufferPoolQueueWriteDurability {
        self.durability
    }

    pub const fn range_posture(self) -> PhysicalWritebackRangePosture {
        self.range_posture
    }
}
