//! Per-face plane-cut logic.
//!
//! DOMAIN: Split a single face by a cutting plane using MakeEdgeFace.
//! DEPENDENCIES: schema (CutPoint, EdgeCutMap, LocalVertexDedup, SharedVertexRegistry, PlaneTable).
//! INVARIANTS:
//!   - `compute_face_chord` is the SOLE GATE for whether a face needs cutting.
//!     It replaces the old `has_vertices_on_both_sides` gate with the
//!     literature-correct approach (Thibault-Naylor/CGAL/Cherchi): compute the
//!     intersection LINE of the two planes and clip it to the face polygon.
//!   - Once the gate passes, cut-point LOCATIONS are found via the original
//!     vertex-sign-walk in `find_cut_points_provenance` — this part is unchanged
//!     from the well-tested original and ensures exact provenance tracking.
//!   - Exactly ONE cut pair is applied per call; both fragments are re-enqueued
//!     by the caller for re-testing against the same and remaining planes.

use std::collections::{BTreeMap, BTreeSet};

use forge_core::KernelError;
use forge_core::result::{TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_geom::primitives::plane::{Plane, classify_point_exact, intersect_three_planes_exact};
use forge_geom::{compute_intersection_line, clip_line_to_face_polygon};
use forge_math::arithmetic::Rational;
use forge_math::sign::TriSign;
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{FaceId, VertexId, HalfEdgeId};
use forge_topo::state::MutableDraft;
use forge_topo::traverse::FaceEdgeIterator;
use forge_topo::operator::apply_op;
use forge_topo::euler::split_edge::SplitEdge;
use forge_topo::euler::make_edge_face::MakeEdgeFace;

use crate::geometry_store::GeometryStore;
use crate::core::ModelingContext;
use crate::operations::boolean::eval::VertexMatchKey;

use super::schema::{CutPoint, EdgeCutMap, LocalVertexDedup, PlaneTable, SharedVertexRegistry, make_edge_key};



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
    _face_plane_idx: usize,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    config: &crate::core::ToleranceConfig,
    plane_table: &PlaneTable,
    face_plane_map: &BTreeMap<FaceId, usize>,
    shared_registry: &mut SharedVertexRegistry,
    ctx: &mut ModelingContext,
) -> Result<Vec<FaceId>, KernelError> {

    // ── Gate: chord-clip decides IF this face needs cutting ──────────────────
    let chord = compute_face_chord(draft.arena(), geometry, face, face_plane, cut_plane, config)?;
    if chord.is_none() {
        let mut decision = TracedDecision::new(
            DecisionId(face.index() as u64),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: format!("Face #{} rejected by chord gate (plane #{})",
                    face.index(), cut_plane_idx),
            },
        );
        decision.set_entity_scope(EntityRef::new("Face", face.index()));
        ctx.get_decision_log_mut().record(decision);
        return Ok(Vec::new());
    }

    // ── Location: vertex sign-walk finds WHERE to cut ────────────────────────
    let cut_points = find_cut_points_provenance(
        draft.arena(), geometry, face,
        cut_plane, cut_plane_idx,
        dedup, face_plane_map, edge_cut_map, shared_registry, plane_table,
        config,
    )?;

    if cut_points.len() < 2 {
        let mut decision = TracedDecision::new(
            DecisionId(face.index() as u64),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: format!("Face #{} rejected: {} cut point(s) found (need >=2)",
                    face.index(), cut_points.len()),
            },
        );
        decision.set_entity_scope(EntityRef::new("Face", face.index()));
        ctx.get_decision_log_mut().record(decision);
        return Ok(Vec::new());
    }

    // ── Resolve CutPoints to VertexIds ───────────────────────────────────────
    let mut resolved: Vec<VertexId> = Vec::new();
    for cp in &cut_points {
        let vid = resolve_cut_point(cp, draft, geometry, dedup)?;
        resolved.push(vid);
    }

    resolved.dedup_by_key(|v| v.index());
    if resolved.len() < 2 {
        let mut decision = TracedDecision::new(
            DecisionId(face.index() as u64),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: format!("Face #{} rejected: {} resolved vertex/vertices after dedup (need >=2)",
                    face.index(), resolved.len()),
            },
        );
        decision.set_entity_scope(EntityRef::new("Face", face.index()));
        ctx.get_decision_log_mut().record(decision);
        return Ok(Vec::new());
    }

    // ── Sort along a reference direction on the cutting plane ────────────────
    let cut_normal = cut_plane.raw_normal();
    let ref_direction = forge_math::linalg::compute_perpendicular_direction(cut_normal);

    resolved.sort_by(|a, b| {
        let pa = geometry.get_vertex_position(*a)
            .map(|p| p[0]*ref_direction[0] + p[1]*ref_direction[1] + p[2]*ref_direction[2])
            .unwrap_or(0.0);
        let pb = geometry.get_vertex_position(*b)
            .map(|p| p[0]*ref_direction[0] + p[1]*ref_direction[1] + p[2]*ref_direction[2])
            .unwrap_or(0.0);
        pa.partial_cmp(&pb).unwrap_or(std::cmp::Ordering::Equal)
    });

    // ── Build adjacent-pairs set to avoid redundant cuts ────────────────────
    let face_edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), face)?
        .collect::<Result<Vec<_>, _>>()?;
    let mut adjacent_pairs: BTreeSet<(u32, u32)> = BTreeSet::new();
    for he in &face_edges {
        let origin = draft.arena().get_half_edge(*he)?.origin();
        let next_he = draft.arena().get_half_edge(*he)?.next();
        let dest = draft.arena().get_half_edge(next_he)?.origin();
        let key = if origin.index() <= dest.index() {
            (origin.index(), dest.index())
        } else {
            (dest.index(), origin.index())
        };
        adjacent_pairs.insert(key);
    }

    // ── Apply ONE MakeEdgeFace cut ────────────────────────────────────────────
    let cut_result = resolved.chunks_exact(2)
        .filter(|pair| pair[0] != pair[1])
        .filter(|pair| {
            let key = if pair[0].index() <= pair[1].index() {
                (pair[0].index(), pair[1].index())
            } else {
                (pair[1].index(), pair[0].index())
            };
            if adjacent_pairs.contains(&key) {
                false
            } else {
                true
            }
        })
        .find_map(|pair| {
            let v_a = pair[0];
            let v_b = pair[1];
            let op = MakeEdgeFace { vertex_a: v_a, vertex_b: v_b, face };
            match apply_op(draft, op) {
                Ok(res) => {
                    let edge_key = make_edge_key(v_a, v_b);
                    edge_cut_map.insert(edge_key, cut_plane_idx);

                    let new_face = res.get_value().new_face;
                    geometry.set_face_plane(new_face, face_plane.clone());

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

                    Some(vec![new_face, face])
                }
                Err(_) => {
                    None
                }
            }
        });

    if let Some(result) = cut_result {
        return Ok(result);
    }

    let mut decision = TracedDecision::new(
        DecisionId(face.index() as u64),
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!("Face #{} fell through: no valid cut pair found for plane #{}",
                face.index(), cut_plane_idx),
        },
    );
    decision.set_entity_scope(EntityRef::new("Face", face.index()));
    ctx.get_decision_log_mut().record(decision);
    Ok(Vec::new())
}

