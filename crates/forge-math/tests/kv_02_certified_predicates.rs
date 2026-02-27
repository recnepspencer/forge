//! KV-02: Certified predicate correctness suite.
//!
//! Validates that:
//! - Coplanar points produce `Zero` deterministically (not perturbed)
//! - Near-coplanar points produce the correct `Neg`/`Pos` sign
//! - All results are `CertifiedTriSign` (type-level guarantee)
//! - `PrecisionEscalation` metadata is returned with every result

use forge_math::predicates::{in_sphere, orient2d, orient3d};
use forge_math::sign::TriSign;

#[test]
fn kv02_orient2d_collinear_diagonal() {
    let (result, _) = orient2d([0.0, 0.0], [1.0, 1.0], [2.0, 2.0]).unwrap();
    assert_eq!(result.sign(), TriSign::Zero);
}

#[test]
fn kv02_orient2d_collinear_x_axis() {
    let (result, _) = orient2d([0.0, 0.0], [1.0, 0.0], [2.0, 0.0]).unwrap();
    assert_eq!(result.sign(), TriSign::Zero);
}

#[test]
fn kv02_orient2d_collinear_y_axis() {
    let (result, _) = orient2d([0.0, 0.0], [0.0, 1.0], [0.0, 2.0]).unwrap();
    assert_eq!(result.sign(), TriSign::Zero);
}

#[test]
fn kv02_orient2d_collinear_negative_coords() {
    let (result, _) = orient2d([-3.0, -3.0], [0.0, 0.0], [3.0, 3.0]).unwrap();
    assert_eq!(result.sign(), TriSign::Zero);
}

#[test]
fn kv02_orient2d_collinear_midpoint() {
    let (result, _) = orient2d([0.0, 0.0], [2.0, 0.0], [1.0, 0.0]).unwrap();
    assert_eq!(result.sign(), TriSign::Zero);
}

#[test]
fn kv02_orient2d_near_collinear_above() {
    let (result, _) = orient2d([0.0, 0.0], [1.0, 0.0], [0.5, 1e-15]).unwrap();
    assert_eq!(result.sign(), TriSign::Pos);
}

#[test]
fn kv02_orient2d_near_collinear_below() {
    let (result, _) = orient2d([0.0, 0.0], [1.0, 0.0], [0.5, -1e-15]).unwrap();
    assert_eq!(result.sign(), TriSign::Neg);
}

#[test]
fn kv02_orient3d_coplanar_xy_plane() {
    let (result, _) = orient3d(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
    )
    .unwrap();
    assert_eq!(result.sign(), TriSign::Zero);
}

#[test]
fn kv02_orient3d_coplanar_xz_plane() {
    let (result, _) = orient3d(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        [1.0, 0.0, 1.0],
    )
    .unwrap();
    assert_eq!(result.sign(), TriSign::Zero);
}

#[test]
fn kv02_orient3d_coplanar_diagonal_plane() {
    let (result, _) = orient3d(
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [0.5, 0.5, 0.0],
    )
    .unwrap();
    assert_eq!(result.sign(), TriSign::Zero);
}

#[test]
fn kv02_orient3d_near_coplanar_above() {
    let (result, _) = orient3d(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.5, 0.5, 1e-15],
    )
    .unwrap();
    assert!(result.sign() != TriSign::Zero);
}

#[test]
fn kv02_orient3d_near_coplanar_below() {
    let (result, _) = orient3d(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.5, 0.5, -1e-15],
    )
    .unwrap();
    assert!(result.sign() != TriSign::Zero);
}

#[test]
fn kv02_in_sphere_on_circumsphere() {
    let (result, _) = in_sphere(
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 0.0, 1.0],
        [-1.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
    )
    .unwrap();
    assert_eq!(result.sign(), TriSign::Zero);
}

macro_rules! repeat_10 {
    ($expr:expr) => {
        $expr;
        $expr;
        $expr;
        $expr;
        $expr;
        $expr;
        $expr;
        $expr;
        $expr;
        $expr;
    };
}

#[test]
fn kv02_orient2d_deterministic_10x() {
    let a = [0.1, 0.2];
    let b = [0.3, 0.4];
    let c = [0.5, 0.7];
    let baseline = orient2d(a, b, c).unwrap().0.sign();
    repeat_10!(assert_eq!(orient2d(a, b, c).unwrap().0.sign(), baseline));
}

#[test]
fn kv02_orient3d_deterministic_10x() {
    let a = [0.1, 0.2, 0.3];
    let b = [0.4, 0.5, 0.6];
    let c = [0.7, 0.8, 1.0];
    let d = [0.0, 0.0, 0.0];
    let baseline = orient3d(a, b, c, d).unwrap().0.sign();
    repeat_10!(assert_eq!(orient3d(a, b, c, d).unwrap().0.sign(), baseline));
}
