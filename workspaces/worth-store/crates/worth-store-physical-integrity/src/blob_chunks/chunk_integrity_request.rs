use crate::{
    ChunkIntegrityCounters, ChunkIntegrityDenial, ChunkIntegrityDenialKind,
    ChunkIntegrityStreamingWindowDenial, ScopedPhysicalValidatorInput,
};
use std::num::NonZeroU64;
use worth_store::physical_runtime::BlobPhysicalAllocation;
use worth_store_physical_format::PhysicalScopeFamily;

#[derive(Debug, Clone, Copy)]
pub struct ChunkIntegrityStreamingWindow<'allocation, 'runtime> {
    allocation: &'allocation BlobPhysicalAllocation<'runtime>,
    object_bytes: u64,
    window_bytes: u64,
}

impl<'allocation, 'runtime> ChunkIntegrityStreamingWindow<'allocation, 'runtime> {
    pub fn admit(
        allocation: &'allocation BlobPhysicalAllocation<'runtime>,
        object_bytes: u64,
        window_bytes: NonZeroU64,
    ) -> Result<Self, ChunkIntegrityStreamingWindowDenial> {
        if window_bytes.get() > allocation.bytes() {
            return Err(
                ChunkIntegrityStreamingWindowDenial::WindowExceedsBlobAllocation {
                    requested: window_bytes.get(),
                    allocation: allocation.bytes(),
                },
            );
        }
        Self::bounded(allocation, object_bytes, window_bytes.get())
    }

    fn bounded(
        allocation: &'allocation BlobPhysicalAllocation<'runtime>,
        object_bytes: u64,
        window_bytes: u64,
    ) -> Result<Self, ChunkIntegrityStreamingWindowDenial> {
        if window_bytes >= object_bytes {
            return Err(ChunkIntegrityStreamingWindowDenial::WholeObjectWindow);
        }
        Ok(Self {
            allocation,
            object_bytes,
            window_bytes,
        })
    }

    pub const fn allocation(&self) -> &'allocation BlobPhysicalAllocation<'runtime> {
        self.allocation
    }

    pub const fn object_bytes(self) -> u64 {
        self.object_bytes
    }

    pub const fn window_bytes(self) -> u64 {
        self.window_bytes
    }
}

#[derive(Debug)]
pub struct ChunkIntegrityInspectionRequest<'allocation, 'runtime, 'lease> {
    input: ScopedPhysicalValidatorInput<'lease>,
    streaming_window: ChunkIntegrityStreamingWindow<'allocation, 'runtime>,
}

impl<'allocation, 'runtime, 'lease> ChunkIntegrityInspectionRequest<'allocation, 'runtime, 'lease> {
    pub fn from_store_blob_window(
        input: ScopedPhysicalValidatorInput<'lease>,
        streaming_window: ChunkIntegrityStreamingWindow<'allocation, 'runtime>,
    ) -> Result<Self, ChunkIntegrityDenial> {
        reject_non_chunk_family(&input)?;
        reject_mismatched_store_authority(&input, streaming_window.allocation())?;
        Ok(Self {
            input,
            streaming_window,
        })
    }

    pub(crate) const fn input(&self) -> &ScopedPhysicalValidatorInput<'lease> {
        &self.input
    }

    pub(crate) const fn streaming_window(
        &self,
    ) -> ChunkIntegrityStreamingWindow<'allocation, 'runtime> {
        self.streaming_window
    }
}

fn reject_mismatched_store_authority(
    input: &ScopedPhysicalValidatorInput<'_>,
    allocation: &BlobPhysicalAllocation<'_>,
) -> Result<(), ChunkIntegrityDenial> {
    let chunk = input.admission().store_chunk_basis();
    if chunk.store_identity() != allocation.store_identity() {
        return Err(ChunkIntegrityDenial::new(
            ChunkIntegrityDenialKind::BlobAllocationStoreMismatch,
            ChunkIntegrityCounters::start(1, 1, 0),
        )
        .with_basis(input.admission().basis().clone()));
    }
    if chunk.store_generation() != allocation.store_generation() {
        return Err(ChunkIntegrityDenial::new(
            ChunkIntegrityDenialKind::BlobAllocationGenerationMismatch,
            ChunkIntegrityCounters::start(1, 1, 0),
        )
        .with_basis(input.admission().basis().clone()));
    }
    Ok(())
}

fn reject_non_chunk_family(
    input: &ScopedPhysicalValidatorInput<'_>,
) -> Result<(), ChunkIntegrityDenial> {
    if input.family() == PhysicalScopeFamily::ChunkLike {
        return Ok(());
    }
    Err(ChunkIntegrityDenial::new(
        ChunkIntegrityDenialKind::WrongPhysicalFamily,
        ChunkIntegrityCounters::start(1, 1, 0),
    )
    .with_basis(input.admission().basis().clone()))
}
