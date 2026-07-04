use schema::facade::platform::authority::replay_undo_semantic_graph::ReplayUndoPlannerRouteFamily;

use crate::replay_undo_inventory::ReplayUndoInventorySourceIdentity;
use crate::replay_undo_transaction_boundary::ReplayUndoTransactionBoundaryPacket;

use super::family_catalog::{
    current_replay_undo_planner_route_family_row, ReplayUndoPlannerRouteFamilyRow,
};

#[derive(Clone, Debug)]
pub(crate) struct AdmittedReplayUndoPlannerRouteInput {
    family_row: ReplayUndoPlannerRouteFamilyRow,
    transaction_boundary_packet: ReplayUndoTransactionBoundaryPacket,
    scope_route_product_identity: String,
    source_identity: ReplayUndoInventorySourceIdentity,
    source_path: String,
    inventory_row_count: usize,
    forbidden_surface_denial_count: usize,
}

impl AdmittedReplayUndoPlannerRouteInput {
    pub(crate) fn new(
        family: ReplayUndoPlannerRouteFamily,
        transaction_boundary_packet: ReplayUndoTransactionBoundaryPacket,
        scope_route_product_identity: impl Into<String>,
        source_identity: ReplayUndoInventorySourceIdentity,
        source_path: impl Into<String>,
        inventory_row_count: usize,
        forbidden_surface_denial_count: usize,
    ) -> Self {
        Self {
            family_row: current_replay_undo_planner_route_family_row(family),
            transaction_boundary_packet,
            scope_route_product_identity: scope_route_product_identity.into(),
            source_identity,
            source_path: source_path.into(),
            inventory_row_count,
            forbidden_surface_denial_count,
        }
    }

    pub(crate) const fn family(&self) -> ReplayUndoPlannerRouteFamily {
        self.family_row.family()
    }

    pub(crate) const fn transaction_boundary_packet(&self) -> &ReplayUndoTransactionBoundaryPacket {
        &self.transaction_boundary_packet
    }

    pub(crate) fn scope_route_product_identity(&self) -> &str {
        &self.scope_route_product_identity
    }

    pub(crate) const fn source_identity(&self) -> ReplayUndoInventorySourceIdentity {
        self.source_identity
    }

    pub(crate) fn source_path(&self) -> &str {
        &self.source_path
    }

    pub(crate) const fn inventory_row_count(&self) -> usize {
        self.inventory_row_count
    }

    pub(crate) const fn forbidden_surface_denial_count(&self) -> usize {
        self.forbidden_surface_denial_count
    }
}
