use crate::facade::{
    worth_topology_runtime, WorthTopologyQueryAssembly, WorthTopologyRuntimeAdapters,
};
use crate::query::WorthTopologyQuerySnapshotIndex;
use crate::runtime_invariants::build_worth_milestone_one_runtime;
use worth_schema::facade::topology_authoring::{
    seed_milestone_one_primitive, WorthMilestoneOnePrimitiveCase,
};
use worth_schema::facade::WorthTopologyRelationKind;

#[test]
fn snapshot_index_indexes_half_edge_neighbors_for_edge_fan_witnesses() {
    let mut runtime = build_worth_milestone_one_runtime().expect("worth runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "query.snapshot-index.edge-fan",
        &WorthMilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = WorthTopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        worth_topology_runtime(adapters, "query.snapshot-index.edge-fan.runtime").expect("runtime");
    let assembly = WorthTopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let entity_rows = workspace.read(assembly.entities());
    let relation_rows = workspace.read(assembly.relations());
    let index = WorthTopologyQuerySnapshotIndex::new(&entity_rows, &relation_rows)
        .expect("snapshot index should build");
    let source_identity = index
        .first_source_identity_for_relation_kind(WorthTopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose a radial source");
    let sharing_vertex = index
        .half_edge_identities_sharing_vertex(&source_identity)
        .expect("shared-vertex lookup");
    let source_edge_identity = index
        .edge_identity_of_half_edge(&source_identity)
        .expect("source edge identity");

    let same_edge = entity_rows
        .iter()
        .map(|row| row.identity.as_str())
        .filter(|identity| {
            index
                .edge_identity_of_half_edge(identity)
                .is_ok_and(|edge_identity| edge_identity == source_edge_identity)
        })
        .collect::<Vec<_>>();
    let different_edge = entity_rows
        .iter()
        .map(|row| row.identity.as_str())
        .filter(|identity| {
            index
                .edge_identity_of_half_edge(identity)
                .is_ok_and(|edge_identity| edge_identity != source_edge_identity)
        })
        .collect::<Vec<_>>();

    assert!(same_edge.len() >= 2);
    assert!(!different_edge.is_empty());
    assert!(!sharing_vertex.is_empty());
    assert!(same_edge.iter().all(|identity| {
        index
            .edge_identity_of_half_edge(identity)
            .expect("same-edge identity")
            == source_edge_identity
    }));
    assert!(different_edge.iter().all(|identity| {
        index
            .edge_identity_of_half_edge(identity)
            .expect("different-edge identity")
            != source_edge_identity
    }));
    assert!(sharing_vertex.iter().all(|identity| {
        let candidate_vertices = index
            .half_edge_vertex_identities(identity)
            .expect("candidate vertex identities");
        let source_vertices = index
            .half_edge_vertex_identities(&source_identity)
            .expect("source vertex identities");
        candidate_vertices
            .iter()
            .any(|vertex| source_vertices.contains(vertex))
    }));
}
