//! Spatial-hashed reverse-edge proximity matching.
//!
//! DOMAIN: Given a set of directed edges (id, origin, dest), find pairs of
//! edges that face opposite directions and share endpoints within a tolerance.
//! Uses a quantized spatial grid for O(n) average-case lookup instead of O(n²).
//!
//! DEPENDENCIES: `worth_math::linalg` for vector operations.
//! INVARIANTS: Deterministic ordering — ties broken by edge index.

use worth_math::linalg::{norm_sq, sub};
use std::collections::BTreeMap;

/// A directed edge with an ID, origin position, and destination position.
#[derive(Clone, Debug)]
pub struct DirectedEdge {
    /// Caller-assigned ID (typically a half-edge index).
    pub id: u32,
    /// An associated group (typically a face index) — edges in the same group won't match.
    pub group: Option<u32>,
    /// Optional origin vertex ID for index-based matching.
    pub origin_index: Option<u32>,
    /// Optional destination vertex ID for index-based matching.
    pub dest_index: Option<u32>,
    /// Origin position in 3D space.
    pub origin: [f64; 3],
    /// Destination position in 3D space.
    pub dest: [f64; 3],
}

/// A matched pair of reverse-oriented edges.
#[derive(Clone, Debug)]
pub struct EdgeMatch {
    /// ID of the first edge.
    pub edge_a: u32,
    /// ID of the matched reverse edge.
    pub edge_b: u32,
    /// Sum of squared distances between paired endpoints.
    pub distance_sq: f64,
}

/// Match mode for fuzzy edge matching.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuzzyMatchMode {
    /// Both endpoints must match by position within tolerance.
    FullEndpoint,
    /// One endpoint matches by vertex index, the other by position.
    SingleVertex,
}

/// Spatial-hashed edge matcher for O(n) average reverse-edge pairing.
///
/// Builds a grid over the destination endpoints. For each query edge
/// (origin→dest), looks up grid cells near its REVERSED endpoints
/// (query origin ≈ candidate dest, query dest ≈ candidate origin).
pub struct EdgeMatcher {
    cell_size: f64,
    inv_cell_size: f64,
    tolerance_sq: f64,
    dest_grid: BTreeMap<(i64, i64, i64), Vec<usize>>,
    edges: Vec<DirectedEdge>,
}

impl EdgeMatcher {
    /// Build a matcher from a set of directed edges.
    ///
    /// `tolerance_sq` is the maximum squared distance between paired endpoints.
    /// Cell size is derived from the tolerance for optimal bucket density.
    pub fn new(edges: Vec<DirectedEdge>, tolerance_sq: f64) -> Self {
        let cell_size = tolerance_sq.sqrt().max(1e-15);
        let inv_cell_size = 1.0 / cell_size;

        let mut dest_grid: BTreeMap<(i64, i64, i64), Vec<usize>> = BTreeMap::new();
        for (idx, edge) in edges.iter().enumerate() {
            let key = quantize(edge.dest, inv_cell_size);
            dest_grid.entry(key).or_default().push(idx);
        }

        Self {
            cell_size,
            inv_cell_size,
            tolerance_sq,
            dest_grid,
            edges,
        }
    }

    /// Find all reverse-oriented edge pairs within tolerance.
    ///
    /// For each unpaired edge A with endpoints (oA, dA), searches for an
    /// unpaired edge B whose endpoints satisfy:
    ///   `norm_sq(oA - dB) + norm_sq(dA - oB) ≤ tolerance_sq`
    ///
    /// Edges in the same group (same face) are excluded.
    /// Each edge is matched at most once. Ties are broken by lowest distance,
    /// then by lowest edge index for determinism.
    pub fn find_full_matches(&self) -> Vec<EdgeMatch> {
        let mut paired = std::collections::BTreeSet::<u32>::new();
        let mut matches = Vec::new();

        for (idx_a, edge_a) in self.edges.iter().enumerate() {
            if paired.contains(&edge_a.id) {
                // already matched — skip without processing
            } else {
                let best = self.find_best_reverse(idx_a, &paired, MatchMode::FullEndpoint);
                if let Some(m) = best {
                    paired.insert(m.edge_a);
                    paired.insert(m.edge_b);
                    matches.push(m);
                }
            }
        }

        matches
    }

    /// Find matches where one vertex matches by index and the other by position.
    ///
    /// For edge A (origA→destA) and candidate B (origB→destB):
    /// - If origA.index == destB.index, match destA↔origB by position
    /// - If destA.index == origB.index, match origA↔destB by position
    pub fn find_single_vertex_matches(&self) -> Vec<EdgeMatch> {
        let mut paired = std::collections::BTreeSet::<u32>::new();
        let mut matches = Vec::new();

        for (idx_a, edge_a) in self.edges.iter().enumerate() {
            if paired.contains(&edge_a.id) {
                // already matched — skip
            } else {
                let best = self.find_best_reverse(idx_a, &paired, MatchMode::SingleVertex);
                if let Some(m) = best {
                    paired.insert(m.edge_a);
                    paired.insert(m.edge_b);
                    matches.push(m);
                }
            }
        }

        matches
    }

