use crate::workload_composition::planner_owned_routing::{
    PlannerOwnedRoutingError, PlannerOwnedRoutingErrorKind,
};
use crate::workload_composition::worth_workload::current_worth_workload_ordinary_consumer_cutover;
use crate::workload_composition::BatchAdmissionExecutionReceipt;

use super::admitted_input::admit_batch_admission_planner_route_input;
use super::packet::{lower_batch_admission_planner_route_packet, BatchAdmissionPlannerRoutePacket};

pub(crate) fn current_worth_touched_graph_conflict_batch_admission_route_packet(
) -> Result<BatchAdmissionPlannerRoutePacket, PlannerOwnedRoutingError> {
    current_worth_touched_graph_conflict_batch_admission_route_packet_with_receipt_loader(
        |receipt| receipt,
    )
}

fn current_worth_touched_graph_conflict_batch_admission_route_packet_with_receipt_loader(
    override_receipt: impl FnOnce(BatchAdmissionExecutionReceipt) -> BatchAdmissionExecutionReceipt,
) -> Result<BatchAdmissionPlannerRoutePacket, PlannerOwnedRoutingError> {
    let receipt = current_worth_workload_ordinary_consumer_cutover()
        .map_err(current_route_error)?
        .batch_execution_receipt()
        .clone();
    let admitted = admit_batch_admission_planner_route_input(override_receipt(receipt))?;
    Ok(lower_batch_admission_planner_route_packet(admitted))
}

#[cfg(test)]
pub(crate) fn current_worth_touched_graph_conflict_batch_admission_route_packet_with_receipt_override(
    override_receipt: impl FnOnce(BatchAdmissionExecutionReceipt) -> BatchAdmissionExecutionReceipt,
) -> Result<BatchAdmissionPlannerRoutePacket, PlannerOwnedRoutingError> {
    current_worth_touched_graph_conflict_batch_admission_route_packet_with_receipt_loader(
        override_receipt,
    )
}

fn current_route_error<E: std::fmt::Debug>(error: E) -> PlannerOwnedRoutingError {
    PlannerOwnedRoutingError::new(
        PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
        format!("planner-owned batch-admission route did not assemble: {error:?}"),
    )
}
