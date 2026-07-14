use crate::placement::movement::{
    classification::MovementEligibilityCase,
    types::{BlobPlacementMovementFreshness, BlobPlacementMovementRequest},
};

pub(crate) fn require_current_freshness(
    request: &BlobPlacementMovementRequest,
) -> Option<MovementEligibilityCase> {
    if request.freshness() == BlobPlacementMovementFreshness::Current {
        None
    } else {
        Some(MovementEligibilityCase::Stale)
    }
}