    /// Core spatial lookup: find the best reverse-oriented match for edge at `idx_a`.
    fn find_best_reverse(
        &self,
        idx_a: usize,
        paired: &std::collections::BTreeSet<u32>,
        mode: MatchMode,
    ) -> Option<EdgeMatch> {
        let edge_a = &self.edges[idx_a];
        let query_origin = edge_a.origin;

        let center_cell = quantize(query_origin, self.inv_cell_size);
        let mut best: Option<EdgeMatch> = None;

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let cell = (center_cell.0 + dx, center_cell.1 + dy, center_cell.2 + dz);
                    if let Some(bucket) = self.dest_grid.get(&cell) {
                        for &idx_b in bucket {
                            if idx_a == idx_b {
                                // same edge — skip
                            } else {
                                let edge_b = &self.edges[idx_b];
                                if paired.contains(&edge_b.id) {
                                    // already paired — skip
                                } else if edge_a.group.is_some() && edge_a.group == edge_b.group {
                                    // same group (face) — skip
                                } else {
                                    let candidate = match mode {
                                        MatchMode::FullEndpoint => {
                                            compute_full_distance(edge_a, edge_b, self.tolerance_sq)
                                        }
                                        MatchMode::SingleVertex => compute_single_vertex_distance(
                                            edge_a,
                                            edge_b,
                                            self.tolerance_sq,
                                        ),
                                    };

                                    if let Some(dist_sq) = candidate {
                                        let is_better = match &best {
                                            None => true,
                                            Some(prev) => {
                                                dist_sq < prev.distance_sq
                                                    || (dist_sq == prev.distance_sq
                                                        && edge_b.id < prev.edge_b)
                                            }
                                        };
                                        if is_better {
                                            best = Some(EdgeMatch {
                                                edge_a: edge_a.id,
                                                edge_b: edge_b.id,
                                                distance_sq: dist_sq,
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        best
    }
}

/// Match mode for the spatial lookup.
enum MatchMode {
    /// Both endpoints must match by position within tolerance.
    FullEndpoint,
    /// One endpoint matches by vertex index, the other by position.
    SingleVertex,
}

/// Fuzzy-match a set of directed edges with deterministic reverse-edge pairing.
///
/// This is the high-level geometry API used by kernel stitch fallbacks.
pub fn fuzzy_match_edges(
    edges: Vec<DirectedEdge>,
    tolerance_sq: f64,
    mode: FuzzyMatchMode,
) -> Vec<EdgeMatch> {
    let matcher = EdgeMatcher::new(edges, tolerance_sq);
    match mode {
        FuzzyMatchMode::FullEndpoint => matcher.find_full_matches(),
        FuzzyMatchMode::SingleVertex => matcher.find_single_vertex_matches(),
    }
}

/// Compute full reverse-endpoint distance (pass 3 style).
///
/// Returns `Some(total_dist_sq)` if `norm_sq(oA - dB) + norm_sq(dA - oB) ≤ tol`.
fn compute_full_distance(a: &DirectedEdge, b: &DirectedEdge, tol_sq: f64) -> Option<f64> {
    let d_od = norm_sq(sub(a.origin, b.dest));
    if d_od > tol_sq {
        return None;
    }
    let d_do = norm_sq(sub(a.dest, b.origin));
    if d_do > tol_sq {
        return None;
    }
    let total = d_od + d_do;
    if total <= tol_sq {
        Some(total)
    } else {
        None
    }
}

/// Compute single-vertex-match distance (pass 4 style).
///
/// One endpoint must share a vertex index; the other must be within tolerance.
fn compute_single_vertex_distance(a: &DirectedEdge, b: &DirectedEdge, tol_sq: f64) -> Option<f64> {
    let origin_match = a.origin_index.is_some() && a.origin_index == b.dest_index;
    let dest_match = a.dest_index.is_some() && a.dest_index == b.origin_index;

    if origin_match && !dest_match {
        let dsq = norm_sq(sub(a.dest, b.origin));
        if dsq <= tol_sq {
            Some(dsq)
        } else {
            None
        }
    } else if !origin_match && dest_match {
        let dsq = norm_sq(sub(a.origin, b.dest));
        if dsq <= tol_sq {
            Some(dsq)
        } else {
            None
        }
    } else {
        None
    }
}

/// Quantize a 3D position to grid cell coordinates.
fn quantize(pos: [f64; 3], inv_cell_size: f64) -> (i64, i64, i64) {
    (
        (pos[0] * inv_cell_size).floor() as i64,
        (pos[1] * inv_cell_size).floor() as i64,
        (pos[2] * inv_cell_size).floor() as i64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

/// Select the best candidate edge around a radial junction by sorting normal dot products.
///
/// `candidates` is a list of `(candidate_id, face_normal)`. The candidate with the highest
/// dot product against `source_normal` is chosen. Ties are broken deterministically by ID.
pub fn select_best_radial_match(source_normal: [f64; 3], candidates: &[(u32, [f64; 3])]) -> u32 {
    let mut best_id = candidates[0].0;
    let mut best_dot = f64::NEG_INFINITY;

    for &(cand_id, cand_normal) in candidates {
        let dot = worth_math::linalg::dot(source_normal, cand_normal);
        if dot > best_dot || (dot == best_dot && cand_id < best_id) {
            best_dot = dot;
            best_id = cand_id;
        }
    }
    best_id
}