/// Gate: does the cut_plane produce an interior chord segment in this face?
///
/// Literature-correct gate (Thibault-Naylor/CGAL/Cherchi): compute the
/// intersection LINE of face_plane and cut_plane, then clip it to the face
/// polygon via Cyrus-Beck. Returns None for parallel planes, faces not
/// intersected, or near-zero-length chords (grazing cuts).
///
/// This replaces the old `has_vertices_on_both_sides` gate, which missed
/// the "all vertices on one side but boundary exactly on the plane" case
/// that arises from coplanar boundaries in chained boolean operations.
fn compute_face_chord(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
    config: &crate::core::ToleranceConfig,
) -> Result<Option<([f64; 3], [f64; 3])>, KernelError> {
    if forge_geom::primitives::plane::are_parallel_exact(face_plane, cut_plane) {
        return Ok(None);
    }

    let fn_a = face_plane.normal();
    let fo_a = face_plane.offset();
    let fn_b = cut_plane.normal();
    let fo_b = cut_plane.offset();
    let min_chord = config.get_min_edge_length();

    let line = compute_intersection_line(fn_a, fo_a, fn_b, fo_b, config.get_degeneracy());
    let (line_pt, line_dir) = match line {
        None => return Ok(None),
        Some(l) => l,
    };

    let edges: Vec<_> = FaceEdgeIterator::new(arena, face)?
        .collect::<Result<Vec<_>, _>>()?;

    let mut verts: Vec<[f64; 3]> = Vec::with_capacity(edges.len());
    for he in &edges {
        let v = arena.get_half_edge(*he)?.origin();
        if let Some(p) = geometry.get_vertex_position(v) {
            verts.push(*p);
        }
    }

    if verts.len() < 3 {
        return Ok(None);
    }

    let chord = clip_line_to_face_polygon(line_pt, line_dir, &verts, fn_a, min_chord);
    if chord.is_some() {
        return Ok(chord);
    }
    // Fallback 1: winding may be CW relative to stored plane normal.
    let fn_a_neg = [-fn_a[0], -fn_a[1], -fn_a[2]];
    let chord_neg = clip_line_to_face_polygon(line_pt, line_dir, &verts, fn_a_neg, min_chord);
    if chord_neg.is_some() {
        return Ok(chord_neg);
    }
    // Fallback 2: for pre-cut fragment faces the Cyrus-Beck polygon may be
    // numerically degenerate (very thin or non-convex strip). If the vertex
    // sign walk would find a Pos↔Neg crossing, treat that as a valid gate.
    // Return a synthetic chord (start, end) from the first two crossing midpoints.
    let edges2: Vec<_> = FaceEdgeIterator::new(arena, face)?
        .collect::<Result<Vec<_>, _>>()?;
    let mut crossings: Vec<[f64; 3]> = Vec::new();
    for he in &edges2 {
        let he_data = arena.get_half_edge(*he)?;
        let origin = he_data.origin();
        let next_data = arena.get_half_edge(he_data.next())?;
        let dest = next_data.origin();
        if let (Some(p_o), Some(p_d)) = (geometry.get_vertex_position(origin), geometry.get_vertex_position(dest)) {
            let s_o = exact_sign_for_vertex(geometry, origin, p_o, cut_plane);
            let s_d = exact_sign_for_vertex(geometry, dest, p_d, cut_plane);
            let is_crossing = (s_o == TriSign::Pos && s_d == TriSign::Neg)
                           || (s_o == TriSign::Neg && s_d == TriSign::Pos);
            if is_crossing {
                let mid = forge_geom::primitives::plane::intersect_edge_plane(cut_plane, p_o, p_d, config.get_edge_split_degeneracy());
                crossings.push(mid);
            } else if s_o == TriSign::Zero {
                crossings.push(*p_o);
            }
        }
        if crossings.len() >= 2 {
            // Stop scanning: we have enough crossings.
        }
    }
    if crossings.len() >= 2 {
        return Ok(Some((crossings[0], crossings[1])));
    }

    Ok(None)
}

