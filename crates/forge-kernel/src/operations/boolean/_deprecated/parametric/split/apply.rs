//! MakeEdgeFace application and pair selection for the face-cut phase.
//!
//! DOMAIN: Translate a sorted list of cut vertices into topology mutations.
//! DEPENDENCIES: schema (EdgeCutMap, ExpectedCutHint), expected (try_expected_pair),
//!   forge_topo (MakeEdgeFace, face_adjacent_vertex_pairs).
//! INVARIANTS:
//!   - Exactly ONE cut pair is attempted per call.
//!   - Pairs that are already adjacent on the face are silently skipped.

use std::collections::BTreeSet;

use forge_core::KernelError;
use forge_topo::euler::make_edge_face::MakeEdgeFace;
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::operator::apply_op;
use forge_topo::state::MutableDraft;
use forge_topo::topology::queries::polygon::face_adjacent_vertex_pairs;

use crate::geom_facade::Plane;
use crate::core::ModelingContext;
use crate::geometry_state::GeometryState;

use super::expected::{can_use_scaffold_fallback, try_expected_pair};
use super::log::{log_rejection, log_split_success};
use super::schema::{make_edge_key, EdgeCutMap, ExpectedCutHint};

/// Apply ONE MakeEdgeFace cut from the sorted cut vertices.
///
/// Strategy:
/// 1. Try `try_expected_pair` first (proof-system directed).
/// 2. If an expected hint exists and `can_use_scaffold_fallback` fails, defer.
/// 3. Otherwise iterate `select_non_adjacent_pairs` and try each until one succeeds.
pub(super) fn apply_one_cut(
    sorted: Vec<VertexId>,
    draft: &mut MutableDraft,
    geometry: &mut GeometryState,
    edge_cut_map: &mut EdgeCutMap,
    face: FaceId,
    face_plane: &Plane,
    cut_plane: &Plane,
    cut_plane_idx: usize,
    expected_hint: Option<&ExpectedCutHint>,
    ctx: &mut ModelingContext,
) -> Result<Vec<FaceId>, KernelError> {
    let adjacent = face_adjacent_vertex_pairs(draft.arena(), face)?;

    if let Some(result) = try_expected_pair(
        &sorted,
        expected_hint,
        &adjacent,
        draft,
        geometry,
        edge_cut_map,
        face,
        face_plane,
        cut_plane_idx,
        ctx,
    )? {
        return Ok(result);
    }

    if expected_hint.is_some()
        && !can_use_scaffold_fallback(
            &sorted,
            expected_hint.unwrap(),
            geometry,
            &adjacent,
            face_plane,
            cut_plane,
        )
    {
        log_rejection(
            face,
            cut_plane_idx,
            "deferred: expected overlap endpoints not bracketed by scaffold fragment",
            ctx,
        );
        return Ok(Vec::new());
    }

    let pairs = select_non_adjacent_pairs(&sorted, &adjacent);
    if let Some(result) = apply_scaffold_pass(
        &pairs,
        draft,
        geometry,
        edge_cut_map,
        face,
        face_plane,
        cut_plane_idx,
        expected_hint,
        ctx,
    )? {
        return Ok(result);
    }

    log_rejection(face, cut_plane_idx, "no valid cut pair found", ctx);
    Ok(Vec::new())
}

/// Filter a sorted vertex list down to non-identical, non-adjacent chunk pairs.
///
/// Pure function — no mutation, easily unit-testable in isolation.
pub(super) fn select_non_adjacent_pairs(
    sorted: &[VertexId],
    adjacent: &BTreeSet<(u32, u32)>,
) -> Vec<(VertexId, VertexId)> {
    sorted
        .chunks_exact(2)
        .filter(|pair| pair[0] != pair[1])
        .filter(|pair| !adjacent.contains(&make_edge_key(pair[0], pair[1])))
        .map(|pair| (pair[0], pair[1]))
        .collect()
}

/// Apply a `MakeEdgeFace` cut between `v_a` and `v_b` on `face`.
///
/// On success: records the cut in `edge_cut_map`, assigns the face plane,
/// logs the decision, and returns `Some([new_face, face])`.
/// On topology failure: returns `None` so the caller can try the next pair.
pub(super) fn execute_make_edge_face(
    draft: &mut MutableDraft,
    geometry: &mut GeometryState,
    edge_cut_map: &mut EdgeCutMap,
    face: FaceId,
    face_plane: &Plane,
    cut_plane_idx: usize,
    v_a: VertexId,
    v_b: VertexId,
    ctx: &mut ModelingContext,
) -> Option<Vec<FaceId>> {
    let op = MakeEdgeFace {
        vertex_a: v_a,
        vertex_b: v_b,
        face,
    };
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
}

/// Iterate non-adjacent pairs and apply the first successful `MakeEdgeFace`.
fn apply_scaffold_pass(
    pairs: &[(VertexId, VertexId)],
    draft: &mut MutableDraft,
    geometry: &mut GeometryState,
    edge_cut_map: &mut EdgeCutMap,
    face: FaceId,
    face_plane: &Plane,
    cut_plane_idx: usize,
    expected_hint: Option<&ExpectedCutHint>,
    ctx: &mut ModelingContext,
) -> Result<Option<Vec<FaceId>>, KernelError> {
    for &(v_a, v_b) in pairs {
        if expected_hint.is_some() {
            eprintln!(
                "[cut-expected] face#{} plane#{} fallback trying {} {}",
                face.index(),
                cut_plane_idx,
                v_a,
                v_b
            );
        }
        if let Some(result) =
            execute_make_edge_face(draft, geometry, edge_cut_map, face, face_plane, cut_plane_idx, v_a, v_b, ctx)
        {
            return Ok(Some(result));
        }
    }
    Ok(None)
}
