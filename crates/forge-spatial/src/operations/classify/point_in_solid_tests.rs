//! Point-in-solid classification tests.
//!
//! DOMAIN: Validates classify_point_in_solid against a manually-built cube arena.
//! Tests cover interior, exterior, boundary, near-boundary, and mass classification.

use forge_core::{FlatToleranceProvider, KernelError};
use forge_topo::b_rep::{FaceData, HalfEdgeData, LoopData, TopologyArena, VertexData};
use forge_topo::handles::{EdgeId, HalfEdgeId, LoopId, ShellId};

use super::point_in_solid::classify_point_in_solid;
use super::schema::PointClassification;

fn build_cube_arena() -> (TopologyArena, Vec<[f64; 3]>) {
    let mut arena = TopologyArena::new();

    let positions = vec![
        [-1.0, -1.0, -1.0],
        [1.0, -1.0, -1.0],
        [1.0, 1.0, -1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, -1.0, 1.0],
        [1.0, -1.0, 1.0],
        [1.0, 1.0, 1.0],
        [-1.0, 1.0, 1.0],
    ];

    let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
    let placeholder_loop = LoopId::new(u32::MAX, 0);
    let placeholder_shell_q = ShellId::new(u32::MAX, 0);
    let placeholder_e_q = EdgeId::new(u32::MAX, 0);

    let mut verts = Vec::new();
    for _ in 0..8 {
        verts.push(arena.insert_vertex(VertexData::new(placeholder_he)));
    }

    let quad_faces: [[usize; 4]; 6] = [
        [0, 3, 2, 1],
        [4, 5, 6, 7],
        [0, 1, 5, 4],
        [2, 3, 7, 6],
        [0, 4, 7, 3],
        [1, 2, 6, 5],
    ];

    for quad in &quad_faces {
        let face = arena.insert_face(FaceData::new(placeholder_loop, placeholder_shell_q));
        let loop_id = arena.insert_loop(LoopData::new(placeholder_he, face));
        arena.get_face_mut(face).unwrap().set_outer_loop(loop_id);

        let mut he_ids = Vec::new();
        for i in 0..4 {
            let origin = verts[quad[i]];
            let he = arena.insert_half_edge(HalfEdgeData::new(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                face,
                origin,
                placeholder_e_q,
            ));
            he_ids.push(he);
        }
        for i in 0..4 {
            arena
                .get_half_edge_mut(he_ids[i])
                .unwrap()
                .set_next(he_ids[(i + 1) % 4]);
            arena
                .get_half_edge_mut(he_ids[i])
                .unwrap()
                .set_prev(he_ids[(i + 3) % 4]);
        }
        arena
            .get_loop_mut(loop_id)
            .unwrap()
            .set_half_edge(he_ids[0]);
        arena
            .get_vertex_mut(verts[quad[0]])
            .unwrap()
            .set_primary_disk(he_ids[0]);
    }

    let all_hes: Vec<(HalfEdgeId, u32, u32)> = arena
        .iter_half_edges()
        .map(|(id, data)| {
            let origin = data.origin().index();
            let next_he = arena.get_half_edge(data.next()).unwrap();
            let target = next_he.origin().index();
            (id, origin, target)
        })
        .collect();

    for i in 0..all_hes.len() {
        let (he_id, origin, target) = all_hes[i];
        if arena.get_half_edge(he_id).unwrap().radial_next() != placeholder_he {
            if arena.get_half_edge(he_id).unwrap().radial_next() != he_id {
                continue;
            }
        }
        for j in (i + 1)..all_hes.len() {
            let (other_id, other_origin, other_target) = all_hes[j];
            if origin == other_target && target == other_origin {
                arena
                    .get_half_edge_mut(he_id)
                    .unwrap()
                    .set_radial_next(other_id);
                arena
                    .get_half_edge_mut(other_id)
                    .unwrap()
                    .set_radial_next(he_id);
                break;
            }
        }
    }

    let unmatched: Vec<HalfEdgeId> = arena
        .iter_half_edges()
        .filter(|(_, data)| data.radial_next() == placeholder_he)
        .map(|(id, _)| id)
        .collect();
    for he_id in unmatched {
        arena
            .get_half_edge_mut(he_id)
            .unwrap()
            .set_radial_next(he_id);
    }

    (arena, positions)
}

fn position_fn(positions: &[[f64; 3]]) -> impl Fn(u32) -> Result<[f64; 3], KernelError> + '_ {
    move |idx: u32| {
        positions
            .get(idx as usize)
            .copied()
            .ok_or_else(|| KernelError::InternalError {
                message: format!("No position for vertex index {}", idx),
                context: None,
            })
    }
}

