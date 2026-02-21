//! Assembly operations for the disjoint/contained zero-split path.
//!
//! DOMAIN: Copy complete solids into fresh arenas for disjoint,
//! contained, and touching (flush coplanar) results.
//!
//! DEPENDENCIES: copy (face copying), stitch (twin stitching),
//! cleanup (degenerate topology removal), classify (coplanar detection).
//! INVARIANTS:
//! - Each shell gets its own vertex dedup — no cross-shell vertex merging.
//! - Touching union drops coplanar face pairs (internal boundary elimination).

use std::collections::BTreeMap;

use forge_core::{KernelError, TracedDecision, DecisionId, DecisionKind, DecisionTier, DecisionContext, EntityRef};
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::replay::ReplayLog;
use forge_topo::lineage::{LineageEvent, Lineage, OpSignature};

use crate::core::{ModelingContext, ArenaSnapshot, compute_topology_delta};
use crate::geometry_store::GeometryStore;
use crate::operations::boolean::classify::find_coplanar_face_pairs;
use crate::operations::boolean::eval::VertexMatchKey;
use crate::operations::boolean::schema::{BooleanResult, BooleanOp, BooleanIntrospection};
use super::super::copy::{copy_faces, VertexDedup, SpatialVertexIndex};
use super::super::stitch::stitch_twins;
use super::super::cleanup::cleanup_degenerate_topology;
use super::eval::{are_solids_coincident, compute_disjoint_scale};

// ── Contained ────────────────────────────────────────────────────────────────

/// Handle Booleans where tool is contained inside target (or vice versa).
pub(super) fn execute_contained_boolean(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    tool_inside_target: bool,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    let target_fc = target_topo.arena().face_count();
    let tool_fc = tool_topo.arena().face_count();

    match (operation, tool_inside_target) {
        (BooleanOp::Union, true) => {
            let mut r = assemble_single_shell(target_topo, target_geom, false, ctx)?;
            r.set_face_counts(target_fc, 0);
            Ok(r)
        }
        (BooleanOp::Union, false) => {
            let mut r = assemble_single_shell(tool_topo, tool_geom, false, ctx)?;
            r.set_face_counts(0, tool_fc);
            Ok(r)
        }
        (BooleanOp::Intersection, true) => {
            let mut r = assemble_single_shell(tool_topo, tool_geom, false, ctx)?;
            r.set_face_counts(0, tool_fc);
            Ok(r)
        }
        (BooleanOp::Intersection, false) => {
            let mut r = assemble_single_shell(target_topo, target_geom, false, ctx)?;
            r.set_face_counts(target_fc, 0);
            Ok(r)
        }
        (BooleanOp::Subtraction, true) => {
            if are_solids_coincident(target_topo, target_geom, tool_topo, tool_geom)? {
                return Ok(empty_result());
            }
            let mut r = assemble_two_shells(
                target_topo, target_geom, tool_topo, tool_geom, true, ctx,
            )?;
            r.set_face_counts(target_fc, tool_fc);
            Ok(r)
        }
        (BooleanOp::Subtraction, false) => Ok(empty_result()),
    }
}

// ── Disjoint ─────────────────────────────────────────────────────────────────

/// Handle Booleans where the two solids are disjoint.
pub(super) fn execute_disjoint_boolean(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    match operation {
        BooleanOp::Union => assemble_two_shells(
            target_topo, target_geom, tool_topo, tool_geom, false, ctx,
        ),
        BooleanOp::Intersection => Ok(empty_result()),
        BooleanOp::Subtraction => assemble_single_shell(target_topo, target_geom, false, ctx),
    }
}

// ── Touching ─────────────────────────────────────────────────────────────────

