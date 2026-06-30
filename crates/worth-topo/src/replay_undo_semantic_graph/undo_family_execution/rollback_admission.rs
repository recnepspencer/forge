use crate::undo_family_catalog::TopologyUndoFamilyIdentityAuthority;

use super::{MaterializedGraphRollbackRequest, TraversalViewsRollbackRequest};
use crate::replay_undo_semantic_graph::{
    admit_topology_undo_semantic_graph_input,
    lower_topology_undo_equivalence_basis_from_admitted_input,
    lower_topology_undo_scope_identity_from_admitted_input, TopologyUndoPlanError,
    TopologyUndoScopeProduct, TopologyUndoScopeProductCounters,
    TopologyUndoSemanticGraphAdmissionError, TopologyUndoSemanticGraphAdmissionRequest,
};

pub fn lower_topology_undo_scope_product_from_traversal_views_request<'a>(
    request: TraversalViewsRollbackRequest<'a>,
) -> Result<TopologyUndoScopeProduct<'a>, TopologyUndoFamilyExecutionError> {
    let admitted_input =
        admit_topology_undo_semantic_graph_input(TopologyUndoSemanticGraphAdmissionRequest::new(
            TopologyUndoFamilyIdentityAuthority::traversal_views().identity(),
            request.touched_closure(),
            request.invalidation_receipt(),
        ))?;
    let equivalence_basis =
        lower_topology_undo_equivalence_basis_from_admitted_input(&admitted_input);
    let scope_identity = lower_topology_undo_scope_identity_from_admitted_input(&admitted_input);
    let counters =
        TopologyUndoScopeProductCounters::new(equivalence_basis.touched_subjects().len());
    Ok(TopologyUndoScopeProduct::new(
        admitted_input.family_identity(),
        request.touched_closure(),
        admitted_input.prior_proof_identity().clone(),
        admitted_input.stage_index_identity().clone(),
        admitted_input.semantic_graph_identity().to_string(),
        counters,
        equivalence_basis,
        scope_identity,
    ))
}

pub fn lower_topology_undo_scope_product_from_materialized_graph_request<'a>(
    request: MaterializedGraphRollbackRequest<'a>,
) -> Result<TopologyUndoScopeProduct<'a>, TopologyUndoFamilyExecutionError> {
    let admitted_input =
        admit_topology_undo_semantic_graph_input(TopologyUndoSemanticGraphAdmissionRequest::new(
            TopologyUndoFamilyIdentityAuthority::materialized_graph().identity(),
            request.touched_closure(),
            request.invalidation_receipt(),
        ))?;
    let equivalence_basis =
        lower_topology_undo_equivalence_basis_from_admitted_input(&admitted_input);
    let scope_identity = lower_topology_undo_scope_identity_from_admitted_input(&admitted_input);
    let counters =
        TopologyUndoScopeProductCounters::new(equivalence_basis.touched_subjects().len());
    Ok(TopologyUndoScopeProduct::new(
        admitted_input.family_identity(),
        request.touched_closure(),
        admitted_input.prior_proof_identity().clone(),
        admitted_input.stage_index_identity().clone(),
        admitted_input.semantic_graph_identity().to_string(),
        counters,
        equivalence_basis,
        scope_identity,
    ))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TopologyUndoFamilyExecutionError {
    Admission(TopologyUndoSemanticGraphAdmissionError),
    Plan(TopologyUndoPlanError),
}

impl From<TopologyUndoSemanticGraphAdmissionError> for TopologyUndoFamilyExecutionError {
    fn from(value: TopologyUndoSemanticGraphAdmissionError) -> Self {
        Self::Admission(value)
    }
}

impl From<TopologyUndoPlanError> for TopologyUndoFamilyExecutionError {
    fn from(value: TopologyUndoPlanError) -> Self {
        Self::Plan(value)
    }
}
