//! Tests for the geometry store.

use super::schema::GeometryState;
use forge_core::ToleranceProvider;
use crate::geom_facade::Plane;
use forge_topo::handles::{FaceId, VertexId};

#[test]
fn store_and_retrieve_vertex_position() {
    let mut store = GeometryState::new();
    let vertex = VertexId::from_raw_parts(0, 0);
    let position = [1.0, 2.0, 3.0];

    store.set_vertex_position(vertex, position);

    let retrieved = store.get_vertex_position(vertex);
    assert_eq!(retrieved, Some(&position));
}

#[test]
fn store_and_retrieve_face_plane() {
    let mut store = GeometryState::new();
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
    let store = GeometryState::new();
    let vertex = VertexId::from_raw_parts(99, 0);
    assert_eq!(store.get_vertex_position(vertex), None);
}

#[test]
fn missing_face_returns_none() {
    let store = GeometryState::new();
    let face = FaceId::from_raw_parts(99, 0);
    assert!(store.get_face_plane(face).is_none());
}

#[test]
fn stale_generation_returns_none() {
    let mut store = GeometryState::new();
    let vertex_gen0 = VertexId::from_raw_parts(0, 0);
    let vertex_gen1 = VertexId::from_raw_parts(0, 1);

    store.set_vertex_position(vertex_gen0, [1.0, 2.0, 3.0]);

    assert_eq!(
        store.get_vertex_position(vertex_gen0),
        Some(&[1.0, 2.0, 3.0])
    );
    assert_eq!(store.get_vertex_position(vertex_gen1), None);
}

#[test]
fn counts_reflect_insertions() {
    let mut store = GeometryState::new();
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

    let mut store = GeometryState::new();
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

    let store = GeometryState::new();
    let result = store.get_plane(42);
    assert!(result.is_err());
}

// ── Phase A — Scale-aware tolerance tests ─────────────────────────────────────

#[test]
fn empty_store_returns_conservative_fallback() {
    let store = GeometryState::new();
    // No vertices → scale == 0.0 → max(0,1) = 1.0 → 1e-7 * 1.0 = 1e-7.
    let tol = store.global_default();
    assert!(
        (tol - 1e-7).abs() < 1e-15,
        "empty store should return 1e-7, got {}",
        tol
    );
}

#[test]
fn one_meter_cube_produces_correct_tolerance() {
    let mut store = GeometryState::new();
    // 1 m = 1000 mm cube — diagonal ≈ 1732 mm.
    // global_default = 1e-7 * 1732 ≈ 1.732e-4.
    let v0 = VertexId::from_raw_parts(0, 0);
    let v1 = VertexId::from_raw_parts(1, 0);
    store.set_vertex_position(v0, [0.0, 0.0, 0.0]);
    store.set_vertex_position(v1, [1000.0, 1000.0, 1000.0]); // mm
    let tol = store.global_default();
    let expected = (3f64.sqrt() * 1000.0 * 1e-7).max(1e-13);
    assert!(
        (tol - expected).abs() < 1e-18,
        "1 m cube: got {}, want {}",
        tol,
        expected
    );
}

#[test]
fn sub_millimeter_model_is_floored_at_absolute_minimum() {
    use crate::core::tolerance::ABSOLUTE_MINIMUM_TOLERANCE;
    let mut store = GeometryState::new();
    // 0.001 mm model — scale = 0.001, max(0.001, 1.0) = 1.0, so tol = 1e-7.
    // The floor 1e-13 cannot kick in here because 1e-7 > 1e-13, but let's
    // confirm the floor is respected when scale would underflow:
    // (Directly via ToleranceConfig, not GeometryState.)
    let mut cfg = crate::core::tolerance::ToleranceConfig::default();
    cfg.set_model_scale_mm(0.0); // edge case: effectively 1.0 clamp.
    let t = cfg.scaled_vertex_tolerance();
    assert!(t >= ABSOLUTE_MINIMUM_TOLERANCE);
    assert!((t - 1e-7).abs() < 1e-20);
}

#[test]
fn edge_tolerance_is_capped_at_1e_6_for_large_models() {
    let mut store = GeometryState::new();
    // A 10 km model (diameter 1e7 mm) would give global_default ≈ 1e0, but
    // edge_tolerance must stay ≤ 1e-6 for classification snap safety.
    let v0 = VertexId::from_raw_parts(0, 0);
    let v1 = VertexId::from_raw_parts(1, 0);
    store.set_vertex_position(v0, [0.0, 0.0, 0.0]);
    store.set_vertex_position(v1, [1.0e7, 0.0, 0.0]); // 1e7 mm = 10 km
    let et = store.edge_tolerance(0, 0);
    assert!(
        et <= 1e-6,
        "edge tolerance must be capped at 1e-6, got {}",
        et
    );
}