/// Handle Booleans where solids share flush coplanar contact.
///
/// For Union: drops coplanar face pairs (internal boundaries), copies
/// remaining faces into a shared arena with vertex merging, and stitches.
/// For other operations: delegates to disjoint handler.
pub(super) fn execute_touching_boolean(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    if operation != BooleanOp::Union {
        return execute_disjoint_boolean(
            target_topo, target_geom, tool_topo, tool_geom, operation, ctx,
        );
    }

    let (excluded_target, excluded_tool) = find_coplanar_face_pairs(
        target_topo, target_geom, tool_topo, tool_geom,
    );

    let target_faces = filter_faces(target_topo, &excluded_target);
    let tool_faces = filter_faces(tool_topo, &excluded_tool);
    let target_count = target_faces.len();
    let tool_count = tool_faces.len();

    let scale = compute_disjoint_scale(
        target_topo.arena(), target_geom,
        Some((tool_topo.arena(), tool_geom)),
    );

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut result_geom = GeometryStore::new();
    let mut global_vertex_map: BTreeMap<VertexMatchKey, VertexId> = BTreeMap::new();
    let mut spatial_index = SpatialVertexIndex::new(scale);
    let mut all_he_ids: Vec<HalfEdgeId> = Vec::new();

    let pre_target = ArenaSnapshot::capture(draft.arena());

    copy_shell(
        &mut draft, &mut result_geom, &mut all_he_ids,
        &mut global_vertex_map, &mut spatial_index,
        target_topo.arena(), target_geom, &target_faces, false,
    )?;

    let target_delta = compute_topology_delta(&pre_target, draft.arena());
    if !target_delta.is_empty() {
        let mut decision = TracedDecision::new(
            DecisionId(0),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: format!(
                    "Copy target shell: {}F {}HE {}V (excluded {} coplanar)",
                    target_delta.created_faces.len(),
                    target_delta.created_halfedges.len(),
                    target_delta.created_vertices.len(),
                    excluded_target.len(),
                ),
            },
        );
        decision.set_topology_delta(target_delta);
        ctx.get_decision_log_mut().record(decision);
    }

    let pre_tool = ArenaSnapshot::capture(draft.arena());

    copy_shell(
        &mut draft, &mut result_geom, &mut all_he_ids,
        &mut global_vertex_map, &mut spatial_index,
        tool_topo.arena(), tool_geom, &tool_faces, false,
    )?;

    let tool_delta = compute_topology_delta(&pre_tool, draft.arena());
    if !tool_delta.is_empty() {
        let mut decision = TracedDecision::new(
            DecisionId(1),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: format!(
                    "Copy tool shell: {}F {}HE {}V (excluded {} coplanar)",
                    tool_delta.created_faces.len(),
                    tool_delta.created_halfedges.len(),
                    tool_delta.created_vertices.len(),
                    excluded_tool.len(),
                ),
            },
        );
        decision.set_topology_delta(tool_delta);
        ctx.get_decision_log_mut().record(decision);
    }

    cleanup_degenerate_topology(&mut draft, &result_geom)?;
    stitch_twins(&mut draft, &all_he_ids, &result_geom, spatial_index.weld_tolerance_sq(), ctx)?;

    let topo = draft.commit()?;
    let lineage = build_lineage_events(topo.arena(), "touching_boolean_union");
    Ok(BooleanResult::new(
        topo, result_geom, target_count, tool_count,
        BooleanIntrospection::default(), ReplayLog::new(), lineage,
    ))
}

// ── Shell assembly helpers ───────────────────────────────────────────────────

/// Copy a single solid into a fresh arena and stitch its shells.
fn assemble_single_shell(
    topo: &TopologyState,
    geom: &GeometryStore,
    reverse: bool,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    let faces: Vec<FaceId> = topo.arena().iter_faces().map(|(fid, _)| fid).collect();
    let face_count = faces.len();
    let scale = compute_disjoint_scale(topo.arena(), geom, None);

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut result_geom = GeometryStore::new();
    let mut he_ids: Vec<HalfEdgeId> = Vec::new();
    let mut vertex_map: BTreeMap<VertexMatchKey, VertexId> = BTreeMap::new();
    let mut spatial = SpatialVertexIndex::new(scale);

    let pre_copy = ArenaSnapshot::capture(draft.arena());

    copy_shell(
        &mut draft, &mut result_geom, &mut he_ids,
        &mut vertex_map, &mut spatial,
        topo.arena(), geom, &faces, reverse,
    )?;

    let copy_delta = compute_topology_delta(&pre_copy, draft.arena());
    if !copy_delta.is_empty() {
        let mut decision = TracedDecision::new(
            DecisionId(0),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: format!(
                    "Copy shell: {}F {}HE {}V",
                    copy_delta.created_faces.len(),
                    copy_delta.created_halfedges.len(),
                    copy_delta.created_vertices.len(),
                ),
            },
        );
        decision.set_topology_delta(copy_delta);
        ctx.get_decision_log_mut().record(decision);
    }

    stitch_twins(&mut draft, &he_ids, &result_geom, spatial.weld_tolerance_sq(), ctx)?;

    let result_topo = draft.commit()?;
    let lineage = build_lineage_events(result_topo.arena(), "assemble_single_shell");
    Ok(BooleanResult::new(
        result_topo, result_geom, face_count, 0,
        BooleanIntrospection::default(), ReplayLog::new(), lineage,
    ))
}

