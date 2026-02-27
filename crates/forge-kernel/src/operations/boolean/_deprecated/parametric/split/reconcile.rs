//! Post-split boundary reconciliation.
//!
//! DOMAIN: Enforce split symmetry between target and tool after independent
//! splitting. For each cut vertex present on one solid but absent on the
//! other, locate the edge on the opposing solid and insert a SplitEdge.
//!
//! DEPENDENCIES: schema (LocalVertexDedup), IntersectionRegistry, GeometryState,
//! forge_topo (MutableDraft, SplitEdge).
//!
//! INVARIANTS:
//!   - Multiple orphans on a single edge are sorted by parametric position
//!     and split sequentially with handle tracking (no stale IDs).
//!   - Orphans coincident with existing endpoints are welded, not split.
//!   - Both drafts are mutated in-place; caller commits after reconciliation.

use std::collections::{BTreeMap, BTreeSet};

use forge_core::KernelError;
use forge_topo::euler::split_edge::SplitEdge;
use forge_topo::handles::{HalfEdgeId, VertexId};
use forge_topo::operator::apply_op;
use forge_topo::queries::edge_endpoint_ids;
use forge_topo::state::MutableDraft;
use forge_math::linalg::distance_sq;

use super::schema::LocalVertexDedup;
use crate::geometry_state::GeometryState;
use crate::geom_facade::{EpsilonWelder, point_on_segment};
use crate::shared_ops::vertex::identity::VertexMatchKey;
use crate::shared_ops::intersection_registry::IntersectionRegistry;

/// Reconcile boundary vertices between target and tool after splitting.
///
/// Finds vertices present in one solid's dedup but absent in the other,
/// then injects them by splitting the appropriate edge on the opposing solid.
pub fn reconcile_boundary_vertices(
    target_draft: &mut MutableDraft,
    target_geom: &mut GeometryState,
    target_dedup: &mut LocalVertexDedup,
    tool_draft: &mut MutableDraft,
    tool_geom: &mut GeometryState,
    tool_dedup: &mut LocalVertexDedup,
    shared_registry: &IntersectionRegistry,
    weld_tolerance_sq: f64,
    expected_position_tolerance_sq: f64,
    expected_shared_positions: &[[f64; 3]],
    target_original_vids: &BTreeSet<VertexId>,
    tool_original_vids: &BTreeSet<VertexId>,
) -> Result<usize, KernelError> {
    let target_orphans = find_orphan_vertices(
        target_dedup,
        tool_dedup,
        shared_registry,
        expected_shared_positions,
        expected_position_tolerance_sq,
        target_original_vids,
    );
    let tool_orphans = find_orphan_vertices(
        tool_dedup,
        target_dedup,
        shared_registry,
        expected_shared_positions,
        expected_position_tolerance_sq,
        tool_original_vids,
    );

    eprintln!(
        "[reconcile] target cut-orphans: {}, tool cut-orphans: {}",
        target_orphans.len(),
        tool_orphans.len()
    );

    let mut injected = 0;

    let target_into_tool = inject_orphans_into_solid(
        tool_draft,
        tool_geom,
        tool_dedup,
        &target_orphans,
        weld_tolerance_sq,
    )?;
    eprintln!(
        "[reconcile] injected {} target orphans into tool",
        target_into_tool
    );
    injected += target_into_tool;

    let tool_into_target = inject_orphans_into_solid(
        target_draft,
        target_geom,
        target_dedup,
        &tool_orphans,
        weld_tolerance_sq,
    )?;
    eprintln!(
        "[reconcile] injected {} tool orphans into target",
        tool_into_target
    );
    injected += tool_into_target;

    Ok(injected)
}

/// An orphan vertex: present on one solid but missing on the other.
struct OrphanVertex {
    key: VertexMatchKey,
    position: [f64; 3],
}

