use worth_store_physical_format::store_namespace::StableStoreIdentity;

use super::{OperationAllocationGrant, PhysicalFrameKey, PhysicalResidencyIncarnation};

/// Monotonic identity assigned when dirty bytes become resident.
///
/// This is buffer-pool source truth only. It is not checkpoint publication,
/// durability, retention, or recovery authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalDirtyGeneration(u64);

impl PhysicalDirtyGeneration {
    pub(crate) const GENESIS: Self = Self(0);

    pub(crate) const fn successor(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    pub const fn get(self) -> u64 {
        self.0
    }

    #[cfg(test)]
    pub(crate) const fn for_test(value: u64) -> Self {
        Self(value)
    }
}

/// One resident dirty frame fixed at a dirty-generation capture boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalDirtyFrameBasis {
    frame: PhysicalFrameKey,
    generation: PhysicalDirtyGeneration,
}

impl PhysicalDirtyFrameBasis {
    pub(crate) const fn new(frame: PhysicalFrameKey, generation: PhysicalDirtyGeneration) -> Self {
        Self { frame, generation }
    }

    pub const fn frame(self) -> PhysicalFrameKey {
        self.frame
    }

    pub const fn generation(self) -> PhysicalDirtyGeneration {
        self.generation
    }
}

/// Move-owned cursor over the dirty source fixed at one capture boundary.
///
/// Slots are visited once. A passed slot cannot later acquire an in-range
/// generation, while an in-range dirty frame cannot move or be evicted before
/// durable writeback makes its captured version unnecessary.
#[derive(Debug)]
pub struct PhysicalDirtyGenerationCaptureSession {
    store: StableStoreIdentity,
    pool: PhysicalResidencyIncarnation,
    frontier: PhysicalDirtyGeneration,
    next_slot: usize,
    slot_limit: usize,
}

impl PhysicalDirtyGenerationCaptureSession {
    pub(crate) const fn new(
        store: StableStoreIdentity,
        pool: PhysicalResidencyIncarnation,
        frontier: PhysicalDirtyGeneration,
        slot_limit: usize,
    ) -> Self {
        Self {
            store,
            pool,
            frontier,
            next_slot: 0,
            slot_limit,
        }
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.store
    }

    pub const fn pool_incarnation(&self) -> PhysicalResidencyIncarnation {
        self.pool
    }

    pub const fn frontier(&self) -> PhysicalDirtyGeneration {
        self.frontier
    }

    pub(crate) const fn next_slot(&self) -> usize {
        self.next_slot
    }

    pub(crate) const fn slot_limit(&self) -> usize {
        self.slot_limit
    }
}

/// Allocation-backed metadata emitted by one bounded capture advance.
///
/// The allocation remains charged until this slice is dropped.
#[derive(Debug)]
pub struct PhysicalDirtyGenerationSlice {
    frames: Vec<PhysicalDirtyFrameBasis>,
    metadata_bytes: u64,
    allocation: OperationAllocationGrant,
}

impl PhysicalDirtyGenerationSlice {
    pub(crate) const fn new(
        frames: Vec<PhysicalDirtyFrameBasis>,
        metadata_bytes: u64,
        allocation: OperationAllocationGrant,
    ) -> Self {
        Self {
            frames,
            metadata_bytes,
            allocation,
        }
    }

    pub fn frames(&self) -> &[PhysicalDirtyFrameBasis] {
        &self.frames
    }

    pub const fn metadata_bytes(&self) -> u64 {
        self.metadata_bytes
    }

    pub const fn admitted_bytes(&self) -> u64 {
        self.allocation.bytes()
    }
}

/// Proof that every fixed frame-table slot was inspected for one frontier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalDirtyGenerationCaptureCompletion {
    store: StableStoreIdentity,
    pool: PhysicalResidencyIncarnation,
    frontier: PhysicalDirtyGeneration,
}

impl PhysicalDirtyGenerationCaptureCompletion {
    pub(crate) const fn new(session: &PhysicalDirtyGenerationCaptureSession) -> Self {
        Self {
            store: session.store,
            pool: session.pool,
            frontier: session.frontier,
        }
    }

    pub const fn store_identity(self) -> StableStoreIdentity {
        self.store
    }

    pub const fn pool_incarnation(self) -> PhysicalResidencyIncarnation {
        self.pool
    }

    pub const fn frontier(self) -> PhysicalDirtyGeneration {
        self.frontier
    }
}

/// Compile-time capture progression: an unfinished session or proven scan.
#[derive(Debug)]
pub enum PhysicalDirtyGenerationCaptureStep {
    More {
        session: PhysicalDirtyGenerationCaptureSession,
        slice: PhysicalDirtyGenerationSlice,
    },
    Complete {
        completion: PhysicalDirtyGenerationCaptureCompletion,
        slice: PhysicalDirtyGenerationSlice,
    },
}

impl PhysicalDirtyGenerationCaptureStep {
    pub(crate) fn from_advance(
        mut session: PhysicalDirtyGenerationCaptureSession,
        next_slot: usize,
        slice: PhysicalDirtyGenerationSlice,
    ) -> Self {
        session.next_slot = next_slot;
        if next_slot == session.slot_limit {
            Self::Complete {
                completion: PhysicalDirtyGenerationCaptureCompletion::new(&session),
                slice,
            }
        } else {
            Self::More { session, slice }
        }
    }
}

pub(crate) const fn dirty_frame_basis_bytes() -> usize {
    std::mem::size_of::<PhysicalDirtyFrameBasis>()
}
