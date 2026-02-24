//! Hole splicing orchestrator (Bridge Edge caller).
//!
//! DOMAIN: For each face with inner loops (holes), identify bridge vertices
//! via raycasting in the face's local 2D plane, then invoke the BridgeEdge
//! Euler operator to absorb each hole into the outer boundary.
//!
//! ALGORITHM:
//!   1. Find faces with inner_loop_count() > 0
//!   2. For each inner loop: find extremal vertex H_max (max X in local 2D)
//!   3. Raycast from H_max in +X against outer boundary edges
//!   4. Pick target vertex (mutually visible, closest)
//!   5. Apply BridgeEdge Euler operator
//!
//! DEPENDENCIES: forge_topo (BridgeEdge, arena), GeometryStore, forge_geom

use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_core::tracing::TopologyDelta;
use forge_topo::handles::{FaceId, HalfEdgeId, LoopId, VertexId};
use forge_topo::state::{TopologyState, MutableDraft};
use forge_topo::algorithms::bridge_edge::bridge_edge;

use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta};
use crate::geometry_store::GeometryStore;

/// Splice all inner holes on all faces into their outer boundaries.
///
/// Iterates faces with inner loops, finds bridge vertices via 2D raycasting,
/// and applies the BridgeEdge Euler operator. Returns the count of holes spliced.
pub fn splice_inner_holes(
    topo: TopologyState,
    geom: &GeometryStore,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, usize), KernelError> {
    let faces_with_holes = find_faces_with_holes(topo.arena());
    if faces_with_holes.is_empty() {
        return Ok((topo, 0));
    }

    let mut draft = topo.into_mutation();
    let pre_snapshot = ArenaSnapshot::capture(draft.arena());
    let mut total_spliced = 0usize;

    for face_id in faces_with_holes {
        let holes_on_face = get_face_inner_loops(&draft, face_id)?;
        for inner_loop_id in holes_on_face {
            let splice_result = splice_one_hole(&mut draft, face_id, inner_loop_id, geom)?;
            if splice_result {
                total_spliced += 1;
            }
        }
    }

    if total_spliced > 0 {
        let delta = compute_topology_delta(&pre_snapshot, draft.arena());
        log_splice(total_spliced, delta, ctx);
    }

    Ok((draft.commit()?, total_spliced))
}

/// Find all face IDs that have at least one inner loop.
fn find_faces_with_holes(
    arena: &forge_topo::arena::TopologyArena,
) -> Vec<FaceId> {
    arena.iter_faces()
        .filter(|(_, face_data)| face_data.inner_loop_count() > 0)
        .map(|(face_id, _)| face_id)
        .collect()
}

/// Get the inner loop IDs for a face (snapshot before mutation).
fn get_face_inner_loops(
    draft: &MutableDraft,
    face: FaceId,
) -> Result<Vec<LoopId>, KernelError> {
    let face_data = draft.arena().get_face(face)?;
    Ok(face_data.inner_loops().to_vec())
}

/// Splice one inner hole into the outer boundary of a face.
fn splice_one_hole(
    draft: &mut MutableDraft,
    face: FaceId,
    inner_loop: LoopId,
    geom: &GeometryStore,
) -> Result<bool, KernelError> {
    let face_data = draft.arena().get_face(face)?;
    let face_plane = match geom.get_face_plane(face) {
        Some(p) => p,
        None => return Ok(false),
    };

    let inner_he_start = draft.arena().get_loop(inner_loop)?.half_edge();
    let (h_max_he, h_max_vertex, h_max_pos) =
        find_extremal_vertex(draft, inner_he_start, geom, face_plane)?;

    let outer_loop = face_data.outer_loop();
    let outer_he_start = draft.arena().get_loop(outer_loop)?.half_edge();

    let (target_he, _target_vertex) =
        raycast_to_outer_boundary(draft, outer_he_start, h_max_pos, geom, face_plane)?;

    bridge_edge(draft, target_he, h_max_he)?;

    Ok(true)
}

/// Find the vertex on the inner loop with the maximum X coordinate in the face plane.
fn find_extremal_vertex(
    draft: &MutableDraft,
    start_he: HalfEdgeId,
    geom: &GeometryStore,
    plane: &forge_geom::Plane,
) -> Result<(HalfEdgeId, VertexId, [f64; 3]), KernelError> {
    let normal = plane.normal();
    let u_axis = forge_math::linalg::compute_perpendicular_direction(normal);

    let mut best_he = start_he;
    let mut best_vertex = draft.arena().get_half_edge(start_he)?.origin();
    let mut best_x = f64::NEG_INFINITY;
    let mut best_pos = [0.0, 0.0, 0.0];

    let mut current = start_he;
    let mut steps = 0usize;

    loop {
        let he_data = draft.arena().get_half_edge(current)?;
        let vid = he_data.origin();
        if let Some(pos) = geom.get_vertex_position(vid) {
            let local_x = forge_math::linalg::dot(*pos, u_axis);
            if local_x > best_x {
                best_x = local_x;
                best_he = current;
                best_vertex = vid;
                best_pos = *pos;
            }
        }

        current = he_data.next();
        steps += 1;
        if current == start_he || steps > 100_000 {
            break;
        }
    }

    if best_x == f64::NEG_INFINITY {
        return Err(KernelError::InternalError {
            message: "No vertex positions found on inner loop".to_string(),
            context: None,
        });
    }

    Ok((best_he, best_vertex, best_pos))
}

