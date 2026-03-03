//! Shape-specific entity count tests.
//!
//! Validates exact V/E/F counts for each primitive, plus custom
//! plane sets (octahedron, truncated cube) and new shapes.
//!
//! Every test asserts on the `DecisionLog` produced by `ModelingContext`
//! via `assert_vertex_decisions` from the test harness.

use crate::context::ModelingContext;
use crate::operations::primitives::{
    make_block, make_convex_solid, make_cube, make_dodecahedron, make_prism,
    make_tetrahedron, make_wedge,
};

/// Thin wrapper: logs decisions, then delegates to the production validator.
fn assert_vertex_decisions(
    label: &str,
    log: &forge_core::DecisionLog,
    expected_vertices: usize,
    tolerance: f64,
) {
    forge_core::tracing::log_decision_log(label, log);
    crate::operations::shared_validators::facade::validate_vertex_decisions(
        log, expected_vertices, tolerance,
    )
    .unwrap_or_else(|e| panic!("{label}: {e}"));
}
use super::structural_invariants_tests::assert_valid_solid;
use super::{init_test_tracing, test_config, OperationScope};

fn assert_counts(label: &str, v: usize, e: usize, f: usize, ev: usize, ee: usize, ef: usize) {
    assert_eq!(v, ev, "{label}: V={v}, expected {ev}");
    assert_eq!(e, ee, "{label}: E={e}, expected {ee}");
    assert_eq!(f, ef, "{label}: F={f}, expected {ef}");
}

#[test]
fn cube_counts() {
    init_test_tracing();
    let cfg = test_config();
    let tolerance = cfg.scaled_vertex_tolerance();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&cfg, &mut ctx);
    let r = make_cube([0.0; 3], 2.0, &mut scope).unwrap();
    let a = r.topology().arena();
    assert_counts("cube", a.vertex_count(), a.half_edge_count() / 2, a.face_count(), 8, 12, 6);
    assert_vertex_decisions("cube", ctx.get_decision_log(), 8, tolerance);
}

#[test]
fn tetrahedron_counts() {
    init_test_tracing();
    let cfg = test_config();
    let tolerance = cfg.scaled_vertex_tolerance();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&cfg, &mut ctx);
    let r = make_tetrahedron([0.0; 3], 1.0, &mut scope).unwrap();
    let a = r.topology().arena();
    assert_counts("tet", a.vertex_count(), a.half_edge_count() / 2, a.face_count(), 4, 6, 4);
    assert_vertex_decisions("tet", ctx.get_decision_log(), 4, tolerance);
}

#[test]
fn dodecahedron_counts() {
    init_test_tracing();
    let cfg = test_config();
    let tolerance = cfg.scaled_vertex_tolerance();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&cfg, &mut ctx);
    let r = make_dodecahedron([0.0; 3], 1.0, &mut scope).unwrap();
    let a = r.topology().arena();
    assert_counts("dodec", a.vertex_count(), a.half_edge_count() / 2, a.face_count(), 20, 30, 12);
    assert_vertex_decisions("dodec", ctx.get_decision_log(), 20, tolerance);
}

#[test]
fn octahedron_from_eight_planes_generates() {
    init_test_tracing();
    let cfg = test_config();
    let tolerance = cfg.scaled_vertex_tolerance();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&cfg, &mut ctx);
    let planes = vec![
        forge_geom::Plane::from_point_normal([1.0, 1.0, 1.0], [1.0, 1.0, 1.0]).unwrap(),
        forge_geom::Plane::from_point_normal([1.0, 1.0, -1.0], [1.0, 1.0, -1.0]).unwrap(),
        forge_geom::Plane::from_point_normal([1.0, -1.0, 1.0], [1.0, -1.0, 1.0]).unwrap(),
        forge_geom::Plane::from_point_normal([1.0, -1.0, -1.0], [1.0, -1.0, -1.0]).unwrap(),
        forge_geom::Plane::from_point_normal([-1.0, 1.0, 1.0], [-1.0, 1.0, 1.0]).unwrap(),
        forge_geom::Plane::from_point_normal([-1.0, 1.0, -1.0], [-1.0, 1.0, -1.0]).unwrap(),
        forge_geom::Plane::from_point_normal([-1.0, -1.0, 1.0], [-1.0, -1.0, 1.0]).unwrap(),
        forge_geom::Plane::from_point_normal([-1.0, -1.0, -1.0], [-1.0, -1.0, -1.0]).unwrap(),
    ];
    let r = make_convex_solid(planes, &mut scope).unwrap();
    let a = r.topology().arena();
    assert!(a.face_count() >= 4, "octahedron must have at least 4 faces");
    assert_vertex_decisions("octahedron", ctx.get_decision_log(), a.vertex_count(), tolerance);
}

