use forge_relational::facade::identity::{EntityId, PartitionId, RelationId};
use schema::facade::platform::relations::TopologyRelationKind;

use crate::derived_invalidation_execution::DerivedInvalidationExecutionReceipt;
use crate::derived_invalidation_migrated_products::{
    MaterializedGraphReadSource, MaterializedGraphReadStageExecutor,
    MaterializedGraphReadStageReceipt, TraversalViewsReadSource, TraversalViewsReadStageExecutor,
    TraversalViewsReadStageReceipt,
};
use crate::derived_topology::invalidation_plan::selection::selection_test_fixtures::{
    admitted_legality_support, admitted_query_support, catalog_closeout,
    loop_cycles_touched_closure,
};
use crate::derived_topology::invalidation_plan::selection::{
    DerivedInvalidationDensityPolicy, DerivedInvalidationSelectedPlan,
};
use crate::replay_family_catalog::TopologyReplayFamilyIdentityAuthority;
use crate::replay_undo_semantic_graph::{
    prepare_topology_replay_semantic_graph_request,
    prepare_topology_replay_semantic_graph_stage_identity,
    TopologyReplaySemanticGraphPreparationRequest, TopologyReplaySemanticGraphPreparedRequest,
    TopologyReplaySemanticGraphStageReceiptAuthority,
};
use crate::topology_operators::{
    test_basis_from_parts, TopologyTouchedAspect, TopologyTouchedEntity, TopologyTouchedRelation,
    TopologyTouchedScope,
};

pub(super) fn prepare_traversal_replay_request<'a>(
    touched_closure: &'a crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure,
    invalidation_receipt: &'a DerivedInvalidationExecutionReceipt,
    stage_receipt: &'a TraversalViewsReadStageReceipt,
    declared_stage_receipt: Option<&TraversalViewsReadStageReceipt>,
) -> TopologyReplaySemanticGraphPreparedRequest<'a> {
    prepare_topology_replay_semantic_graph_request(
        TopologyReplaySemanticGraphPreparationRequest::new(
            TopologyReplayFamilyIdentityAuthority::traversal_views().identity(),
            touched_closure,
            invalidation_receipt,
            Some(TopologyReplaySemanticGraphStageReceiptAuthority::TraversalViews(stage_receipt)),
            declared_stage_receipt.map(|receipt| {
                prepare_topology_replay_semantic_graph_stage_identity(
                    TopologyReplaySemanticGraphStageReceiptAuthority::TraversalViews(receipt),
                )
            }),
        ),
    )
}

pub(super) fn reordered_traversal_touch_proof(
) -> crate::topology_operators::TopologyDeclaredTouchedGraphBasisProof {
    let basis = test_basis_from_parts(
        vec![
            TopologyTouchedEntity::new(entity_id(121)),
            TopologyTouchedEntity::new(entity_id(120)),
        ],
        vec![TopologyTouchedRelation::new(relation_id(160))],
        vec![TopologyRelationKind::HalfEdgeNext],
        vec![TopologyTouchedAspect::TopologyBoundary],
        vec![TopologyTouchedScope::Relation, TopologyTouchedScope::Loop],
    )
    .with_operating_world_for_tests(
        crate::topology_operators::TopologyTouchedOperatingWorld::mainline(),
    );
    crate::topology_operators::TopologyDeclaredTouchedGraphBasisProof::from_basis(
        "traversal-views-touch",
        basis,
    )
    .expect("reordered traversal touch proof should lower")
}

pub(super) fn invalidation_receipt_for(
    touched_closure: &crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure,
) -> DerivedInvalidationExecutionReceipt {
    invalidation_receipt_for_density(touched_closure, DerivedInvalidationDensityPolicy::Sparse)
}

pub(super) fn invalidation_receipt_for_density(
    touched_closure: &crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure,
    density_policy: DerivedInvalidationDensityPolicy,
) -> DerivedInvalidationExecutionReceipt {
    let selected_plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        touched_closure,
        &admitted_query_support(),
        &admitted_legality_support(),
        density_policy,
    )
    .expect("test invalidation plan should lower");
    DerivedInvalidationExecutionReceipt::execute_selected_plan(&selected_plan)
        .expect("selected plan should execute")
}

pub(super) fn loop_cycles_invalidation_receipt() -> DerivedInvalidationExecutionReceipt {
    invalidation_receipt_for(&loop_cycles_touched_closure("loop-touch"))
}

pub(super) fn selected_traversal_receipt(
    semantic_family_key: &'static str,
) -> TraversalViewsReadStageReceipt {
    selected_traversal_receipt_with_density(
        semantic_family_key,
        DerivedInvalidationDensityPolicy::Sparse,
    )
}

pub(super) fn selected_traversal_receipt_with_density(
    semantic_family_key: &'static str,
    density_policy: DerivedInvalidationDensityPolicy,
) -> TraversalViewsReadStageReceipt {
    let touched_closure = selected_traversal_touched_closure(semantic_family_key);
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &touched_closure,
        &admitted_query_support(),
        &admitted_legality_support(),
        density_policy,
    )
    .expect("traversal replay plan should lower");
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_shell_nmt_fan_view(4);
    let read_source =
        TraversalViewsReadSource::select_from_touched_closure(&plan, &touched_closure, &topology)
            .expect("traversal read source should select");
    TraversalViewsReadStageExecutor::execute(&plan, read_source)
        .expect("traversal read stage receipt should execute")
}

pub(super) fn selected_materialized_receipt(
    semantic_family_key: &'static str,
) -> MaterializedGraphReadStageReceipt {
    let plan = DerivedInvalidationSelectedPlan::lower(
        &catalog_closeout(),
        &loop_cycles_touched_closure(semantic_family_key),
        &admitted_query_support(),
        &admitted_legality_support(),
        DerivedInvalidationDensityPolicy::Sparse,
    )
    .expect("materialized replay plan should lower");
    let topology =
        crate::test_support::hostile_neighborhoods::interpretation_neighborhoods::open_wire_chain_view(
            4,
        );
    let read_source =
        MaterializedGraphReadSource::from_topology_view_with_selected_prefix(&topology, 2, 1)
            .expect("materialized read source should select");
    MaterializedGraphReadStageExecutor::execute(&plan, read_source)
        .expect("materialized read stage receipt should execute")
}

pub(super) fn selected_traversal_touched_closure(
    semantic_family_key: &'static str,
) -> crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure {
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
    .with_operating_world_for_tests(
        crate::topology_operators::TopologyTouchedOperatingWorld::mainline(),
    );
    let proof = crate::topology_operators::TopologyDeclaredTouchedGraphBasisProof::from_basis(
        semantic_family_key,
        basis,
    )
    .expect("traversal-view touch should lower");
    crate::derived_invalidation_selected_plan::DerivedInvalidationTouchedClosure::from_declared_touch(
        &proof,
    )
}

fn entity_id(slot: u64) -> EntityId {
    EntityId::new(PartitionId::main(), slot, 1)
}

fn relation_id(slot: u64) -> RelationId {
    RelationId::new(PartitionId::main(), slot, 1)
}
