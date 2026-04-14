//! Per-face plane-cut application — entry point.
//!
//! DOMAIN: Accept a face and a cut plane; orchestrate the three-phase cut:
//!   gate → sign-walk → apply.
//! DEPENDENCIES: gate, walk, apply, expected, log.
//! INVARIANTS:
//!   - `gate::compute_face_chord` decides IF a cut happens.
//!   - `walk::find_cut_points_provenance` decides WHERE.
//!   - `apply::apply_one_cut` applies the MakeEdgeFace mutation.
//!   - Exactly ONE cut pair is applied per call.

use forge_core::KernelError;
use worth_math::linalg::{plane_cut_direction, sort_points_along_direction};
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::transactions::MutableDraft;

use crate::geom_facade::Plane;
use crate::core::ModelingContext;
use crate::geometry_state::GeometryState;

use super::gate::compute_face_chord;
use super::schema::{
    CutPoint, EdgeCutMap, ExpectedCutHint, LocalVertexDedup, SplitConfig,
};
use crate::shared_ops::intersection_registry::IntersectionRegistry;

use super::apply::apply_one_cut;
use super::hint_norm::localize_expected_hint;
use super::log::log_rejection;
use super::walk::{find_cut_points_provenance, resolve_cut_point};

/// Split a face by a cutting plane — applies exactly ONE cut pair per call.
///
/// Returns `[new_face, original_face]` on success so the caller can re-enqueue
/// both for re-testing against remaining cut planes.
pub fn split_face_by_plane(
    draft: &mut MutableDraft,
    geometry: &mut GeometryState,
    dedup: &mut LocalVertexDedup,
    edge_cut_map: &mut EdgeCutMap,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    split_cfg: &SplitConfig<'_>,
    shared_registry: &mut IntersectionRegistry,
    expected_hint: Option<&ExpectedCutHint>,
    ctx: &mut ModelingContext,
) -> Result<Vec<FaceId>, KernelError> {
    let Some(face_chord) = gate_chord(
        draft, geometry, face, face_plane, cut_plane, cut_plane_idx, split_cfg, ctx,
    )?
    else {
        return Ok(Vec::new());
    };

    let cut_points = find_cut_points_provenance(
        draft.arena(),
        geometry,
        face,
        cut_plane,
        cut_plane_idx,
        dedup,
        shared_registry,
        split_cfg,
    )?;

    let resolved = resolve_all_cut_points(&cut_points, draft, geometry, dedup)?;
    if resolved.len() < 2 {
        let nan3 = [f64::NAN; 3];
        eprintln!(
            "[cut-diag] Face#{} by plane#{}: {} cut_points found, {} unique after dedup",
            face.index(), cut_plane_idx, cut_points.len(), resolved.len()
        );
        for (i, vid) in resolved.iter().enumerate() {
            let pos = geometry.get_vertex_position(*vid).unwrap_or(&nan3);
            eprintln!(
                "  [cut-diag]   resolved[{}]: vid={} pos=[{:.6},{:.6},{:.6}]",
                i, vid, pos[0], pos[1], pos[2]
            );
        }
        log_rejection(
            face,
            cut_plane_idx,
            &format!("{} resolved vertices after dedup (need >=2)", resolved.len()),
            ctx,
        );
        return Ok(Vec::new());
    }

    let sorted = sort_cut_vertices(resolved, face_plane, cut_plane, geometry);
    let had_hint = expected_hint.is_some();
    let localized_hint = expected_hint.and_then(|hint| {
        localize_expected_hint(hint, face_chord, split_cfg.tolerance.get_min_edge_length())
    });
    if had_hint && localized_hint.is_none() {
        log_rejection(
            face,
            cut_plane_idx,
            "deferred: fragment chord does not overlap expected segment interval",
            ctx,
        );
        return Ok(Vec::new());
    }

    apply_one_cut(
        sorted, draft, geometry, edge_cut_map, face, face_plane, cut_plane, cut_plane_idx,
        localized_hint.as_ref(), ctx,
    )
}

// ── Phase helpers ────────────────────────────────────────────────────────────

/// Run the chord-gate and return the face chord.
fn gate_chord(
    draft: &MutableDraft,
    geometry: &GeometryState,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    split_cfg: &SplitConfig<'_>,
    ctx: &mut ModelingContext,
) -> Result<Option<([f64; 3], [f64; 3])>, KernelError> {
    let chord = compute_face_chord(
        draft.arena(), geometry, face, face_plane, cut_plane, split_cfg.tolerance,
    )?;
    if chord.is_none() {
        log_rejection(face, cut_plane_idx, "rejected by chord gate", ctx);
    }
    Ok(chord)
}

/// Resolve `CutPoint`s to `VertexId`s, applying `SplitEdge` where needed, and dedup.
fn resolve_all_cut_points(
    cut_points: &[CutPoint],
    draft: &mut MutableDraft,
    geometry: &mut GeometryState,
    dedup: &mut LocalVertexDedup,
) -> Result<Vec<VertexId>, KernelError> {
    let mut resolved: Vec<VertexId> = Vec::new();
    for cp in cut_points {
        resolved.push(resolve_cut_point(cp, draft, geometry, dedup)?);
    }
    resolved.dedup_by_key(|v| v.index());
    Ok(resolved)
}

/// Sort resolved cut vertices along the plane-chord direction for pairing.
fn sort_cut_vertices(
    verts: Vec<VertexId>,
    face_plane: &Plane,
    cut_plane: &Plane,
    geometry: &GeometryState,
) -> Vec<VertexId> {
    let dir = plane_cut_direction(face_plane.raw_normal(), cut_plane.raw_normal(), 1e-24);
    let items: Vec<(VertexId, [f64; 3])> = verts
        .into_iter()
        .map(|v| {
            let pos = geometry.get_vertex_position(v).copied().unwrap_or([0.0; 3]);
            (v, pos)
        })
        .collect();
    sort_points_along_direction(items, dir)
        .into_iter()
        .map(|(v, _)| v)
        .collect()
}
