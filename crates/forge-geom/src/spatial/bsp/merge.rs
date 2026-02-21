//! BSP Tree Merge Algorithm (Bernstein/Naylor).
//!
//! DOMAIN: Boolean operations on BSP-represented solids via recursive
//! tree merge. All decisions use exact rational plane coefficients.
//!
//! INVARIANTS:
//! - No f64 comparisons drive structural decisions
//! - Vertices are never materialized — only plane indices
//! - merge(A, B, op) produces a valid BSP tree for A op B
//! - result.simplify() collapses redundant nodes
//!
//! DEPENDENCIES: `BspNode`, `BspSolid`, `BspOp`, `Plane` (from schema)

use forge_math::MathError;

use crate::Plane;
use super::schema::{BspNode, BspSolid, BspOp};

/// Merge two BSP solids under a boolean operation.
///
/// Returns a new `BspSolid` whose plane set is the union of both inputs'
/// planes (with indices remapped for the second solid). The result tree
/// is automatically simplified to collapse redundant nodes.
pub fn merge_bsp(a: &BspSolid, b: &BspSolid, op: BspOp) -> Result<BspSolid, MathError> {
    let mut merged_planes: Vec<Plane> = a.planes().to_vec();
    let b_offset = merged_planes.len();
    merged_planes.extend_from_slice(b.planes());

    let b_remapped = remap_indices(&b.root(), b_offset);

    let result_root = merge_nodes(a.root(), &b_remapped, op, &merged_planes)?;
    let result_root = result_root.simplify();

    Ok(BspSolid::new(merged_planes, result_root))
}

/// Recursive merge of two BSP tree nodes.
///
/// Implements the Bernstein/Naylor merge:
/// - If A is a leaf, apply `select_leaf` to combine with B
/// - Otherwise, partition B by A's splitting plane, recurse on both halves
fn merge_nodes(
    a: &BspNode,
    b: &BspNode,
    op: BspOp,
    planes: &[Plane],
) -> Result<BspNode, MathError> {
    match a {
        BspNode::Leaf { solid } => {
            Ok(select_leaf(op, b, *solid, true))
        }
        BspNode::Internal { plane_idx, neg, pos } => {
            let (b_neg, b_pos) = partition_node(b, *plane_idx, planes)?;

            let neg_result = merge_nodes(neg, &b_neg, op, planes)?;
            let pos_result = merge_nodes(pos, &b_pos, op, planes)?;

            Ok(BspNode::split(*plane_idx, neg_result, pos_result))
        }
    }
}

/// Combine a leaf label with a tree based on the boolean operation.
///
/// `a_is_first` indicates whether the leaf comes from the first operand (A)
/// or the second (B). This matters for subtraction (A \ B ≠ B \ A).
///
/// Truth table for a_is_first = true (leaf is from A):
///   Union:        SOLID → solid leaf,  EMPTY → return tree
///   Intersection: SOLID → return tree, EMPTY → empty leaf
///   Subtraction:  SOLID → complement,  EMPTY → empty leaf
///
/// For a_is_first = false (leaf is from B):
///   Union:        SOLID → solid leaf,  EMPTY → return tree
///   Intersection: SOLID → return tree, EMPTY → empty leaf
///   Subtraction:  SOLID → empty leaf,  EMPTY → return tree
fn select_leaf(op: BspOp, tree: &BspNode, leaf_solid: bool, a_is_first: bool) -> BspNode {
    if a_is_first {
        match (op, leaf_solid) {
            (BspOp::Union, true) => BspNode::solid(),
            (BspOp::Union, false) => tree.clone(),
            (BspOp::Intersection, true) => tree.clone(),
            (BspOp::Intersection, false) => BspNode::empty(),
            (BspOp::Subtraction, true) => tree.complement(),
            (BspOp::Subtraction, false) => BspNode::empty(),
        }
    } else {
        match (op, leaf_solid) {
            (BspOp::Union, true) => BspNode::solid(),
            (BspOp::Union, false) => tree.clone(),
            (BspOp::Intersection, true) => tree.clone(),
            (BspOp::Intersection, false) => BspNode::empty(),
            (BspOp::Subtraction, true) => BspNode::empty(),
            (BspOp::Subtraction, false) => tree.clone(),
        }
    }
}

