//! Per-solid split loop and face split attempt.
//!
//! DOMAIN: Apply proposed cuts to a single solid via a queue with retry rounds.
//! DEPENDENCIES: schema, cut (split_face_by_plane), plane_table (assign_original_vertex_provenance),
//!   hint_norm (normalize_hint_map), forge_topo, forge_core tracing.
//! INVARIANTS:
//!   - Processes each face-cut pair atomically via MutableDraft.
//!   - Retries deferred faces up to 8 times to allow vertex propagation to settle.

use std::collections::BTreeMap;
use std::sync::Arc;

use forge_core::ToleranceProvider;
use forge_core::{
    DecisionContext, DecisionId, DecisionKind, DecisionTier, EntityRef, KernelError, TracedDecision,
};
use forge_topo::handles::{FaceId, VertexId};
use forge_topo::transactions::{MutableDraft, TopologyState};
use forge_topo::validate::{validate_topology, ValidationLevel};

use crate::core::{compute_topology_delta, ArenaSnapshot, ModelingContext};
use crate::geometry_state::GeometryState;

use super::cut::split_face_by_plane;
use super::hint_norm::normalize_hint_map;
use super::plane_table::assign_original_vertex_provenance;
use super::schema::{
    EdgeCutMap, ExpectedCutEndpointMap, LocalVertexDedup, PlaneTable,
    SplitConfig,
};
use crate::shared_ops::intersection_registry::IntersectionRegistry;

/// Apply all proposed cuts to one solid, returning the mutated draft, geometry,
/// split count, dedup map, and the set of pre-split vertex IDs.
///
/// Uses a two-pass queue strategy — failed cuts are deferred and retried up to
/// 8 times after neighboring SplitEdge operations have propagated new vertices.
// DEFECT(D5): Deferred retry abandons grazing cuts instead of properly resolving them.
pub(super) fn split_solid(
    topo: TopologyState,
    mut geom: GeometryState,
    cuts_map: BTreeMap<FaceId, Vec<usize>>,
    initial_face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &mut PlaneTable,
    config: &crate::core::ToleranceConfig,
    shared_registry: &mut IntersectionRegistry,
    mut expected_cut_endpoints: ExpectedCutEndpointMap,
    ctx: &mut ModelingContext,
) -> Result<
    (MutableDraft, GeometryState, usize, LocalVertexDedup, std::collections::BTreeSet<VertexId>),
    KernelError,
> {
    let mut draft = topo.into_mutation();
    let mut splits = 0;
    let mut dedup = LocalVertexDedup::new();
    let mut edge_cut_map: EdgeCutMap = BTreeMap::new();

    assign_original_vertex_provenance(
        draft.arena(), &mut dedup, &geom, initial_face_planes, plane_table,
    )?;

    let original_vids: std::collections::BTreeSet<VertexId> =
        draft.arena().iter_vertices().map(|(vid, _)| vid).collect();

    let mut queue: Vec<(FaceId, Arc<Vec<usize>>, usize)> = cuts_map
        .into_iter()
        .map(|(fid, cuts)| (fid, Arc::new(cuts), 0))
        .collect();
    let mut current_face_planes = initial_face_planes.clone();
    let mut deferred: Vec<(FaceId, Arc<Vec<usize>>, usize)> = Vec::new();

    while let Some((fid, cuts, cut_pos)) = queue.pop() {
        let Some(&cut_idx) = cuts.get(cut_pos) else { continue; };

        match try_split_face(
            &mut draft, &mut geom, &mut dedup, &mut edge_cut_map,
            fid, cut_idx, &current_face_planes, plane_table, config,
            shared_registry, &expected_cut_endpoints, ctx,
        )? {
            SplitAttempt::Split(new_faces, face_plane_idx) => {
                splits += 1;
                for &nf in &new_faces {
                    current_face_planes.insert(nf, face_plane_idx);
                }
                propagate_expected_cut_endpoints(&mut expected_cut_endpoints, fid, &new_faces);
                for nf in new_faces {
                    queue.push((nf, Arc::clone(&cuts), cut_pos));
                }
            }
            SplitAttempt::NoSplit => {
                let next_pos = cut_pos + 1;
                if next_pos < cuts.len() {
                    queue.push((fid, cuts, next_pos));
                } else {
                    deferred.push((fid, cuts, cut_pos));
                }
            }
        }
    }

    let mut retry_queue = deferred;
    let mut retry_round = 0usize;
    while !retry_queue.is_empty() && retry_round < 8 {
        retry_round += 1;
        let mut next_retry: Vec<(FaceId, Arc<Vec<usize>>, usize)> = Vec::new();
        let mut progress = false;

        while let Some((fid, cuts, cut_pos)) = retry_queue.pop() {
            if !current_face_planes.contains_key(&fid) { continue; }
            let Some(&cut_idx) = cuts.get(cut_pos) else { continue; };

            match try_split_face(
                &mut draft, &mut geom, &mut dedup, &mut edge_cut_map,
                fid, cut_idx, &current_face_planes, plane_table, config,
                shared_registry, &expected_cut_endpoints, ctx,
            )? {
                SplitAttempt::Split(new_faces, face_plane_idx) => {
                    progress = true;
                    splits += 1;
                    for &nf in &new_faces {
                        current_face_planes.insert(nf, face_plane_idx);
                    }
                    propagate_expected_cut_endpoints(&mut expected_cut_endpoints, fid, &new_faces);
                    for nf in new_faces {
                        next_retry.push((nf, Arc::clone(&cuts), cut_pos));
                    }
                }
                SplitAttempt::NoSplit => next_retry.push((fid, cuts, cut_pos)),
            }
        }

        if !progress { break; }
        retry_queue = next_retry;
    }

    Ok((draft, geom, splits, dedup, original_vids))
}

