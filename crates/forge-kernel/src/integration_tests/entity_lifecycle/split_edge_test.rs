//! Integration tests: SplitEdge on real cube geometry.
//!
//! DOMAIN: Splits edges of a BSP-generated cube and verifies
//! structural invariants, halfedge rewiring, and entity counts.

use crate::integration_tests::harness::assertions::{
    assert_all_invariants, assert_counts, assert_face_valence, EntityCounts,
};
use crate::integration_tests::harness::shapes::{
    collect_face_loop, first_halfedge_of_face, unit_cube,
};
use forge_topo::entity_lifecycle::split_edge::SplitEdge;

/// Split one edge of a cube → cube gains 1 vertex, 2 halfedges, 1 edge.
/// Two adjacent faces go from 4-sided to 5-sided.
#[test]
fn split_single_cube_edge() {
    let (topo, handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let face = handles.faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let result = draft.execute(SplitEdge {
        edge: start_he,
        parameter: 0.5,
    }).unwrap().into_value();

    assert_all_invariants(draft.arena());

    assert_counts(draft.arena(), EntityCounts {
        faces: 6,
        vertices: 9,     // 8 + 1 midpoint
        half_edges: 26,   // 24 + 2 new
        edges: 13,        // 12 + 1 new
        loops: 6,
        shells: 1,
        bodies: 1,
    });

    let new_v = result.new_vertex;
    let v_hes = draft.arena().halfedges_from_vertex(new_v);
    assert!(
        !v_hes.is_empty(),
        "New vertex has no incident halfedges"
    );
}

/// Split every edge of one face → that face becomes an 8-gon.
/// A cube face has 4 edges; splitting each adds 1 vertex and 1 edge per split.
#[test]
fn split_all_edges_of_one_face() {
    let (topo, handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let face = handles.faces[0];

    let loop_hes = {
        let start = first_halfedge_of_face(draft.arena(), face).unwrap();
        collect_face_loop(draft.arena(), start).unwrap()
    };

    assert_eq!(loop_hes.len(), 4, "Cube face should have 4 halfedges");

    for he in &loop_hes {
        draft.execute(SplitEdge {
            edge: *he,
            parameter: 0.5,
        }).unwrap().into_value();
    }

    assert_all_invariants(draft.arena());

    assert_counts(draft.arena(), EntityCounts {
        faces: 6,
        vertices: 12,     // 8 + 4 new midpoints
        half_edges: 32,    // 24 + 8
        edges: 16,         // 12 + 4
        loops: 6,
        shells: 1,
        bodies: 1,
    });

    assert_face_valence(draft.arena(), face, 8);

    let committed = draft.commit().unwrap();
    assert_all_invariants(committed.arena());
}

/// Split the same edge twice → creates two midpoints.
/// Tests that the output handles from the first split are still valid.
#[test]
fn split_same_edge_twice() {
    let (topo, handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let face = handles.faces[0];
    let start_he = first_halfedge_of_face(draft.arena(), face).unwrap();

    let first_split = draft.execute(SplitEdge {
        edge: start_he,
        parameter: 0.5,
    }).unwrap().into_value();

    let second_split = draft.execute(SplitEdge {
        edge: first_split.he_mb,
        parameter: 0.5,
    }).unwrap().into_value();

    assert_all_invariants(draft.arena());

    assert_ne!(
        first_split.new_vertex, second_split.new_vertex,
        "Two splits must create distinct vertices"
    );

    assert_counts(draft.arena(), EntityCounts {
        faces: 6,
        vertices: 10,     // 8 + 2
        half_edges: 28,    // 24 + 4
        edges: 14,         // 12 + 2
        loops: 6,
        shells: 1,
        bodies: 1,
    });

    let committed = draft.commit().unwrap();
    assert_all_invariants(committed.arena());
}

/// Split every edge of the entire cube → 12 splits.
/// Tests massive rewiring with all 24 halfedges affected.
#[test]
fn split_all_cube_edges() {
    let (topo, _handles) = unit_cube().unwrap();
    let mut draft = topo.into_mutation();

    let all_edges: Vec<_> = draft.arena().iter_edges()
        .map(|(id, _)| id)
        .collect();

    assert_eq!(all_edges.len(), 12, "Cube should have 12 edges");

    for edge_id in &all_edges {
        let he_id = draft.arena().get_edge(*edge_id).unwrap().half_edge();
        draft.execute(SplitEdge {
            edge: he_id,
            parameter: 0.5,
        }).unwrap().into_value();
    }

    assert_all_invariants(draft.arena());

    assert_counts(draft.arena(), EntityCounts {
        faces: 6,
        vertices: 20,     // 8 + 12 midpoints
        half_edges: 48,    // 24 + 24
        edges: 24,         // 12 + 12
        loops: 6,
        shells: 1,
        bodies: 1,
    });

    let committed = draft.commit().unwrap();
    assert_all_invariants(committed.arena());
}