/// Find where the cut plane enters the face boundary — exact sign-walk.
///
/// Ported from the original well-tested `find_cut_points_provenance`.
/// Walks every edge of the face:
/// - Origin vertex with TriSign::Zero → existing vertex CutPoint
/// - Edge crossing Pos↔Neg → new vertex CutPoint on the edge
///
/// Returns the ordered list of cut points (usually 2 for a convex face).
fn find_cut_points_provenance(
    arena: &TopologyArena,
    geometry: &GeometryStore,
    face: FaceId,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    dedup: &LocalVertexDedup,
    face_plane_map: &BTreeMap<FaceId, usize>,
    _edge_cut_map: &EdgeCutMap,
    shared_registry: &mut SharedVertexRegistry,
    plane_table: &PlaneTable,
    config: &crate::core::ToleranceConfig,
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
                let s_o = exact_sign_for_vertex(geometry, origin, p_o, cut_plane);
                let s_d = exact_sign_for_vertex(geometry, dest, p_d, cut_plane);

                if s_o == TriSign::Zero && s_d != TriSign::Zero {
                    points.push(CutPoint::Existing(origin));
                } else if (s_o == TriSign::Pos && s_d == TriSign::Neg)
                       || (s_o == TriSign::Neg && s_d == TriSign::Pos)
                {
                    let twin = he_data.twin();
                    let twin_face = arena.get_half_edge(twin)?.face();

                    let p_face_idx = *face_plane_map.get(&face).unwrap_or(&0);
                    let p_twin_idx = *face_plane_map.get(&twin_face).unwrap_or(&p_face_idx);

                    let (exact_pos, computed_pos): (Option<[Rational; 3]>, [f64; 3]) = {
                        if p_face_idx != p_twin_idx {
                            let p0 = plane_table.get(p_face_idx);
                            let p1 = plane_table.get(p_twin_idx);
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
                                    (Some(ep), f64_pos)
                                }
                                Err(_) => (None, forge_geom::primitives::plane::intersect_edge_plane(cut_plane, p_o, p_d, config.get_edge_split_degeneracy())),
                            }
                        } else {
                            let f64_pos = forge_geom::primitives::plane::intersect_edge_plane(cut_plane, p_o, p_d, config.get_edge_split_degeneracy());
                            let ep = Rational::try_from_f64_3(&f64_pos);
                            (ep, f64_pos)
                        }
                    };

                    let provenance = build_provenance(&exact_pos, computed_pos);
                    let canonical_pos = shared_registry.canonical_position(&provenance, computed_pos);

                    if let Some(vid) = dedup.find_by_provenance(&provenance) {
                        points.push(CutPoint::Existing(vid));
                    } else {
                        points.push(CutPoint::NewOnEdge {
                            half_edge: he,
                            provenance,
                            position: canonical_pos,
                            exact_position: exact_pos,
                        });
                    }
                }
            }
        }
    }

    Ok(points)
}

