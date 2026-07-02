use forge_store_physical_format::PhysicalFutureChunkReference;

use super::{TierMovementStabilityCounterSnapshot, TierMovementStabilityDenial};
use crate::{ChunkEpoch, PhysicalReadReachabilityBarrier};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FutureChunkStabilityBasis {
    reference: PhysicalFutureChunkReference,
    epoch: ChunkEpoch,
    reachability: PhysicalReadReachabilityBarrier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalChunkStabilityPlaceholder {
    reference: PhysicalFutureChunkReference,
    epoch: ChunkEpoch,
    basis: FutureChunkStabilityBasis,
    counters: TierMovementStabilityCounterSnapshot,
}

impl FutureChunkStabilityBasis {
    pub const fn from_stability_receipt(
        reference: PhysicalFutureChunkReference,
        epoch: ChunkEpoch,
        reachability: PhysicalReadReachabilityBarrier,
    ) -> Self {
        Self {
            reference,
            epoch,
            reachability,
        }
    }

    pub const fn reference(self) -> PhysicalFutureChunkReference {
        self.reference
    }

    pub const fn epoch(self) -> ChunkEpoch {
        self.epoch
    }

    pub const fn reachability(self) -> PhysicalReadReachabilityBarrier {
        self.reachability
    }
}

impl PhysicalChunkStabilityPlaceholder {
    pub fn admit(
        reference: PhysicalFutureChunkReference,
        epoch: Option<ChunkEpoch>,
        basis: FutureChunkStabilityBasis,
    ) -> Result<Self, TierMovementStabilityDenial> {
        let epoch = epoch.ok_or(TierMovementStabilityDenial::MissingChunkEpoch)?;
        if basis.reference() != reference || basis.epoch() != epoch {
            return Err(TierMovementStabilityDenial::PlaceholderBasisMismatch);
        }
        Ok(Self {
            reference,
            epoch,
            basis,
            counters: TierMovementStabilityCounterSnapshot::default().with_chunk_placeholder(),
        })
    }

    pub fn admit_with_epoch(
        reference: PhysicalFutureChunkReference,
        epoch: ChunkEpoch,
        basis: FutureChunkStabilityBasis,
    ) -> Result<Self, TierMovementStabilityDenial> {
        Self::admit(reference, Some(epoch), basis)
    }

    pub const fn reference(self) -> PhysicalFutureChunkReference {
        self.reference
    }

    pub const fn epoch(self) -> ChunkEpoch {
        self.epoch
    }

    pub const fn basis(self) -> FutureChunkStabilityBasis {
        self.basis
    }

    pub const fn reachability(self) -> PhysicalReadReachabilityBarrier {
        self.basis.reachability()
    }

    pub const fn counters(self) -> TierMovementStabilityCounterSnapshot {
        self.counters
    }
}
