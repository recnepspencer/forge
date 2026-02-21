//! Exact pseudo-angular sorting (Anti-Trig Sort).
//!
//! DOMAIN: Sort edge vectors radially around a vertex without trigonometry.
//! Uses only dot products, cross products, and sign comparisons — safe
//! for exact rational arithmetic and deterministic across platforms.
//!
//! ALGORITHM:
//!   1. Project each edge vector to local 2D via dot/cross with reference
//!   2. Assign quadrant (0–3) from signs of local X and Y
//!   3. Sort: by quadrant first, then by cross-product determinant within quadrant
//!
//! DEPENDENCIES: `forge_math::linalg` (dot, cross)

use forge_math::linalg;

/// Radial projection of an edge vector into local 2D coordinates.
struct RadialKey {
    /// Local X coordinate: `dot(edge, reference)`
    x: f64,
    /// Local Y coordinate: `dot(cross(reference, edge), normal)`
    y: f64,
    /// Quadrant (0–3) derived from signs of x and y
    quadrant: u8,
    /// Original index in the input slice
    index: usize,
}

/// Sort edge vectors radially (counter-clockwise) around a vertex.
///
/// Given a set of outgoing edge vectors at a vertex, a face normal,
/// and an arbitrary reference direction (typically the first edge),
/// returns a permutation of indices sorted in CCW radial order.
///
/// Uses only multiplication, addition, and sign checks — no `atan2`,
/// no `sqrt`, no `f64` comparisons for topology. Safe for exact
/// rational math or `i128` integers.
///
/// # Parameters
/// - `edge_vectors`: outgoing 3D edge direction vectors
/// - `normal`: face normal (defines the "up" direction for CCW)
/// - `reference`: reference direction in the face plane (quadrant 0 starts here)
///
/// # Returns
/// Permutation indices sorted in CCW order. Empty input → empty output.
pub fn sort_edges_radially(
    edge_vectors: &[[f64; 3]],
    normal: [f64; 3],
    reference: [f64; 3],
) -> Vec<usize> {
    let mut keys: Vec<RadialKey> = edge_vectors
        .iter()
        .enumerate()
        .map(|(i, v)| compute_radial_key(*v, normal, reference, i))
        .collect();

    keys.sort_by(compare_radial_keys);
    keys.iter().map(|k| k.index).collect()
}

/// Compute the radial key for a single edge vector.
fn compute_radial_key(
    edge: [f64; 3],
    normal: [f64; 3],
    reference: [f64; 3],
    index: usize,
) -> RadialKey {
    let x = linalg::dot(edge, reference);
    let ref_cross_edge = linalg::cross(reference, edge);
    let y = linalg::dot(ref_cross_edge, normal);
    let quadrant = assign_quadrant(x, y);
    RadialKey { x, y, quadrant, index }
}

/// Assign a quadrant (0–3) from the signs of local X and Y.
///
/// Quadrant 0: X ≥ 0 and Y ≥ 0 (reference direction, CCW up to 90°)
/// Quadrant 1: X < 0 and Y ≥ 0 (90° to 180°)
/// Quadrant 2: X < 0 and Y < 0 (180° to 270°)
/// Quadrant 3: X ≥ 0 and Y < 0 (270° to 360°)
fn assign_quadrant(x: f64, y: f64) -> u8 {
    let x_neg = x < 0.0;
    let y_neg = y < 0.0;
    match (x_neg, y_neg) {
        (false, false) => 0,
        (true, false) => 1,
        (true, true) => 2,
        (false, true) => 3,
    }
}

/// Compare two radial keys for CCW ordering.
///
/// 1. Different quadrants → smaller quadrant first
/// 2. Same quadrant → use 2D cross-product determinant:
///    `D = x_a * y_b - x_b * y_a`
///    If D > 0, `a` is before `b` (a is more CCW).
///    If D < 0, `b` is before `a`.
///    If D == 0, break ties by original index for stability.
fn compare_radial_keys(a: &RadialKey, b: &RadialKey) -> std::cmp::Ordering {
    match a.quadrant.cmp(&b.quadrant) {
        std::cmp::Ordering::Equal => {
            let det = a.x * b.y - b.x * a.y;
            if det > 0.0 {
                std::cmp::Ordering::Less
            } else if det < 0.0 {
                std::cmp::Ordering::Greater
            } else {
                a.index.cmp(&b.index)
            }
        }
        ord => ord,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_four_axis_aligned_vectors() {
        let normal = [0.0, 0.0, 1.0];
        let reference = [1.0, 0.0, 0.0];
        let edges = [
            [0.0, -1.0, 0.0],
            [-1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [1.0, 0.0, 0.0],
        ];
        let order = sort_edges_radially(&edges, normal, reference);
        assert_eq!(order, vec![3, 2, 1, 0]);
    }

    #[test]
    fn sort_single_vector_is_identity() {
        let normal = [0.0, 0.0, 1.0];
        let reference = [1.0, 0.0, 0.0];
        let edges = [[0.5, 0.5, 0.0]];
        let order = sort_edges_radially(&edges, normal, reference);
        assert_eq!(order, vec![0]);
    }

    #[test]
    fn sort_empty_input() {
        let normal = [0.0, 0.0, 1.0];
        let reference = [1.0, 0.0, 0.0];
        let edges: [[f64; 3]; 0] = [];
        let order = sort_edges_radially(&edges, normal, reference);
        assert!(order.is_empty());
    }

    #[test]
    fn sort_collinear_same_quadrant_stable() {
        let normal = [0.0, 0.0, 1.0];
        let reference = [1.0, 0.0, 0.0];
        let edges = [
            [2.0, 1.0, 0.0],
            [4.0, 2.0, 0.0],
        ];
        let order = sort_edges_radially(&edges, normal, reference);
        assert_eq!(order, vec![0, 1]);
    }

    #[test]
    fn sort_eight_directions() {
        let normal = [0.0, 0.0, 1.0];
        let reference = [1.0, 0.0, 0.0];
        let edges = [
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [-1.0, 1.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [1.0, -1.0, 0.0],
        ];
        let order = sort_edges_radially(&edges, normal, reference);
        assert_eq!(order, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn sort_non_axis_aligned_normal() {
        let normal = [0.0, 1.0, 0.0];
        let reference = [1.0, 0.0, 0.0];
        let edges = [
            [0.0, 0.0, -1.0],
            [-1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
        ];
        let order = sort_edges_radially(&edges, normal, reference);
        assert_eq!(order, vec![3, 0, 1, 2]);
    }
}
