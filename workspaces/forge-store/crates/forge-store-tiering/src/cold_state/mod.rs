mod cold_posture_classifier;
mod s7_cold_placement_state;

pub use cold_posture_classifier::{classify_cold_posture_permit, ColdPosturePermit};
pub use s7_cold_placement_state::ColdPlacementState;

pub const fn cold_posture_permits_movement(state: ColdPlacementState) -> bool {
    matches!(
        classify_cold_posture_permit(state),
        ColdPosturePermit::Movement
    )
}

pub const fn cold_posture_permits_compaction(state: ColdPlacementState) -> bool {
    cold_posture_permits_movement(state)
}
