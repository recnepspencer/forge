#![forbid(unsafe_code)]

mod cold_state;
mod cold_tier_posture;
mod io_readiness;

pub use cold_state::{
    classify_cold_posture_permit, cold_posture_permits_compaction, cold_posture_permits_movement,
    ColdPosturePermit, S7ColdPlacementState,
};
pub use cold_tier_posture::{S6ColdTierIoPosture, S6ColdTierIoPostureDenial};
pub use io_readiness::{admit_s7_placement_io_readiness_seed, S7PlacementIoReadinessSeed};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TierResidenceClass {
    Hot,
    Warm,
    Cold,
}
