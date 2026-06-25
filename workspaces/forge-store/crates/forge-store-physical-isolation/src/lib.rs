#![forbid(unsafe_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalEpoch(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StableReadPlan {
    root_epoch: PhysicalEpoch,
}

impl StableReadPlan {
    pub const fn new(root_epoch: PhysicalEpoch) -> Self {
        Self { root_epoch }
    }
}
