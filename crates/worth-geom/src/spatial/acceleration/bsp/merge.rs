//! BSP Tree Merge Algorithm (Bernstein/Naylor).
//!
//! DOMAIN: Boolean operations on BSP-represented solids via recursive
//! tree merge. All decisions use exact rational plane coefficients.
//!
//! INVARIANTS:
//! - No f64 comparisons drive structural decisions
//! - Vertices are never materialized â€” only plane indices
//! - merge(A, B, op) produces a valid BSP tree for A op B
//! - result.simplify() collapses redundant nodes
//!
//! DEPENDENCIES: `BspNode`, `BspSolid`, `BspOp`, `Plane` (from schema)

use worth_math::arithmetic::rational::Rational;
use worth_math::sign::TriSign;
use worth_math::MathError;

use super::schema::{BspNode, BspOp, BspSolid};
use crate::Plane;

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
        BspNode::Leaf { solid } => Ok(select_leaf(op, b, *solid, true)),
        BspNode::Internal {
            plane_idx,
            neg,
            pos,
        } => {
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
/// or the second (B). This matters for subtraction (A \ B â‰  B \ A).
///
/// Truth table for a_is_first = true (leaf is from A):
///   Union:        SOLID â†’ solid leaf,  EMPTY â†’ return tree
///   Intersection: SOLID â†’ return tree, EMPTY â†’ empty leaf
///   Subtraction:  SOLID â†’ complement,  EMPTY â†’ empty leaf
///
/// For a_is_first = false (leaf is from B):
///   Union:        SOLID â†’ solid leaf,  EMPTY â†’ return tree
///   Intersection: SOLID â†’ return tree, EMPTY â†’ empty leaf
///   Subtraction:  SOLID â†’ empty leaf,  EMPTY â†’ return tree
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
/// Returns `(neg_half, pos_half)` â€” two trees representing the portions
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
        BspNode::Leaf { .. } => Ok((node.clone(), node.clone())),
        BspNode::Internal {
            plane_idx,
            neg,
            pos,
        } => {
            if *plane_idx == split_plane_idx {
                return Ok((*neg.clone(), *pos.clone()));
            }

            let relation = classify_plane_pair(planes, *plane_idx, split_plane_idx)?;

            match relation {
                PlaneRelation::AllNegative
                | PlaneRelation::AllPositive
                | PlaneRelation::Spanning => {
                    let (neg_neg, neg_pos) = partition_node(neg, split_plane_idx, planes)?;
                    let (pos_neg, pos_pos) = partition_node(pos, split_plane_idx, planes)?;

                    let neg_half = BspNode::split(*plane_idx, neg_neg, pos_neg);
                    let pos_half = BspNode::split(*plane_idx, neg_pos, pos_pos);

                    Ok((neg_half, pos_half))
                }
                PlaneRelation::Coplanar => Ok((*neg.clone(), *pos.clone())),
                PlaneRelation::AntiCoplanar => Ok((*pos.clone(), *neg.clone())),
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
    let node_coefficients = acquire_exact_coefficients(&planes[node_plane_idx]);
    let split_coefficients = acquire_exact_coefficients(&planes[split_plane_idx]);
    if let Some(relation) = classify_parallel_relation(&node_coefficients, &split_coefficients)? {
        return Ok(relation);
    }
    Ok(PlaneRelation::Spanning)
}

#[derive(Clone)]
struct ExactPlaneCoefficients {
    a: Rational,
    b: Rational,
    c: Rational,
    d: Rational,
}

fn acquire_exact_coefficients(plane: &Plane) -> ExactPlaneCoefficients {
    let (a, b, c, d) = plane.exact_coefficients();
    ExactPlaneCoefficients {
        a: a.clone(),
        b: b.clone(),
        c: c.clone(),
        d: d.clone(),
    }
}

fn classify_parallel_relation(
    node: &ExactPlaneCoefficients,
    split: &ExactPlaneCoefficients,
) -> Result<Option<PlaneRelation>, MathError> {
    let cross_x = &node.b * &split.c - &node.c * &split.b;
    let cross_y = &node.c * &split.a - &node.a * &split.c;
    let cross_z = &node.a * &split.b - &node.b * &split.a;
    let cross_is_zero = cross_x.is_zero() && cross_y.is_zero() && cross_z.is_zero();

    if cross_is_zero {
        let dot = &node.a * &split.a + &node.b * &split.b + &node.c * &split.c;
        match dot.sign() {
            TriSign::Pos => {
                return Ok(Some(classify_same_direction_parallel(node, split)));
            }
            TriSign::Neg => {
                return Ok(Some(classify_opposite_direction_parallel(node, split)));
            }
            TriSign::Zero => {
                return Err(MathError::InvalidInput(
                    "Both plane normals are zero".to_string(),
                ));
            }
        }
    }
    Ok(None)
}

fn classify_same_direction_parallel(
    node: &ExactPlaneCoefficients,
    split: &ExactPlaneCoefficients,
) -> PlaneRelation {
    let scale_a = &node.a * &split.d - &split.a * &node.d;
    let scale_b = &node.b * &split.d - &split.b * &node.d;
    let scale_c = &node.c * &split.d - &split.c * &node.d;
    if scale_a.is_zero() && scale_b.is_zero() && scale_c.is_zero() {
        return PlaneRelation::Coplanar;
    }
    let representative = if !scale_a.is_zero() {
        scale_a
    } else if !scale_b.is_zero() {
        scale_b
    } else {
        scale_c
    };
    match representative.sign() {
        TriSign::Pos => PlaneRelation::AllPositive,
        TriSign::Neg => PlaneRelation::AllNegative,
        TriSign::Zero => unreachable!(),
    }
}

fn classify_opposite_direction_parallel(
    node: &ExactPlaneCoefficients,
    split: &ExactPlaneCoefficients,
) -> PlaneRelation {
    let scale_a = &node.a * &split.d + &split.a * &node.d;
    let scale_b = &node.b * &split.d + &split.b * &node.d;
    let scale_c = &node.c * &split.d + &split.c * &node.d;
    if scale_a.is_zero() && scale_b.is_zero() && scale_c.is_zero() {
        return PlaneRelation::AntiCoplanar;
    }
    let representative = if !scale_a.is_zero() {
        scale_a
    } else if !scale_b.is_zero() {
        scale_b
    } else {
        scale_c
    };
    match representative.sign() {
        TriSign::Pos => PlaneRelation::AllPositive,
        TriSign::Neg => PlaneRelation::AllNegative,
        TriSign::Zero => unreachable!(),
    }
}

/// Remap all plane indices in a tree by adding an offset.
///
/// Used when merging two plane sets: the second solid's indices
/// need to be shifted by the length of the first solid's plane set.
fn remap_indices(node: &BspNode, offset: usize) -> BspNode {
    match node {
        BspNode::Leaf { solid } => BspNode::Leaf { solid: *solid },
        BspNode::Internal {
            plane_idx,
            neg,
            pos,
        } => BspNode::Internal {
            plane_idx: plane_idx + offset,
            neg: Box::new(remap_indices(neg, offset)),
            pos: Box::new(remap_indices(pos, offset)),
        },
    }
}
#[cfg(test)]
mod tests;