#[test]
fn point_inside_solid_classified_inside() {
    let tol = FlatToleranceProvider::new(1e-10);
    let (arena, positions) = build_cube_arena();
    let pos_fn = position_fn(&positions);

    let inside = classify_point_in_solid(&arena, &pos_fn, None, &[0.0, 0.0, 0.0], &tol).unwrap();
    assert!(
        matches!(inside, PointClassification::Inside { .. }),
        "Origin must be Inside, got {:?}",
        inside
    );

    let outside =
        classify_point_in_solid(&arena, &pos_fn, None, &[10.0, 10.0, 10.0], &tol).unwrap();
    assert!(
        matches!(outside, PointClassification::Outside { .. }),
        "(10,10,10) must be Outside, got {:?}",
        outside
    );
}

#[test]
fn point_on_face_classified_on_boundary() {
    let tol = FlatToleranceProvider::new(1e-10);
    let (arena, positions) = build_cube_arena();
    let pos_fn = position_fn(&positions);

    let on_face = classify_point_in_solid(&arena, &pos_fn, None, &[1.0, 0.0, 0.0], &tol).unwrap();
    assert!(
        matches!(on_face, PointClassification::OnBoundary(_)),
        "Face point must be OnBoundary, got {:?}",
        on_face
    );
}

#[test]
fn point_outside_left_classified_outside() {
    let tol = FlatToleranceProvider::new(1e-10);
    let (arena, positions) = build_cube_arena();
    let pos_fn = position_fn(&positions);

    let outside = classify_point_in_solid(&arena, &pos_fn, None, &[-5.0, 0.0, 0.0], &tol).unwrap();
    assert!(
        matches!(outside, PointClassification::Outside { .. }),
        "Point (-5,0,0) must be Outside, got {:?}",
        outside
    );
}

#[test]
fn point_near_boundary_inside() {
    let tol = FlatToleranceProvider::new(1e-10);
    let (arena, positions) = build_cube_arena();
    let pos_fn = position_fn(&positions);

    let just_inside =
        classify_point_in_solid(&arena, &pos_fn, None, &[0.99, 0.0, 0.0], &tol).unwrap();
    assert!(
        matches!(just_inside, PointClassification::Inside { .. }),
        "Point (0.99,0,0) must be Inside, got {:?}",
        just_inside
    );
}

#[test]
fn point_near_boundary_outside() {
    let tol = FlatToleranceProvider::new(1e-10);
    let (arena, positions) = build_cube_arena();
    let pos_fn = position_fn(&positions);

    let just_outside =
        classify_point_in_solid(&arena, &pos_fn, None, &[1.01, 0.0, 0.0], &tol).unwrap();
    assert!(
        matches!(just_outside, PointClassification::Outside { .. }),
        "Point (1.01,0,0) must be Outside, got {:?}",
        just_outside
    );
}

#[test]
fn mass_classification_grid() {
    let tol = FlatToleranceProvider::new(1e-10);
    let (arena, positions) = build_cube_arena();
    let pos_fn = position_fn(&positions);

    let mut inside_count = 0usize;
    let mut outside_count = 0usize;
    let mut boundary_count = 0usize;
    let mut error_count = 0usize;

    let steps = 10;
    for ix in 0..steps {
        for iy in 0..steps {
            for iz in 0..steps {
                let x = -2.0 + 4.0 * (ix as f64) / (steps as f64 - 1.0);
                let y = -2.0 + 4.0 * (iy as f64) / (steps as f64 - 1.0);
                let z = -2.0 + 4.0 * (iz as f64) / (steps as f64 - 1.0);

                match classify_point_in_solid(&arena, &pos_fn, None, &[x, y, z], &tol) {
                    Ok(PointClassification::Inside { .. }) => inside_count += 1,
                    Ok(PointClassification::Outside { .. }) => outside_count += 1,
                    Ok(PointClassification::OnBoundary(_)) => boundary_count += 1,
                    Err(_) => error_count += 1,
                }
            }
        }
    }

    let total = inside_count + outside_count + boundary_count + error_count;
    assert_eq!(total, steps * steps * steps, "All points must be processed");
    assert!(inside_count > 0, "Must have some interior points, got 0");
    assert!(outside_count > 0, "Must have some exterior points, got 0");
    assert!(
        inside_count < outside_count,
        "Cube [-1,1]³ in [-2,2]³ grid: outside ({}) must exceed inside ({})",
        outside_count,
        inside_count
    );
}
