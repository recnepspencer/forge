mod admitted_public_proof_input;
mod batch_admission_route;
mod compiled_product_reuse_route;
mod conflict_independence_route;
mod derived_diagnostics;
pub(crate) mod public_proof;
mod replay_undo_route;
mod selected_route;

#[cfg(test)]
pub(crate) use admitted_public_proof_input::current_worth_touched_graph_conflict_public_proof_input_with_packet_loader;
pub use admitted_public_proof_input::{
    admit_worth_touched_graph_conflict_public_proof_input,
    current_worth_touched_graph_conflict_public_proof_input,
    WorthTouchedGraphConflictAdmittedPublicProofInput,
};
pub use public_proof::{
    current_worth_touched_graph_conflict_milestone_fifteen_seed,
    current_worth_touched_graph_conflict_public_closeout,
    WorthTouchedGraphConflictMilestoneFifteenPlannerProofInput,
    WorthTouchedGraphConflictProofChain,
};
#[cfg(test)]
pub(crate) use batch_admission_route::current_worth_touched_graph_conflict_batch_admission_route_packet;
#[cfg(test)]
pub(crate) use batch_admission_route::current_worth_touched_graph_conflict_batch_admission_route_packet_with_receipt_override;
pub(crate) use batch_admission_route::BatchAdmissionPlannerRoutePacket;
pub use compiled_product_reuse_route::{
    current_worth_touched_graph_conflict_compiled_product_reuse_route_packet,
    CompiledProductReusePlannerRoutePacket,
};
#[cfg(test)]
pub(crate) use conflict_independence_route::current_worth_touched_graph_conflict_independence_route_packet;
#[cfg(test)]
pub(crate) use conflict_independence_route::current_worth_touched_graph_conflict_independence_route_packet_with_receipt_override;
pub(crate) use conflict_independence_route::ConflictIndependencePlannerRoutePacket;
pub use derived_diagnostics::current_worth_touched_graph_conflict_derived_read_diagnostic_input;
#[cfg(test)]
pub(crate) use derived_diagnostics::current_worth_touched_graph_conflict_derived_read_diagnostic_input_with_packet_loader;
#[cfg(test)]
pub(crate) use replay_undo_route::{
    current_replay_undo_transaction_route_input_for_tests,
    current_replay_undo_transaction_route_packet_with_input_override,
    current_replay_undo_undo_route_packet,
};
pub(crate) use replay_undo_route::{
    current_replay_undo_transaction_route_packet, lower_replay_undo_boundary_execution_proof,
    ReplayUndoBoundaryExecutionProof, ReplayUndoPlannerRoutePacket,
};
#[cfg(test)]
pub(crate) use selected_route::current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders;
#[cfg(test)]
pub(crate) use selected_route::SpatialRouteProjectionMarkers;
pub use selected_route::{
    current_worth_touched_graph_conflict_selected_route_packet,
    WorthTouchedGraphConflictSelectedRoutePacket,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlannerOwnedRoutingErrorKind {
    CurrentProofUnavailable,
    IncompleteSelectedRoutePacket,
    MismatchedSelectedRouteSupport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlannerOwnedRoutingError {
    kind: PlannerOwnedRoutingErrorKind,
    detail: String,
}

impl PlannerOwnedRoutingError {
    pub(crate) fn new(kind: PlannerOwnedRoutingErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> PlannerOwnedRoutingErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
