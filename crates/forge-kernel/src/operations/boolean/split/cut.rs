//! Per-face plane-cut application.
//!
//! DOMAIN: Apply a single face cut using MakeEdgeFace after the gate passes.
//! DEPENDENCIES: gate (compute_face_chord), schema (CutPoint, SplitConfig).
//! INVARIANTS:
//!   - `gate::compute_face_chord` is called first to decide IF a cut happens.
//!   - `find_cut_points_provenance` decides WHERE (vertex sign walk).
//!   - Exactly ONE cut pair is applied per call; both fragments are re-enqueued
//!     by the caller for re-testing against remaining planes.

use std::collections::{BTreeMap, BTreeSet};

use forge_core::KernelError;
use forge_core::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_geom::primitives::plane::{Plane, intersect_three_planes_exact};
use forge_math::arithmetic::Rational;
use forge_topo::handles::{FaceId, VertexId, HalfEdgeId};
use forge_topo::state::MutableDraft;
use forge_topo::traverse::FaceEdgeIterator;
use forge_topo::operator::apply_op;
use forge_topo::euler::split_edge::SplitEdge;
use forge_topo::euler::make_edge_face::MakeEdgeFace;

use crate::geometry_store::GeometryStore;
use crate::core::ModelingContext;
use crate::operations::boolean::eval::VertexMatchKey;

use super::gate::{compute_face_chord, exact_sign_for_vertex};
use super::schema::{CutPoint, EdgeCutMap, LocalVertexDedup, PlaneTable, SharedVertexRegistry, SplitConfig, make_edge_key};

/// Split a face by a cutting plane — applies exactly ONE cut pair per call.
///
/// Gate: `compute_face_chord` decides IF the face needs cutting.
/// Location: `find_cut_points_provenance` decides WHERE (vertex sign walk).
///
/// Returns `[new_face, original_face]` on success so the caller can
/// re-enqueue both with the current cut plane prepended.
pub fn split_face_by_plane(
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    dedup: &mut LocalVertexDedup,
    edge_cut_map: &mut EdgeCutMap,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    split_cfg: &SplitConfig<'_>,
    shared_registry: &mut SharedVertexRegistry,
    ctx: &mut ModelingContext,
) -> Result<Vec<FaceId>, KernelError> {

    if !gate_passes(draft, geometry, face, face_plane, cut_plane, cut_plane_idx, split_cfg, ctx)? {
        return Ok(Vec::new());
    }

    let cut_points = find_cut_points_provenance(
        draft.arena(), geometry, face,
        cut_plane, cut_plane_idx,
        dedup, shared_registry, split_cfg,
    )?;

    let resolved = resolve_all_cut_points(&cut_points, draft, geometry, dedup)?;
    if resolved.len() < 2 {
        log_rejection(face, cut_plane_idx, &format!("{} resolved vertices after dedup (need >=2)", resolved.len()), ctx);
        return Ok(Vec::new());
    }

    let sorted = sort_along_cut_direction(resolved, cut_plane, geometry);
    apply_one_cut(sorted, draft, geometry, edge_cut_map, face, face_plane, cut_plane_idx, ctx)
}

/// Run the chord-gate and log a decision if rejected.
fn gate_passes(
    draft: &MutableDraft,
    geometry: &GeometryStore,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    split_cfg: &SplitConfig<'_>,
    ctx: &mut ModelingContext,
) -> Result<bool, KernelError> {
    let chord = compute_face_chord(draft.arena(), geometry, face, face_plane, cut_plane, split_cfg.tolerance)?;
    if chord.is_none() {
        log_rejection(face, cut_plane_idx, "rejected by chord gate", ctx);
        return Ok(false);
    }
    Ok(true)
}

/// Resolve CutPoints to VertexIds, dedup, and validate count.
fn resolve_all_cut_points(
    cut_points: &[CutPoint],
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    dedup: &mut LocalVertexDedup,
) -> Result<Vec<VertexId>, KernelError> {
    let mut resolved: Vec<VertexId> = Vec::new();
    for cp in cut_points {
        resolved.push(resolve_cut_point(cp, draft, geometry, dedup)?);
    }
    resolved.dedup_by_key(|v| v.index());
    Ok(resolved)
}

