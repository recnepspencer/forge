use forge_query::facade::ForgeQueryExistingTruthAssertionMode;
use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};

use super::relation_update_successor_span_support::successor_span_relocation_batch;
use super::relation_update_support::RelationUpdateQuerySupport;
use crate::edit::{WorthTopologyEditApplicationMode, WorthTopologyEditFamily};
use crate::query::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::runtime_invariants::build_worth_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_four_half_edge_span_relocation_on_larger_loop() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "worth.current-head.query-edit-rewire-successor-four-span",
        &WorthMilestoneOnePrimitiveCase::SheetDisk { edge_count: 7 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = worth_topology_runtime(
        adapters,
        "worth.current-head.query-edit-rewire-successor-four-span",
    )
    .expect("workspace");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let support = RelationUpdateQuerySupport::load(&workspace, &assembly);
    let moved_start_identity = support.first_source_identity_for_relation_kind(
        worth_schema::facade::WorthTopologyRelationKind::HalfEdgeNext,
    );
    let cycle = support.successor_cycle_identities(&mut workspace, &moved_start_identity, 7);
    let moved_end_id = support.find_entity_id_by_identity(cycle[3].as_str());
    let new_successor_id = support.find_entity_id_by_identity(cycle[6].as_str());
    let batch = successor_span_relocation_batch(
        &mut workspace,
        &support,
        &moved_start_identity,
        cycle[6].as_str(),
        4,
    );

    let execution = assembly
        .apply_edit(
            &mut workspace,
            batch,
            WorthTopologyEditApplicationMode::Mainline,
        )
        .expect("four-halfedge span relocation should execute through the contiguous span lane");

    assert!(execution
        .families
        .iter()
        .all(|family| *family == WorthTopologyEditFamily::RewireLoopSuccessor));
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_update_count(),
        6
    );
    assert!(execution
        .inspection
        .component_operations()
        .iter()
        .all(|operation| {
            operation.family() == "update"
                && operation.target_collection() == Some("WorthTopologyRelation")
                && operation
                    .existing_truth_assertion_evidence()
                    .is_some_and(|evidence| {
                        evidence.mode()
                            == ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
                    })
        }));
    let moved_end = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_end_id)
        .expect("moved end halfedge should remain present");
    assert_eq!(moved_end.next_half_edge_id, Some(new_successor_id));
}
