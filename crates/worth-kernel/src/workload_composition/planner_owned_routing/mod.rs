mod admitted_public_proof_input;
mod batch_admission_route;
mod compiled_product_reuse_route;
mod conflict_independence_route;
mod derived_diagnostics;
pub(crate) mod ordinary_consumer_authority;
mod public_facade;
pub(crate) mod public_proof;
mod replay_undo_route;
mod selected_route;
#[cfg(test)]
mod test_support;
#[cfg(test)]
pub(crate) use test_support::run_stack_heavy_planner_owned_routing_test;

#[cfg(test)]
pub(crate) use admitted_public_proof_input::current_worth_touched_graph_conflict_public_proof_input_with_packet_loader;
pub(crate) use admitted_public_proof_input::{
    admit_worth_touched_graph_conflict_public_proof_input,
    current_worth_touched_graph_conflict_public_proof_input,
    WorthTouchedGraphConflictAdmittedPublicProofInput,
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
#[cfg(test)]
pub(crate) use derived_diagnostics::current_worth_touched_graph_conflict_derived_diagnostic_projection_with_packet_loader;
pub(crate) use derived_diagnostics::{
    current_worth_touched_graph_conflict_derived_diagnostic_projection,
    current_worth_touched_graph_conflict_derived_diagnostic_projection_with_artifact_policy,
    select_rich_localization,
};
pub use derived_diagnostics::{
    WorthTouchedGraphConflictDerivedDiagnosticArtifactPolicy,
    WorthTouchedGraphConflictDerivedDiagnosticProjection,
};
pub use ordinary_consumer_authority::{
    current_completed_split_batch_execution_cluster_witness,
    current_lookup_consumed_batch_execution_cluster_witness,
    current_replay_undo_boundary_batch_execution_cluster_witness,
    current_worth_workload_ordinary_consumer_batch_execution_receipt,
    WorthWorkloadOrdinaryConsumerCurrentRouteWitness, WorthWorkloadOrdinaryConsumerCutoverError,
    WorthWorkloadOrdinaryConsumerCutoverErrorKind, WorthWorkloadOrdinaryConsumerRouteKind,
};
pub(crate) use ordinary_consumer_authority::{
    current_worth_workload_ordinary_consumer_cutover, WorthWorkloadOrdinaryConsumerCutover,
    WorthWorkloadOrdinaryConsumerCutoverPosture, WorthWorkloadOrdinaryConsumerCutoverRow,
};
#[cfg(test)]
pub(crate) use ordinary_consumer_authority::{
    ordinary_consumer_cutover_from_inventory_for_tests,
    ordinary_consumer_cutover_from_inventory_with_test_replay_undo_identity_override,
};
pub(crate) use public_facade::require_matching_projection_authority;
pub use public_facade::{
    current_public_closeout_consumer_residue_manifest,
    current_worth_touched_graph_conflict_public_facade,
    current_worth_touched_graph_conflict_public_facade_with_artifact_policy,
    PublicCloseoutConsumerResidueBoundaryPosture, PublicCloseoutConsumerResidueDisposition,
    PublicCloseoutConsumerResidueManifestError, PublicCloseoutConsumerResidueOwner,
    PublicCloseoutConsumerResidueRow, WorthTouchedGraphConflictPublicFacade,
    WorthTouchedGraphConflictPublicFacadeError, WorthTouchedGraphConflictPublicFacadeErrorKind,
    WorthTouchedGraphConflictPublicProofInspection,
};
pub(crate) use public_proof::{
    current_worth_touched_graph_conflict_milestone_fifteen_seed,
    current_worth_touched_graph_conflict_public_closeout,
};
pub use public_proof::{
    WorthTouchedGraphConflictArchitectureAlignmentReport,
    WorthTouchedGraphConflictArchitectureAlignmentReportRow,
    WorthTouchedGraphConflictDeletionAlignmentRow, WorthTouchedGraphConflictMilestoneFifteenSeed,
    WorthTouchedGraphConflictPublicCloseout, WorthTouchedGraphConflictPublicCloseoutError,
    WorthTouchedGraphConflictPublicCloseoutErrorKind, WorthTouchedGraphConflictQueryGapKind,
    WorthTouchedGraphConflictResidueBoundaryPosture, WorthTouchedGraphConflictResidueChain,
    WorthTouchedGraphConflictResidueDisposition, WorthTouchedGraphConflictResidueRow,
};
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
pub(crate) use selected_route::{
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
