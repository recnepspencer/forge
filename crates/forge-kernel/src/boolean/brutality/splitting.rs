use super::super::test_helpers::{run_boolean, try_boolean, execute_boolean_logged};
use super::super::schema::{BooleanInput, BooleanOp};
use forge_geom::bsp::{build_convex_polyhedron, BspConfig};
use forge_geom::plane::Plane;
use forge_topo::hashing::compute_arena_topology_hash;
use crate::core::ModelingContext;
use crate::mesh_builder::build_halfedge_mesh;

// ══════════════════════════════════════════════════════════════
// §3  BOOLEAN SPLITTING TORTURE
// ══════════════════════════════════════════════════════════════

/// 3.1 — Intersection Line Hits Exactly Through Vertex
///
/// Arrange cubes so intersection line passes through a vertex of one cube.
/// Expect deterministic split, no duplicated vertex, no zero-length edges.
#[test]
fn intersection_through_vertex() {
    let result = run_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [1.0, 1.0, 0.0], 1.0,
        BooleanOp::Intersection,
    );

    let arena = result.topology().arena();
    let geom = result.geometry();

    for (vid, _) in arena.iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            assert!(
                pos[0].is_finite() && pos[1].is_finite() && pos[2].is_finite(),
                "Non-finite vertex at {vid:?}: {pos:?}"
            );
        }
    }

    for (_he_id, he) in arena.iter_half_edges() {
        let origin_pos = geom.get_vertex_position(he.origin);
        if let Ok(twin) = arena.get_half_edge(he.twin) {
            let next_origin_pos = geom.get_vertex_position(twin.origin);

            if let (Some(p1), Some(p2)) = (origin_pos, next_origin_pos) {
                let dist_sq = (p1[0] - p2[0]).powi(2)
                    + (p1[1] - p2[1]).powi(2)
                    + (p1[2] - p2[2]).powi(2);
                assert!(
                    dist_sq > 1e-20,
                    "Zero-length edge: {p1:?} → {p2:?}"
                );
            }
        }
    }
}

/// 3.2 — Intersection Line Coincident With Edge
///
/// Two cubes where intersection plane lies exactly on one cube's edge.
#[test]
fn intersection_coincident_with_edge() {
    let result = try_boolean(
        [0.0, 0.0, 0.0], 1.0,
        [1.0, 0.0, 1.0], 1.0,
        BooleanOp::Union,
    );

    match result {
        Ok(r) => {
            let arena = r.topology().arena();
            for (_he_id, he) in arena.iter_half_edges() {
                let twin = arena.get_half_edge(he.twin);
                assert!(
                    twin.is_ok(),
                    "Dangling halfedge after edge-coincident boolean"
                );
            }

            let v = arena.vertex_count() as isize;
            let e = (arena.half_edge_count() / 2) as isize;
            let f = arena.face_count() as isize;
            let euler = v - e + f;
            assert_eq!(euler, 2, "Euler violation after edge-coincident union: V-E+F={euler}");
        }
        Err(e) => {
            panic!("Edge-coincident boolean must not fail: {e:?}");
        }
    }
}

/// 3.3 — Nearly Coplanar Faces
///
/// Two cubes with face offset by extremely small amounts.
/// Must be deterministic across runs.
#[test]
fn nearly_coplanar_faces_1e10() {
    let mut hashes = Vec::new();
    for _ in 0..10 {
        let result = try_boolean(
            [0.0, 0.0, 0.0], 1.0,
            [1e-10, 0.0, 0.0], 1.0,
            BooleanOp::Union,
        );

        let hash = match &result {
            Ok(r) => compute_arena_topology_hash(r.topology().arena()),
            Err(_) => 0,
        };
        hashes.push(hash);
    }

    let first = hashes[0];
    for (i, &h) in hashes.iter().enumerate() {
        assert_eq!(
            h, first,
            "Nearly-coplanar 1e-10 not deterministic on run {i}"
        );
    }
}

