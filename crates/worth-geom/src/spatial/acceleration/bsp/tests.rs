//! Tests for BSP convex polyhedron construction.

use crate::primitives::plane::{signed_distance, Plane};
use crate::spatial::acceleration::bsp::{build_convex_polyhedron, BspConfig, ConvexCell};

const TEST_TOLERANCE: f64 = 1e-8;
const TEST_DEGENERACY: f64 = 1e-14;

/// Helper: count only the faces corresponding to user-supplied planes
/// (not the bounding-box planes, which are indices 0..5).
fn count_user_faces(cell: &ConvexCell, bbox_plane_count: usize) -> usize {
    cell.faces()
        .iter()
        .filter(|f| f.plane_idx() >= bbox_plane_count)
        .count()
}

/// Helper: verify Euler formula V - E + F = 2 for a convex polyhedron.
fn assert_euler(cell: &ConvexCell) {
    let v = cell.vertex_count() as i64;
    let f = cell.face_count() as i64;
    let e = cell.edge_count() as i64;
    assert_eq!(
        v - e + f,
        2,
        "Euler formula V-E+F=2 failed: V={v} E={e} F={f}"
    );
}

/// Helper: verify all vertices satisfy all planes (signed_distance ≤ eps).
fn assert_vertices_inside_planes(cell: &ConvexCell) {
    let eps = TEST_TOLERANCE;
    let planes = cell.planes();
    for v in cell.vertices() {
        for (i, plane) in planes.iter().enumerate() {
            let dist = signed_distance(plane, v.position());
            assert!(
                dist < eps,
                "Vertex {:?} violates plane {} with distance {dist}",
                v.position(),
                i
            );
        }
    }
}

/// Tetrahedron from 4 planes: should have 4 user faces.
#[test]
fn tetrahedron_from_four_planes() {
    let config = BspConfig::default();
    let planes = vec![
        Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, 0.0], [0.0, -1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, 0.0], [-1.0, 0.0, 0.0]).unwrap(),
        Plane::from_point_normal([1.0, 1.0, 1.0], [1.0, 1.0, 1.0]).unwrap(),
    ];

    let cell = build_convex_polyhedron(&planes, &config).unwrap();

    assert_eq!(count_user_faces(&cell, 6), 4);
    assert_eq!(cell.vertex_count(), 4);
    assert_euler(&cell);
    assert_vertices_inside_planes(&cell);
}

/// Cube from 6 planes: V=8, user-F=6.
#[test]
fn cube_from_six_planes() {
    let config = BspConfig::default();
    let planes = vec![
        Plane::from_point_normal([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap(),
        Plane::from_point_normal([-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 1.0, 0.0], [0.0, 1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, -1.0, 0.0], [0.0, -1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, -1.0], [0.0, 0.0, -1.0]).unwrap(),
    ];

    let cell = build_convex_polyhedron(&planes, &config).unwrap();

    assert_eq!(count_user_faces(&cell, 6), 6);
    assert_eq!(cell.vertex_count(), 8);
    assert_euler(&cell);
    assert_vertices_inside_planes(&cell);

    for v in cell.vertices() {
        let p = v.position();
        for coord in p {
            assert!(
                (*coord - 1.0).abs() < TEST_TOLERANCE || (*coord + 1.0).abs() < TEST_TOLERANCE,
                "Cube vertex coord {coord} should be ±1"
            );
        }
    }
}

/// Hexagonal prism from 8 planes: V=12, user-F=8.
#[test]
fn hexagonal_prism_from_eight_planes() {
    let config = BspConfig::default();
    let hex_normals: Vec<[f64; 3]> = (0..6)
        .map(|i| {
            let angle = std::f64::consts::PI * i as f64 / 3.0;
            [angle.cos(), angle.sin(), 0.0]
        })
        .collect();

    let mut planes = Vec::new();
    for n in &hex_normals {
        planes.push(Plane::from_point_normal([n[0], n[1], 0.0], *n).unwrap());
    }
    planes.push(Plane::from_point_normal([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]).unwrap());
    planes.push(Plane::from_point_normal([0.0, 0.0, -1.0], [0.0, 0.0, -1.0]).unwrap());

    let cell = build_convex_polyhedron(&planes, &config).unwrap();

    assert_eq!(cell.vertex_count(), 12);
    assert_eq!(count_user_faces(&cell, 6), 8);
    assert_euler(&cell);
    assert_vertices_inside_planes(&cell);
}

/// KV-18: Near-degenerate plane sets produce consistent results.
#[test]
fn kv18_near_degenerate_planes() {
    let config = BspConfig::default();
    let eps = TEST_DEGENERACY;
    let planes = vec![
        Plane::from_point_normal([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap(),
        Plane::from_point_normal([-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 1.0, 0.0], [0.0, 1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, -1.0, 0.0], [0.0, -1.0, 0.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]).unwrap(),
        Plane::from_point_normal([0.0, 0.0, -1.0], [0.0, 0.0, -1.0]).unwrap(),
        Plane::from_point_normal([1.0 + eps, 0.0, 0.0], [1.0, eps, 0.0]).unwrap(),
    ];

    let result = build_convex_polyhedron(&planes, &config);
    assert!(result.is_ok());
    let cell = result.unwrap();
    assert!(cell.vertex_count() >= 4);
    assert_euler(&cell);
    assert_vertices_inside_planes(&cell);
}

/// Contradictory planes that create an empty intersection.
#[test]
fn contradictory_planes_return_error() {
    let config = BspConfig::default();
    let planes = vec![
        Plane::try_new([-1.0, 0.0, 0.0], 5.0).unwrap(),
        Plane::try_new([1.0, 0.0, 0.0], 5.0).unwrap(),
    ];

    let result = build_convex_polyhedron(&planes, &config);
    assert!(result.is_err());
}

/// Determinism: same planes produce same result.
#[test]
fn bsp_output_is_deterministic() {
    let config = BspConfig::default();
    let make_planes = || {
        vec![
            Plane::from_point_normal([1.0, 0.0, 0.0], [1.0, 0.0, 0.0]).unwrap(),
            Plane::from_point_normal([-1.0, 0.0, 0.0], [-1.0, 0.0, 0.0]).unwrap(),
            Plane::from_point_normal([0.0, 1.0, 0.0], [0.0, 1.0, 0.0]).unwrap(),
            Plane::from_point_normal([0.0, -1.0, 0.0], [0.0, -1.0, 0.0]).unwrap(),
            Plane::from_point_normal([0.0, 0.0, 1.0], [0.0, 0.0, 1.0]).unwrap(),
            Plane::from_point_normal([0.0, 0.0, -1.0], [0.0, 0.0, -1.0]).unwrap(),
        ]
    };

    let c1 = build_convex_polyhedron(&make_planes(), &config).unwrap();
    let c2 = build_convex_polyhedron(&make_planes(), &config).unwrap();

    assert_eq!(c1.vertex_count(), c2.vertex_count());
    assert_eq!(c1.face_count(), c2.face_count());
    assert_eq!(c1.edge_count(), c2.edge_count());
}
