use forge_query::facade::ForgeQueryExistingTruthAssertionMode;
use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;

use super::super::query_runtime_support::QueryRuntimeSupport;
use super::successor_span_declaration::successor_span_relocation_declaration;
use crate::certification::support::declaration_runtime::execute_current_head_topology_declaration;
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::test_support::schema_topology_authoring_boundary::seed_milestone_one_primitive_through_schema_execution;
use crate::topology_operators::TopologyMutationFamily;
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_four_half_edge_span_relocation_on_larger_loop() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive_through_schema_execution(
        &mut runtime,
        ".current-head.query-mutation-rewire-successor-four-span",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 7 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(
        adapters,
        ".current-head.query-mutation-rewire-successor-four-span",
    )
    .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let support = QueryRuntimeSupport::load(&mut workspace, &surfaces);
    let moved_start_identity = support.first_source_identity_for_relation_kind(
        schema::facade::platform::relations::TopologyRelationKind::HalfEdgeNext,
    );
    let cycle = support.successor_cycle_identities(&mut workspace, &moved_start_identity, 7);
    let moved_end_id = support.find_entity_id_by_identity(cycle[3].as_str());
    let new_successor_id = support.find_entity_id_by_identity(cycle[6].as_str());
    let declaration = successor_span_relocation_declaration(
        &mut workspace,
        &support,
        &moved_start_identity,
        cycle[6].as_str(),
        4,
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("four-halfedge span relocation should execute through declaration entry");
    let synopsis = execution.accepted_mutation_projection();

    assert!(synopsis
        .mutation_families()
        .iter()
        .all(|family| *family == TopologyMutationFamily::RewireLoopSuccessor));
    assert_eq!(
        execution
            .mutation_evidence()
            .backend_verified_update_count(),
        6
    );
    assert!(execution
        .inspection()
        .component_operations()
        .iter()
        .all(|operation| {
            operation.family() == "update"
                && operation.target_collection() == Some("TopologyRelation")
                && operation
                    .existing_truth_assertion_evidence()
                    .is_some_and(|evidence| {
                        evidence.mode()
                            == ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
                    })
        }));
    let moved_end = execution
        .materialized()
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == moved_end_id)
        .expect("moved end halfedge should remain present");
    assert_eq!(moved_end.next_half_edge_id, Some(new_successor_id));
}