#[test]
fn scaled_vertex_tolerance_on_tolerance_config() {
    use crate::core::tolerance::ToleranceConfig;
    use forge_math::arithmetic::rational::Rational;

    let mut cfg = ToleranceConfig::default();

    // Verify the contract: scaled_vertex_tolerance() == max(scale, 1.0) * 1e-7
    // by computing the same operation in Rational and confirming bit-exact equality.
    cfg.set_model_scale_mm(1000.0);
    let t = cfg.scaled_vertex_tolerance();
    let r_scale = Rational::try_from_f64(1000.0_f64.max(1.0)).unwrap();
    let r_factor = Rational::try_from_f64(1e-7).unwrap();
    let expected = (r_scale * r_factor).to_f64_approx();
    assert_eq!(
        t, expected,
        "1000 mm: implementation diverged from contract: got {}, expected {}",
        t, expected
    );

    cfg.set_model_scale_mm(0.1);
    let t = cfg.scaled_vertex_tolerance();
    // max(0.1, 1.0) = 1.0 → scale * 1e-7 = 1e-7
    let r_scale = Rational::try_from_f64(0.1_f64.max(1.0)).unwrap();
    let r_factor = Rational::try_from_f64(1e-7).unwrap();
    let expected = (r_scale * r_factor).to_f64_approx();
    assert_eq!(
        t, expected,
        "0.1 mm (clamped): implementation diverged from contract: got {}, expected {}",
        t, expected
    );
}

// ── Phase 4 — Surface CRUD tests ──────────────────────────────────────────────

#[test]
fn surface_insert_and_retrieve() {
    use crate::geom_facade::SurfaceData;
    let mut store = GeometryState::new();
    let surface = SurfaceData::sphere([0.0, 0.0, 0.0], 5.0);
    let r = store.insert_surface(surface);
    let retrieved = store.get_surface(r);
    assert!(retrieved.is_ok());
}

#[test]
fn surface_remove_then_stale_get_errors() {
    use crate::geom_facade::SurfaceData;
    let mut store = GeometryState::new();
    let r = store.insert_surface(SurfaceData::plane([0.0, 0.0, 1.0], 0.0));
    assert!(store.remove_surface(r).is_ok());
    assert!(store.get_surface(r).is_err());
}

#[test]
fn surface_count_reflects_active_slots() {
    use crate::geom_facade::SurfaceData;
    let mut store = GeometryState::new();
    assert_eq!(store.surface_count(), 0);
    let r1 = store.insert_surface(SurfaceData::plane([0.0, 0.0, 1.0], 0.0));
    let _r2 = store.insert_surface(SurfaceData::sphere([0.0, 0.0, 0.0], 1.0));
    assert_eq!(store.surface_count(), 2);
    store.remove_surface(r1).unwrap();
    assert_eq!(store.surface_count(), 1);
}

#[test]
fn curve_insert_and_retrieve() {
    use crate::geom_facade::{CurveGeom, CurveKind};
    let mut store = GeometryState::new();
    let curve = CurveGeom::from_analytic(
        CurveKind::Line {
            origin: [0.0, 0.0, 0.0],
            direction: [1.0, 0.0, 0.0],
        },
        [0, 1],
    );
    let r = store.insert_curve(curve);
    assert!(store.get_curve(r).is_ok());
    assert_eq!(store.curve_count(), 1);
}

#[test]
fn coedge_insert_and_retrieve() {
    use crate::geom_facade::{Coedge, ParametricCurve2D};
    let mut store = GeometryState::new();
    let coedge = Coedge {
        uv_curve: ParametricCurve2D::Line {
            start: [0.0, 0.0],
            end: [1.0, 1.0],
        },
        surface: 0,
    };
    let r = store.insert_coedge(coedge);
    assert!(store.get_coedge(r).is_ok());
    assert_eq!(store.coedge_count(), 1);
}

// ── Phase 4 — Attachment round-trip tests ─────────────────────────────────────