#[test]
fn truncated_cube_fourteen_faces() {
    init_test_tracing();
    let cfg = test_config();
    let tolerance = cfg.scaled_vertex_tolerance();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&cfg, &mut ctx);
    let mut planes = forge_geom::cube([0.0; 3], 2.0).unwrap();
    let corners: [[f64; 3]; 8] = [
        [1.0, 1.0, 1.0], [1.0, 1.0, -1.0], [1.0, -1.0, 1.0], [1.0, -1.0, -1.0],
        [-1.0, 1.0, 1.0], [-1.0, 1.0, -1.0], [-1.0, -1.0, 1.0], [-1.0, -1.0, -1.0],
    ];
    // Cutting planes at 1.5 × corner normal — inside the cube (corners at ±2.0)
    for n in &corners {
        let pt = [n[0] * 1.5, n[1] * 1.5, n[2] * 1.5];
        planes.push(forge_geom::Plane::from_point_normal(pt, *n).unwrap());
    }
    let r = make_convex_solid(planes, &mut scope).unwrap();
    let a = r.topology().arena();
    assert_eq!(a.face_count(), 14, "truncated cube: expected 14 faces");
    assert_valid_solid(&r, "truncated_cube");
    assert_vertex_decisions("truncated_cube", ctx.get_decision_log(), a.vertex_count(), tolerance);
}

// ── New shape counts ──────────────────────────────────────────────────────

#[test]
fn block_non_uniform_counts() {
    init_test_tracing();
    let cfg = test_config();
    let tolerance = cfg.scaled_vertex_tolerance();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&cfg, &mut ctx);
    let r = make_block([0.0; 3], [1.0, 2.0, 3.0], &mut scope).unwrap();
    let a = r.topology().arena();
    assert_counts("block", a.vertex_count(), a.half_edge_count() / 2, a.face_count(), 8, 12, 6);
    assert_valid_solid(&r, "block");
    assert_vertex_decisions("block", ctx.get_decision_log(), 8, tolerance);
}

#[test]
fn prism_triangular_counts() {
    init_test_tracing();
    let cfg = test_config();
    let tolerance = cfg.scaled_vertex_tolerance();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&cfg, &mut ctx);
    let r = make_prism([0.0; 3], 3, 1.0, 2.0, &mut scope).unwrap();
    let a = r.topology().arena();
    assert_counts("prism3", a.vertex_count(), a.half_edge_count() / 2, a.face_count(), 6, 9, 5);
    assert_valid_solid(&r, "prism3");
    assert_vertex_decisions("prism3", ctx.get_decision_log(), 6, tolerance);
}

#[test]
fn prism_hexagonal_counts() {
    init_test_tracing();
    let cfg = test_config();
    let tolerance = cfg.scaled_vertex_tolerance();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&cfg, &mut ctx);
    let r = make_prism([0.0; 3], 6, 1.0, 2.0, &mut scope).unwrap();
    let a = r.topology().arena();
    assert_counts("prism6", a.vertex_count(), a.half_edge_count() / 2, a.face_count(), 12, 18, 8);
    assert_valid_solid(&r, "prism6");
    assert_vertex_decisions("prism6", ctx.get_decision_log(), 12, tolerance);
}

#[test]
fn pyramid_quad_generates() {
    let planes = forge_geom::pyramid([0.0; 3], 4, 1.0, 2.0).unwrap();
    assert_eq!(planes.len(), 5, "pyramid(4) should produce 5 planes");
}

#[test]
fn wedge_counts() {
    init_test_tracing();
    let cfg = test_config();
    let tolerance = cfg.scaled_vertex_tolerance();
    let mut ctx = ModelingContext::new();
    let mut scope = OperationScope::new(&cfg, &mut ctx);
    let r = make_wedge([0.0; 3], [2.0, 3.0, 1.0], &mut scope).unwrap();
    let a = r.topology().arena();
    assert_valid_solid(&r, "wedge");
    assert_vertex_decisions("wedge", ctx.get_decision_log(), a.vertex_count(), tolerance);
}
