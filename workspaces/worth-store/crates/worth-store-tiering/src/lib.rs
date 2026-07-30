#![forbid(unsafe_code)]

pub mod layout_projection;

mod cold_state;
mod cold_tier_posture;

pub use cold_state::{
    classify_cold_posture_permit, cold_posture_permits_compaction, cold_posture_permits_movement,
    ColdPlacementState, ColdPosturePermit,
};
#[cfg(feature = "certification-test-authority")]
pub use cold_tier_posture::certification_test_support;
pub use cold_tier_posture::{ColdTierIoPosture, ColdTierIoPostureDenial};
pub use layout_projection::{
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
