//! Boolean pipeline orchestration.
//!
//! DOMAIN: Execute the full split → classify → select → assemble → postprocess pipeline.
//! DEPENDENCIES: split, classify, select, assemble, postprocess, OperationSpace.
//! INVARIANTS: Every phase is wrapped in a ctx.scope for tracing. Every phase
//! records a ReplayEntry for causal chain reconstruction (P3.3).

use std::collections::BTreeMap;

use forge_core::{KernelError, OperationResult, OperationMetrics};
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::replay::{ReplayLog, ReplayEntry};
use forge_topo::lineage::{LineageEvent, EntityKind, Lineage, OpSignature};
use forge_topo::hashing::compute_arena_topology_hash;

use crate::analysis::proof_validation::checkpoint::{run_checkpoint, ValidationCheckpoint};
use crate::core::{ModelingContext, OperationSpace};
use crate::geometry_store::GeometryStore;
use crate::operations::boolean::schema::{
    BooleanInput, BooleanOp, BooleanResult, FaceOrigin, FaceClassification, BooleanIntrospection,
};
use crate::operations::boolean::split::split_all_faces;
use crate::operations::boolean::classify::classify_faces;
use crate::operations::boolean::eval::VertexMatchKey;

use super::super::select::select_faces;
use super::super::disjoint::execute_zero_split;
use super::assemble::{assemble_result, compute_characteristic_scale};

/// Execute a Boolean operation on two solids.
pub fn execute_boolean(input: BooleanInput) -> Result<OperationResult<BooleanResult>, KernelError> {
    let mut ctx = ModelingContext::default();
    ctx.enable_auto_persist();
    execute_boolean_core(input, ctx)
}

/// Execute a Boolean operation with forced classification overrides.
///
/// Re-runs the full pipeline but forces specific face classifications.
/// Used by counterfactual replay to produce different topology from same inputs.
pub fn execute_boolean_with_overrides(
    input: BooleanInput,
    overrides: &[(forge_core::DecisionId, FaceClassification)],
) -> Result<OperationResult<BooleanResult>, KernelError> {
    let mut ctx = ModelingContext::default();
    ctx.enable_auto_persist();
    for &(decision_id, classification) in overrides {
        ctx.set_classification_override(decision_id, classification);
    }
    execute_boolean_core(input, ctx)
}

// ── Pipeline orchestrator ────────────────────────────────────────────────────

