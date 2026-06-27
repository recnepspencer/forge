use schema::facade::platform::authority::replay_undo_semantic_graph::{
    admit_replay_scope_identity, ReplayScopeIdentity, ReplayScopeIdentityInput,
    ReplayUndoSemanticGraphEquivalenceBasis, ReplayUndoSemanticGraphLocalityScope,
    ReplayUndoSemanticGraphTouchedSubject, UndoScopeIdentity,
};
use schema::facade::platform::authority::replay_undo_semantic_graph_internal::admit_topology_derived_invalidation_prior_proof_identity;

use crate::derived_invalidation_execution::DerivedInvalidationExecutionReceipt;
use crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure;
use crate::topology_operators::TopologyTouchedGraphBasis;

use super::admission::{
    admit_topology_undo_semantic_graph_input, TopologyReplaySemanticGraphAdmittedInput,
    TopologyUndoSemanticGraphAdmissionRequest, TopologyUndoSemanticGraphAdmittedInput,
};
use super::planning::{
    select_topology_replay_plan, select_topology_undo_plan, TopologyReplayPlanError,
    TopologyUndoPlanError,
};
use super::scope_product::{TopologyReplayScopeProduct, TopologyUndoScopeProduct};
use crate::undo_family_catalog::TopologyUndoFamilyIdentity;

pub fn lower_topology_replay_scope_identity(
    touched_graph_basis: &TopologyTouchedGraphBasis,
    invalidation_receipt: &DerivedInvalidationExecutionReceipt,
) -> ReplayScopeIdentity {
    admit_replay_scope_identity(ReplayScopeIdentityInput::new(
        lower_topology_replay_equivalence_basis(touched_graph_basis, invalidation_receipt),
    ))
}

pub fn lower_topology_undo_scope_identity(
    family_identity: TopologyUndoFamilyIdentity,
    touched_closure: &DerivedInvalidationTouchedClosure,
    invalidation_receipt: &DerivedInvalidationExecutionReceipt,
) -> UndoScopeIdentity {
    let admitted_input =
        admit_topology_undo_semantic_graph_input(TopologyUndoSemanticGraphAdmissionRequest::new(
            family_identity,
            touched_closure,
            invalidation_receipt,
        ))
        .expect("topology undo lowering should admit traversal-views rollback input");
    lower_topology_undo_scope_identity_from_admitted_input(&admitted_input)
}

pub fn lower_topology_replay_scope_identity_from_admitted_input(
    admitted_input: &TopologyReplaySemanticGraphAdmittedInput<'_>,
) -> ReplayScopeIdentity {
    let replay_plan = select_topology_replay_plan(admitted_input)
        .expect("admitted topology replay input should lower to a replay plan");
    let scope_product = lower_topology_replay_scope_product_from_selected_plan(&replay_plan)
        .expect("selected topology replay plan should lower to a scope product");
    lower_topology_replay_scope_identity_from_scope_product(&scope_product)
}

pub fn lower_topology_replay_equivalence_basis(
    touched_graph_basis: &TopologyTouchedGraphBasis,
    invalidation_receipt: &DerivedInvalidationExecutionReceipt,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    ReplayUndoSemanticGraphEquivalenceBasis::new(
        ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
        lower_topology_touched_subjects(touched_graph_basis),
        admit_topology_derived_invalidation_prior_proof_identity(
            invalidation_receipt.execution_receipt_digest(),
        ),
        None,
    )
}

pub fn lower_topology_replay_equivalence_basis_from_admitted_input(
    admitted_input: &TopologyReplaySemanticGraphAdmittedInput<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    let replay_plan = select_topology_replay_plan(admitted_input)
        .expect("admitted topology replay input should lower to a replay plan");
    super::scope_product::lower_topology_replay_equivalence_basis_from_selected_plan(&replay_plan)
}

pub fn lower_topology_replay_scope_product_from_admitted_input<'a>(
    admitted_input: &'a TopologyReplaySemanticGraphAdmittedInput<'a>,
) -> Result<TopologyReplayScopeProduct<'a>, TopologyReplayPlanError> {
    let replay_plan = select_topology_replay_plan(admitted_input)?;
    super::scope_product::lower_topology_replay_scope_product_from_selected_plan(&replay_plan)
}

pub fn lower_topology_replay_scope_product_from_selected_plan<'a>(
    replay_plan: &super::planning::TopologyReplaySelectedPlan<'a>,
) -> Result<TopologyReplayScopeProduct<'a>, TopologyReplayPlanError> {
    super::scope_product::lower_topology_replay_scope_product_from_selected_plan(replay_plan)
}

pub fn lower_topology_replay_equivalence_basis_from_selected_plan(
    replay_plan: &super::planning::TopologyReplaySelectedPlan<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    super::scope_product::lower_topology_replay_equivalence_basis_from_selected_plan(replay_plan)
}

pub fn lower_topology_replay_scope_identity_from_scope_product(
    scope_product: &TopologyReplayScopeProduct<'_>,
) -> ReplayScopeIdentity {
    super::scope_product::lower_topology_replay_scope_identity_from_scope_product(scope_product)
}

pub fn lower_topology_replay_equivalence_basis_from_scope_product(
    scope_product: &TopologyReplayScopeProduct<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    super::scope_product::lower_topology_replay_equivalence_basis_from_scope_product(scope_product)
}