/// Sort resolved vertices along a reference direction on the cutting plane.
fn sort_along_cut_direction(
    mut verts: Vec<VertexId>,
    cut_plane: &Plane,
    geometry: &GeometryStore,
) -> Vec<VertexId> {
    let ref_dir = forge_math::linalg::compute_perpendicular_direction(cut_plane.raw_normal());
    verts.sort_by(|a, b| {
        let pa = geometry.get_vertex_position(*a)
            .map(|p| p[0]*ref_dir[0] + p[1]*ref_dir[1] + p[2]*ref_dir[2])
            .unwrap_or(0.0);
        let pb = geometry.get_vertex_position(*b)
            .map(|p| p[0]*ref_dir[0] + p[1]*ref_dir[1] + p[2]*ref_dir[2])
            .unwrap_or(0.0);
        pa.partial_cmp(&pb).unwrap_or(std::cmp::Ordering::Equal)
    });
    verts
}

/// Apply ONE MakeEdgeFace cut from sorted vertex pairs.
///
/// Skips pairs that are already adjacent on the face (no-op cuts)
/// and tries each non-adjacent pair until one succeeds.
fn apply_one_cut(
    sorted: Vec<VertexId>,
    draft: &mut MutableDraft,
    geometry: &mut GeometryStore,
    edge_cut_map: &mut EdgeCutMap,
    face: FaceId,
    face_plane: &Plane,
    cut_plane_idx: usize,
    ctx: &mut ModelingContext,
) -> Result<Vec<FaceId>, KernelError> {
    let adjacent = build_adjacent_pairs(draft, face)?;

    let cut_result = sorted.chunks_exact(2)
        .filter(|pair| pair[0] != pair[1])
        .filter(|pair| {
            let key = if pair[0].index() <= pair[1].index() {
                (pair[0].index(), pair[1].index())
            } else {
                (pair[1].index(), pair[0].index())
            };
            !adjacent.contains(&key)
        })
        .find_map(|pair| {
            let v_a = pair[0];
            let v_b = pair[1];
            let op = MakeEdgeFace { vertex_a: v_a, vertex_b: v_b, face };
            match apply_op(draft, op) {
                Ok(res) => {
                    edge_cut_map.insert(make_edge_key(v_a, v_b), cut_plane_idx);
                    let new_face = res.get_value().new_face;
                    geometry.set_face_plane(new_face, face_plane.clone());
                    log_split_success(face, cut_plane_idx, new_face, ctx);
                    Some(vec![new_face, face])
                }
                Err(_) => None,
            }
        });

    if let Some(result) = cut_result {
        return Ok(result);
    }

    log_rejection(face, cut_plane_idx, "no valid cut pair found", ctx);
    Ok(Vec::new())
}

/// Build the set of vertex pairs already adjacent on a face.
fn build_adjacent_pairs(
    draft: &MutableDraft,
    face: FaceId,
) -> Result<BTreeSet<(u32, u32)>, KernelError> {
    let edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), face)?
        .collect::<Result<Vec<_>, _>>()?;
    let mut pairs = BTreeSet::new();
    for he in &edges {
        let origin = draft.arena().get_half_edge(*he)?.origin();
        let next_he = draft.arena().get_half_edge(*he)?.next();
        let dest = draft.arena().get_half_edge(next_he)?.origin();
        let key = if origin.index() <= dest.index() {
            (origin.index(), dest.index())
        } else {
            (dest.index(), origin.index())
        };
        pairs.insert(key);
    }
    Ok(pairs)
}

// ── Cut-point location (vertex sign walk) ───────────────────────────────────

