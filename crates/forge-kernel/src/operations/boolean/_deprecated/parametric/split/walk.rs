//! Edge sign-walk and cut-point provenance computation.
//!
//! DOMAIN: Walk every half-edge of a face to find where the cut plane enters
//!   the face boundary, producing CutPoint records.
//! DEPENDENCIES: schema (CutPoint, SplitConfig), signs, forge_topo traversal.
//! INVARIANTS:
//!   - Zero (on-plane) vertices are emitted as Existing cut points.
//!   - Pos↔Neg edges produce NewOnEdge cut points with exact or f64 positions.

use std::collections::BTreeMap;

use forge_core::KernelError;
use worth_math::arithmetic::Rational;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::entity_lifecycle::split_edge::SplitEdge;
use forge_topo::operator::apply_op;
use forge_topo::transactions::MutableDraft;
use forge_topo::traverse::FaceAllEdgesIterator;
use worth_math::sign::is_sign_crossing;

use crate::geom_facade::Plane;
use crate::core::ModelingContext;
use crate::geometry_state::GeometryState;
use crate::operations::boolean::_deprecated::shared::edge_intersection::compute_edge_plane_intersection_position;
use crate::shared_ops::vertex::identity::{build_vertex_provenance, VertexMatchKey};

use super::signs::exact_sign_for_vertex;
use super::schema::{CutPoint, LocalVertexDedup, SplitConfig};
use crate::shared_ops::intersection_registry::IntersectionRegistry;

/// Find where the cut plane enters the face boundary — exact sign-walk.
///
/// Walks every half-edge of the face:
/// - Origin vertex with `Zero` sign → `CutPoint::Existing`
/// - Edge with `Pos↔Neg` signs → `CutPoint::NewOnEdge`
pub(super) fn find_cut_points_provenance(
    arena: &forge_topo::b_rep::TopologyArena,
    geometry: &GeometryState,
    face: FaceId,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    dedup: &LocalVertexDedup,
    shared_registry: &mut IntersectionRegistry,
    split_cfg: &SplitConfig<'_>,
) -> Result<Vec<CutPoint>, KernelError> {
    let mut points = Vec::new();
    let mut sign_cache: BTreeMap<VertexId, worth_math::sign::TriSign> = BTreeMap::new();

    for he in FaceAllEdgesIterator::new(arena, face)? {
        let he = he?;
        let he_data = arena.get_half_edge(he)?;
        let origin = he_data.origin();
        let next_data = arena.get_half_edge(he_data.next())?;
        let dest = next_data.origin();

        if let Some(p_o) = geometry.get_vertex_position(origin) {
            if let Some(p_d) = geometry.get_vertex_position(dest) {
                let s_o = *sign_cache.entry(origin).or_insert_with(|| {
                    exact_sign_for_vertex(geometry, origin, p_o, cut_plane, cut_plane_idx)
                });
                let s_d = *sign_cache.entry(dest).or_insert_with(|| {
                    exact_sign_for_vertex(geometry, dest, p_d, cut_plane, cut_plane_idx)
                });

                if s_o == worth_math::sign::TriSign::Zero
                    && s_d != worth_math::sign::TriSign::Zero
                {
                    points.push(CutPoint::Existing(origin));
                } else if is_sign_crossing(s_o, s_d) {
                    let cp = compute_crossing_cut_point(
                        arena,
                        he,
                        face,
                        cut_plane,
                        cut_plane_idx,
                        p_o,
                        p_d,
                        dedup,
                        shared_registry,
                        split_cfg,
                    )?;
                    points.push(cp);
                }
            }
        }
    }

    Ok(points)
}

/// Compute the `CutPoint` for a `Pos↔Neg` edge crossing.
///
/// Attempts exact 3-plane intersection when the twin face has a different
/// plane. On failure or same-plane twin, falls back to f64 edge-plane
/// intersection with Rational promotion.
fn compute_crossing_cut_point(
    arena: &forge_topo::b_rep::TopologyArena,
    he: HalfEdgeId,
    face: FaceId,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    p_o: &[f64; 3],
    p_d: &[f64; 3],
    dedup: &LocalVertexDedup,
    shared_registry: &mut IntersectionRegistry,
    split_cfg: &SplitConfig<'_>,
) -> Result<CutPoint, KernelError> {
    let he_data = arena.get_half_edge(he)?;
    let twin = he_data.radial_next();
    let twin_face = arena.get_half_edge(twin)?.face();

    let p_face_idx = *split_cfg.face_plane_map.get(&face).unwrap_or(&0);
    let p_twin_idx = *split_cfg
        .face_plane_map
        .get(&twin_face)
        .unwrap_or(&p_face_idx);

    let (exact_pos, computed_pos, symbolic_planes) = compute_edge_plane_intersection_position(
        p_face_idx,
        p_twin_idx,
        cut_plane_idx,
        split_cfg.plane_table.planes(),
        cut_plane,
        p_o,
        p_d,
        split_cfg.tolerance,
    );

    let provenance = build_vertex_provenance(&exact_pos, computed_pos);
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

/// Resolve a `CutPoint` to a concrete `VertexId`, applying `SplitEdge` when needed.
pub fn resolve_cut_point(
    cp: &CutPoint,
    draft: &mut MutableDraft,
    geom: &mut GeometryState,
    dedup: &mut LocalVertexDedup,
) -> Result<VertexId, KernelError> {
    match cp {
        CutPoint::Existing(v) => Ok(*v),
        CutPoint::NewOnEdge {
            half_edge,
            provenance,
            position,
            exact_position,
            symbolic_planes,
        } => {
            let res = apply_op(
                draft,
                SplitEdge {
                    edge: *half_edge,
                },
            )?;
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
