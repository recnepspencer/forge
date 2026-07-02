#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HazardLeaseSlot(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HazardLeaseGeneration(u64);

impl HazardLeaseSlot {
    pub(crate) const fn from_index(index: usize) -> Self {
        Self(index as u32)
    }

    pub const fn get(self) -> u32 {
        self.0
    }
}

impl HazardLeaseGeneration {
    pub(crate) const fn initial() -> Self {
        Self(1)
    }

    pub(crate) const fn next(self) -> Self {
        Self(self.0 + 1)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}
