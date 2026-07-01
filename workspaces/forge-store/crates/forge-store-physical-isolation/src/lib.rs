#![forbid(unsafe_code)]

mod s5_harness_readiness;

pub use s5_harness_readiness::{
    s5_simulation_harness_readiness_requirement, S5SimulationHarnessReadinessRequirement,
};

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