/// Shared Boolean execution core. All public entry points delegate here.
fn execute_boolean_core(
    input: BooleanInput,
    mut ctx: ModelingContext,
) -> Result<OperationResult<BooleanResult>, KernelError> {
    let start_time = std::time::Instant::now();
    let mut replay = ReplayLog::with_current_target();
    let mut seq = 0u64;

    input.validate()?;
    let (target_topo, mut target_geom, tool_topo, mut tool_geom, operation) = input.into_parts();

    let op_space = OperationSpace::analyze_binary(
        &target_topo, &target_geom,
        &tool_topo, &tool_geom,
        ctx.get_tolerance_config().get_min_edge_length(),
    );
    op_space.transform_geometry(&mut target_geom);
    op_space.transform_geometry(&mut tool_geom);

    // ── Split ────────────────────────────────────────────────────────────────
    let pre_hash = compute_arena_topology_hash(target_topo.arena())
        ^ compute_arena_topology_hash(tool_topo.arena());

    let split_result = ctx.scope("split", |ctx| {
        split_all_faces(target_topo, target_geom, tool_topo, tool_geom, ctx)
    })?;

    let split_count = split_result.split_count();
    let (target_topo, target_geom, tool_topo, tool_geom, target_prov, tool_prov) =
        split_result.into_parts();

    let post_split_hash = compute_arena_topology_hash(target_topo.arena())
        ^ compute_arena_topology_hash(tool_topo.arena());
    record_replay(&mut replay, &mut seq, "boolean_split",
        format!("{{\"split_count\":{split_count}}}"), pre_hash, post_split_hash);

    // ── Zero-split early return ──────────────────────────────────────────────
    if split_count == 0 {
        if let Some(early) = try_zero_split_early_return(
            &target_topo, &target_geom, &tool_topo, &tool_geom,
            operation, &op_space, &mut ctx, start_time,
        )? {
            return Ok(early);
        }
    }

    // ── Classify ─────────────────────────────────────────────────────────────
    let (target_classified, tool_classified) = ctx.scope("classify", |ctx| {
        let tc = classify_faces(
            target_topo.arena(), &target_geom,
            tool_topo.arena(), &tool_geom,
            FaceOrigin::Target, ctx,
        )?;
        let tlc = classify_faces(
            tool_topo.arena(), &tool_geom,
            target_topo.arena(), &target_geom,
            FaceOrigin::Tool, ctx,
        )?;
        Ok::<_, KernelError>((tc, tlc))
    })?;

    record_replay(&mut replay, &mut seq, "classify_faces",
        format!("{{\"target\":{},\"tool\":{}}}", target_classified.len(), tool_classified.len()),
        post_split_hash, post_split_hash);

    // ── Select ───────────────────────────────────────────────────────────────
    let (selected_target, selected_tool) = ctx.scope("select", |ctx| {
        let st = select_faces(&target_classified, FaceOrigin::Target, operation, ctx);
        let stl = select_faces(&tool_classified, FaceOrigin::Tool, operation, ctx);
        (st, stl)
    });

    let target_face_count = selected_target.len();
    let tool_face_count = selected_tool.len();

    record_replay(&mut replay, &mut seq, "select_faces",
        format!("{{\"target_selected\":{target_face_count},\"tool_selected\":{tool_face_count}}}"),
        post_split_hash, post_split_hash);

    let mut introspection = BooleanIntrospection::new(
        split_count, &target_classified, &tool_classified, start_time.elapsed(),
    );

    // ── Empty result fast path ───────────────────────────────────────────────
    if target_face_count == 0 && tool_face_count == 0 {
        return build_empty_result(introspection, replay, start_time, &mut ctx);
    }

    // ── Assemble ─────────────────────────────────────────────────────────────
    let (result_topo, result_geom) = ctx.scope("assemble", |ctx| {
        assemble_result(
            target_topo.arena(), &target_geom, &selected_target, &target_prov,
            tool_topo.arena(), &tool_geom, &selected_tool, &tool_prov,
            operation == BooleanOp::Subtraction, ctx,
        )
    })?;

    let post_assemble_hash = compute_arena_topology_hash(result_topo.arena());
    record_replay(&mut replay, &mut seq, "assemble_result",
        format!("{{\"faces\":{}}}", result_topo.arena().face_count()),
        post_split_hash, post_assemble_hash);

    let lineage_events = record_result_lineage(result_topo.arena(), seq);

    // ── Postprocess ──────────────────────────────────────────────────────────
    let (result_topo, mut result_geom) = ctx.scope("postprocess", |ctx| {
        let (rt, _) = crate::operations::boolean::postprocess::merge_coplanar_faces(result_topo, &result_geom, ctx)?;
        let (rt, _) = crate::operations::boolean::postprocess::remove_redundant_vertices(rt, &result_geom, ctx)?;
        Ok::<_, KernelError>((rt, result_geom))
    })?;

    op_space.restore_geometry(&mut result_geom);

    let post_pp_hash = compute_arena_topology_hash(result_topo.arena());
    record_replay(&mut replay, &mut seq, "postprocess",
        format!("{{\"faces\":{}}}", result_topo.arena().face_count()),
        post_assemble_hash, post_pp_hash);

    // ── Finalize ─────────────────────────────────────────────────────────────
    introspection.duration_micros = start_time.elapsed().as_micros() as u64;

    let result = BooleanResult::new(
        result_topo, result_geom,
        target_face_count, tool_face_count,
        introspection, replay, lineage_events,
    );

    finalize_envelope(result, start_time, &mut ctx)
}

// ── Phase helpers ────────────────────────────────────────────────────────────