/// Compute the exact sign of a vertex relative to a plane.
///
/// Uses exact Rational position if available, otherwise promotes the f64
/// position to Rational (lossless for finite IEEE 754 values). This
/// eliminates FMA-induced sign flips that differ between debug and release.
fn exact_sign_for_vertex(
    geometry: &GeometryStore,
    vertex: VertexId,
    f64_pos: &[f64; 3],
    plane: &Plane,
) -> TriSign {
    if let Some(exact) = geometry.get_vertex_position_exact(vertex) {
        return classify_point_exact(plane, exact);
    }
    if !f64_pos[0].is_finite() || !f64_pos[1].is_finite() || !f64_pos[2].is_finite() {
        return TriSign::Zero;
    }
    let promoted = [
        Rational::try_from_f64(f64_pos[0]).unwrap_or_else(|_| Rational::zero()),
        Rational::try_from_f64(f64_pos[1]).unwrap_or_else(|_| Rational::zero()),
        Rational::try_from_f64(f64_pos[2]).unwrap_or_else(|_| Rational::zero()),
    ];
    classify_point_exact(plane, &promoted)
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



/// Resolve a CutPoint to a concrete VertexId, performing SplitEdge when needed.
pub fn resolve_cut_point(
    cp: &CutPoint,
    draft: &mut MutableDraft,
    geom: &mut GeometryStore,
    dedup: &mut LocalVertexDedup,
) -> Result<VertexId, KernelError> {
    match cp {
        CutPoint::Existing(v) => Ok(*v),
        CutPoint::NewOnEdge { half_edge, provenance, position, exact_position } => {
            let res = apply_op(draft, SplitEdge { edge: *half_edge, parameter: 0.5 })?;
            let v = res.get_value().new_vertex;
            if let Some(exact) = exact_position {
                geom.set_vertex_position_exact(v, exact.clone());
            } else {
                geom.set_vertex_position(v, *position);
            }
            dedup.insert(v, provenance.clone());
            Ok(v)
        }
    }
}
