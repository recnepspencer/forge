use crate::facade::{topology_runtime, TopologyQueryAssembly, TopologyRuntimeAdapters};
use crate::projection::TopologyQueryRowLookup;
use crate::validation::reference_integrity::build_milestone_one_runtime;
use schema::facade::platform::relations::TopologyRelationKind;
use schema::facade::topology_authoring::{seed_milestone_one_primitive, MilestoneOnePrimitiveCase};

#[test]
fn row_lookup_finds_half_edge_neighbors_for_edge_fan_witnesses() {
    let mut runtime = build_milestone_one_runtime().expect(" runtime");
    seed_milestone_one_primitive(
        &mut runtime,
        "query.row-lookup.edge-fan",
        &MilestoneOnePrimitiveCase::NmtEdgeFan { face_count: 4 },
    )
    .expect("seed primitive");
    let adapters = TopologyRuntimeAdapters::current_head(runtime);
    let mut workspace =
        topology_runtime(adapters, "query.row-lookup.edge-fan.runtime").expect("runtime");
    let assembly = TopologyQueryAssembly::declare(&mut workspace).expect("declare assembly");
    let entity_rows = workspace.read(assembly.entities());
    let relation_rows = workspace.read(assembly.relations());
    let lookup = TopologyQueryRowLookup::new(&entity_rows, &relation_rows);
    let source_identity = lookup
        .first_source_identity_for_relation_kind(TopologyRelationKind::HalfEdgeRadialNext)
        .expect("edge fan should expose a radial source");
    let source_edge_identity = lookup
        .edge_identity_of_half_edge(&source_identity)
        .expect("source edge identity");
    let source_vertices = lookup
        .half_edge_vertex_identities(&source_identity)
        .expect("source vertex identities");

    let same_edge = entity_rows
        .iter()
        .map(|row| row.identity.as_str())
        .filter(|identity| {
            lookup
                .edge_identity_of_half_edge(identity)
                .is_ok_and(|edge_identity| edge_identity == source_edge_identity)
        })
        .collect::<Vec<_>>();
    let different_edge = entity_rows
        .iter()
        .map(|row| row.identity.as_str())
        .filter(|identity| {
            lookup
                .edge_identity_of_half_edge(identity)
                .is_ok_and(|edge_identity| edge_identity != source_edge_identity)
        })
        .collect::<Vec<_>>();

    assert!(same_edge.len() >= 2);
    assert!(!different_edge.is_empty());
    assert!(same_edge.iter().all(|identity| {
        lookup
            .edge_identity_of_half_edge(identity)
            .expect("same-edge identity")
            == source_edge_identity
    }));
    assert!(different_edge.iter().all(|identity| {
        lookup
            .edge_identity_of_half_edge(identity)
            .expect("different-edge identity")
            != source_edge_identity
    }));
    let sharing_vertex = entity_rows
        .iter()
        .map(|row| row.identity.as_str())
        .filter(|identity| *identity != source_identity)
        .filter(|identity| {
            lookup
                .half_edge_vertex_identities(identity)
                .is_ok_and(|candidate_vertices| {
                    candidate_vertices
                        .iter()
                        .any(|vertex| source_vertices.contains(vertex))
                })
        })
        .collect::<Vec<_>>();

    assert!(!sharing_vertex.is_empty());
}
