//! Vertex proximity and coincidence queries.
//!
//! DOMAIN: Spatial proximity detection for vertex deduplication during
//! mesh construction. Bridges geometry (point distances) and topology
//! (vertex handles).
//!
//! Current implementation: O(n) linear scan per query. When spatial
//! indexing lands (roadmap), the implementation swaps to grid/KD-tree
//! lookup without changing the public API.
//!
//! DEPENDENCIES: forge-topo (VertexId), forge-geom (is_same_point_within)
//! CONSUMERS: forge-kernel (primitives/eval.rs)

use forge_geom::primitives::point::is_same_point_within;
use forge_topo::handles::VertexId;

/// Result of a vertex proximity query.
///
/// Contains both the coincident match (if any) and the distance to the
/// nearest vertex in the set. `nearest_distance` is always computed as
/// a byproduct of the tolerance scan — zero additional work.
#[derive(Debug, Clone)]
pub struct ProximityResult {
    /// The matching vertex and its distance, if within tolerance.
    pub coincident: Option<(VertexId, f64)>,
    /// Distance to the nearest vertex regardless of tolerance.
    /// `f64::INFINITY` if the inserted set is empty.
    pub nearest_distance: f64,
}

/// Find an existing vertex within `tolerance` of `pos`.
///
/// Scans the `inserted` set and returns:
/// - `coincident`: the first vertex within tolerance (L∞ check via
///   `is_same_point_within`, L2 distance for the match)
/// - `nearest_distance`: L2 distance to the nearest vertex overall
///   (computed as byproduct of the scan, zero additional cost)
///
/// # Performance
///
/// Current: O(n) per call. For convex primitives (8–20 vertices)
/// this is negligible. For Parasolid-scale meshes (thousands of vertices),
/// this function's implementation will be replaced with a spatial index
/// (grid hash or KD-tree) — the signature remains stable.
pub fn find_coincident_vertex(
    inserted: &[(VertexId, [f64; 3])],
    pos: &[f64; 3],
    tolerance: f64,
) -> ProximityResult {
    let mut nearest_distance = f64::INFINITY;
    let mut coincident: Option<(VertexId, f64)> = None;

    for (vid, existing_pos) in inserted {
        let dx = pos[0] - existing_pos[0];
        let dy = pos[1] - existing_pos[1];
        let dz = pos[2] - existing_pos[2];
        let dist = (dx * dx + dy * dy + dz * dz).sqrt();

        if dist < nearest_distance {
            nearest_distance = dist;
        }

        // Only record the first coincident match (dedup uses first-wins).
        if coincident.is_none() && is_same_point_within(pos, existing_pos, tolerance) {
            coincident = Some((*vid, dist));
        }
    }

    ProximityResult {
        coincident,
        nearest_distance,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_set_returns_infinity() {
        let result = find_coincident_vertex(&[], &[0.0, 0.0, 0.0], 1e-6);
        assert!(result.coincident.is_none());
        assert!(result.nearest_distance.is_infinite());
    }

    #[test]
    fn exact_match_returns_coincident() {
        let v = VertexId::new(0, 0);
        let inserted = vec![(v, [1.0, 2.0, 3.0])];
        let result = find_coincident_vertex(&inserted, &[1.0, 2.0, 3.0], 1e-6);
        assert!(result.coincident.is_some());
        let (vid, dist) = result.coincident.unwrap();
        assert_eq!(vid, v);
        assert!(dist < 1e-12);
        assert!(result.nearest_distance < 1e-12);
    }

    #[test]
    fn close_but_outside_tolerance_returns_none() {
        let v = VertexId::new(0, 0);
        let inserted = vec![(v, [0.0, 0.0, 0.0])];
        // Distance = 0.01, tolerance = 0.001 → no match
        let result = find_coincident_vertex(&inserted, &[0.01, 0.0, 0.0], 0.001);
        assert!(result.coincident.is_none());
        assert!((result.nearest_distance - 0.01).abs() < 1e-12);
    }

    #[test]
    fn nearest_distance_tracks_closest_vertex() {
        let v0 = VertexId::new(0, 0);
        let v1 = VertexId::new(1, 0);
        let v2 = VertexId::new(2, 0);
        let inserted = vec![
            (v0, [10.0, 0.0, 0.0]),
            (v1, [5.0, 0.0, 0.0]),
            (v2, [20.0, 0.0, 0.0]),
        ];
        // Query at origin — nearest is v1 at distance 5.0
        let result = find_coincident_vertex(&inserted, &[0.0, 0.0, 0.0], 1e-6);
        assert!(result.coincident.is_none());
        assert!((result.nearest_distance - 5.0).abs() < 1e-12);
    }
}