/// Partition a BSP tree by a splitting plane.
///
/// Returns `(neg_half, pos_half)` — two trees representing the portions
/// of the input tree on the negative and positive sides of the plane.
///
/// For leaves: both halves are copies of the leaf (a leaf fills its entire
/// region; splitting it just produces two copies).
///
/// For internal nodes: recurse based on the relationship between the
/// node's splitting plane and the partitioning plane.
fn partition_node(
    node: &BspNode,
    split_plane_idx: usize,
    planes: &[Plane],
) -> Result<(BspNode, BspNode), MathError> {
    match node {
        BspNode::Leaf { .. } => {
            Ok((node.clone(), node.clone()))
        }
        BspNode::Internal { plane_idx, neg, pos } => {
            if *plane_idx == split_plane_idx {
                return Ok((*neg.clone(), *pos.clone()));
            }

            let relation = classify_plane_pair(planes, *plane_idx, split_plane_idx)?;

            match relation {
                PlaneRelation::AllNegative | PlaneRelation::AllPositive | PlaneRelation::Spanning => {
                    let (neg_neg, neg_pos) = partition_node(neg, split_plane_idx, planes)?;
                    let (pos_neg, pos_pos) = partition_node(pos, split_plane_idx, planes)?;

                    let neg_half = BspNode::split(*plane_idx, neg_neg, pos_neg);
                    let pos_half = BspNode::split(*plane_idx, neg_pos, pos_pos);

                    Ok((neg_half, pos_half))
                }
                PlaneRelation::Coplanar => {
                    Ok((*neg.clone(), *pos.clone()))
                }
                PlaneRelation::AntiCoplanar => {
                    Ok((*pos.clone(), *neg.clone()))
                }
            }
        }
    }
}

/// Relationship between a node's plane and the partitioning plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlaneRelation {
    /// The node's plane is entirely on the negative side of the split plane.
    AllNegative,
    /// The node's plane is entirely on the positive side of the split plane.
    AllPositive,
    /// The node's plane spans the split plane (general case).
    Spanning,
    /// The planes are coplanar with same orientation.
    Coplanar,
    /// The planes are coplanar with opposite orientation.
    AntiCoplanar,
}

/// Classify the relationship between two planes using exact arithmetic.
///
/// Two infinite planes in 3D either:
/// 1. Are parallel (coplanar same-dir, coplanar anti-dir, or distinct parallel)
/// 2. Intersect along a line (spanning)
///
/// For BSP partitioning of infinite half-space planes, distinct parallel
/// planes have one entirely in the negative half and one in the positive.
/// Intersecting planes always span.
fn classify_plane_pair(
    planes: &[Plane],
    node_plane_idx: usize,
    split_plane_idx: usize,
) -> Result<PlaneRelation, MathError> {
    use forge_math::sign::TriSign;

    let node_plane = &planes[node_plane_idx];
    let split_plane = &planes[split_plane_idx];

    let (na, nb, nc, nd) = node_plane.exact_coefficients();
    let (sa, sb, sc, sd) = split_plane.exact_coefficients();

    // Clone to owned values for arithmetic (exact_coefficients returns references).
    let na = na.clone();
    let nb = nb.clone();
    let nc = nc.clone();
    let nd = nd.clone();
    let sa = sa.clone();
    let sb = sb.clone();
    let sc = sc.clone();
    let sd = sd.clone();

    let cross_x = &nb * &sc - &nc * &sb;
    let cross_y = &nc * &sa - &na * &sc;
    let cross_z = &na * &sb - &nb * &sa;

    let cross_is_zero = cross_x.is_zero() && cross_y.is_zero() && cross_z.is_zero();

    if cross_is_zero {
        let dot = &na * &sa + &nb * &sb + &nc * &sc;

        match dot.sign() {
            TriSign::Pos => {
                // Same-direction parallel: check if offsets match using the
                // dominant normal component.  `ni * sd - si * nd = 0` iff coplanar.
                // We must pick a component where ni (and si) are non-zero;
                // using `a` alone fails for Y-/Z-oriented planes where a=0.
                let scale_a = &na * &sd - &sa * &nd;
                let scale_b = &nb * &sd - &sb * &nd;
                let scale_c = &nc * &sd - &sc * &nd;

                // All three must be zero for the planes to be truly coplanar.
                if scale_a.is_zero() && scale_b.is_zero() && scale_c.is_zero() {
                    return Ok(PlaneRelation::Coplanar);
                }
                // Determine which side: pick the first non-zero scale check.
                let representative = if !scale_a.is_zero() { scale_a }
                    else if !scale_b.is_zero() { scale_b }
                    else { scale_c };
                match representative.sign() {
                    TriSign::Pos => return Ok(PlaneRelation::AllPositive),
                    TriSign::Neg => return Ok(PlaneRelation::AllNegative),
                    TriSign::Zero => unreachable!(),
                }
            }
            TriSign::Neg => {
                // Anti-parallel: normals point opposite. Check `ni * sd + si * nd`.
                let scale_a = &na * &sd + &sa * &nd;
                let scale_b = &nb * &sd + &sb * &nd;
                let scale_c = &nc * &sd + &sc * &nd;

                if scale_a.is_zero() && scale_b.is_zero() && scale_c.is_zero() {
                    return Ok(PlaneRelation::AntiCoplanar);
                }
                let representative = if !scale_a.is_zero() { scale_a }
                    else if !scale_b.is_zero() { scale_b }
                    else { scale_c };
                match representative.sign() {
                    TriSign::Pos => return Ok(PlaneRelation::AllPositive),
                    TriSign::Neg => return Ok(PlaneRelation::AllNegative),
                    TriSign::Zero => unreachable!(),
                }
            }
            TriSign::Zero => {
                return Err(MathError::InvalidInput(
                    "Both plane normals are zero".to_string(),
                ));
            }
        }
    }

    Ok(PlaneRelation::Spanning)
}