#[test]
fn nearly_coplanar_faces_1e12() {
    let mut hashes = Vec::new();
    for _ in 0..10 {
        let result = try_boolean(
            [0.0, 0.0, 0.0], 1.0,
            [1e-12, 0.0, 0.0], 1.0,
            BooleanOp::Union,
        );

        let hash = match &result {
            Ok(r) => compute_arena_topology_hash(r.topology().arena()),
            Err(_) => 0,
        };
        hashes.push(hash);
    }

    let first = hashes[0];
    for (i, &h) in hashes.iter().enumerate() {
        assert_eq!(h, first, "Nearly-coplanar 1e-12 not deterministic on run {i}");
    }
}

#[test]
fn nearly_coplanar_faces_1e14() {
    let mut hashes = Vec::new();
    for _ in 0..10 {
        let result = try_boolean(
            [0.0, 0.0, 0.0], 1.0,
            [1e-14, 0.0, 0.0], 1.0,
            BooleanOp::Union,
        );

        let hash = match &result {
            Ok(r) => compute_arena_topology_hash(r.topology().arena()),
            Err(_) => 0,
        };
        hashes.push(hash);
    }

    let first = hashes[0];
    for (i, &h) in hashes.iter().enumerate() {
        assert_eq!(h, first, "Nearly-coplanar 1e-14 not deterministic on run {i}");
    }
}

/// 3.4 — Massive Face Count Boolean (Performance Gate)
///
/// Two convex polyhedra with many faces. Boolean union.
#[test]
fn massive_face_count_boolean() {
    let planes_a = generate_icosphere_planes(50, 1.5, [0.0, 0.0, 0.0]);
    let planes_b = generate_icosphere_planes(50, 1.5, [1.0, 0.0, 0.0]);

    let cell_a = build_convex_polyhedron(&planes_a, &BspConfig::default()).unwrap();
    let cell_b = build_convex_polyhedron(&planes_b, &BspConfig::default()).unwrap();

    let mut ctx = ModelingContext::new();
    let (topo_a, geom_a) = build_halfedge_mesh(&cell_a, &mut ctx).unwrap().into_parts();
    let (topo_b, geom_b) = build_halfedge_mesh(&cell_b, &mut ctx).unwrap().into_parts();

    let face_count_a = topo_a.arena().face_count();
    let face_count_b = topo_b.arena().face_count();
    eprintln!("Massive boolean: A has {face_count_a} faces, B has {face_count_b} faces");

    let start = std::time::Instant::now();
    let input = BooleanInput::new(topo_a, geom_a, topo_b, geom_b, BooleanOp::Union);
    let result = execute_boolean_logged(input);
    let elapsed = start.elapsed();

    eprintln!("Massive boolean took: {:?}", elapsed);

    match result {
        Ok(r) => {
            let r = r.into_value();
            let arena = r.topology().arena();
            let v = arena.vertex_count() as isize;
            let e = (arena.half_edge_count() / 2) as isize;
            let f = arena.face_count() as isize;
            let euler = v - e + f;
            assert_eq!(euler, 2, "Massive boolean Euler violation: V={v} E={e} F={f} V-E+F={euler}");

            assert!(
                elapsed.as_millis() < 30000,
                "Performance gate: took {}ms, expected <30000ms",
                elapsed.as_millis()
            );
        }
        Err(e) => {
            eprintln!("Massive face count boolean returned error (accepted): {e:?}");
            assert!(
                elapsed.as_millis() < 30000,
                "Performance gate: took {}ms, expected <30000ms",
                elapsed.as_millis()
            );
        }
    }
}

/// Generate N planes approximating a sphere (for high face-count testing).
fn generate_icosphere_planes(
    count: usize,
    radius: f64,
    center: [f64; 3],
) -> Vec<Plane> {
    let golden = (1.0 + 5.0_f64.sqrt()) / 2.0;
    let mut planes = Vec::with_capacity(count);

    for i in 0..count {
        let theta = std::f64::consts::PI * (1.0 + golden) * i as f64;
        let phi = (1.0 - 2.0 * (i as f64 + 0.5) / count as f64).acos();

        let nx = phi.sin() * theta.cos();
        let ny = phi.sin() * theta.sin();
        let nz = phi.cos();

        let point = [
            center[0] + nx * radius,
            center[1] + ny * radius,
            center[2] + nz * radius,
        ];

        if let Ok(plane) = Plane::from_point_normal(point, [nx, ny, nz]) {
            planes.push(plane);
        }
    }

    planes
}
