use super::super::PhysicalResidencyIncarnation;

/// Opaque identity for one admitted loading transition in one pool incarnation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalFrameLoadingIdentity {
    pool: PhysicalResidencyIncarnation,
    ordinal: u64,
}

impl PhysicalFrameLoadingIdentity {
    pub(crate) const fn new(pool: PhysicalResidencyIncarnation, ordinal: u64) -> Self {
        Self { pool, ordinal }
    }

    pub const fn pool_incarnation(self) -> PhysicalResidencyIncarnation {
        self.pool
    }

    pub const fn ordinal(self) -> u64 {
        self.ordinal
    }
}
