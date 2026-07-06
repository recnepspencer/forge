use crate::placement::movement::{
    classification::MovementEligibilityCase, types::BlobPlacementMovementRequest,
};
use forge_store_tiering::cold_posture_permits_movement;

pub(crate) fn require_cold_permits_movement(
    request: &BlobPlacementMovementRequest,
) -> Option<MovementEligibilityCase> {
    if cold_posture_permits_movement(request.cold_outcome().state()) {
        None
    } else {
        Some(MovementEligibilityCase::ColdUnavailable)
    }
}