/// Find where the cut plane enters the face boundary — exact sign-walk.
///
/// Walks every edge of the face:
/// - Origin vertex with TriSign::Zero → existing vertex CutPoint
/// - Edge crossing Pos↔Neg → new vertex CutPoint on the edge
fn find_cut_points_provenance(
    arena: &forge_topo::arena::TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    dedup: &LocalVertexDedup,
    shared_registry: &mut SharedVertexRegistry,
    split_cfg: &SplitConfig<'_>,
) -> Result<Vec<CutPoint>, KernelError> {
    let edges: Vec<_> = FaceEdgeIterator::new(arena, face)?
        .collect::<Result<Vec<_>, _>>()?;
    let mut points = Vec::new();

    for he in edges {
        let he_data = arena.get_half_edge(he)?;
        let origin = he_data.origin();
        let next_data = arena.get_half_edge(he_data.next())?;
        let dest = next_data.origin();

        if let Some(p_o) = geometry.get_vertex_position(origin) {
            if let Some(p_d) = geometry.get_vertex_position(dest) {
                let s_o = exact_sign_for_vertex(geometry, origin, p_o, cut_plane, cut_plane_idx);
                let s_d = exact_sign_for_vertex(geometry, dest, p_d, cut_plane, cut_plane_idx);

                if s_o == forge_math::sign::TriSign::Zero && s_d != forge_math::sign::TriSign::Zero {
                    points.push(CutPoint::Existing(origin));
                } else if is_sign_crossing(s_o, s_d) {
                    let cp = compute_crossing_cut_point(
                        arena, geometry, he, face,
                        cut_plane, cut_plane_idx,
                        p_o, p_d,
                        dedup, shared_registry, split_cfg,
                    )?;
                    points.push(cp);
                }
            }
        }
    }

    Ok(points)
}

/// True when signs indicate a Pos↔Neg edge crossing.
fn is_sign_crossing(s_o: forge_math::sign::TriSign, s_d: forge_math::sign::TriSign) -> bool {
    (s_o == forge_math::sign::TriSign::Pos && s_d == forge_math::sign::TriSign::Neg)
 || (s_o == forge_math::sign::TriSign::Neg && s_d == forge_math::sign::TriSign::Pos)
}

/// Compute the CutPoint for a Pos↔Neg edge crossing.
///
/// Attempts exact 3-plane intersection when the twin face has a different
/// plane, otherwise falls back to edge-plane intersection.
fn compute_crossing_cut_point(
    arena: &forge_topo::arena::TopologyArena,
    _geometry: &GeometryStore,
    he: HalfEdgeId,
    face: FaceId,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    p_o: &[f64; 3],
    p_d: &[f64; 3],
    dedup: &LocalVertexDedup,
    shared_registry: &mut SharedVertexRegistry,
    split_cfg: &SplitConfig<'_>,
) -> Result<CutPoint, KernelError> {
    let he_data = arena.get_half_edge(he)?;
    let twin = he_data.twin();
    let twin_face = arena.get_half_edge(twin)?.face();

    let p_face_idx = *split_cfg.face_plane_map.get(&face).unwrap_or(&0);
    let p_twin_idx = *split_cfg.face_plane_map.get(&twin_face).unwrap_or(&p_face_idx);

    let (exact_pos, computed_pos, symbolic_planes) = compute_intersection_position(
        p_face_idx, p_twin_idx, cut_plane_idx,
        split_cfg.plane_table, cut_plane,
        p_o, p_d, split_cfg.tolerance,
    );

    let provenance = build_provenance(&exact_pos, computed_pos);
    let canonical_pos = shared_registry.canonical_position(&provenance, computed_pos);

    if let Some(vid) = dedup.find_by_provenance(&provenance) {
        return Ok(CutPoint::Existing(vid));
    }

    Ok(CutPoint::NewOnEdge {
        half_edge: he,
        provenance,
        position: canonical_pos,
        exact_position: exact_pos,
        symbolic_planes,
    })
}

