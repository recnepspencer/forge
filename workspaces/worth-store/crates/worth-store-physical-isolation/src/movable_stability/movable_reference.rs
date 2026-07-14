use worth_store_physical_format::{
    PhysicalCellReuseDomain, PhysicalFutureChunkReference, PhysicalGeneration,
};

use super::{PhysicalChunkStabilityPlaceholder, TierMovementStabilityDenial};
use crate::{
    ChunkEpoch, CurrentGenerationPhysicalReference, ExtentEpoch,
    GenerationCountedPhysicalReference, PageEpoch, PhysicalReadReachabilityBarrier, SegmentEpoch,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovablePhysicalRefKind {
    Page,
    Extent,
    Segment,
    FutureChunk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MovablePhysicalRef {
    Page {
        reference: CurrentGenerationPhysicalReference,
        epoch: PageEpoch,
        reachability: PhysicalReadReachabilityBarrier,
    },
    Extent {
        reference: CurrentGenerationPhysicalReference,
        epoch: ExtentEpoch,
        reachability: PhysicalReadReachabilityBarrier,
    },
    Segment {
        reference: CurrentGenerationPhysicalReference,
        epoch: SegmentEpoch,
        reachability: PhysicalReadReachabilityBarrier,
    },
    #[non_exhaustive]
    FutureChunk {
        reference: PhysicalFutureChunkReference,
        epoch: ChunkEpoch,
        reachability: PhysicalReadReachabilityBarrier,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TierMovementAdmissionLabel {
    fingerprint: u64,
}

impl MovablePhysicalRef {
    pub fn page(
        reference: CurrentGenerationPhysicalReference,
        epoch: PageEpoch,
        reachability: PhysicalReadReachabilityBarrier,
    ) -> Result<Self, TierMovementStabilityDenial> {
        if reference.owner().domain() != PhysicalCellReuseDomain::SlotAllocation {
            return Err(TierMovementStabilityDenial::WrongMovableReferenceKind);
        }
        Ok(Self::Page {
            reference,
            epoch,
            reachability,
        })
    }

    pub fn extent(
        reference: CurrentGenerationPhysicalReference,
        epoch: ExtentEpoch,
        reachability: PhysicalReadReachabilityBarrier,
    ) -> Result<Self, TierMovementStabilityDenial> {
        if reference.owner().domain() != PhysicalCellReuseDomain::ExtentAllocation {
            return Err(TierMovementStabilityDenial::WrongMovableReferenceKind);
        }
        Ok(Self::Extent {
            reference,
            epoch,
            reachability,
        })
    }

    pub fn segment(
        reference: CurrentGenerationPhysicalReference,
        epoch: SegmentEpoch,
        reachability: PhysicalReadReachabilityBarrier,
    ) -> Result<Self, TierMovementStabilityDenial> {
        if reference.owner().domain() != PhysicalCellReuseDomain::Segment {
            return Err(TierMovementStabilityDenial::WrongMovableReferenceKind);
        }
        Ok(Self::Segment {
            reference,
            epoch,
            reachability,
        })
    }

    pub fn extent_from_generation_counted(
        reference: GenerationCountedPhysicalReference,
        observed_generation: PhysicalGeneration,
        epoch: ExtentEpoch,
        reachability: PhysicalReadReachabilityBarrier,
    ) -> Result<Self, TierMovementStabilityDenial> {
        let current = reference
            .require_current_generation(observed_generation)
            .map_err(TierMovementStabilityDenial::StaleGeneration)?;
        Self::extent(current, epoch, reachability)
    }

    pub fn future_chunk_from_placeholder(placeholder: PhysicalChunkStabilityPlaceholder) -> Self {
        Self::future_chunk(
            placeholder.reference(),
            placeholder.epoch(),
            placeholder.reachability(),
        )
    }

    pub(crate) const fn future_chunk(
        reference: PhysicalFutureChunkReference,
        epoch: ChunkEpoch,
        reachability: PhysicalReadReachabilityBarrier,
    ) -> Self {
        Self::FutureChunk {
            reference,
            epoch,
            reachability,
        }
    }

    pub const fn kind(self) -> MovablePhysicalRefKind {
        match self {
            Self::Page { .. } => MovablePhysicalRefKind::Page,
            Self::Extent { .. } => MovablePhysicalRefKind::Extent,
            Self::Segment { .. } => MovablePhysicalRefKind::Segment,
            Self::FutureChunk { .. } => MovablePhysicalRefKind::FutureChunk,
        }
    }

    pub const fn reachability(self) -> PhysicalReadReachabilityBarrier {
        match self {
            Self::Page { reachability, .. }
            | Self::Extent { reachability, .. }
            | Self::Segment { reachability, .. }
            | Self::FutureChunk { reachability, .. } => reachability,
        }
    }

    pub(crate) fn stability_fingerprint(self) -> u64 {
        let mut digest = 0xcbf29ce484222325_u64;
        mix_u64(&mut digest, self.kind() as u64);
        match self {
            Self::Page {
                reference, epoch, ..
            } => mix_generation_reference(&mut digest, reference, epoch.get()),
            Self::Extent {
                reference, epoch, ..
            } => mix_generation_reference(&mut digest, reference, epoch.get()),
            Self::Segment {
                reference, epoch, ..
            } => mix_generation_reference(&mut digest, reference, epoch.get()),
            Self::FutureChunk {
                reference, epoch, ..
            } => {
                mix_u64(&mut digest, reference.chunk_id().get());
                mix_u64(&mut digest, reference.generation().get());
                mix_u64(&mut digest, epoch.get());
            }
        }
        let reachability = self.reachability().footprint_basis();
        mix_u64(&mut digest, reachability.protected_references());
        mix_u64(&mut digest, reachability.protected_ranges());
        mix_u64(&mut digest, reachability.canonical_digest());
        digest
    }
}

impl TierMovementAdmissionLabel {
    pub(crate) fn for_movable_reference(reference: MovablePhysicalRef) -> Self {
        Self {
            fingerprint: reference.stability_fingerprint(),
        }
    }

    pub fn copied_from(reference: MovablePhysicalRef) -> Self {
        Self::for_movable_reference(reference)
    }

    pub(crate) fn matches(self, reference: MovablePhysicalRef) -> bool {
        self.fingerprint == reference.stability_fingerprint()
    }
}

fn mix_generation_reference(
    digest: &mut u64,
    reference: CurrentGenerationPhysicalReference,
    epoch: u64,
) {
    let owner = reference.owner();
    mix_u64(digest, owner.domain() as u64);
    mix_optional_u64(digest, owner.segment_id().map(|id| id.get()));
    mix_optional_u64(digest, owner.page_id().map(|id| id.get()));
    mix_optional_u64(digest, owner.extent_id().map(|id| id.get()));
    mix_u64(digest, owner.generation().get());
    mix_u64(digest, epoch);
}

fn mix_optional_u64(digest: &mut u64, value: Option<u64>) {
    mix_u64(digest, value.unwrap_or(u64::MAX));
}

fn mix_u64(digest: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *digest ^= u64::from(byte);
        *digest = digest.wrapping_mul(0x100000001b3);
    }
}
