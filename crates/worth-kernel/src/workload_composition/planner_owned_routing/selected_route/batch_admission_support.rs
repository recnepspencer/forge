use super::super::{
    BatchAdmissionPlannerRoutePacket, PlannerOwnedRoutingError, PlannerOwnedRoutingErrorKind,
};
use crate::workload_composition::BatchAdmissionExecutionReceipt;

pub(super) fn require_matching_batch_admission_route_packet(
    route_packet: &BatchAdmissionPlannerRoutePacket,
    receipt: &BatchAdmissionExecutionReceipt,
) -> Result<(), PlannerOwnedRoutingError> {
    if route_packet.selected_batch_plan_digest() != receipt.selected_batch_plan_digest() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet batch-admission route packet does not match the current selected batch plan",
        ));
    }
    if route_packet.batch_execution_receipt_digest() != receipt.execution_receipt_digest() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet batch-admission route packet does not match the current batch execution receipt",
        ));
    }
    let receipt_row_digests = receipt
        .selected_family_rows()
        .iter()
        .map(|row| {
            format!(
                "{}:{}:{}",
                row.identity().as_str(),
                row.posture().as_str(),
                row.declaration_digest()
            )
        })
        .collect::<Vec<_>>();
    if route_packet.selected_family_row_digests() != receipt_row_digests {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "selected-route packet batch-admission route packet does not match the current selected family rows",
        ));
    }
    Ok(())
}