/// Copy two independent solids into a fresh arena, each stitched separately.
fn assemble_two_shells(
    primary_topo: &TopologyState,
    primary_geom: &GeometryStore,
    secondary_topo: &TopologyState,
    secondary_geom: &GeometryStore,
    reverse_secondary: bool,
    ctx: &mut ModelingContext,
) -> Result<BooleanResult, KernelError> {
    let primary_faces: Vec<FaceId> = primary_topo.arena().iter_faces().map(|(fid, _)| fid).collect();
    let secondary_faces: Vec<FaceId> = secondary_topo.arena().iter_faces().map(|(fid, _)| fid).collect();
    let primary_count = primary_faces.len();
    let secondary_count = secondary_faces.len();

    let scale = compute_disjoint_scale(
        primary_topo.arena(), primary_geom,
        Some((secondary_topo.arena(), secondary_geom)),
    );

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut result_geom = GeometryStore::new();

    let mut pri_he: Vec<HalfEdgeId> = Vec::new();
    let mut pri_vm: BTreeMap<VertexMatchKey, VertexId> = BTreeMap::new();
    let mut pri_spatial = SpatialVertexIndex::new(scale);

    let pre_pri = ArenaSnapshot::capture(draft.arena());

    copy_shell(
        &mut draft, &mut result_geom, &mut pri_he,
        &mut pri_vm, &mut pri_spatial,
        primary_topo.arena(), primary_geom, &primary_faces, false,
    )?;

    let pri_delta = compute_topology_delta(&pre_pri, draft.arena());
    if !pri_delta.is_empty() {
        let mut decision = TracedDecision::new(
            DecisionId(0),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: format!(
                    "Copy primary shell: {}F {}HE {}V",
                    pri_delta.created_faces.len(),
                    pri_delta.created_halfedges.len(),
                    pri_delta.created_vertices.len(),
                ),
            },
        );
        decision.set_topology_delta(pri_delta);
        ctx.get_decision_log_mut().record(decision);
    }

    stitch_twins(&mut draft, &pri_he, &result_geom, pri_spatial.weld_tolerance_sq(), ctx)?;

    let mut sec_he: Vec<HalfEdgeId> = Vec::new();
    let mut sec_vm: BTreeMap<VertexMatchKey, VertexId> = BTreeMap::new();
    let mut sec_spatial = SpatialVertexIndex::new(scale);

    let pre_sec = ArenaSnapshot::capture(draft.arena());

    copy_shell(
        &mut draft, &mut result_geom, &mut sec_he,
        &mut sec_vm, &mut sec_spatial,
        secondary_topo.arena(), secondary_geom, &secondary_faces, reverse_secondary,
    )?;

    let sec_delta = compute_topology_delta(&pre_sec, draft.arena());
    if !sec_delta.is_empty() {
        let mut decision = TracedDecision::new(
            DecisionId(1),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: format!(
                    "Copy secondary shell: {}F {}HE {}V",
                    sec_delta.created_faces.len(),
                    sec_delta.created_halfedges.len(),
                    sec_delta.created_vertices.len(),
                ),
            },
        );
        decision.set_topology_delta(sec_delta);
        ctx.get_decision_log_mut().record(decision);
    }

    stitch_twins(&mut draft, &sec_he, &result_geom, sec_spatial.weld_tolerance_sq(), ctx)?;

    let topo = draft.commit()?;
    let lineage = build_lineage_events(topo.arena(), "assemble_two_shells");
    Ok(BooleanResult::new(
        topo, result_geom, primary_count, secondary_count,
        BooleanIntrospection::default(), ReplayLog::new(), lineage,
    ))
}

/// Copy faces from a source arena into a draft (shared helper for all paths).
fn copy_shell(
    draft: &mut forge_topo::state::MutableDraft,
    result_geom: &mut GeometryStore,
    he_ids: &mut Vec<HalfEdgeId>,
    vertex_map: &mut BTreeMap<VertexMatchKey, VertexId>,
    spatial: &mut SpatialVertexIndex,
    source_arena: &forge_topo::arena::TopologyArena,
    source_geom: &GeometryStore,
    faces: &[FaceId],
    reverse: bool,
) -> Result<(), KernelError> {
    let mut dedup = VertexDedup::new();
    copy_faces(
        draft, result_geom, &mut dedup, he_ids,
        vertex_map, spatial,
        source_arena, source_geom, faces, reverse, None,
    )
}

// ── Shared helpers ───────────────────────────────────────────────────────────

/// Create an empty BooleanResult.
fn empty_result() -> BooleanResult {
    BooleanResult::new(
        TopologyState::empty(), GeometryStore::new(), 0, 0,
        BooleanIntrospection::default(), ReplayLog::new(), Vec::new(),
    )
}

/// Build lineage events for all entities in an arena.
fn build_lineage_events(
    arena: &forge_topo::arena::TopologyArena,
    op_name: &str,
) -> Vec<LineageEvent> {
    let mut events: Vec<LineageEvent> = Vec::new();

    for (fid, _) in arena.iter_faces() {
        events.push(LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new("Face", fid.index()),
            lineage: Lineage::root(fid.index() as u64, OpSignature::new(op_name)),
        });
    }

    for (he_id, _) in arena.iter_half_edges() {
        events.push(LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new("HalfEdge", he_id.index()),
            lineage: Lineage::root(he_id.index() as u64, OpSignature::new(op_name)),
        });
    }

    for (vid, _) in arena.iter_vertices() {
        events.push(LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new("Vertex", vid.index()),
            lineage: Lineage::root(vid.index() as u64, OpSignature::new(op_name)),
        });
    }

    events
}

/// Filter faces, excluding those with indices in the exclusion set.
fn filter_faces(
    topo: &TopologyState,
    excluded: &std::collections::BTreeSet<u32>,
) -> Vec<FaceId> {
    topo.arena().iter_faces()
        .map(|(fid, _)| fid)
        .filter(|fid| !excluded.contains(&fid.index()))
        .collect()
}
