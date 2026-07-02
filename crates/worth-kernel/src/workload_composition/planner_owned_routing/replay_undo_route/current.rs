use crate::replay_undo_consumer_cutover::current_replay_undo_forbidden_surface_denial_ledger;
use crate::replay_undo_inventory::{
    current_replay_undo_inventory_report, ReplayUndoInventoryCategory,
    ReplayUndoInventoryDisposition, ReplayUndoInventorySourceIdentity,
    ReplayUndoInventorySourceKind,
};
use crate::replay_undo_transaction_boundary::{
    admit_replay_undo_transaction_boundary_packet, ReplayUndoTransactionBoundaryInput,
};
use crate::workload_composition::planner_owned_routing::{
    PlannerOwnedRoutingError, PlannerOwnedRoutingErrorKind,
};
use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily;

use super::admitted_input::AdmittedReplayUndoPlannerRouteInput;
use super::packet::{lower_replay_undo_planner_route_packet, ReplayUndoPlannerRoutePacket};
use super::scope_route_product::current_replay_undo_planner_scope_route_product;

pub(crate) fn current_replay_undo_transaction_route_packet(
) -> Result<ReplayUndoPlannerRoutePacket, PlannerOwnedRoutingError> {
    current_replay_undo_route_packet(ReplayUndoPlannerRouteFamily::Transaction, |input| input)
}

#[cfg(test)]
pub(crate) fn current_replay_undo_undo_route_packet(
) -> Result<ReplayUndoPlannerRoutePacket, PlannerOwnedRoutingError> {
    current_replay_undo_route_packet(ReplayUndoPlannerRouteFamily::Undo, |input| input)
}

#[cfg(test)]
pub(crate) fn current_replay_undo_transaction_route_packet_with_input_override(
    override_input: impl FnOnce(
        ReplayUndoTransactionBoundaryInput,
    ) -> ReplayUndoTransactionBoundaryInput,
) -> Result<ReplayUndoPlannerRoutePacket, PlannerOwnedRoutingError> {
    current_replay_undo_route_packet(ReplayUndoPlannerRouteFamily::Transaction, override_input)
}

#[cfg(test)]
pub(crate) fn current_replay_undo_transaction_route_input_for_tests(
) -> Result<ReplayUndoTransactionBoundaryInput, PlannerOwnedRoutingError> {
    Ok(current_replay_undo_planner_scope_route_product()?
        .transaction_boundary_input()
        .clone())
}

fn current_replay_undo_route_packet(
    family: ReplayUndoPlannerRouteFamily,
    override_input: impl FnOnce(
        ReplayUndoTransactionBoundaryInput,
    ) -> ReplayUndoTransactionBoundaryInput,
) -> Result<ReplayUndoPlannerRoutePacket, PlannerOwnedRoutingError> {
    let route_product = current_replay_undo_planner_scope_route_product()?;
    let packet = admit_replay_undo_transaction_boundary_packet(override_input(
        route_product.transaction_boundary_input().clone(),
    ))
    .map_err(current_route_error)?;
    require_packet_matches_current_scope_route_product(&packet, &route_product)?;

    let inventory = current_replay_undo_inventory_report().map_err(current_route_error)?;
    inventory
        .require_full_declared_coverage()
        .map_err(current_route_error)?;
    let source_row = inventory
        .require_source(
            ReplayUndoInventorySourceIdentity::KernelBooleanSplitReplayUndoBoundaryAdmission,
        )
        .map_err(current_route_error)?;
    require_replay_undo_boundary_row(source_row)?;
    let forbidden_surface_denials = current_replay_undo_forbidden_surface_denial_ledger();
    forbidden_surface_denials
        .require_phase_eleven_denials()
        .map_err(current_route_error)?;

    Ok(lower_replay_undo_planner_route_packet(
        AdmittedReplayUndoPlannerRouteInput::new(
            family,
            packet,
            route_product.product_identity(),
            source_row.source_identity(),
            source_row.source_path(),
            inventory.rows().len(),
            forbidden_surface_denials.row_count(),
        ),
    ))
}

fn require_packet_matches_current_scope_route_product(
    packet: &crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryPacket,
    route_product: &super::scope_route_product::ReplayUndoPlannerScopeRouteProduct,
) -> Result<(), PlannerOwnedRoutingError> {
    if packet.stage_index_identity().digest() != route_product.stage_index_identity_digest() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "planner-owned replay/undo route requires the admitted packet stage index to match the lowered scope route product",
        ));
    }
    if packet.evidence_lookup_receipt_identity().digest()
        != route_product.lookup_receipt_identity_digest()
    {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "planner-owned replay/undo route requires the admitted packet lookup receipt to match the lowered scope route product",
        ));
    }
    if packet.replay_scope_identity().digest() != route_product.replay_scope_identity_digest() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "planner-owned replay/undo route requires the admitted packet replay scope to match the lowered scope route product",
        ));
    }
    if packet.undo_scope_identity().digest() != route_product.undo_scope_identity_digest() {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::MismatchedSelectedRouteSupport,
            "planner-owned replay/undo route requires the admitted packet undo scope to match the lowered scope route product",
        ));
    }
    Ok(())
}

fn require_replay_undo_boundary_row(
    row: &crate::replay_undo_inventory::ReplayUndoInventoryReportRow,
) -> Result<(), PlannerOwnedRoutingError> {
    if row.category() != ReplayUndoInventoryCategory::UndoScope {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            "planner-owned replay/undo route requires the kernel replay/undo admission row to stay in the undo-scope category",
        ));
    }
    if row.disposition() != ReplayUndoInventoryDisposition::Migrate {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            "planner-owned replay/undo route requires the kernel replay/undo admission row to be migrated into the ordinary lane",
        ));
    }
    if row.source_kind() != ReplayUndoInventorySourceKind::PublicFunction {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            "planner-owned replay/undo route requires the kernel replay/undo admission row to name the public admission function",
        ));
    }
    if row.source_path()
        != "crates/worth-kernel/src/workload_composition/worth_workload/replay_undo_boundary/boolean_split_boundary_admission.rs"
    {
        return Err(PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            "planner-owned replay/undo route requires the kernel replay/undo admission row to stay anchored to the admitted boundary surface",
        ));
    }
    Ok(())
}

fn current_route_error<E: std::fmt::Debug>(error: E) -> PlannerOwnedRoutingError {
    PlannerOwnedRoutingError::new(
        PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
        format!("planner-owned replay/undo route did not assemble: {error:?}"),
    )
}