/// Raycast from h_max in +X direction to find the closest outer boundary edge.
///
/// Returns the half-edge on the outer boundary whose origin is the bridge target,
/// and the target vertex ID.
fn raycast_to_outer_boundary(
    draft: &MutableDraft,
    outer_start: HalfEdgeId,
    h_max_pos: [f64; 3],
    geom: &GeometryStore,
    plane: &forge_geom::Plane,
) -> Result<(HalfEdgeId, VertexId), KernelError> {
    let normal = plane.normal();
    let u_axis = forge_math::linalg::compute_perpendicular_direction(normal);
    let v_axis = forge_math::linalg::cross(normal, u_axis);

    let ray_origin_u = forge_math::linalg::dot(h_max_pos, u_axis);
    let ray_origin_v = forge_math::linalg::dot(h_max_pos, v_axis);

    let mut best_he: Option<HalfEdgeId> = None;
    let mut best_t = f64::MAX;

    let mut current = outer_start;
    let mut steps = 0usize;

    loop {
        let he_data = draft.arena().get_half_edge(current)?;
        let origin_vid = he_data.origin();
        let dest_vid = {
            let next_he = draft.arena().get_half_edge(he_data.next())?;
            next_he.origin()
        };

        if let (Some(p_origin), Some(p_dest)) = (
            geom.get_vertex_position(origin_vid),
            geom.get_vertex_position(dest_vid),
        ) {
            let o_u = forge_math::linalg::dot(*p_origin, u_axis);
            let o_v = forge_math::linalg::dot(*p_origin, v_axis);
            let d_u = forge_math::linalg::dot(*p_dest, u_axis);
            let d_v = forge_math::linalg::dot(*p_dest, v_axis);

            let t = compute_ray_edge_intersection(
                ray_origin_u, ray_origin_v,
                o_u, o_v, d_u, d_v,
            );

            if let Some(t_val) = t {
                if t_val > 0.0 && t_val < best_t {
                    best_t = t_val;
                    best_he = Some(current);
                }
            }
        }

        current = he_data.next();
        steps += 1;
        if current == outer_start || steps > 100_000 {
            break;
        }
    }

    let hit_he = best_he.ok_or_else(|| KernelError::InternalError {
        message: "Raycast found no intersection with outer boundary".to_string(),
        context: None,
    })?;

    let hit_data = draft.arena().get_half_edge(hit_he)?;
    let hit_next_data = draft.arena().get_half_edge(hit_data.next())?;
    let origin_vid = hit_data.origin();
    let dest_vid = hit_next_data.origin();

    let target = pick_closer_vertex(
        draft, geom, h_max_pos, origin_vid, dest_vid, hit_he,
    )?;

    Ok(target)
}

/// Compute the ray-edge intersection parameter t for a +X ray.
///
/// Ray: (ray_u + t, ray_v) for t > 0
/// Edge: from (o_u, o_v) to (d_u, d_v)
///
/// Returns Some(t) if the ray crosses the edge, None otherwise.
fn compute_ray_edge_intersection(
    ray_u: f64, ray_v: f64,
    o_u: f64, o_v: f64,
    d_u: f64, d_v: f64,
) -> Option<f64> {
    let dv = d_v - o_v;
    if dv.abs() < 1e-15 {
        return None;
    }

    let s = (ray_v - o_v) / dv;
    if s < 0.0 || s > 1.0 {
        return None;
    }

    let intersect_u = o_u + s * (d_u - o_u);
    let t = intersect_u - ray_u;

    if t > 1e-15 {
        Some(t)
    } else {
        None
    }
}

/// Pick the closer of two edge endpoints to h_max as the bridge target.
fn pick_closer_vertex(
    draft: &MutableDraft,
    geom: &GeometryStore,
    h_max_pos: [f64; 3],
    origin_vid: VertexId,
    dest_vid: VertexId,
    hit_he: HalfEdgeId,
) -> Result<(HalfEdgeId, VertexId), KernelError> {
    let p_origin = geom.get_vertex_position(origin_vid);
    let p_dest = geom.get_vertex_position(dest_vid);

    match (p_origin, p_dest) {
        (Some(po), Some(pd)) => {
            let dist_o = forge_math::linalg::norm_sq(forge_math::linalg::sub(*po, h_max_pos));
            let dist_d = forge_math::linalg::norm_sq(forge_math::linalg::sub(*pd, h_max_pos));
            if dist_o <= dist_d {
                Ok((hit_he, origin_vid))
            } else {
                let next_he = draft.arena().get_half_edge(hit_he)?.next();
                Ok((next_he, dest_vid))
            }
        }
        (Some(_), None) => Ok((hit_he, origin_vid)),
        (None, Some(_)) => {
            let next_he = draft.arena().get_half_edge(hit_he)?.next();
            Ok((next_he, dest_vid))
        }
        (None, None) => Err(KernelError::InternalError {
            message: "Neither vertex of hit edge has a position".to_string(),
            context: None,
        }),
    }
}

/// Log a hole splice decision.
fn log_splice(count: usize, delta: TopologyDelta, ctx: &mut ModelingContext) {
    let mut decision = TracedDecision::new(
        DecisionId(0),
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!("Spliced {} inner holes into outer boundaries", count),
        },
    );
    decision.set_topology_delta(delta);
    ctx.get_decision_log_mut().record(decision);
}
