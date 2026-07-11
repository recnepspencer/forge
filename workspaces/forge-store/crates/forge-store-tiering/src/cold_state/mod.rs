mod cold_posture_classifier;
mod placement;

pub use cold_posture_classifier::{classify_cold_posture_permit, ColdPosturePermit};
pub use placement::ColdPlacementState;

pub const fn cold_posture_permits_movement(state: ColdPlacementState) -> bool {
    matches!(
        classify_cold_posture_permit(state),
        ColdPosturePermit::Movement
    )
}

pub const fn cold_posture_permits_compaction(state: ColdPlacementState) -> bool {
    cold_posture_permits_movement(state)
}
