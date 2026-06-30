use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_undo_scope_identity, ReplayUndoSemanticGraphEquivalenceBasis,
    ReplayUndoSemanticGraphLocalityScope, UndoScopeIdentity, UndoScopeIdentityInput,
};

use super::undo_scope_product_counters::TopologyUndoScopeProductCounters;
use super::TopologyUndoScopeProduct;
use crate::replay_undo_semantic_graph::lowering::lower_topology_touched_subjects;
use crate::replay_undo_semantic_graph::{TopologyUndoPlanError, TopologyUndoSelectedPlan};

pub fn lower_topology_undo_scope_product_from_selected_plan<'a>(
    undo_plan: &TopologyUndoSelectedPlan<'a>,
) -> Result<TopologyUndoScopeProduct<'a>, TopologyUndoPlanError> {
    let admitted_input = undo_plan.admitted_input();
    let equivalence_basis = lower_topology_undo_equivalence_basis_from_selected_plan(undo_plan);
    let scope_identity =
        admit_undo_scope_identity(UndoScopeIdentityInput::new(equivalence_basis.clone()));
    let counters =
        TopologyUndoScopeProductCounters::new(equivalence_basis.touched_subjects().len());
    Ok(TopologyUndoScopeProduct::new(
        undo_plan.family_identity(),
        admitted_input.touched_closure(),
        admitted_input.prior_proof_identity().clone(),
        admitted_input.stage_index_identity().clone(),
        admitted_input.semantic_graph_identity().to_string(),
        counters,
        equivalence_basis,
        scope_identity,
    ))
}

pub fn lower_topology_undo_scope_identity_from_scope_product(
    scope_product: &TopologyUndoScopeProduct<'_>,
) -> UndoScopeIdentity {
    scope_product.scope_identity().clone()
}

pub fn lower_topology_undo_equivalence_basis_from_scope_product(
    scope_product: &TopologyUndoScopeProduct<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    scope_product.equivalence_basis().clone()
}

pub fn lower_topology_undo_equivalence_basis_from_selected_plan(
    undo_plan: &TopologyUndoSelectedPlan<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    let admitted_input = undo_plan.admitted_input();
    ReplayUndoSemanticGraphEquivalenceBasis::new(
        ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
        lower_topology_touched_subjects(admitted_input.touched_closure().basis()),
        admitted_input.prior_proof_identity().clone(),
        Some(admitted_input.stage_index_identity().clone()),
    )
}
