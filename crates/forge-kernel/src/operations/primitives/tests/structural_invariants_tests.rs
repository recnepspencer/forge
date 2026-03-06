//! Structural invariant tests for mesh_builder output.
//!
//! Validates that every solid produced by `make_convex_solid` satisfies
//! the fundamental B-Rep invariants: Euler formula, manifold edges,
//! twin reciprocity, closed loops, outward normals, and complete geometry.

// use crate::context::ModelingContext; (removed intentionally)
// struct removed
use crate::engine::facade::SolidEnvelope;
use crate::geometry::facade::GeometryView;
use crate::operations::primitives::{make_cube, make_dodecahedron, make_tetrahedron};
use forge_topo::b_rep::TopologyArena;

use super::{test_config, OperationScope};

fn assert_euler(arena: &TopologyArena, label: &str) {
    let v = arena.vertex_count() as i64;
    let e = (arena.half_edge_count() / 2) as i64;
    let f = arena.face_count() as i64;
    let chi = v - e + f;
    assert_eq!(
        chi, 2,
        "{label}: Euler V={v} − E={e} + F={f} = {chi}, expected 2"
    );
}

fn assert_manifold_twins(arena: &TopologyArena, label: &str) {
    for (he_id, he_data) in arena.iter_half_edges() {
        let twin_id = he_data.radial_next();
        assert_ne!(
            he_id,
            twin_id,
            "{label}: HE#{} has self-twin",
            he_id.index()
        );
        let twin_data = arena.get_half_edge(twin_id).unwrap();
        assert_ne!(
            he_data.face(),
            twin_data.face(),
            "{label}: HE#{} and twin HE#{} on same face F#{}",
            he_id.index(),
            twin_id.index(),
            he_data.face().index()
        );
        assert_eq!(
            twin_data.radial_next(),
            he_id,
            "{label}: twin(twin(HE#{})) != self",
            he_id.index()
        );
    }
}

fn assert_closed_loops(arena: &TopologyArena, label: &str) {
    for (face_id, face_data) in arena.iter_faces() {
        let loop_data = arena.get_loop(face_data.loops.outer()).unwrap();
        let start_he = loop_data.half_edge();
        let mut current = start_he;
        let mut count = 0;
        loop {
            count += 1;
            current = arena.get_half_edge(current).unwrap().next();
            if current == start_he {
                break;
            }
            assert!(
                count < 1000,
                "{label}: F#{} loop not closed",
                face_id.index()
            );
        }
        assert!(
            count >= 3,
            "{label}: F#{} loop has {count} edges (min 3)",
            face_id.index()
        );
    }
}

fn assert_geometry_complete(arena: &TopologyArena, geom: &impl GeometryView, label: &str) {
    for (face_id, _) in arena.iter_faces() {
        assert!(
            geom.get_face_plane(face_id).is_some(),
            "{label}: F#{} missing plane",
            face_id.index()
        );
    }
    for (vert_id, _) in arena.iter_vertices() {
        assert!(
            geom.get_vertex_position(vert_id).is_some(),
            "{label}: V#{} missing position",
            vert_id.index()
        );
    }
}

fn compute_mesh_centroid(arena: &TopologyArena, geom: &impl GeometryView) -> [f64; 3] {
    let mut sum = [0.0; 3];
    let mut count = 0;
    for (vid, _) in arena.iter_vertices() {
        if let Some(pos) = geom.get_vertex_position(vid) {
            sum[0] += pos[0];
            sum[1] += pos[1];
            sum[2] += pos[2];
            count += 1;
        }
    }
    if count == 0 {
        return [0.0; 3];
    }
    [
        sum[0] / count as f64,
        sum[1] / count as f64,
        sum[2] / count as f64,
    ]
}

fn compute_face_centroid(
    arena: &TopologyArena,
    geom: &impl GeometryView,
    face_id: forge_topo::handles::FaceId,
) -> [f64; 3] {
    let face_data = arena.get_face(face_id).unwrap();
    let loop_data = arena.get_loop(face_data.loops.outer()).unwrap();
    let start_he = loop_data.half_edge();
    let mut sum = [0.0; 3];
    let mut count = 0;
    let mut current = start_he;
    loop {
        let he = arena.get_half_edge(current).unwrap();
        if let Some(pos) = geom.get_vertex_position(he.origin()) {
            sum[0] += pos[0];
            sum[1] += pos[1];
            sum[2] += pos[2];
            count += 1;
        }
        current = he.next();
        if current == start_he {
            break;
        }
    }
    if count == 0 {
        return [0.0; 3];
    }
    [
        sum[0] / count as f64,
        sum[1] / count as f64,
        sum[2] / count as f64,
    ]
}

fn assert_outward_normals(arena: &TopologyArena, geom: &impl GeometryView, label: &str) {
    let centroid = compute_mesh_centroid(arena, geom);
    for (face_id, _) in arena.iter_faces() {
        let plane = geom.get_face_plane(face_id).unwrap();
        let n = plane.normal();
        let fc = compute_face_centroid(arena, geom, face_id);
        let to_face = [
            fc[0] - centroid[0],
            fc[1] - centroid[1],
            fc[2] - centroid[2],
        ];
        let dot = n[0] * to_face[0] + n[1] * to_face[1] + n[2] * to_face[2];
        assert!(
            dot > 0.0,
            "{label}: F#{} normal points inward (dot={dot:.6})",
            face_id.index()
        );
    }
}

/// Run full invariant suite. Exported for reuse by other test modules.
pub(super) fn assert_valid_solid(result: &SolidEnvelope, label: &str) {
    let arena = result.topology().arena();
    let geom = result.geometry();
    assert_euler(arena, label);
    assert_manifold_twins(arena, label);
    assert_closed_loops(arena, label);
    assert_geometry_complete(arena, geom, label);
    assert_outward_normals(arena, geom, label);
}

#[test]
fn cube_structural_invariants() {
    let cfg = test_config();
    let res = make_cube([0.0; 3], 2.0, &cfg).unwrap();
    assert_valid_solid(res.get_value(), "cube");
}

#[test]
fn tetrahedron_structural_invariants() {
    let cfg = test_config();
    let res = make_tetrahedron([0.0; 3], 1.0, &cfg).unwrap();
    assert_valid_solid(res.get_value(), "tet");
}

#[test]
fn dodecahedron_structural_invariants() {
    let cfg = test_config();
    let res = make_dodecahedron([0.0; 3], 1.0, &cfg).unwrap();
    assert_valid_solid(res.get_value(), "dodec");
}
