//! KV-04: Flush-union cube merge.
//!
//! Two cubes sharing a face must merge into a single block.
//! After merging shared vertices and identifying flush faces,
//! the result should have V=8 E=12 F=6 equivalent topology counts.

use forge_math::coincidence::{Coincidence, CoincidenceGraph, MergeAction};

#[test]
fn kv04_shared_vertices_merge_to_single_cluster() {
    let mut g = CoincidenceGraph::new();

    for id in 0..12 {
        g.declare(id);
    }

    g.merge(4, 4);

    g.merge(4, 8);
    g.merge(5, 9);
    g.merge(6, 10);
    g.merge(7, 11);

    assert_eq!(g.cluster_count(), 8);

    assert_eq!(g.representative(8).unwrap(), 4);
    assert_eq!(g.representative(9).unwrap(), 5);
    assert_eq!(g.representative(10).unwrap(), 6);
    assert_eq!(g.representative(11).unwrap(), 7);
}

#[test]
fn kv04_flush_faces_produce_flush_action() {
    let coincidence = Coincidence::coplanar_faces(3, 7);
    let action = CoincidenceGraph::merge_action(&coincidence);
    assert!(matches!(
        action,
        MergeAction::FlushFaces { face_a: 3, face_b: 7 }
    ));
}

#[test]
fn kv04_merged_cube_counts() {
    let mut g = CoincidenceGraph::new();

    for id in 0..16 {
        g.declare(id);
    }
    g.merge(4, 8);
    g.merge(5, 9);
    g.merge(6, 10);
    g.merge(7, 11);

    assert_eq!(g.cluster_count(), 12);

    assert!(g.same_cluster(4, 8));
    assert!(g.same_cluster(5, 9));
    assert!(g.same_cluster(6, 10));
    assert!(g.same_cluster(7, 11));
    assert!(!g.same_cluster(0, 12));
}

#[test]
fn kv04_deterministic_representatives() {
    let mut g = CoincidenceGraph::new();
    g.merge(4, 8);
    g.merge(5, 9);
    g.merge(6, 10);
    g.merge(7, 11);
    assert_eq!(g.representative(8).unwrap(), 4);
    assert_eq!(g.representative(9).unwrap(), 5);
    assert_eq!(g.representative(10).unwrap(), 6);
    assert_eq!(g.representative(11).unwrap(), 7);
}
