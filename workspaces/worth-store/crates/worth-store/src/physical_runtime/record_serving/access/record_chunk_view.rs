use std::ops::Range;

use worth_store_physical_format::{
    store_namespace::StableStoreIdentity, PhysicalGenerationOwner, RecordExtentGenerationCell,
    RecordFrameCoordinate, SlotGenerationCell,
};

use crate::physical_runtime::LifecycleGeneration;

use super::super::PhysicalRecordId;

#[derive(Clone, Copy)]
pub(in crate::physical_runtime::record_serving::access) struct RecordReadIdentity {
    store: StableStoreIdentity,
    generation: LifecycleGeneration,
    record: PhysicalRecordId,
    physical_owner: PhysicalGenerationOwner,
}

impl RecordReadIdentity {
    pub(in crate::physical_runtime::record_serving::access) const fn for_inline(
        store: StableStoreIdentity,
        generation: LifecycleGeneration,
        record: PhysicalRecordId,
        slot: SlotGenerationCell,
    ) -> Self {
        Self {
            store,
            generation,
            record,
            physical_owner: slot.owner(),
        }
    }

    pub(in crate::physical_runtime::record_serving::access) const fn for_extent(
        store: StableStoreIdentity,
        generation: LifecycleGeneration,
        record: PhysicalRecordId,
        extent: RecordExtentGenerationCell,
    ) -> Self {
        Self {
            store,
            generation,
            record,
            physical_owner: extent.owner(),
        }
    }

    const fn chunk_basis(self, frame: RecordFrameCoordinate) -> PhysicalRecordChunkBasis {
        PhysicalRecordChunkBasis::from_read_identity(self, frame)
    }

    pub(in crate::physical_runtime::record_serving::access) fn chunk_view<'session>(
        self,
        bytes: &'session [u8],
        frame: RecordFrameCoordinate,
        logical_range: Range<u64>,
    ) -> PhysicalRecordChunkView<'session> {
        PhysicalRecordChunkView::new(bytes, self.chunk_basis(frame), logical_range)
    }

    pub(in crate::physical_runtime::record_serving::access) fn pressure_evidence(
        self,
        failure: super::super::PhysicalRecordResidencyFailure,
        frame: RecordFrameCoordinate,
    ) -> Option<super::super::PhysicalRecordPressureEvidence> {
        let basis = super::super::PhysicalRecordPressureBasis::for_store(self.store)
            .with_record(self.record)
            .with_frame_coordinate(frame);
        super::super::PhysicalRecordPressureEvidence::from_failure(failure, self.generation, basis)
    }
}

/// Physical identity carried by a borrowed record chunk.
///
/// The basis identifies the Store, serving generation, record, durable owner,
/// and exact frame coordinate. It is observation for successor validation, not
/// pool control or proof of semantic residency, integrity, or durability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalRecordChunkBasis {
    store: StableStoreIdentity,
    generation: LifecycleGeneration,
    record: PhysicalRecordId,
    physical_owner: PhysicalGenerationOwner,
    frame: RecordFrameCoordinate,
}

impl PhysicalRecordChunkBasis {
    const fn from_read_identity(
        identity: RecordReadIdentity,
        frame: RecordFrameCoordinate,
    ) -> Self {
        Self {
            store: identity.store,
            generation: identity.generation,
            record: identity.record,
            physical_owner: identity.physical_owner,
            frame,
        }
    }

    /// Returns the stable physical Store identity.
    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store
    }

    /// Returns the serving lifecycle generation that minted the chunk.
    pub const fn store_generation(self) -> LifecycleGeneration {
        self.generation
    }

    /// Returns the physical record identity.
    pub const fn record(self) -> PhysicalRecordId {
        self.record
    }

    /// Returns the durable generation owner of the record bytes.
    pub const fn physical_owner(self) -> PhysicalGenerationOwner {
        self.physical_owner
    }

    /// Returns the exact resident frame coordinate.
    pub const fn frame_coordinate(self) -> RecordFrameCoordinate {
        self.frame
    }
}

/// A lease-scoped borrowed view of one decoded record payload range.
///
/// The view owns no bytes. Its lifetime is tied to the mutable borrow of the
/// originating `RecordReadSession`, which prevents advancing or dropping the
/// session while the view remains live.
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

    /// Returns decoded payload bytes, excluding physical framing metadata.
    pub const fn bytes(&self) -> &'session [u8] {
        self.bytes
    }

    /// Returns the physical identity and generation context for these bytes.
    pub const fn basis(&self) -> PhysicalRecordChunkBasis {
        self.basis
    }

    /// Returns the byte range represented inside the logical record.
    pub fn logical_range(&self) -> Range<u64> {
        self.logical_range.clone()
    }
}
