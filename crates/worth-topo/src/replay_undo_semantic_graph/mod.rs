mod admission;
mod current_boundary;
mod current_declaration_support;
pub(crate) mod current_invalidation_proof;
mod lowering;
mod planning;
mod scope_product;
#[cfg(any(test, feature = "test-support-lowering"))]
mod test_support;
mod undo_family_execution;

pub use admission::{
    admit_prepared_topology_replay_semantic_graph_input,
    admit_topology_replay_semantic_graph_input, admit_topology_undo_semantic_graph_input,
    prepare_topology_replay_semantic_graph_request,
    prepare_topology_replay_semantic_graph_stage_identity,
    TopologyReplaySemanticGraphAdmissionError, TopologyReplaySemanticGraphAdmissionRequest,
    TopologyReplaySemanticGraphAdmittedInput, TopologyReplaySemanticGraphPreparationRequest,
    TopologyReplaySemanticGraphPreparedRequest, TopologyReplaySemanticGraphPreparedStageAuthority,
    TopologyReplaySemanticGraphSelectedPlanIdentity, TopologyReplaySemanticGraphStageIdentity,
    TopologyReplaySemanticGraphStageReceiptAuthority, TopologyUndoSemanticGraphAdmissionError,
    TopologyUndoSemanticGraphAdmissionRequest, TopologyUndoSemanticGraphAdmittedInput,
};
pub use current_boundary::{
    current_replay_undo_topology_boundary, CurrentReplayUndoTopologyBoundary,
    CurrentReplayUndoTopologyBoundaryError,
};
pub(crate) use current_invalidation_proof::{
    current_topology_invalidation_proof, CurrentTopologyInvalidationProofError,
};
pub use lowering::{
    lower_topology_replay_equivalence_basis,
    lower_topology_replay_equivalence_basis_from_admitted_input,
    lower_topology_replay_equivalence_basis_from_scope_product,
    lower_topology_replay_equivalence_basis_from_selected_plan,
    lower_topology_replay_equivalence_basis_from_touched_closure,
    lower_topology_replay_scope_identity, lower_topology_replay_scope_identity_from_admitted_input,
    lower_topology_replay_scope_identity_from_scope_product,
    lower_topology_replay_scope_identity_from_touched_closure,
    lower_topology_replay_scope_product_from_admitted_input,
    lower_topology_replay_scope_product_from_selected_plan, lower_topology_undo_equivalence_basis,
    lower_topology_undo_equivalence_basis_from_admitted_input,
    lower_topology_undo_equivalence_basis_from_scope_product,
    lower_topology_undo_equivalence_basis_from_selected_plan,
    lower_topology_undo_equivalence_basis_from_touched_closure, lower_topology_undo_scope_identity,
    lower_topology_undo_scope_identity_from_admitted_input,
    lower_topology_undo_scope_identity_from_scope_product,
    lower_topology_undo_scope_identity_from_touched_closure,
    lower_topology_undo_scope_product_from_admitted_input,
    lower_topology_undo_scope_product_from_selected_plan,
};
pub use planning::{
    select_topology_replay_plan, select_topology_undo_plan, TopologyReplayPlanError,
    TopologyReplaySelectedPlan, TopologyUndoPlanError, TopologyUndoSelectedPlan,
};
pub use scope_product::{
    TopologyReplayScopeProduct, TopologyReplayScopeProductCounters, TopologyUndoScopeProduct,
    TopologyUndoScopeProductCounters,
};
#[cfg(any(test, feature = "test-support-lowering"))]
pub use test_support::{traversal_views_topology_undo_fixture, TraversalViewsTopologyUndoFixture};
pub use undo_family_execution::{
    lower_topology_undo_scope_product_from_materialized_graph_request,
    lower_topology_undo_scope_product_from_traversal_views_request,
    MaterializedGraphRollbackRequest, TopologyUndoFamilyExecutionError,
    TraversalViewsRollbackRequest,
};
