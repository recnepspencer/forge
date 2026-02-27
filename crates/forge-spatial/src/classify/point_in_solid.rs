//! Ray-casting parity classifier for point-in-solid queries.
//!
//! DOMAIN: Determine whether a 3D point is inside, outside, or on the boundary
//!         of a solid represented as a closed halfedge mesh.
//!
//! ALGORITHM: Two-pass structure:
//!   Pass 1: Tolerance boundary check via `classify_point_on_face` — catches
//!           points physically on the surface before SoS perturbs them off.
//!   Pass 2: SoS parity count — casts a +X axis ray, counts face crossings
//!           using Plücker + YZ-projection winding number with SoS tie-breaking.
//!           Odd count → Inside.
//!
//! DEPENDENCIES: forge-topo (arena, handles, FaceEdgeIterator),
//!               forge-geom (Aabb), forge-math (orient2d, orient3d, TriSign).

use forge_core::{KernelError, ToleranceProvider};
use forge_math::sign::TriSign;
use forge_math::predicates::{orient2d, orient3d};
use forge_geom::Aabb;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::traverse::FaceEdgeIterator;

use super::schema::{PointClassification, SpatialAccelerator};
use super::point_on_face::{classify_point_on_face, FacePointClassification};
use super::sos::{sos_orient2d_tiebreak, sos_orient3d_tiebreak, sos_edge_crossing_yz};

