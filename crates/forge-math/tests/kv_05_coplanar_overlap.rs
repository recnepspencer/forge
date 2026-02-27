//! KV-05: Coplanar overlap cases merge deterministically.
//!
//! Validates that the CoincidenceGraph handles various overlap scenarios
//! correctly and produces identical results across runs (D1 enforcement).

use forge_math::coincidence::{Coincidence, CoincidenceGraph};

#[test]
fn kv05_two_coplanar_faces_same_cluster() {
    let mut g = CoincidenceGraph::new();
    let c = Coincidence::coplanar_faces(1, 2);
    let (a, b) = c.ids();
    g.merge(a, b);
    assert!(g.same_cluster(1, 2));
    assert_eq!(g.representative(2).unwrap(), 1);
}

#[test]
fn kv05_three_overlapping_faces_single_cluster() {
    let mut g = CoincidenceGraph::new();
    g.merge(1, 2);
    g.merge(2, 3);
    assert!(g.same_cluster(1, 3));
    assert_eq!(g.representative(3).unwrap(), 1);
    assert_eq!(g.cluster_count(), 1);
}

#[test]
fn kv05_non_overlapping_faces_separate_clusters() {
    let mut g = CoincidenceGraph::new();
    g.merge(1, 2);
    g.merge(10, 20);
    assert!(!g.same_cluster(1, 10));
    assert_eq!(g.cluster_count(), 2);
}

#[test]
fn kv05_chain_merge_all_reach_smallest() {
    let mut g = CoincidenceGraph::new();
    g.merge(50, 40);
    g.merge(40, 30);
    g.merge(30, 20);
    g.merge(20, 10);

    assert_eq!(g.representative(50).unwrap(), 10);
    assert_eq!(g.representative(40).unwrap(), 10);
    assert_eq!(g.representative(30).unwrap(), 10);
    assert_eq!(g.representative(20).unwrap(), 10);
    assert_eq!(g.cluster_count(), 1);
}

#[test]
fn kv05_reverse_order_merge_same_result() {
    let mut g1 = CoincidenceGraph::new();
    g1.merge(1, 2);
    g1.merge(2, 3);
    g1.merge(3, 4);

    let mut g2 = CoincidenceGraph::new();
    g2.merge(4, 3);
    g2.merge(3, 2);
    g2.merge(2, 1);

    assert_eq!(g1.representative(4).unwrap(), g2.representative(4).unwrap());
    assert_eq!(g1.cluster_count(), g2.cluster_count());
}

#[test]
fn kv05_mixed_coincidence_types() {
    let mut g = CoincidenceGraph::new();

    let v = Coincidence::coincident_vertices(100, 200);
    let (a, b) = v.ids();
    g.merge(a, b);

    let e = Coincidence::collinear_edges(300, 400);
    let (a, b) = e.ids();
    g.merge(a, b);

    let f = Coincidence::coplanar_faces(500, 600);
    let (a, b) = f.ids();
    g.merge(a, b);

    assert_eq!(g.cluster_count(), 3);
    assert_eq!(g.representative(200).unwrap(), 100);
    assert_eq!(g.representative(400).unwrap(), 300);
    assert_eq!(g.representative(600).unwrap(), 500);
}

#[test]
fn kv05_deterministic_clusters() {
    let mut g = CoincidenceGraph::new();
    g.merge(5, 10);
    g.merge(10, 15);
    g.merge(20, 25);

    let clusters = g.clusters();
    assert_eq!(clusters.len(), 2);
    assert_eq!(clusters[0], (5, vec![5, 10, 15]));
    assert_eq!(clusters[1], (20, vec![20, 25]));
}

#[test]
fn kv05_large_cluster_merge() {
    let mut g = CoincidenceGraph::new();
    g.merge(0, 1);
    g.merge(0, 2);
    g.merge(0, 3);
    g.merge(0, 4);

    assert_eq!(g.representative(4).unwrap(), 0);
    assert_eq!(g.representative(1).unwrap(), 0);
}
