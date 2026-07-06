use crate::placement::movement::{
    classification::MovementEligibilityCase,
    types::{BlobPlacementMovementForegroundReservation, BlobPlacementMovementRequest},
};

pub(crate) fn require_foreground_reservation_scope(
    request: &BlobPlacementMovementRequest,
) -> Option<MovementEligibilityCase> {
    match request.foreground_reservation() {
        BlobPlacementMovementForegroundReservation::Violated(_) => {
            Some(MovementEligibilityCase::ForegroundViolated)
        }
        BlobPlacementMovementForegroundReservation::Admitted(reservation)
            if reservation.security_scope_identity()
                != request
                    .lifecycle()
                    .declaration()
                    .security_metadata()
                    .identity() =>
        {
            Some(MovementEligibilityCase::ForegroundScopeMismatch)
        }
        BlobPlacementMovementForegroundReservation::Admitted(_) => None,
    }
}