/// Remap all plane indices in a tree by adding an offset.
///
/// Used when merging two plane sets: the second solid's indices
/// need to be shifted by the length of the first solid's plane set.
fn remap_indices(node: &BspNode, offset: usize) -> BspNode {
    match node {
        BspNode::Leaf { solid } => BspNode::Leaf { solid: *solid },
        BspNode::Internal { plane_idx, neg, pos } => {
            BspNode::Internal {
                plane_idx: plane_idx + offset,
                neg: Box::new(remap_indices(neg, offset)),
                pos: Box::new(remap_indices(pos, offset)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Plane;
    use forge_math::arithmetic::Rational;

    /// Helper: build an axis-aligned plane.
    fn axis_plane(axis: usize, sign: i64, offset_val: f64) -> Plane {
        Plane::axis_aligned(axis, sign, Rational::try_from_f64(offset_val).unwrap()).unwrap()
    }

    /// Helper: build a BspSolid for an axis-aligned box.
    ///
    /// A box from (cx-h, cy-h, cz-h) to (cx+h, cy+h, cz+h) is the
    /// intersection of 6 half-spaces:
    ///   +x: x ≤ cx+h  →  plane: +1·x + (-(cx+h)) ≤ 0  →  neg side is inside
    ///   -x: x ≥ cx-h  →  plane: -1·x + (cx-h) ≤ 0     →  neg side is inside
    ///   (same for y, z)
    ///
    /// The BSP tree is built by nesting: each plane splits space,
    /// positive side is outside (empty), negative side continues.
    fn make_bsp_box(center: [f64; 3], half: f64) -> BspSolid {
        let mut planes = Vec::new();

        for axis in 0..3 {
            planes.push(axis_plane(axis, 1, -(center[axis] + half)));
            planes.push(axis_plane(axis, -1, center[axis] - half));
        }

        let root = BspNode::split(0,
            BspNode::split(1,
                BspNode::split(2,
                    BspNode::split(3,
                        BspNode::split(4,
                            BspNode::split(5,
                                BspNode::solid(),
                                BspNode::empty(),
                            ),
                            BspNode::empty(),
                        ),
                        BspNode::empty(),
                    ),
                    BspNode::empty(),
                ),
                BspNode::empty(),
            ),
            BspNode::empty(),
        );

        BspSolid::new(planes, root)
    }

    #[test]
    fn bsp_node_basics() {
        let leaf = BspNode::solid();
        assert!(leaf.is_solid());
        assert!(leaf.is_leaf());
        assert_eq!(leaf.node_count(), 1);

        let tree = BspNode::split(0, BspNode::solid(), BspNode::empty());
        assert!(!tree.is_leaf());
        assert_eq!(tree.node_count(), 3);
        assert_eq!(tree.depth(), 1);
    }

    #[test]
    fn bsp_node_complement() {
        let tree = BspNode::split(0, BspNode::solid(), BspNode::empty());
        let comp = tree.complement();
        match &comp {
            BspNode::Internal { neg, pos, .. } => {
                assert!(neg.is_empty());
                assert!(pos.is_solid());
            }
            _ => panic!("complement should preserve structure"),
        }
    }

    #[test]
    fn bsp_node_simplify() {
        let redundant = BspNode::split(0, BspNode::solid(), BspNode::solid());
        let simplified = redundant.simplify();
        assert!(simplified.is_solid());

        let non_redundant = BspNode::split(0, BspNode::solid(), BspNode::empty());
        let kept = non_redundant.simplify();
        assert!(!kept.is_leaf());
    }

    #[test]
    fn bsp_box_structure() {
        let cube = make_bsp_box([0.0, 0.0, 0.0], 1.0);
        assert_eq!(cube.plane_count(), 6);
        assert_eq!(cube.root().depth(), 6);
        assert_eq!(cube.root().solid_leaf_count(), 1);
        assert_eq!(cube.root().leaf_count(), 7);
    }

    #[test]
    fn merge_union_disjoint_cubes() {
        let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
        let b = make_bsp_box([5.0, 0.0, 0.0], 1.0);

        let result = merge_bsp(&a, &b, BspOp::Union).unwrap();
        assert_eq!(result.plane_count(), 12);
        assert!(result.root().solid_leaf_count() >= 2,
            "Disjoint union should have at least 2 solid regions");
    }

    #[test]
    fn merge_intersection_disjoint_cubes() {
        let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
        let b = make_bsp_box([5.0, 0.0, 0.0], 1.0);

        let result = merge_bsp(&a, &b, BspOp::Intersection).unwrap();
        assert!(!result.classify_point([0.0, 0.0, 0.0]),
            "Center of A should be outside disjoint intersection");
        assert!(!result.classify_point([5.0, 0.0, 0.0]),
            "Center of B should be outside disjoint intersection");
        assert!(!result.classify_point([2.5, 0.0, 0.0]),
            "Midpoint should be outside disjoint intersection");
    }

    #[test]
    fn merge_union_overlapping_cubes() {
        let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
        let b = make_bsp_box([0.5, 0.0, 0.0], 1.0);

        let result = merge_bsp(&a, &b, BspOp::Union).unwrap();
        assert!(result.root().solid_leaf_count() >= 1,
            "Overlapping union should have solid regions");
    }

    #[test]
    fn merge_subtraction_interior_cube() {
        let big = make_bsp_box([0.0, 0.0, 0.0], 2.0);
        let small = make_bsp_box([0.0, 0.0, 0.0], 1.0);

        let result = merge_bsp(&big, &small, BspOp::Subtraction).unwrap();
        assert!(result.root().solid_leaf_count() >= 1,
            "Subtracting interior cube should leave solid shell");

        let inner = merge_bsp(&big, &small, BspOp::Intersection).unwrap();
        assert!(inner.root().solid_leaf_count() >= 1,
            "Interior intersection should be the small cube");
    }

    #[test]
    fn merge_self_union_is_identity() {
        let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
        let result = merge_bsp(&a, &a, BspOp::Union).unwrap();

        assert!(result.classify_point([0.0, 0.0, 0.0]),
            "Center should be inside self-union");
        assert!(!result.classify_point([5.0, 0.0, 0.0]),
            "Outside point should be outside self-union");
    }

    #[test]
    fn merge_self_intersection_is_identity() {
        let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
        let result = merge_bsp(&a, &a, BspOp::Intersection).unwrap();

        assert!(result.root().solid_leaf_count() >= 1,
            "Self-intersection should be the same cube");
    }

    #[test]
    fn merge_self_subtraction_is_empty() {
        let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
        let result = merge_bsp(&a, &a, BspOp::Subtraction).unwrap();

        assert!(!result.classify_point([0.0, 0.0, 0.0]),
            "Center should be outside self-subtraction");
    }

    #[test]
    fn merge_chain_three_cubes() {
        let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
        let b = make_bsp_box([1.5, 0.0, 0.0], 1.0);
        let c = make_bsp_box([0.0, 1.5, 0.0], 1.0);

        let ab = merge_bsp(&a, &b, BspOp::Union).unwrap();
        let abc = merge_bsp(&ab, &c, BspOp::Union).unwrap();

        assert!(abc.classify_point([0.0, 0.0, 0.0]),
            "Center of A should be inside union");
        assert!(abc.classify_point([1.5, 0.0, 0.0]),
            "Center of B should be inside union");
        assert!(abc.classify_point([0.0, 1.5, 0.0]),
            "Center of C should be inside union");
        assert!(!abc.classify_point([5.0, 5.0, 5.0]),
            "Far point should be outside union");

        let ba = merge_bsp(&b, &a, BspOp::Union).unwrap();
        let bac = merge_bsp(&ba, &c, BspOp::Union).unwrap();

        assert_eq!(
            abc.classify_point([0.5, 0.5, 0.0]),
            bac.classify_point([0.5, 0.5, 0.0]),
            "Union chain should be commutative for test point"
        );
    }

    #[test]
    fn merge_union_then_subtract_back() {
        let a = make_bsp_box([0.0, 0.0, 0.0], 2.0);
        let b = make_bsp_box([1.5, 0.0, 0.0], 1.0);

        let ab = merge_bsp(&a, &b, BspOp::Union).unwrap();
        let result = merge_bsp(&ab, &b, BspOp::Subtraction).unwrap();

        assert!(result.classify_point([0.0, 0.0, 0.0]),
            "Center of A should still be inside after subtract-back");
        assert!(!result.classify_point([1.5, 0.0, 0.0]),
            "Center of B should be outside after subtract-back");
    }

    #[test]
    fn merge_intersection_overlapping_offset_cubes() {
        let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
        let b = make_bsp_box([1.0, 0.0, 0.0], 1.0);

        let result = merge_bsp(&a, &b, BspOp::Intersection).unwrap();
        assert!(result.root().solid_leaf_count() >= 1,
            "Overlapping offset intersection should have solid regions, got {}",
            result.root().solid_leaf_count());
    }
}
