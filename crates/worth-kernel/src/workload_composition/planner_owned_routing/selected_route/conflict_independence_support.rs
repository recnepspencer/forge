use super::super::{
    ConflictIndependencePlannerRoutePacket, PlannerOwnedRoutingError, PlannerOwnedRoutingErrorKind,
};
use crate::workload_composition::BatchAdmissionExecutionReceipt;

pub(super) fn require_matching_conflict_independence_route_packet(
    route_packet: &ConflictIndependencePlannerRoutePacket,
    receipt: &BatchAdmissionExecutionReceipt,
) -> Result<(), PlannerOwnedRoutingError> {
    if route_packet.overlap_identity_digests() != receipt.overlap_identity_digests() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet conflict/independence route packet does not match the current overlap identity basis",
        ));
    }
    if route_packet.locality_footprint_digests() != receipt.locality_footprint_digests() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet conflict/independence route packet does not match the current locality footprint basis",
        ));
    }
    if route_packet.selected_batch_plan_digest() != receipt.selected_batch_plan_digest() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet conflict/independence route packet does not match the current selected batch plan",
        ));
    }
    if route_packet.batch_execution_receipt_digest() != receipt.execution_receipt_digest() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet conflict/independence route packet does not match the current batch execution receipt",
        ));
    }
    if route_packet.selected_conflict_plan_digests() != receipt.selected_conflict_plan_digests() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet conflict/independence route packet does not match the current selected conflict plans",
        ));
    }
    if route_packet.independence_proof_identities() != receipt.independence_proof_identities() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet conflict/independence route packet does not match the current independence proofs",
        ));
    }
    Ok(())
}