/// Handle the zero-split path: disjoint or fully-contained solids.
fn try_zero_split_early_return(
    target_topo: &TopologyState,
    target_geom: &GeometryStore,
    tool_topo: &TopologyState,
    tool_geom: &GeometryStore,
    operation: BooleanOp,
    op_space: &OperationSpace,
    ctx: &mut ModelingContext,
    start_time: std::time::Instant,
) -> Result<Option<OperationResult<BooleanResult>>, KernelError> {
    let disjoint_result = ctx.scope("zero_split", |ctx| {
        execute_zero_split(target_topo, target_geom, tool_topo, tool_geom, operation, ctx)
    })?;

    let Some(mut result) = disjoint_result else {
        return Ok(None);
    };

    result.update_duration(start_time.elapsed());
    let (topo, mut geom) = result.into_parts();
    op_space.restore_geometry(&mut geom);

    let envelope_result = BooleanResult::new(
        topo, geom,
        target_topo.arena().face_count(),
        tool_topo.arena().face_count(),
        BooleanIntrospection::default(),
        ReplayLog::new(),
        Vec::new(),
    );

    let envelope = finalize_envelope(envelope_result, start_time, ctx)?;
    Ok(Some(envelope))
}

/// Build an empty result when no faces are selected on either side.
fn build_empty_result(
    mut introspection: BooleanIntrospection,
    replay: ReplayLog,
    start_time: std::time::Instant,
    ctx: &mut ModelingContext,
) -> Result<OperationResult<BooleanResult>, KernelError> {
    introspection.duration_micros = start_time.elapsed().as_micros() as u64;
    let result = BooleanResult::new(
        TopologyState::empty(), GeometryStore::new(),
        0, 0, introspection, replay, Vec::new(),
    );
    finalize_envelope(result, start_time, ctx)
}

/// Record lineage events for all faces in the result topology.
fn record_result_lineage(arena: &forge_topo::arena::TopologyArena, seq: u64) -> Vec<LineageEvent> {
    let op = OpSignature::with_id("assemble_result", seq);
    arena.iter_faces()
        .map(|(fid, _)| LineageEvent::EntityCreated {
            entity_kind: EntityKind::Face,
            lineage: Lineage::root(fid.index() as u64, op.clone()),
        })
        .collect()
}

// ── Replay logging ───────────────────────────────────────────────────────────

/// Record a replay entry with pre/post hashes.
fn record_replay(
    log: &mut ReplayLog,
    seq: &mut u64,
    name: &str,
    payload: String,
    pre_hash: u128,
    post_hash: u128,
) {
    *seq += 1;
    let mut entry = ReplayEntry::new(
        OpSignature::with_id(name, *seq), payload, *seq, pre_hash,
    );
    entry.set_post_hash(post_hash);
    log.record(entry);
}

// ── Envelope finalization ────────────────────────────────────────────────────

/// Wrap a BooleanResult in an OperationResult, run validation, attach decision log.
fn finalize_envelope(
    result: BooleanResult,
    start_time: std::time::Instant,
    ctx: &mut ModelingContext,
) -> Result<OperationResult<BooleanResult>, KernelError> {
    let state_hash = compute_arena_topology_hash(result.topology().arena());
    let metrics = OperationMetrics {
        duration: start_time.elapsed(),
        entities_created: result.topology().arena().face_count() as u32,
        entities_deleted: 0,
        entities_modified: 0,
        exact_predicate_calls: 0,
        policy_decisions_made: 0,
    };

    let mut envelope = OperationResult::new(result);
    envelope.set_metrics(metrics);
    envelope.set_state_hash_after(state_hash);
    envelope.set_decision_log(ctx.take_decision_log());

    let geom_ref = envelope.get_value().geometry();
    let pos_fn = |vid| geom_ref.get_vertex_position(vid).copied();
    let validation = run_checkpoint(
        envelope.get_value().topology().arena(),
        ctx.get_validation_config(),
        ValidationCheckpoint::PostBoolean,
        Some(&pos_fn),
        1e-10, 1e-12,
    )?;
    envelope.add_validation_result(format!("{validation:?}"));

    Ok(envelope)
}
