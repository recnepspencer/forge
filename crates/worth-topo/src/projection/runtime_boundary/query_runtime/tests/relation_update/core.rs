use forge_query::facade::{ForgeQueryExistingRelationTarget, ForgeQueryExistingTruthAssertionMode};
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

use super::super::declaration_runtime_support::execute_current_head_topology_declaration;
use super::super::query_runtime_support::{query_entity_id_from_row, query_relation_id_from_row};
use crate::projection::runtime_boundary::query_runtime::{
    topology_runtime, TopologyRuntimeAdapters,
};
use crate::topology_operators::{
    LoopEndpointKind, TopologyEditFamily, TopologyRewireLoopEndpointDeclaration,
};
use crate::validation::reference_integrity::build_milestone_one_runtime;

#[test]
fn current_head_runtime_executes_identity_preserving_relation_updates_on_real_runtime() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-update-rewire-endpoint",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace = topology_runtime(adapters, ".current-head.query-update-rewire-endpoint")
        .expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let relation_rows = workspace.read(surfaces.relations());
    let relation = relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| {
                    kind_name == TopologyRelationKind::HalfEdgeNext.kind_name()
                })
        })
        .expect("seeded topology should contain half-edge successor relation");
    let source_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
        .expect("seeded relation should expose topology.source_identity")
        .to_string();
    let target_vertex_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(|value| value.as_str())
        .expect("seeded relation should expose topology.target_identity")
        .to_string();
    let binding = workspace
        .bind_existing_relation(
            ForgeQueryExistingRelationTarget::new(
                format!("{:?}", query_relation_id_from_row(relation)),
                relation.identity.clone(),
            )
            .expect("non-empty relation binding identity")
            .in_target_collection("TopologyRelation")
            .expect("target collection"),
        )
        .expect("bind existing relation");

    let receipt = workspace
        .update_existing(binding, |mutation| {
            mutation
                .aspect(
                    "topology.kind",
                    TopologyRelationKind::HalfEdgeNext.kind_name(),
                )
                .aspect("topology.source_identity", &source_identity)
                .aspect("topology.target_identity", &target_vertex_identity)
        })
        .expect("direct relation update substrate should execute through the real runtime");

    assert_eq!(
        receipt.mutation_family(),
        forge_query::facade::ForgeQueryMutationFamily::Update
    );
    assert_eq!(
        receipt
            .existing_truth_binding_evidence()
            .expect("relation update receipt should retain direct binding evidence")
            .family(),
        forge_query::facade::ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
    );

    let relation_rows_after = workspace.read(surfaces.relations());
    let original_relation = relation_rows_after
        .iter()
        .find(|row| query_relation_id_from_row(row) == query_relation_id_from_row(relation))
        .expect("relation should remain visible after denied update");
    assert_eq!(
        original_relation
            .payload
            .get("topology")
            .and_then(|value| value.get("target_identity"))
            .and_then(|value| value.as_str()),
        Some(target_vertex_identity.as_str())
    );
}

#[test]
fn current_head_runtime_executes_rewire_loop_endpoint_through_topology_operator_runner() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        ".current-head.query-edit-rewire-endpoint",
        &MilestoneOnePrimitiveCase::SheetDisk { edge_count: 4 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, ".current-head.query-edit-rewire-endpoint").expect("workspace");
    let surfaces =
        crate::projection::runtime_boundary::declared_query_surfaces::declare_topology_query_surfaces(
            &mut workspace,
        )
        .expect("declare surfaces");
    let relation_rows = workspace.read(surfaces.relations());
    let entity_rows = workspace.read(surfaces.entities());
    let relation = relation_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| {
                    kind_name == TopologyRelationKind::HalfEdgeEndsAtVertex.kind_name()
                })
        })
        .expect("seeded topology should contain an endpoint relation");
    let current_target_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("target_identity"))
        .and_then(|value| value.as_str())
        .expect("endpoint relation should expose topology.target_identity");
    let source_identity = relation
        .payload
        .get("topology")
        .and_then(|value| value.get("source_identity"))
        .and_then(|value| value.as_str())
        .expect("endpoint relation should expose topology.source_identity");
    let target_vertex_id = entity_rows
        .iter()
        .find(|row| {
            row.payload
                .get("topology")
                .and_then(|value| value.get("kind"))
                .and_then(|value| value.as_str())
                .is_some_and(|kind_name| kind_name == ".vertex")
                && row.identity != current_target_identity
        })
        .map(query_entity_id_from_row)
        .expect("seeded sheet disk should provide an alternate vertex");
    let half_edge_id = entity_rows
        .iter()
        .find(|row| row.identity == source_identity)
        .map(query_entity_id_from_row)
        .expect("relation source identity should resolve to a halfedge");
    let declaration = TopologyRewireLoopEndpointDeclaration::new(
        query_relation_id_from_row(relation),
        LoopEndpointKind::End,
        half_edge_id,
        target_vertex_id,
    );
    let execution =
        execute_current_head_topology_declaration(&mut workspace, &surfaces, declaration)
            .expect("endpoint rewire should execute through declaration entry");

    assert_eq!(
        execution.families,
        vec![TopologyEditFamily::RewireLoopEndpoint]
    );
    assert_eq!(
        execution
            .receipt
            .batch_mutation_evidence()
            .backend_verified_update_count(),
        1
    );
    assert_eq!(
        execution.inspection.component_operations()[0]
            .existing_truth_assertion_evidence()
            .expect("rewire receipt should retain backend verification evidence")
            .mode(),
        ForgeQueryExistingTruthAssertionMode::BackendVerifiedAssertion
    );
    let half_edge = execution
        .materialized
        .topology()
        .half_edges
        .iter()
        .find(|half_edge| half_edge.entity_id == half_edge_id)
        .expect("rewired halfedge should remain present");
    assert_eq!(half_edge.target_vertex_id, Some(target_vertex_id));
}
