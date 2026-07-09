use crate::{AdmittedBlobPlacement, LifecycleReceipt};

use crate::placement::movement::classification::MovementEligibilityCase;

pub(crate) fn placement_matches_lifecycle(
    placement: &AdmittedBlobPlacement,
    receipt: &LifecycleReceipt,
) -> bool {
    placement.stored_digest() == receipt.reachability().stored_digest()
        && placement.security_metadata() == receipt.reachability().security_metadata()
        && placement.stored_digest() == receipt.placement().stored_digest()
        && placement.security_metadata() == receipt.placement().security_metadata()
}

pub(crate) fn require_source_lifecycle_placement_basis(
    request: &crate::placement::movement::types::BlobPlacementMovementRequest,
) -> Option<MovementEligibilityCase> {
    if placement_matches_lifecycle(request.source(), request.lifecycle()) {
        None
    } else {
        Some(MovementEligibilityCase::LifecycleSourceBasisMismatch)
    }
}

pub(crate) fn require_target_lifecycle_placement_basis(
    request: &crate::placement::movement::types::BlobPlacementMovementRequest,
) -> Option<MovementEligibilityCase> {
    if placement_matches_lifecycle(request.target(), request.lifecycle()) {
        None
    } else {
        Some(MovementEligibilityCase::LifecycleTargetBasisMismatch)
    }
}