#[test]
fn attach_surface_to_face_round_trip() {
    use crate::geom_facade::SurfaceData;
    use forge_topo::handles::SurfaceRef;
    let mut store = GeometryState::new();
    let face = FaceId::from_raw_parts(0, 0);
    let sr = store.insert_surface(SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 2.0));
    store.attach_surface_to_face(face, sr);
    assert_eq!(store.get_face_surface(face), Some(sr));
}

#[test]
fn attach_coedge_to_halfedge_round_trip() {
    use crate::geom_facade::{Coedge, ParametricCurve2D};
    use forge_topo::handles::{CoedgeRef, HalfEdgeId};
    let mut store = GeometryState::new();
    let he = HalfEdgeId::from_raw_parts(0, 0);
    let coedge = Coedge {
        uv_curve: ParametricCurve2D::Line {
            start: [0.0, 0.0],
            end: [1.0, 1.0],
        },
        surface: 0,
    };
    let cr = store.insert_coedge(coedge);
    store.attach_coedge_to_halfedge(he, cr, true);
    assert_eq!(store.get_halfedge_coedge(he), Some((cr, true)));
}

#[test]
fn attach_curve_to_edge_round_trip() {
    use crate::geom_facade::{CurveGeom, CurveKind};
    use forge_topo::handles::{CurveRef, EdgeId};
    let mut store = GeometryState::new();
    let edge = EdgeId::from_raw_parts(0, 0);
    let curve = CurveGeom::from_analytic(
        CurveKind::Circle {
            center: [0.0, 0.0, 0.0],
            normal: [0.0, 0.0, 1.0],
            radius: 1.0,
        },
        [0, 1],
    );
    let cr = store.insert_curve(curve);
    store.attach_curve_to_edge(edge, cr);
    assert_eq!(store.get_edge_curve(edge), Some(cr));
}

// ── Phase 4 — face_is_planar with real surfaces ───────────────────────────────

#[test]
fn face_is_planar_true_when_no_surface_attached() {
    let store = GeometryState::new();
    let face = FaceId::from_raw_parts(0, 0);
    assert!(store.face_is_planar(face));
}

#[test]
fn face_is_planar_true_when_plane_surface_attached() {
    use crate::geom_facade::SurfaceData;
    let mut store = GeometryState::new();
    let face = FaceId::from_raw_parts(0, 0);
    let sr = store.insert_surface(SurfaceData::plane([0.0, 0.0, 1.0], 0.0));
    store.attach_surface_to_face(face, sr);
    assert!(store.face_is_planar(face));
}

#[test]
fn face_is_planar_false_when_cylinder_surface_attached() {
    use crate::geom_facade::SurfaceData;
    let mut store = GeometryState::new();
    let face = FaceId::from_raw_parts(0, 0);
    let sr = store.insert_surface(SurfaceData::cylinder([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 2.0));
    store.attach_surface_to_face(face, sr);
    assert!(!store.face_is_planar(face));
}

// ── Phase 4 — validate_geometry_bindings ──────────────────────────────────────

#[test]
fn validate_bindings_passes_when_all_refs_live() {
    use crate::geom_facade::SurfaceData;
    use forge_topo::arena::{FaceData, TopologyArena};
    use forge_topo::handles::{LoopId, ShellId};

    let mut store = GeometryState::new();
    let mut arena = TopologyArena::new();
    let face = arena.insert_face(
        FaceData::new(LoopId::from_raw_parts(0, 0), ShellId::from_raw_parts(0, 0)),
        None,
    );

    let sr = store.insert_surface(SurfaceData::plane([0.0, 0.0, 1.0], 0.0));
    store.attach_surface_to_face(face, sr);
    assert!(store.validate_geometry_bindings(&arena).is_ok());
}

#[test]
fn validate_bindings_fails_on_dangling_surface_ref() {
    use crate::geom_facade::SurfaceData;
    use forge_topo::arena::{FaceData, TopologyArena};
    use forge_topo::handles::{LoopId, ShellId};

    let mut store = GeometryState::new();
    let mut arena = TopologyArena::new();
    let face = arena.insert_face(
        FaceData::new(LoopId::from_raw_parts(0, 0), ShellId::from_raw_parts(0, 0)),
        None,
    );

    let sr = store.insert_surface(SurfaceData::plane([0.0, 0.0, 1.0], 0.0));
    store.attach_surface_to_face(face, sr);
    store.remove_surface(sr).unwrap();
    assert!(store.validate_geometry_bindings(&arena).is_err());
}