pub fn lower_topology_undo_equivalence_basis(
    family_identity: TopologyUndoFamilyIdentity,
    touched_closure: &DerivedInvalidationTouchedClosure,
    invalidation_receipt: &DerivedInvalidationExecutionReceipt,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    let admitted_input =
        admit_topology_undo_semantic_graph_input(TopologyUndoSemanticGraphAdmissionRequest::new(
            family_identity,
            touched_closure,
            invalidation_receipt,
        ))
        .expect("topology undo lowering should admit traversal-views rollback input");
    lower_topology_undo_equivalence_basis_from_admitted_input(&admitted_input)
}

pub fn lower_topology_undo_scope_identity_from_admitted_input(
    admitted_input: &TopologyUndoSemanticGraphAdmittedInput<'_>,
) -> UndoScopeIdentity {
    let undo_plan = select_topology_undo_plan(admitted_input)
        .expect("admitted topology undo input should lower to an undo plan");
    let scope_product = lower_topology_undo_scope_product_from_selected_plan(&undo_plan)
        .expect("selected topology undo plan should lower to a scope product");
    lower_topology_undo_scope_identity_from_scope_product(&scope_product)
}

pub fn lower_topology_undo_equivalence_basis_from_admitted_input(
    admitted_input: &TopologyUndoSemanticGraphAdmittedInput<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    let undo_plan = select_topology_undo_plan(admitted_input)
        .expect("admitted topology undo input should lower to an undo plan");
    super::scope_product::lower_topology_undo_equivalence_basis_from_selected_plan(&undo_plan)
}

pub fn lower_topology_undo_scope_product_from_admitted_input<'a>(
    admitted_input: &'a TopologyUndoSemanticGraphAdmittedInput<'a>,
) -> Result<TopologyUndoScopeProduct<'a>, TopologyUndoPlanError> {
    let undo_plan = select_topology_undo_plan(admitted_input)?;
    lower_topology_undo_scope_product_from_selected_plan(&undo_plan)
}

pub fn lower_topology_undo_scope_product_from_selected_plan<'a>(
    undo_plan: &super::planning::TopologyUndoSelectedPlan<'a>,
) -> Result<TopologyUndoScopeProduct<'a>, TopologyUndoPlanError> {
    super::scope_product::lower_topology_undo_scope_product_from_selected_plan(undo_plan)
}

pub fn lower_topology_undo_equivalence_basis_from_selected_plan(
    undo_plan: &super::planning::TopologyUndoSelectedPlan<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    super::scope_product::lower_topology_undo_equivalence_basis_from_selected_plan(undo_plan)
}

pub fn lower_topology_undo_scope_identity_from_scope_product(
    scope_product: &TopologyUndoScopeProduct<'_>,
) -> UndoScopeIdentity {
    super::scope_product::lower_topology_undo_scope_identity_from_scope_product(scope_product)
}

pub fn lower_topology_undo_equivalence_basis_from_scope_product(
    scope_product: &TopologyUndoScopeProduct<'_>,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    super::scope_product::lower_topology_undo_equivalence_basis_from_scope_product(scope_product)
}

pub fn lower_topology_replay_scope_identity_from_touched_closure(
    touched_closure: &DerivedInvalidationTouchedClosure,
    invalidation_receipt: &DerivedInvalidationExecutionReceipt,
) -> ReplayScopeIdentity {
    admit_replay_scope_identity(ReplayScopeIdentityInput::new(
        ReplayUndoSemanticGraphEquivalenceBasis::new(
            ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
            lower_topology_touched_subjects(touched_closure.basis()),
            admit_topology_derived_invalidation_prior_proof_identity(
                invalidation_receipt.execution_receipt_digest(),
            ),
            None,
        ),
    ))
}

pub fn lower_topology_replay_equivalence_basis_from_touched_closure(
    touched_closure: &DerivedInvalidationTouchedClosure,
    invalidation_receipt: &DerivedInvalidationExecutionReceipt,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    ReplayUndoSemanticGraphEquivalenceBasis::new(
        ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
        lower_topology_touched_subjects(touched_closure.basis()),
        admit_topology_derived_invalidation_prior_proof_identity(
            invalidation_receipt.execution_receipt_digest(),
        ),
        None,
    )
}

pub fn lower_topology_undo_scope_identity_from_touched_closure(
    family_identity: TopologyUndoFamilyIdentity,
    touched_closure: &DerivedInvalidationTouchedClosure,
    invalidation_receipt: &DerivedInvalidationExecutionReceipt,
) -> UndoScopeIdentity {
    let admitted_input = admit_topology_undo_semantic_graph_input(
        TopologyUndoSemanticGraphAdmissionRequest::new(
            family_identity,
            touched_closure,
            invalidation_receipt,
        ),
    )
    .expect("topology undo touched-closure lowering should admit traversal-views rollback input");
    lower_topology_undo_scope_identity_from_admitted_input(&admitted_input)
}

pub fn lower_topology_undo_equivalence_basis_from_touched_closure(
    family_identity: TopologyUndoFamilyIdentity,
    touched_closure: &DerivedInvalidationTouchedClosure,
    invalidation_receipt: &DerivedInvalidationExecutionReceipt,
) -> ReplayUndoSemanticGraphEquivalenceBasis {
    let admitted_input = admit_topology_undo_semantic_graph_input(
        TopologyUndoSemanticGraphAdmissionRequest::new(
            family_identity,
            touched_closure,
            invalidation_receipt,
        ),
    )
    .expect("topology undo touched-closure lowering should admit traversal-views rollback input");
    lower_topology_undo_equivalence_basis_from_admitted_input(&admitted_input)
}

pub(crate) fn lower_topology_touched_subjects(
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

#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;
