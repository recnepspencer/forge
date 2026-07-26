use std::ops::Range;

use worth_store_physical_format::{store_namespace::StableStoreIdentity, RecordFrameCoordinate};

use crate::physical_runtime::LifecycleGeneration;

use super::super::PhysicalRecordId;

#[derive(Clone, Copy)]
pub(in crate::physical_runtime::record_serving::access) struct RecordReadIdentity {
    store: StableStoreIdentity,
    generation: LifecycleGeneration,
    record: PhysicalRecordId,
}

impl RecordReadIdentity {
    pub(in crate::physical_runtime::record_serving::access) const fn new(
        store: StableStoreIdentity,
        generation: LifecycleGeneration,
        record: PhysicalRecordId,
    ) -> Self {
        Self {
            store,
            generation,
            record,
        }
    }

    const fn chunk_basis(self, frame: RecordFrameCoordinate) -> PhysicalRecordChunkBasis {
        PhysicalRecordChunkBasis::new(self.store, self.generation, self.record, frame)
    }

    pub(in crate::physical_runtime::record_serving::access) fn chunk_view<'session>(
        self,
        bytes: &'session [u8],
        frame: RecordFrameCoordinate,
        logical_range: Range<u64>,
    ) -> PhysicalRecordChunkView<'session> {
        PhysicalRecordChunkView::new(bytes, self.chunk_basis(frame), logical_range)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalRecordChunkBasis {
    store: StableStoreIdentity,
    generation: LifecycleGeneration,
    record: PhysicalRecordId,
    frame: RecordFrameCoordinate,
}

impl PhysicalRecordChunkBasis {
    const fn new(
        store: StableStoreIdentity,
        generation: LifecycleGeneration,
        record: PhysicalRecordId,
        frame: RecordFrameCoordinate,
    ) -> Self {
        Self {
            store,
            generation,
            record,
            frame,
        }
    }

    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn store_generation(self) -> LifecycleGeneration {
        self.generation
    }

    pub const fn record(self) -> PhysicalRecordId {
        self.record
    }

    pub const fn frame_coordinate(self) -> RecordFrameCoordinate {
        self.frame
    }
}

pub struct PhysicalRecordChunkView<'session> {
    bytes: &'session [u8],
    basis: PhysicalRecordChunkBasis,
    logical_range: Range<u64>,
}

impl<'session> PhysicalRecordChunkView<'session> {
    const fn new(
        bytes: &'session [u8],
        basis: PhysicalRecordChunkBasis,
        logical_range: Range<u64>,
    ) -> Self {
        Self {
            bytes,
            basis,
            logical_range,
        }
    }

    pub const fn bytes(&self) -> &'session [u8] {
        self.bytes
    }

    pub const fn basis(&self) -> PhysicalRecordChunkBasis {
        self.basis
    }

    pub fn logical_range(&self) -> Range<u64> {
        self.logical_range.clone()
    }
}
