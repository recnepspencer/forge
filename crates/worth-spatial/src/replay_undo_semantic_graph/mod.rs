mod admission;
mod current_boundary;
mod lowering;
mod planning;
mod scope_product;
#[cfg(any(test, feature = "test-support-lowering"))]
mod test_support;
mod undo_family_execution;

pub(crate) use lowering::lower_spatial_touched_subjects;

pub use admission::{
    admit_prepared_spatial_replay_semantic_graph_input, admit_spatial_replay_semantic_graph_input,
    admit_spatial_undo_semantic_graph_input, prepare_spatial_replay_semantic_graph_request,
    SpatialReplaySemanticGraphAdmissionError, SpatialReplaySemanticGraphAdmissionRequest,
    SpatialReplaySemanticGraphAdmittedInput, SpatialReplaySemanticGraphPreparationRequest,
    SpatialReplaySemanticGraphPreparedRequest, SpatialUndoSemanticGraphAdmissionRequest,
    SpatialUndoSemanticGraphAdmittedInput,
};
pub use current_boundary::{
    current_boolean_event_ledger_spatial_boundary, current_boolean_split_spatial_boundary,
    current_projection_receipt_spatial_boundary, CurrentReplayUndoSpatialBoundary,
    CurrentReplayUndoSpatialBoundaryError,
};
pub use lowering::{
    lower_spatial_replay_equivalence_basis,
    lower_spatial_replay_equivalence_basis_from_admitted_input,
    lower_spatial_replay_equivalence_basis_from_scope_product,
    lower_spatial_replay_equivalence_basis_from_selected_plan, lower_spatial_replay_scope_identity,
    lower_spatial_replay_scope_identity_from_admitted_input,
    lower_spatial_replay_scope_identity_from_scope_product,
    lower_spatial_replay_scope_product_from_admitted_input,
    lower_spatial_replay_scope_product_from_selected_plan, lower_spatial_undo_equivalence_basis,
    lower_spatial_undo_equivalence_basis_from_admitted_input,
    lower_spatial_undo_equivalence_basis_from_scope_product, lower_spatial_undo_scope_identity,
    lower_spatial_undo_scope_identity_from_admitted_input,
    lower_spatial_undo_scope_identity_from_scope_product,
    lower_spatial_undo_scope_product_from_admitted_input,
    lower_spatial_undo_scope_product_from_selected_plan, ReplayUndoSemanticGraphLoweringError,
};
pub use planning::{
    select_spatial_replay_plan, select_spatial_undo_plan, SpatialReplayPlanError,
    SpatialReplaySelectedPlan, SpatialUndoPlanError, SpatialUndoSelectedPlan,
};
pub use scope_product::{
    SpatialReplayScopeProduct, SpatialReplayScopeProductCounters,
    SpatialReplayScopeProductIdentity, SpatialUndoScopeProduct, SpatialUndoScopeProductCounters,
};
#[cfg(any(test, feature = "test-support-lowering"))]
pub use test_support::{
    boolean_event_ledger_query_required_sibling_spatial_boundary_fixture,
    boolean_event_ledger_spatial_boundary_fixture, projection_receipt_spatial_boundary_fixture,
    ReplayUndoSpatialBoundaryFixture,
};
pub use undo_family_execution::{
    lower_spatial_undo_scope_product_from_boolean_event_ledger_request,
    lower_spatial_undo_scope_product_from_projection_receipt_request,
    BooleanEventLedgerRollbackRequest, ProjectionReceiptRollbackRequest,
    SpatialUndoFamilyExecutionError,
};
