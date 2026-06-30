use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_replay_scope_identity, ReplayScopeIdentity, ReplayScopeIdentityInput,
    ReplayUndoSemanticGraphEquivalenceBasis, ReplayUndoSemanticGraphLocalityScope,
    ReplayUndoSemanticGraphTouchedSubject,
};

use super::replay_scope_product::TopologyReplayScopeProduct;
use super::replay_scope_product_counters::TopologyReplayScopeProductCounters;
use crate::replay_undo_semantic_graph::{TopologyReplayPlanError, TopologyReplaySelectedPlan};
use crate::topology_operators::TopologyTouchedGraphBasis;

pub fn lower_topology_replay_scope_product_from_selected_plan<'a>(
    replay_plan: &TopologyReplaySelectedPlan<'a>,
) -> Result<TopologyReplayScopeProduct<'a>, TopologyReplayPlanError> {
    let admitted_input = replay_plan.admitted_input();
    let equivalence_basis = lower_topology_replay_equivalence_basis_from_selected_plan(replay_plan);
    let scope_identity =
        admit_replay_scope_identity(ReplayScopeIdentityInput::new(equivalence_basis.clone()));
    let counters =
        TopologyReplayScopeProductCounters::new(equivalence_basis.touched_subjects().len());
    Ok(TopologyReplayScopeProduct::new(
        replay_plan.family_identity(),
        admitted_input.touched_closure(),
        admitted_input.prior_proof_identity().clone(),
        admitted_input.selected_plan_identity().clone(),
        admitted_input.stage_identity().clone(),
        counters,
        equivalence_basis,
        scope_identity,
    ))
}

pub fn lower_topology_replay_scope_identity_from_scope_product(
    scope_product: &TopologyReplayScopeProduct<'_>,
) -> ReplayScopeIdentity {
    scope_product.scope_identity().clone()
}

pub fn lower_topology_replay_equivalence_basis_from_scope_product(
    scope_product: &TopologyReplayScopeProduct<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    scope_product.equivalence_basis().clone()
}

pub fn lower_topology_replay_equivalence_basis_from_selected_plan<'a>(
    replay_plan: &TopologyReplaySelectedPlan<'a>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    let admitted_input = replay_plan.admitted_input();
    ReplayUndoSemanticGraphEquivalenceBasis::new(
        ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
        lower_topology_touched_subjects(admitted_input.touched_closure().basis()),
        admitted_input.prior_proof_identity().clone(),
        Some(admitted_input.stage_index_identity().clone()),
    )
}

fn lower_topology_touched_subjects(
    touched_graph_basis: &TopologyTouchedGraphBasis,
) -> Vec<ReplayUndoSemanticGraphTouchedSubject> {
    let mut touched_subjects = Vec::new();
    touched_subjects.extend(touched_graph_basis.entities().iter().map(|entity| {
        let entity_id = entity.entity_id();
        ReplayUndoSemanticGraphTouchedSubject::TopologyEntity {
            entity_identity: format!(
                "{}:{}:{}",
                entity_id.partition_id.0, entity_id.local_slot.0, entity_id.generation.0
            ),
        }
    }));
    touched_subjects.extend(touched_graph_basis.relations().iter().map(|relation| {
        let relation_id = relation.relation_id();
        ReplayUndoSemanticGraphTouchedSubject::TopologyRelation {
            relation_identity: format!(
                "{}:{}:{}",
                relation_id.partition_id.0, relation_id.local_slot.0, relation_id.generation.0
            ),
        }
    }));
    touched_subjects.extend(
        touched_graph_basis
            .relation_kinds()
            .iter()
            .map(
                |relation_kind| ReplayUndoSemanticGraphTouchedSubject::TopologyRelationKind {
                    relation_kind: relation_kind.kind_name().to_string(),
                },
            ),
    );
    touched_subjects.extend(
        touched_graph_basis
            .aspects()
            .iter()
            .copied()
            .map(|aspect| ReplayUndoSemanticGraphTouchedSubject::TopologyAspect { aspect }),
    );
    touched_subjects.extend(
        touched_graph_basis
            .topology_scopes()
            .iter()
            .copied()
            .map(|scope| ReplayUndoSemanticGraphTouchedSubject::TopologyScope { scope }),
    );
    touched_subjects
}
