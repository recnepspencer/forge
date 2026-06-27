use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use schema::facade::platform::relations::TopologyRelationKind;

use crate::derived_invalidation_execution::DerivedInvalidationExecutionReceipt;
use crate::derived_invalidation_selected_plan::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
    DerivedInvalidationTouchedClosure,
};
use crate::replay_undo_semantic_graph::{
    lower_topology_undo_scope_product_from_traversal_views_request,
    TopologyUndoFamilyExecutionError, TopologyUndoScopeProduct, TraversalViewsRollbackRequest,
};
use crate::topology_operators::{
    test_basis_from_parts, TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedOperatingWorld, TopologyTouchedRelation,
    TopologyTouchedScope,
};

#[derive(Clone)]
pub struct TraversalViewsTopologyUndoFixture {
    touched_closure: DerivedInvalidationTouchedClosure,
    invalidation_receipt: DerivedInvalidationExecutionReceipt,
}

impl TraversalViewsTopologyUndoFixture {
    pub fn lower_undo_scope_product(
        &self,
    ) -> Result<TopologyUndoScopeProduct<'_>, TopologyUndoFamilyExecutionError> {
        lower_topology_undo_scope_product_from_traversal_views_request(
            TraversalViewsRollbackRequest::new(&self.touched_closure, &self.invalidation_receipt),
        )
    }
}

pub fn traversal_views_topology_undo_fixture() -> TraversalViewsTopologyUndoFixture {
    let touched_closure = selected_traversal_touched_closure("traversal-views-touch");
    let invalidation_receipt = invalidation_receipt_for(&touched_closure);
    TraversalViewsTopologyUndoFixture {
        touched_closure,
        invalidation_receipt,
    }
}

fn invalidation_receipt_for(
    touched_closure: &DerivedInvalidationTouchedClosure,
) -> DerivedInvalidationExecutionReceipt {
    let selected_plan = DerivedInvalidationSelectedPlan::lower(
        &crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::catalog_closeout(),
        touched_closure,
        &crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::admitted_query_support(),
        &crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .expect("test invalidation plan should lower");
    DerivedInvalidationExecutionReceipt::execute_selected_plan(&selected_plan)
        .expect("selected plan should execute")
}

fn selected_traversal_touched_closure(
    semantic_family_key: &'static str,
) -> DerivedInvalidationTouchedClosure {
    let basis = test_basis_from_parts(
        vec![
            TopologyTouchedEntity::new(entity_id(120)),
            TopologyTouchedEntity::new(entity_id(121)),
        ],
        vec![TopologyTouchedRelation::new(relation_id(160))],
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![TopologyTouchedAspect::TopologyBoundary],
        vec![TopologyTouchedScope::Loop, TopologyTouchedScope::Relation],
    )
    .with_operating_world_for_tests(TopologyTouchedOperatingWorld::mainline());
    let proof =
        TopologyDeclaredTouchedGraphBasisProof::from_basis_for_tests(semantic_family_key, basis)
            .expect("traversal-view touch should lower");
    DerivedInvalidationTouchedClosure::from_declared_touch(&proof)
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}
