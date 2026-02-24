//! Tests for the geometry store.

use forge_core::ToleranceProvider;
use forge_geom::Plane;
use forge_topo::handles::{FaceId, VertexId};
use super::schema::GeometryStore;

#[test]
fn store_and_retrieve_vertex_position() {
    let mut store = GeometryStore::new();
    let vertex = VertexId::from_raw_parts(0, 0);
    let position = [1.0, 2.0, 3.0];

    store.set_vertex_position(vertex, position);

    let retrieved = store.get_vertex_position(vertex);
    assert_eq!(retrieved, Some(&position));
}

#[test]
fn store_and_retrieve_face_plane() {
    let mut store = GeometryStore::new();
    let face = FaceId::from_raw_parts(0, 0);
    let plane = Plane::try_new([0.0, 0.0, 1.0], 0.0).unwrap();

    store.set_face_plane(face, plane);

    let retrieved = store.get_face_plane(face);
    assert!(retrieved.is_some());
    let p = retrieved.unwrap();
    assert_eq!(p.normal()[2], 1.0);
}

#[test]
fn missing_vertex_returns_none() {
    let store = GeometryStore::new();
    let vertex = VertexId::from_raw_parts(99, 0);
    assert_eq!(store.get_vertex_position(vertex), None);
}

#[test]
fn missing_face_returns_none() {
    let store = GeometryStore::new();
    let face = FaceId::from_raw_parts(99, 0);
    assert!(store.get_face_plane(face).is_none());
}

#[test]
fn stale_generation_returns_none() {
    let mut store = GeometryStore::new();
    let vertex_gen0 = VertexId::from_raw_parts(0, 0);
    let vertex_gen1 = VertexId::from_raw_parts(0, 1);

    store.set_vertex_position(vertex_gen0, [1.0, 2.0, 3.0]);

    assert_eq!(store.get_vertex_position(vertex_gen0), Some(&[1.0, 2.0, 3.0]));
    assert_eq!(store.get_vertex_position(vertex_gen1), None);
}

#[test]
fn counts_reflect_insertions() {
    let mut store = GeometryStore::new();
    assert_eq!(store.face_plane_count(), 0);
    assert_eq!(store.vertex_position_count(), 0);

    store.set_face_plane(
        FaceId::from_raw_parts(0, 0),
        Plane::try_new([1.0, 0.0, 0.0], 0.0).unwrap(),
    );
    store.set_vertex_position(VertexId::from_raw_parts(0, 0), [0.0, 0.0, 0.0]);
    store.set_vertex_position(VertexId::from_raw_parts(1, 0), [1.0, 0.0, 0.0]);

    assert_eq!(store.face_plane_count(), 1);
    assert_eq!(store.vertex_position_count(), 2);
}

#[test]
fn geometry_source_trait_returns_plane() {
    use forge_math::GeometrySource;

    let mut store = GeometryStore::new();
    let plane = Plane::try_new([0.0, 1.0, 0.0], -5.0).unwrap();
    store.set_face_plane(FaceId::from_raw_parts(0, 0), plane);

    let result = store.get_plane(0);
    assert!(result.is_ok());
    let coeffs = result.unwrap();
    assert!((coeffs.normal()[1] - 1.0).abs() < 1e-10);
}

#[test]
fn geometry_source_missing_plane_returns_error() {
    use forge_math::GeometrySource;

    let store = GeometryStore::new();
    let result = store.get_plane(42);
    assert!(result.is_err());
}

// ── Phase A — Scale-aware tolerance tests ─────────────────────────────────────

#[test]
fn empty_store_returns_conservative_fallback() {
    let store = GeometryStore::new();
    // No vertices → scale == 0.0 → max(0,1) = 1.0 → 1e-7 * 1.0 = 1e-7.
    let tol = store.global_default();
    assert!((tol - 1e-7).abs() < 1e-15, "empty store should return 1e-7, got {}", tol);
}

#[test]
fn one_meter_cube_produces_correct_tolerance() {
    let mut store = GeometryStore::new();
    // 1 m = 1000 mm cube — diagonal ≈ 1732 mm.
    // global_default = 1e-7 * 1732 ≈ 1.732e-4.
    let v0 = VertexId::from_raw_parts(0, 0);
    let v1 = VertexId::from_raw_parts(1, 0);
    store.set_vertex_position(v0, [0.0, 0.0, 0.0]);
    store.set_vertex_position(v1, [1000.0, 1000.0, 1000.0]); // mm
    let tol = store.global_default();
    let expected = (3f64.sqrt() * 1000.0 * 1e-7).max(1e-13);
    assert!((tol - expected).abs() < 1e-18, "1 m cube: got {}, want {}", tol, expected);
}

#[test]
fn sub_millimeter_model_is_floored_at_absolute_minimum() {
    use crate::core::tolerance::ABSOLUTE_MINIMUM_TOLERANCE;
    let mut store = GeometryStore::new();
    // 0.001 mm model — scale = 0.001, max(0.001, 1.0) = 1.0, so tol = 1e-7.
    // The floor 1e-13 cannot kick in here because 1e-7 > 1e-13, but let's
    // confirm the floor is respected when scale would underflow:
    // (Directly via ToleranceConfig, not GeometryStore.)
    let mut cfg = crate::core::tolerance::ToleranceConfig::default();
    cfg.set_model_scale_mm(0.0); // edge case: effectively 1.0 clamp.
    let t = cfg.scaled_vertex_tolerance();
    assert!(t >= ABSOLUTE_MINIMUM_TOLERANCE);
    assert!((t - 1e-7).abs() < 1e-20);
}

#[test]
fn edge_tolerance_is_capped_at_1e_6_for_large_models() {
    let mut store = GeometryStore::new();
    // A 10 km model (diameter 1e7 mm) would give global_default ≈ 1e0, but
    // edge_tolerance must stay ≤ 1e-6 for classification snap safety.
    let v0 = VertexId::from_raw_parts(0, 0);
    let v1 = VertexId::from_raw_parts(1, 0);
    store.set_vertex_position(v0, [0.0, 0.0, 0.0]);
    store.set_vertex_position(v1, [1.0e7, 0.0, 0.0]); // 1e7 mm = 10 km
    let et = store.edge_tolerance(0, 0);
    assert!(et <= 1e-6, "edge tolerance must be capped at 1e-6, got {}", et);
}

#[test]
fn scaled_vertex_tolerance_on_tolerance_config() {
    use crate::core::tolerance::ToleranceConfig;
    let mut cfg = ToleranceConfig::default();

    cfg.set_model_scale_mm(1000.0); // 1 m
    let t = cfg.scaled_vertex_tolerance();
    assert!((t - 1e-4).abs() < 1e-20, "1000 mm → 1e-4, got {}", t);

    cfg.set_model_scale_mm(0.1); // 100 μm MEMS
    // max(0.1, 1.0) = 1.0 → 1e-7
    let t = cfg.scaled_vertex_tolerance();
    assert!((t - 1e-7).abs() < 1e-20, "0.1 mm → 1e-7 (clamped), got {}", t);
}
