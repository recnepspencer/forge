//! Area validator poison tests.

use super::test_support::*;
use forge_core::{KernelError, TopologyError};
use forge_spatial::validators::area::validate_zero_area_faces;
use forge_topo::b_rep::{FaceData, HalfEdgeData, LoopData, VertexData};
use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId};

#[test]
fn valid_face_passes() {
    let mut draft = empty_test_draft();
    let (face, v0, v1, v2) = build_triangle_face(&mut draft);
    let arena = draft.arena();
    let tol = MockToleranceProvider::default();

    let result = validate_zero_area_faces(
        arena,
        &|v| {
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else if v == v1 {
                Some([1.0, 0.0, 0.0])
            } else if v == v2 {
                Some([0.0, 1.0, 0.0])
            } else {
                None
            }
        },
        &|f| f == face,
        &tol,
    );
    assert!(result.is_ok());
}

#[test]
fn zero_area_detected() {
    let mut draft = empty_test_draft();
    let (face, v0, v1, v2) = build_triangle_face(&mut draft);
    let arena = draft.arena();
    let tol = MockToleranceProvider {
        default_tolerance: 1e-6,
    };

    let result = validate_zero_area_faces(
        arena,
        &|v| {
            if v == v0 || v == v1 || v == v2 {
                Some([5.0, 5.0, 5.0])
            } else {
                None
            }
        },
        &|f| f == face,
        &tol,
    );
    assert!(result.is_err());
    match result.unwrap_err() {
        KernelError::TopologyViolation {
            err: TopologyError::ZeroAreaFace { computed_area, .. },
            ..
        } => {
            assert_eq!(computed_area, 0.0);
        }
        other => panic!("Expected ZeroAreaFace, got: {:?}", other),
    }
}

#[test]
fn sub_tolerance_area_detected() {
    let mut draft = empty_test_draft();
    let (face, v0, v1, v2) = build_triangle_face(&mut draft);
    let arena = draft.arena();
    let tol = MockToleranceProvider {
        default_tolerance: 1e-6,
    };

    let result = validate_zero_area_faces(
        arena,
        &|v| {
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else if v == v1 {
                Some([1e-7, 0.0, 0.0])
            } else if v == v2 {
                Some([0.0, 1e-7, 0.0])
            } else {
                None
            }
        },
        &|f| f == face,
        &tol,
    );
    assert!(result.is_err());
    match result.unwrap_err() {
        KernelError::TopologyViolation {
            err:
                TopologyError::ZeroAreaFace {
                    computed_area,
                    threshold,
                    ..
                },
            ..
        } => {
            assert!(computed_area > 0.0);
            assert!(computed_area < threshold);
        }
        other => panic!("Expected ZeroAreaFace, got: {:?}", other),
    }
}

#[test]
fn collinear_sliver_detected() {
    let mut draft = empty_test_draft();
    let (face, v0, v1, v2) = build_triangle_face(&mut draft);
    let arena = draft.arena();
    let tol = MockToleranceProvider {
        default_tolerance: 1e-6,
    };

    let result = validate_zero_area_faces(
        arena,
        &|v| {
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else if v == v1 {
                Some([5.0, 5.0, 5.0])
            } else if v == v2 {
                Some([10.0, 10.0, 10.0])
            } else {
                None
            }
        },
        &|f| f == face,
        &tol,
    );
    assert!(result.is_err());
    match result.unwrap_err() {
        KernelError::TopologyViolation {
            err: TopologyError::ZeroAreaFace { computed_area, .. },
            ..
        } => {
            assert_eq!(computed_area, 0.0);
        }
        other => panic!(
            "Expected ZeroAreaFace for collinear sliver, got: {:?}",
            other
        ),
    }
}

#[test]
fn degenerate_face_skipped() {
    let mut draft = empty_test_draft();
    let placeholder_he = HalfEdgeId::new(0, 0);

    let v0 = draft.insert_vertex(VertexData::new(placeholder_he));
    let v1 = draft.insert_vertex(VertexData::new(placeholder_he));
    let shell = insert_test_solid_shell(&mut draft);
    let loop_id = draft.insert_loop(LoopData::new(placeholder_he, FaceId::new(0, 0)));
    let face = draft.insert_face(FaceData::new(loop_id, shell));

    let h0 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v0,
        EdgeId::new(0, 0),
    ));
    let h1 = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        v1,
        EdgeId::new(0, 0),
    ));

    let arena = draft.arena_mut();
    arena.get_half_edge_mut(h0).unwrap().set_next(h1);
    arena.get_half_edge_mut(h1).unwrap().set_next(h0);
    arena.get_loop_mut(loop_id).unwrap().set_half_edge(h0);
    arena.get_loop_mut(loop_id).unwrap().set_face(face);

    let tol = MockToleranceProvider::default();
    let result = validate_zero_area_faces(
        arena,
        &|v| {
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else {
                Some([1.0, 1.0, 1.0])
            }
        },
        &|f| f == face,
        &tol,
    );
    assert!(
        result.is_ok(),
        "Degenerate face (<3 vertices) should be skipped"
    );
}

#[test]
fn missing_positions_error() {
    let mut draft = empty_test_draft();
    let (face, v0, v1, _v2) = build_triangle_face(&mut draft);
    let arena = draft.arena();
    let tol = MockToleranceProvider::default();

    let result = validate_zero_area_faces(
        arena,
        &|v| {
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else if v == v1 {
                Some([1.0, 0.0, 0.0])
            } else {
                None
            }
        },
        &|f| f == face,
        &tol,
    );
    assert!(result.is_err());
    match result.unwrap_err() {
        KernelError::TopologyViolation {
            err: TopologyError::MissingVertexPosition { .. },
            ..
        } => {}
        other => panic!("Expected MissingVertexPosition, got: {:?}", other),
    }
}
