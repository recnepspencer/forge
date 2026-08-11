use super::{DirectedEdge, EdgeMatcher};

fn make_edge(id: u32, group: Option<u32>, origin: [f64; 3], dest: [f64; 3]) -> DirectedEdge {
    DirectedEdge {
        id,
        group,
        origin_index: None,
        dest_index: None,
        origin,
        dest,
    }
}

fn make_edge_with_indices(
    id: u32,
    group: Option<u32>,
    oi: u32,
    di: u32,
    origin: [f64; 3],
    dest: [f64; 3],
) -> DirectedEdge {
    DirectedEdge {
        id,
        group,
        origin_index: Some(oi),
        dest_index: Some(di),
        origin,
        dest,
    }
}

#[test]
fn exact_reverse_pair_matches() {
    let edges = vec![
        make_edge(0, Some(1), [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        make_edge(1, Some(2), [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
    ];
    let matcher = EdgeMatcher::new(edges, 1e-10);
    let matches = matcher.find_full_matches();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].edge_a, 0);
    assert_eq!(matches[0].edge_b, 1);
    assert!(matches[0].distance_sq < 1e-20);
}

#[test]
fn no_match_when_same_group() {
    let edges = vec![
        make_edge(0, Some(1), [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        make_edge(1, Some(1), [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
    ];
    let matcher = EdgeMatcher::new(edges, 1e-10);
    let matches = matcher.find_full_matches();
    assert_eq!(matches.len(), 0);
}

#[test]
fn no_match_beyond_tolerance() {
    let edges = vec![
        make_edge(0, Some(1), [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        make_edge(1, Some(2), [1.0, 0.0, 0.0], [0.0, 5.0, 0.0]),
    ];
    let matcher = EdgeMatcher::new(edges, 1e-10);
    let matches = matcher.find_full_matches();
    assert_eq!(matches.len(), 0);
}

#[test]
fn within_tolerance_matches() {
    let eps = 1e-8;
    let edges = vec![
        make_edge(0, Some(1), [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        make_edge(1, Some(2), [1.0 + eps, 0.0, 0.0], [0.0 - eps, 0.0, 0.0]),
    ];
    let matcher = EdgeMatcher::new(edges, 1e-6);
    let matches = matcher.find_full_matches();
    assert_eq!(matches.len(), 1);
}

#[test]
fn best_match_chosen_from_multiple_candidates() {
    let edges = vec![
        make_edge(0, Some(1), [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        make_edge(1, Some(2), [1.0, 0.0, 0.0], [0.1, 0.0, 0.0]),
        make_edge(2, Some(3), [1.0, 0.0, 0.0], [0.001, 0.0, 0.0]),
    ];
    let matcher = EdgeMatcher::new(edges, 1.0);
    let matches = matcher.find_full_matches();
    assert_eq!(matches.len(), 1);
    assert_eq!(matches[0].edge_b, 2);
}

#[test]
fn single_vertex_match_origin_shared() {
    let edges = vec![
        make_edge_with_indices(0, Some(1), 10, 20, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        make_edge_with_indices(1, Some(2), 30, 10, [1.0 + 1e-9, 0.0, 0.0], [0.0, 0.0, 0.0]),
    ];
    let matcher = EdgeMatcher::new(edges, 1e-6);
    let matches = matcher.find_single_vertex_matches();
    assert_eq!(matches.len(), 1);
}

#[test]
fn single_vertex_no_match_when_neither_index_shared() {
    let edges = vec![
        make_edge_with_indices(0, Some(1), 10, 20, [0.0, 0.0, 0.0], [1.0, 0.0, 0.0]),
        make_edge_with_indices(1, Some(2), 30, 40, [1.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
    ];
    let matcher = EdgeMatcher::new(edges, 1e-6);
    let matches = matcher.find_single_vertex_matches();
    assert_eq!(matches.len(), 0);
}