/// Find cut vertices in `source_dedup` that are absent from `dest_dedup`.
///
/// Only considers vertices created during splitting (not in `original_vids`).
fn find_orphan_vertices(
    source_dedup: &LocalVertexDedup,
    dest_dedup: &LocalVertexDedup,
    shared_registry: &IntersectionRegistry,
    expected_shared_positions: &[[f64; 3]],
    expected_position_tolerance_sq: f64,
    original_vids: &BTreeSet<VertexId>,
) -> Vec<OrphanVertex> {
    let mut orphans = Vec::new();
    for (vid, key) in source_dedup.iter_provenance() {
        if original_vids.contains(vid) {
            continue;
        }
        if !dest_dedup.has_key(key) {
            if let Some(&pos) = shared_registry.get_position(key) {
                if !is_expected_shared_position(
                    &pos,
                    expected_shared_positions,
                    expected_position_tolerance_sq,
                ) {
                    continue;
                }
                orphans.push(OrphanVertex {
                    key: key.clone(),
                    position: pos,
                });
            }
        }
    }
    orphans
}

fn is_expected_shared_position(
    pos: &[f64; 3],
    expected_shared_positions: &[[f64; 3]],
    expected_position_tolerance_sq: f64,
) -> bool {
    expected_shared_positions
        .iter()
        .any(|p| distance_sq(*pos, *p) <= expected_position_tolerance_sq)
}

/// Inject a set of orphan vertices into a solid by splitting edges.
///
/// Phase 1: vertex-vertex coincidence — if the orphan position matches an
/// existing vertex on this solid, weld (register provenance key) without
/// any topology mutation.
///
/// Phase 2: edge-location — for remaining orphans, find which edge they
/// lie on, group by edge, sort by parametric t, and apply SplitEdge
/// sequentially with handle tracking.
fn inject_orphans_into_solid(
    draft: &mut MutableDraft,
    geom: &mut GeometryState,
    dedup: &mut LocalVertexDedup,
    orphans: &[OrphanVertex],
    weld_tolerance_sq: f64,
) -> Result<usize, KernelError> {
    // Build the welder once from all draft vertices — O(n) construction, O(1) per query.
    let weld_tol = weld_tolerance_sq.sqrt();
    let mut welder = EpsilonWelder::new(weld_tol);
    let vertex_index_map: Vec<VertexId> = draft
        .arena()
        .iter_vertices()
        .filter_map(|(vid, _)| {
            geom.get_vertex_position(vid).map(|pos| {
                welder.add_vertex(*pos);
                vid
            })
        })
        .collect();

    let mut injected = 0;
    let mut remaining_orphan_indices: Vec<usize> = Vec::new();

    for (idx, orphan) in orphans.iter().enumerate() {
        if let Some(welder_idx) = welder.find_nearest(&orphan.position) {
            if let Some(&existing_vid) = vertex_index_map.get(welder_idx) {
                eprintln!(
                    "[reconcile]   welded orphan [{:.4},{:.4},{:.4}] to vertex {}",
                    orphan.position[0], orphan.position[1], orphan.position[2], existing_vid
                );
                dedup.insert(existing_vid, orphan.key.clone());
                injected += 1;
            } else {
                remaining_orphan_indices.push(idx);
            }
        } else {
            remaining_orphan_indices.push(idx);
        }
    }

    if remaining_orphan_indices.is_empty() {
        return Ok(injected);
    }

    let mut edge_groups: BTreeMap<u32, Vec<(usize, f64)>> = BTreeMap::new();
    let mut edge_ids: BTreeMap<u32, HalfEdgeId> = BTreeMap::new();
    let mut unmatched = 0usize;

    for &orphan_idx in &remaining_orphan_indices {
        let orphan = &orphans[orphan_idx];
        match locate_edge_for_vertex(draft, geom, &orphan.position, weld_tolerance_sq) {
            Some((he_id, t)) => {
                let edge_key = he_id.index();
                edge_groups
                    .entry(edge_key)
                    .or_default()
                    .push((orphan_idx, t));
                edge_ids.entry(edge_key).or_insert(he_id);
            }
            None => {
                eprintln!("[reconcile]   orphan NOT matched: pos=[{:.6},{:.6},{:.6}]",
                    orphan.position[0], orphan.position[1], orphan.position[2]);
                unmatched += 1;
            }
        }
    }

    if unmatched > 0 {
        eprintln!(
            "[reconcile]   {} orphans could not be matched to any edge or vertex",
            unmatched
        );
    }

    for (edge_key, mut group) in edge_groups {
        let he_id = edge_ids[&edge_key];

        group.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let mut current_he = he_id;
        let mut accumulated_t = 0.0;

        for (orphan_idx, global_t) in &group {
            let orphan = &orphans[*orphan_idx];

            let local_t = if accumulated_t >= 1.0 {
                0.5
            } else {
                (global_t - accumulated_t) / (1.0 - accumulated_t)
            };

            if is_endpoint_coincident(draft, geom, current_he, &orphan.position, weld_tolerance_sq)?
            {
                let coincident_vid = find_coincident_endpoint(
                    draft,
                    geom,
                    current_he,
                    &orphan.position,
                    weld_tolerance_sq,
                )?;
                if let Some(vid) = coincident_vid {
                    eprintln!(
                        "[reconcile]   welded edge-orphan to endpoint vertex {}",
                        vid
                    );
                    dedup.insert(vid, orphan.key.clone());
                }
            } else {
                let result = apply_op(
                    draft,
                    SplitEdge {
                        edge: current_he,
                        parameter: local_t,
                    },
                )?;
                let new_vertex = result.get_value().new_vertex;

                geom.set_vertex_position(new_vertex, orphan.position);
                dedup.insert(new_vertex, orphan.key.clone());

                eprintln!(
                    "[reconcile]   split edge HE#{} at t={:.4} → vertex {}",
                    current_he.index(),
                    local_t,
                    new_vertex
                );

                current_he = result.get_value().he_mb;
                accumulated_t = *global_t;
                injected += 1;
            }
        }
    }

    Ok(injected)
}