/// Compute the intersection position for a new cut vertex.
///
/// If the face and twin have different planes, uses exact 3-plane
/// intersection. Otherwise falls back to edge-plane intersection
/// with f64→Rational promotion.
fn compute_intersection_position(
    face_plane_idx: usize,
    twin_plane_idx: usize,
    cut_plane_idx: usize,
    plane_table: &PlaneTable,
    cut_plane: &Plane,
    p_o: &[f64; 3],
    p_d: &[f64; 3],
    config: &crate::core::ToleranceConfig,
) -> (Option<[Rational; 3]>, [f64; 3], Option<[usize; 3]>) {
    if face_plane_idx != twin_plane_idx {
        let p0 = plane_table.get(face_plane_idx);
        let p1 = plane_table.get(twin_plane_idx);
        let p2 = plane_table.get(cut_plane_idx);
        match intersect_three_planes_exact(p0, p1, p2) {
            Ok(ep) => {
                let fx = ep[0].to_f64_approx();
                let fy = ep[1].to_f64_approx();
                let fz = ep[2].to_f64_approx();
                let f64_pos = if fx.is_finite() && fy.is_finite() && fz.is_finite() {
                    [fx, fy, fz]
                } else {
                    forge_geom::primitives::plane::intersect_edge_plane(cut_plane, p_o, p_d, config.get_edge_split_degeneracy())
                };
                (Some(ep), f64_pos, Some([face_plane_idx, twin_plane_idx, cut_plane_idx]))
            }
            Err(_) => (None, forge_geom::primitives::plane::intersect_edge_plane(cut_plane, p_o, p_d, config.get_edge_split_degeneracy()), None),
        }
    } else {
        let f64_pos = forge_geom::primitives::plane::intersect_edge_plane(cut_plane, p_o, p_d, config.get_edge_split_degeneracy());
        let ep = Rational::try_from_f64_3(&f64_pos);
        (ep, f64_pos, None)
    }
}

// ── Decision logging ─────────────────────────────────────────────────────────

/// Log a face-cut rejection decision.
fn log_rejection(face: FaceId, cut_plane_idx: usize, reason: &str, ctx: &mut ModelingContext) {
    let mut decision = TracedDecision::new(
        DecisionId(face.index() as u64),
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!("Face #{} {reason} (plane #{cut_plane_idx})", face.index()),
        },
    );
    decision.set_entity_scope(EntityRef::new("Face", face.index()));
    ctx.get_decision_log_mut().record(decision);
}

/// Log a successful face split decision.
fn log_split_success(face: FaceId, cut_plane_idx: usize, new_face: FaceId, ctx: &mut ModelingContext) {
    let mut decision = TracedDecision::new(
        DecisionId(face.index() as u64),
        DecisionKind::PolicyApplied {
            policy: forge_core::PolicyKind::CoincidentGeometry,
            default_used: true,
        },
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!("Split face #{} by plane #{} -> new face #{}",
                face.index(), cut_plane_idx, new_face.index()),
        },
    );
    decision.set_entity_scope(EntityRef::new("Face", face.index()));
    ctx.get_decision_log_mut().record(decision);
}

// ── Utilities ────────────────────────────────────────────────────────────────

/// Resolve a CutPoint to a concrete VertexId, performing SplitEdge when needed.
pub fn resolve_cut_point(
    cp: &CutPoint,
    draft: &mut MutableDraft,
    geom: &mut GeometryStore,
    dedup: &mut LocalVertexDedup,
) -> Result<VertexId, KernelError> {
    match cp {
        CutPoint::Existing(v) => Ok(*v),
        CutPoint::NewOnEdge { half_edge, provenance, position, exact_position, symbolic_planes } => {
            let res = apply_op(draft, SplitEdge { edge: *half_edge, parameter: 0.5 })?;
            let v = res.get_value().new_vertex;
            if let Some(exact) = exact_position {
                if let Some(planes) = symbolic_planes {
                    geom.set_vertex_position_symbolic(v, exact.clone(), *position, *planes);
                } else {
                    geom.set_vertex_position_exact(v, exact.clone());
                }
            } else {
                geom.set_vertex_position(v, *position);
            }
            dedup.insert(v, provenance.clone());
            Ok(v)
        }
    }
}

/// Build a provenance key from an optional exact position.
fn build_provenance(exact_pos: &Option<[Rational; 3]>, fallback: [f64; 3]) -> VertexMatchKey {
    match exact_pos {
        Some(ep) => VertexMatchKey::from_exact_position(ep[0].clone(), ep[1].clone(), ep[2].clone()),
        None => {
            let rx = Rational::try_from_f64(fallback[0]).unwrap_or_else(|_| Rational::zero());
            let ry = Rational::try_from_f64(fallback[1]).unwrap_or_else(|_| Rational::zero());
            let rz = Rational::try_from_f64(fallback[2]).unwrap_or_else(|_| Rational::zero());
            VertexMatchKey::from_exact_position(rx, ry, rz)
        }
    }
}
