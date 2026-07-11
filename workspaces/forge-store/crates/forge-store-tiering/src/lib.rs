#![forbid(unsafe_code)]

pub mod layout_access;

mod cold_state;
mod cold_tier_posture;
mod io_readiness;

pub use cold_state::{
    classify_cold_posture_permit, cold_posture_permits_compaction, cold_posture_permits_movement,
    ColdPosturePermit, S7ColdPlacementState,
};
#[cfg(feature = "certification-test-authority")]
pub use cold_tier_posture::certification_test_support;
pub use cold_tier_posture::{ColdTierIoPosture, ColdTierIoPostureDenial};
pub use io_readiness::{admit_s7_placement_io_readiness_seed, S7PlacementIoReadinessSeed};
pub use layout_access::{
    ColdRecallAccessBudget, ColdRecallInterferencePosture, ColdRecallLayoutReport,
    RecallAmplificationAccessBudget, RecallAmplificationInterferencePosture,
    RecallAmplificationLayoutReport, TierPlacementAccessBudget, TierPlacementInterferencePosture,
    TierPlacementLayoutReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TierResidenceClass {
    Hot,
    Warm,
    Cold,
}
