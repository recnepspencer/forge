#![forbid(unsafe_code)]

mod s6_cold_tier_posture;
mod s7_placement_io_readiness;

pub use s6_cold_tier_posture::{S6ColdTierIoPosture, S6ColdTierIoPostureDenial};
pub use s7_placement_io_readiness::{
    admit_s7_placement_io_readiness_seed, S7PlacementIoReadinessSeed,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TierResidenceClass {
    Hot,
    Warm,
    Cold,
}
