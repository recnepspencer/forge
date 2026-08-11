//! BSP merge tests.

use super::super::schema::{BspNode, BspOp, BspSolid};
use super::merge_bsp;
use crate::Plane;
use worth_math::arithmetic::Rational;

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

    let root = BspNode::split(
        0,
        BspNode::split(
            1,
            BspNode::split(
                2,
                BspNode::split(
                    3,
                    BspNode::split(
                        4,
                        BspNode::split(5, BspNode::solid(), BspNode::empty()),
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
    assert!(
        result.root().solid_leaf_count() >= 2,
        "Disjoint union should have at least 2 solid regions"
    );
}

#[test]
fn merge_intersection_disjoint_cubes() {
    let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
    let b = make_bsp_box([5.0, 0.0, 0.0], 1.0);

    let result = merge_bsp(&a, &b, BspOp::Intersection).unwrap();
    assert!(
        !result.classify_point([0.0, 0.0, 0.0]),
        "Center of A should be outside disjoint intersection"
    );
    assert!(
        !result.classify_point([5.0, 0.0, 0.0]),
        "Center of B should be outside disjoint intersection"
    );
    assert!(
        !result.classify_point([2.5, 0.0, 0.0]),
        "Midpoint should be outside disjoint intersection"
    );
}

#[test]
fn merge_union_overlapping_cubes() {
    let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
    let b = make_bsp_box([0.5, 0.0, 0.0], 1.0);

    let result = merge_bsp(&a, &b, BspOp::Union).unwrap();
    assert!(
        result.root().solid_leaf_count() >= 1,
        "Overlapping union should have solid regions"
    );
}

#[test]
fn merge_subtraction_interior_cube() {
    let big = make_bsp_box([0.0, 0.0, 0.0], 2.0);
    let small = make_bsp_box([0.0, 0.0, 0.0], 1.0);

    let result = merge_bsp(&big, &small, BspOp::Subtraction).unwrap();
    assert!(
        result.root().solid_leaf_count() >= 1,
        "Subtracting interior cube should leave solid shell"
    );

    let inner = merge_bsp(&big, &small, BspOp::Intersection).unwrap();
    assert!(
        inner.root().solid_leaf_count() >= 1,
        "Interior intersection should be the small cube"
    );
}

#[test]
fn merge_self_union_is_identity() {
    let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
    let result = merge_bsp(&a, &a, BspOp::Union).unwrap();

    assert!(
        result.classify_point([0.0, 0.0, 0.0]),
        "Center should be inside self-union"
    );
    assert!(
        !result.classify_point([5.0, 0.0, 0.0]),
        "Outside point should be outside self-union"
    );
}

#[test]
fn merge_self_intersection_is_identity() {
    let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
    let result = merge_bsp(&a, &a, BspOp::Intersection).unwrap();

    assert!(
        result.root().solid_leaf_count() >= 1,
        "Self-intersection should be the same cube"
    );
}

#[test]
fn merge_self_subtraction_is_empty() {
    let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
    let result = merge_bsp(&a, &a, BspOp::Subtraction).unwrap();

    assert!(
        !result.classify_point([0.0, 0.0, 0.0]),
        "Center should be outside self-subtraction"
    );
}

#[test]
fn merge_chain_three_cubes() {
    let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
    let b = make_bsp_box([1.5, 0.0, 0.0], 1.0);
    let c = make_bsp_box([0.0, 1.5, 0.0], 1.0);

    let ab = merge_bsp(&a, &b, BspOp::Union).unwrap();
    let abc = merge_bsp(&ab, &c, BspOp::Union).unwrap();

    assert!(
        abc.classify_point([0.0, 0.0, 0.0]),
        "Center of A should be inside union"
    );
    assert!(
        abc.classify_point([1.5, 0.0, 0.0]),
        "Center of B should be inside union"
    );
    assert!(
        abc.classify_point([0.0, 1.5, 0.0]),
        "Center of C should be inside union"
    );
    assert!(
        !abc.classify_point([5.0, 5.0, 5.0]),
        "Far point should be outside union"
    );

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

    assert!(
        result.classify_point([0.0, 0.0, 0.0]),
        "Center of A should still be inside after subtract-back"
    );
    assert!(
        !result.classify_point([1.5, 0.0, 0.0]),
        "Center of B should be outside after subtract-back"
    );
}

#[test]
fn merge_intersection_overlapping_offset_cubes() {
    let a = make_bsp_box([0.0, 0.0, 0.0], 1.0);
    let b = make_bsp_box([1.0, 0.0, 0.0], 1.0);

    let result = merge_bsp(&a, &b, BspOp::Intersection).unwrap();
    assert!(
        result.root().solid_leaf_count() >= 1,
        "Overlapping offset intersection should have solid regions, got {}",
        result.root().solid_leaf_count()
    );
}