// ── Internal types and helpers ───────────────────────────────────────────────

enum SplitAttempt {
    Split(Vec<FaceId>, usize),
    NoSplit,
}

fn try_split_face(
    draft: &mut MutableDraft,
    geom: &mut GeometryState,
    dedup: &mut LocalVertexDedup,
    edge_cut_map: &mut EdgeCutMap,
    fid: FaceId,
    cut_idx: usize,
    current_face_planes: &BTreeMap<FaceId, usize>,
    plane_table: &mut PlaneTable,
    config: &crate::core::ToleranceConfig,
    shared_registry: &mut IntersectionRegistry,
    expected_cut_endpoints: &ExpectedCutEndpointMap,
    ctx: &mut ModelingContext,
) -> Result<SplitAttempt, KernelError> {
    let face_plane_idx = *current_face_planes
        .get(&fid)
        .ok_or(KernelError::InternalError {
            message: "Missing plane for face".into(),
            context: None,
        })?;
    let cut_plane = plane_table.get(cut_idx).clone();
    let face_plane = plane_table.get(face_plane_idx).clone();

    let split_cfg = SplitConfig {
        plane_table,
        face_plane_map: current_face_planes,
        tolerance: config,
    };

    let pre_snapshot = ArenaSnapshot::capture(draft.arena());
    let debug_validate =
        std::env::var("FORGE_DEBUG_VALIDATE_PHASES").ok().as_deref() == Some("1");
    let was_valid_before = if debug_validate {
        validate_topology(draft.arena(), ValidationLevel::Full).is_ok()
    } else {
        false
    };

    let new_faces = split_face_by_plane(
        draft, geom, dedup, edge_cut_map, fid, &face_plane, &cut_plane, cut_idx,
        &split_cfg, shared_registry, expected_cut_endpoints.get(&(fid, cut_idx)), ctx,
    )?;

    if new_faces.is_empty() {
        return Ok(SplitAttempt::NoSplit);
    }

    if debug_validate {
        if let Err(e) = validate_topology(draft.arena(), ValidationLevel::Full) {
            eprintln!(
                "[phase-check] split op invalid after face#{} by plane#{} -> {:?}: {}",
                fid.index(), cut_idx,
                new_faces.iter().map(|f: &FaceId| f.index()).collect::<Vec<_>>(),
                e
            );
            if was_valid_before {
                eprintln!(
                    "[phase-check] split op FIRST invalid transition face#{} plane#{}",
                    fid.index(), cut_idx
                );
            }
        }
    }

    let delta = compute_topology_delta(&pre_snapshot, draft.arena());
    let mut decision = TracedDecision::new(
        DecisionId(fid.index() as u64),
        DecisionKind::Exact,
        DecisionTier::Deterministic,
        1.0,
        DecisionContext::Degeneracy {
            description: format!(
                "Split face {} by plane {} → {} new faces",
                fid, cut_idx, new_faces.len()
            ),
        },
    );
    decision.set_entity_scope(EntityRef::new(forge_core::EntityKind::Face, fid.index()));
    decision.set_topology_delta(delta);
    ctx.get_decision_log_mut().record(decision);

    Ok(SplitAttempt::Split(new_faces, face_plane_idx))
}

fn propagate_expected_cut_endpoints(
    expected_cut_endpoints: &mut ExpectedCutEndpointMap,
    parent_face: FaceId,
    new_faces: &[FaceId],
) {
    if new_faces.is_empty() { return; }

    let inherited: Vec<(usize, super::schema::ExpectedCutHint)> = expected_cut_endpoints
        .iter()
        .filter(|((fid, _), _)| *fid == parent_face)
        .map(|((_, cut_idx), hint)| (*cut_idx, hint.clone()))
        .collect();

    for &nf in new_faces {
        for (cut_idx, hint) in &inherited {
            let entry = expected_cut_endpoints.entry((nf, *cut_idx)).or_default();
            entry.endpoints.extend(hint.endpoints.iter().copied());
            entry.intervals.extend(hint.intervals.iter().cloned());
        }
    }
    normalize_hint_map(expected_cut_endpoints, 1e-6);
}