/// Classify a point relative to a solid using ray-casting parity counting.
///
/// # Arguments
/// - `arena`: topology arena
/// - `vertex_positions`: maps vertex slot index → 3D position
/// - `spatial_index`: optional BVH to accelerate candidate selection
/// - `point`: the 3D query point
/// - `tolerance_provider`: per-entity tolerance for the boundary pre-check
// DEFECT(D8): classify_point_in_solid has no multi-direction retry if first ray hits degenerate components.
pub fn classify_point_in_solid(
    arena: &TopologyArena,
    vertex_positions: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    spatial_index: Option<&dyn SpatialAccelerator>,
    point: &[f64; 3],
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<PointClassification, KernelError> {
    let pos_fn = |v: VertexId| vertex_positions(v.index()).ok();

    let faces_to_check: Vec<FaceId> = if let Some(bvh) = spatial_index {
        let pt_aabb = Aabb::from_points(&[*point, *point]).unwrap();
        bvh.candidates(&pt_aabb)
    } else {
        arena.iter_faces().map(|(id, _)| id).collect()
    };

    for &face_id in &faces_to_check {
        match classify_point_on_face(arena, face_id, point, &pos_fn, tolerance_provider)? {
            FacePointClassification::Outside => {}
            _ => return Ok(PointClassification::OnBoundary(face_id)),
        }
    }

    let mut crossing_count: i32 = 0;
    let all_faces: Vec<FaceId> = arena.iter_faces().map(|(id, _)| id).collect();
    for face_id in all_faces {
        if sos_ray_intersects_face(arena, vertex_positions, face_id, point)? {
            crossing_count += 1;
        }
    }

    if crossing_count % 2 == 1 {
        Ok(PointClassification::Inside { escalation: None })
    } else {
        Ok(PointClassification::Outside { escalation: None })
    }
}

/// Resolve a classification ambiguity by symmetric perturbation along a direction.
///
/// Samples `point ± epsilon * direction` and re-runs `classify_point_in_solid`.
/// Returns `Some(classification)` only when both samples agree exactly.
pub fn classify_point_with_perturbation(
    arena: &TopologyArena,
    vertex_positions: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    spatial_index: Option<&dyn SpatialAccelerator>,
    point: &[f64; 3],
    direction: [f64; 3],
    epsilon: f64,
    tolerance_provider: &dyn ToleranceProvider,
) -> Result<Option<PointClassification>, KernelError> {
    let pos_sample = [
        point[0] + epsilon * direction[0],
        point[1] + epsilon * direction[1],
        point[2] + epsilon * direction[2],
    ];
    let neg_sample = [
        point[0] - epsilon * direction[0],
        point[1] - epsilon * direction[1],
        point[2] - epsilon * direction[2],
    ];

    let pos_class = classify_point_in_solid(arena, vertex_positions, spatial_index, &pos_sample, tolerance_provider)?;
    let neg_class = classify_point_in_solid(arena, vertex_positions, spatial_index, &neg_sample, tolerance_provider)?;

    if pos_class == neg_class { Ok(Some(pos_class)) } else { Ok(None) }
}

/// Determine if the +X axis ray from `point` intersects a face using SoS.
fn sos_ray_intersects_face(
    arena: &TopologyArena,
    vertex_positions: &dyn Fn(u32) -> Result<[f64; 3], KernelError>,
    face_id: FaceId,
    point: &[f64; 3],
) -> Result<bool, KernelError> {
    let mut verts: Vec<[f64; 3]> = Vec::new();
    for he_res in FaceEdgeIterator::new(arena, face_id)? {
        let he_id = he_res?;
        let he = arena.get_half_edge(he_id)?;
        verts.push(vertex_positions(he.origin().index())?);
    }

    if verts.len() < 3 {
        return Ok(false);
    }

    let (basis_a, basis_b, basis_c) = match find_nondegenerate_basis(&verts)? {
        Some(b) => b,
        None => return Ok(false),
    };

    let (o_nx, _) = orient2d(
        [basis_a[1], basis_a[2]],
        [basis_b[1], basis_b[2]],
        [basis_c[1], basis_c[2]],
    ).map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;

    let nx_sign = o_nx.sign();
    if nx_sign == TriSign::Zero {
        return Ok(false);
    }

    let n = verts.len();
    let mut winding: i32 = 0;
    for i in 0..n {
        let v0 = verts[i];
        let v1 = verts[(i + 1) % n];
        winding += sos_edge_crossing_yz(
            point[1], point[2],
            v0[1], v0[2],
            v1[1], v1[2],
        )?;
    }

    if winding == 0 {
        return Ok(false);
    }

    let (o3, _) = orient3d(basis_a, basis_b, basis_c, *point)
        .map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;

    let p_sign = if o3.sign() != TriSign::Zero {
        o3.sign()
    } else {
        sos_orient3d_tiebreak(basis_a, basis_b, basis_c)?
    };

    if p_sign == TriSign::Zero {
        return Ok(false);
    }

    Ok(p_sign != nx_sign)
}

/// Find the first non-collinear triplet of vertices in the list.
fn find_nondegenerate_basis(
    verts: &[[f64; 3]],
) -> Result<Option<([f64; 3], [f64; 3], [f64; 3])>, KernelError> {
    if verts.len() < 3 { return Ok(None); }

    let p0 = verts[0];
    let p1 = verts[1];

    for &pk in &verts[2..] {
        let (o, _) = orient2d(
            [p0[1], p0[2]], [p1[1], p1[2]], [pk[1], pk[2]],
        ).map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;

        if o.sign() != TriSign::Zero {
            return Ok(Some((p0, p1, pk)));
        }

        let (o_xy, _) = orient2d(
            [p0[0], p0[1]], [p1[0], p1[1]], [pk[0], pk[1]],
        ).map_err(|e| KernelError::InternalError { message: e.to_string(), context: None })?;

        if o_xy.sign() != TriSign::Zero {
            return Ok(Some((p0, p1, pk)));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use forge_core::FlatToleranceProvider;
    use forge_topo::arena::{FaceData, HalfEdgeData, LoopData, TopologyArena, VertexData};
    use forge_topo::handles::{EdgeId, HalfEdgeId, LoopId, ShellId};

    fn build_cube_arena() -> (TopologyArena, Vec<[f64; 3]>) {
        let mut arena = TopologyArena::new();

        let positions = vec![
            [-1.0, -1.0, -1.0], [ 1.0, -1.0, -1.0],
            [ 1.0,  1.0, -1.0], [-1.0,  1.0, -1.0],
            [-1.0, -1.0,  1.0], [ 1.0, -1.0,  1.0],
            [ 1.0,  1.0,  1.0], [-1.0,  1.0,  1.0],
        ];

        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);
        let placeholder_shell_q = ShellId::new(u32::MAX, 0);
        let placeholder_e_q     = EdgeId::new(u32::MAX, 0);

        let mut verts = Vec::new();
        for _ in 0..8 {
            verts.push(arena.insert_vertex(VertexData::new(placeholder_he), None));
        }

        let quad_faces: [[usize; 4]; 6] = [
            [0, 3, 2, 1], [4, 5, 6, 7],
            [0, 1, 5, 4], [2, 3, 7, 6],
            [0, 4, 7, 3], [1, 2, 6, 5],
        ];

        for quad in &quad_faces {
            let face = arena.insert_face(FaceData::new(placeholder_loop, placeholder_shell_q), None);
            let loop_id = arena.insert_loop(LoopData::new(placeholder_he, face), None);
            arena.get_face_mut(face).unwrap().set_outer_loop(loop_id);

            let mut he_ids = Vec::new();
            for i in 0..4 {
                let origin = verts[quad[i]];
                let he = arena.insert_half_edge(HalfEdgeData::new(
                    placeholder_he, placeholder_he, placeholder_he, face, origin, placeholder_e_q,
                ), None);
                he_ids.push(he);
            }
            for i in 0..4 {
                arena.get_half_edge_mut(he_ids[i]).unwrap().set_next(he_ids[(i + 1) % 4]);
                arena.get_half_edge_mut(he_ids[i]).unwrap().set_prev(he_ids[(i + 3) % 4]);
            }
            arena.get_loop_mut(loop_id).unwrap().set_half_edge(he_ids[0]);
            arena.get_vertex_mut(verts[quad[0]]).unwrap().set_outgoing(he_ids[0]);
        }

        let all_hes: Vec<(HalfEdgeId, u32, u32)> = arena.iter_half_edges()
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
                if arena.get_half_edge(he_id).unwrap().radial_next() != he_id { continue; }
            }
            for j in (i+1)..all_hes.len() {
                let (other_id, other_origin, other_target) = all_hes[j];
                if origin == other_target && target == other_origin {
                    arena.get_half_edge_mut(he_id).unwrap().set_radial_next(other_id);
                    arena.get_half_edge_mut(other_id).unwrap().set_radial_next(he_id);
                    break;
                }
            }
        }

        let unmatched: Vec<HalfEdgeId> = arena.iter_half_edges()
            .filter(|(_, data)| data.radial_next() == placeholder_he)
            .map(|(id, _)| id)
            .collect();
        for he_id in unmatched {
            arena.get_half_edge_mut(he_id).unwrap().set_radial_next(he_id);
        }

        (arena, positions)
    }

    #[test]
    fn point_inside_solid_classified_inside() {
        let tol = FlatToleranceProvider::new(1e-10);
        let (arena, positions) = build_cube_arena();
        let position_fn = |idx: u32| -> Result<[f64; 3], KernelError> {
            positions.get(idx as usize).copied().ok_or_else(|| KernelError::InternalError {
                message: format!("No position for vertex {}", idx), context: None,
            })
        };

        let inside = classify_point_in_solid(&arena, &position_fn, None, &[0.0, 0.0, 0.0], &tol).unwrap();
        assert!(matches!(inside, PointClassification::Inside { .. }), "Origin must be Inside, got {:?}", inside);

        let outside = classify_point_in_solid(&arena, &position_fn, None, &[10.0, 10.0, 10.0], &tol).unwrap();
        assert!(matches!(outside, PointClassification::Outside { .. }), "(10,10,10) must be Outside, got {:?}", outside);
    }

    #[test]
    fn point_on_face_classified_on_boundary() {
        let tol = FlatToleranceProvider::new(1e-10);
        let (arena, positions) = build_cube_arena();
        let position_fn = |idx: u32| -> Result<[f64; 3], KernelError> {
            positions.get(idx as usize).copied().ok_or_else(|| KernelError::InternalError {
                message: format!("No position for vertex {}", idx), context: None,
            })
        };

        let on_face = classify_point_in_solid(&arena, &position_fn, None, &[1.0, 0.0, 0.0], &tol).unwrap();
        assert!(matches!(on_face, PointClassification::OnBoundary(_)), "Face point must be OnBoundary, got {:?}", on_face);
    }
}