/// Locate which edge on the solid contains the given point.
///
/// Walks all half-edges and uses `point_on_segment` from forge-geom.
/// Returns the half-edge ID and parametric t value (0..1) along the edge.
fn locate_edge_for_vertex(
    draft: &MutableDraft,
    geom: &GeometryState,
    point: &[f64; 3],
    weld_tolerance_sq: f64,
) -> Option<(HalfEdgeId, f64)> {
    let mut best_match: Option<(HalfEdgeId, f64, f64)> = None;

    for (he_id, he_data) in draft.arena().iter_half_edges() {
        let origin = he_data.origin();
        let Ok((_, dest)) = edge_endpoint_ids(draft.arena(), he_id) else { continue; };

        if origin == dest {
            continue;
        }

        let p_o = match geom.get_vertex_position(origin) {
            Some(p) => p,
            None => continue,
        };
        let p_d = match geom.get_vertex_position(dest) {
            Some(p) => p,
            None => continue,
        };

        if let Some((t, dist_sq_val)) = point_on_segment(point, p_o, p_d, weld_tolerance_sq) {
            let is_better = best_match.map(|(_, _, d)| dist_sq_val < d).unwrap_or(true);
            if is_better {
                best_match = Some((he_id, t, dist_sq_val));
            }
        }
    }

    best_match.map(|(he, t, _)| (he, t))
}


/// Check if the orphan point is coincident with either endpoint of the edge.
fn is_endpoint_coincident(
    draft: &MutableDraft,
    geom: &GeometryState,
    he_id: HalfEdgeId,
    point: &[f64; 3],
    weld_tolerance_sq: f64,
) -> Result<bool, KernelError> {
    let (origin, dest) = edge_endpoint_ids(draft.arena(), he_id)?;

    let near_origin = geom
        .get_vertex_position(origin)
        .map(|p| distance_sq(*p, *point) < weld_tolerance_sq)
        .unwrap_or(false);

    let near_dest = geom
        .get_vertex_position(dest)
        .map(|p| distance_sq(*p, *point) < weld_tolerance_sq)
        .unwrap_or(false);

    Ok(near_origin || near_dest)
}

/// Find which endpoint (if any) is coincident, returning its VertexId.
fn find_coincident_endpoint(
    draft: &MutableDraft,
    geom: &GeometryState,
    he_id: HalfEdgeId,
    point: &[f64; 3],
    weld_tolerance_sq: f64,
) -> Result<Option<VertexId>, KernelError> {
    let (origin, dest) = edge_endpoint_ids(draft.arena(), he_id)?;

    if let Some(p) = geom.get_vertex_position(origin) {
        if distance_sq(*p, *point) < weld_tolerance_sq {
            return Ok(Some(origin));
        }
    }

    if let Some(p) = geom.get_vertex_position(dest) {
        if distance_sq(*p, *point) < weld_tolerance_sq {
            return Ok(Some(dest));
        }
    }

    Ok(None)
}

// dist_sq_3d, find_nearest_vertex_distance, and find_nearest_edge_distance
// have been removed. Use forge_math::linalg::distance_sq directly.
