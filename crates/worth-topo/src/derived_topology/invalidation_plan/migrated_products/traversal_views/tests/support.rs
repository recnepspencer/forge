use super::super::{
    close_traversal_views_migration_slice, TraversalViewsExecutionInput,
    TraversalViewsMigrationCloseout, TraversalViewsReadSource, TraversalViewsReadStageExecutor,
    TraversalViewsReadStageReceipt,
};
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    legality_support_missing_selected_legality_plan, loop_cycles_touched_closure,
    query_support_missing_native_read,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
    DerivedInvalidationTouchedClosure,
};
use crate::topology_operators::{
    test_basis_from_parts, TopologyDeclaredTouchedGraphBasisProof, TopologyTouchedAspect,
    TopologyTouchedEntity, TopologyTouchedOperatingWorld, TopologyTouchedRelation,
    TopologyTouchedScope,
};
use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use schema::facade::platform::relations::TopologyRelationKind;

pub(super) fn selected_traversal_views_plan() -> DerivedInvalidationSelectedPlan {
    selected_traversal_views_plan_with_key("traversal-views-touch")
}

fn selected_traversal_views_plan_with_key(
    semantic_family_key: &'static str,
) -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &selected_traversal_touched_closure_with_key(semantic_family_key),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}

pub(super) fn traversal_views_plan_missing_native_read() -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &loop_cycles_touched_closure("traversal-views-missing-native-read"),
        &query_support_missing_native_read(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}

pub(super) fn traversal_views_plan_missing_legality() -> DerivedInvalidationSelectedPlan {
    DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &loop_cycles_touched_closure("traversal-views-missing-legality"),
        &admitted_query_support(),
        &legality_support_missing_selected_legality_plan(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .unwrap()
}

pub(super) fn selected_traversal_receipt() -> TraversalViewsReadStageReceipt {
    let plan = selected_traversal_views_plan();
    let touched_closure = selected_traversal_touched_closure();
    let read_source = selected_traversal_read_source_for_plan_and_closure(&plan, &touched_closure);
    TraversalViewsReadStageExecutor::execute(&plan, read_source).unwrap()
}

pub(super) fn selected_traversal_read_source() -> TraversalViewsReadSource {
    let plan = selected_traversal_views_plan();
    let touched_closure = selected_traversal_touched_closure();
    selected_traversal_read_source_for_plan_and_closure(&plan, &touched_closure)
}

fn selected_traversal_read_source_for_plan_and_closure(
    plan: &DerivedInvalidationSelectedPlan,
    touched_closure: &DerivedInvalidationTouchedClosure,
) -> TraversalViewsReadSource {
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_shell_nmt_fan_view(4);
    TraversalViewsReadSource::select_from_touched_closure(&plan, &touched_closure, &topology)
        .unwrap()
}

pub(super) fn selected_traversal_touched_closure() -> DerivedInvalidationTouchedClosure {
    selected_traversal_touched_closure_with_key("traversal-views-touch")
}

pub(super) fn close_traversal_views_slice(
    receipt: TraversalViewsReadStageReceipt,
) -> TraversalViewsMigrationCloseout {
    let plan = selected_traversal_views_plan();
    let input =
        TraversalViewsExecutionInput::from_selected_plan_and_read_stage(&plan, receipt).unwrap();
    close_traversal_views_migration_slice(&plan, input).unwrap()
}

pub(super) fn assert_plan_selects_traversal_views(plan: &DerivedInvalidationSelectedPlan) {
    assert!(plan
        .selected_rows()
        .iter()
        .any(|row| row.family_identity() == DerivedTopologyProductFamilyIdentity::TraversalViews));
}

fn selected_traversal_touched_closure_with_key(
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
    let proof = TopologyDeclaredTouchedGraphBasisProof::from_basis(semantic_family_key, basis)
        .expect("traversal-view touch should lower to Query descriptor");
    DerivedInvalidationTouchedClosure::from_declared_touch(&proof)
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}
