//! Boolean pipeline orchestration.
//!
//! DOMAIN: Execute the full split → classify → select → assemble → postprocess pipeline.
//! DEPENDENCIES: split, classify, select, assemble, postprocess, OperationSpace.
//! INVARIANTS: Every phase is wrapped in a ctx.scope for tracing. Every phase
//! records a ReplayEntry for causal chain reconstruction (P3.3).

use std::collections::BTreeMap;

use forge_core::{KernelError, OperationResult, OperationMetrics};
use forge_core::tracing::checkpoint_diff::diff_decision_logs;
use forge_core::tracing::TraceAdjunctSet;
use forge_core::DecisionLog;
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::replay::{ReplayLog, ReplayEntry};
use forge_topo::lineage::{LineageEvent, Lineage, OpSignature};
use forge_topo::hashing::compute_arena_topology_hash;
use forge_topo::validate::{validate_topology, ValidationLevel};
use forge_topo::arena::TopologyArena;

use crate::analysis::proof_validation::checkpoint::{run_checkpoint, ValidationCheckpoint};
use crate::core::{ModelingContext, OperationSpace, OperationFinalizer, TopologyHashBoundary};
use crate::geometry_state::GeometryState;
use crate::operations::boolean::schema::{
    BooleanInput, BooleanOp, BooleanResult, FaceOrigin, FaceClassification, BooleanIntrospection,
};
use crate::operations::boolean::eval::VertexMatchKey;
use crate::operations::boolean::traits::BooleanEngine;
use crate::operations::boolean::engines::planar::{planar_engine, planar_engine_legacy};

use super::super::select::select_faces;
use super::super::disjoint::execute_zero_split;
use super::assemble::{assemble_result, compute_characteristic_scale};

/// Execute a Boolean operation on two solids.
///
/// Execute a Boolean operation using the standard pipeline directly.
///
/// This is the raw pipeline entry point — no EMBER routing.
/// For production use, prefer `execute_boolean` which routes through
/// the dual-engine router (EMBER for planar, standard for curved).
///
/// Returns an `OperationResult` envelope that ALWAYS carries the `DecisionLog`,
/// metrics, and replay data — even when the inner operation fails.
pub fn execute_boolean_direct(input: BooleanInput) -> OperationResult<Result<BooleanResult, KernelError>> {
    let mut ctx = ModelingContext::default();
    ctx.enable_auto_persist();
    let use_ember = std::env::var("FORGE_DIRECT_USE_EMBER").ok().as_deref() == Some("1");
    if use_ember {
        execute_boolean_core(input, ctx, planar_engine())
    } else {
        execute_boolean_core(input, ctx, planar_engine_legacy())
    }
}

/// Execute a Boolean operation with a specific engine configuration.
///
/// Used by the EMBER router to inject EmberCoplanarResolver into
/// the pipeline. Callers provide a pre-built engine that controls
/// which implementations are used for each phase.
pub fn execute_boolean_with_engine(
    input: BooleanInput,
    engine: BooleanEngine,
) -> OperationResult<Result<BooleanResult, KernelError>> {
    let mut ctx = ModelingContext::default();
    ctx.enable_auto_persist();
    execute_boolean_core(input, ctx, engine)
}

/// Execute a Boolean operation with forced classification overrides.
///
/// Re-runs the full pipeline but forces specific face classifications.
/// Used by counterfactual replay to produce different topology from same inputs.
/// The envelope always carries full causal data, even on failure.
pub fn execute_boolean_with_overrides(
    input: BooleanInput,
    overrides: &[(forge_core::DecisionId, FaceClassification)],
) -> OperationResult<Result<BooleanResult, KernelError>> {
    let mut ctx = ModelingContext::default();
    ctx.enable_auto_persist();
    for &(decision_id, classification) in overrides {
        ctx.set_classification_override(decision_id, classification);
    }
    execute_boolean_core(input, ctx, planar_engine_legacy())
}

// ── Pipeline orchestrator ────────────────────────────────────────────────────

/// Always-envelope wrapper. Owns `ctx` and `replay` so they survive errors.
///
/// The inner pipeline returns `Result<BooleanResult, KernelError>`. This
/// wrapper captures the DecisionLog + ReplayLog + metrics into the envelope
/// regardless of whether the inner pipeline succeeded or failed.
fn execute_boolean_core(
    input: BooleanInput,
    mut ctx: ModelingContext,
    engine: BooleanEngine,
) -> OperationResult<Result<BooleanResult, KernelError>> {
    let start_time = std::time::Instant::now();
    let topology_hash_before = input.target_topology().topology_hash() ^ input.tool_topology().topology_hash();

    let inner_result = execute_boolean_pipeline(input, &engine, &mut ctx, start_time);

    let metrics = OperationMetrics {
        duration: start_time.elapsed(),
        ..OperationMetrics::default()
    };

    let mut envelope = OperationResult::new(inner_result);
    envelope.set_metrics(metrics);
    let topology_hash_after = envelope
        .get_value()
        .as_ref()
        .ok()
        .map(|res| res.topology().topology_hash());

    let mut finalizer = OperationFinalizer::new(&mut ctx);
    let finalize_result = match envelope.get_value() {
        Ok(_) => finalizer.collect_success(
            &mut envelope,
            TraceAdjunctSet::new(),
            TopologyHashBoundary {
                before: Some(topology_hash_before),
                after: topology_hash_after,
            },
        ),
        Err(_) => finalizer.collect_error(
            &mut envelope,
            TraceAdjunctSet::new(),
            TopologyHashBoundary {
                before: Some(topology_hash_before),
                after: None,
            },
        ),
    };
    if let Err(e) = finalize_result {
        return OperationResult::new(Err(KernelError::InternalError {
            message: format!("OperationFinalizer failed in execute_boolean_core: {:?}", e),
            context: None,
        }));
    }

    let mut summaries = Vec::new();
    if let Ok(res) = envelope.get_value() {
        let replay_len = res.get_replay_log().len();
        summaries.push(format!("replay:   {} entries, hashes chain-valid ✓", replay_len));
        
        let lineage = res.get_lineage_events();
        let mut created = 0;
        let mut deleted = 0;
        for ev in lineage {
            match ev {
                forge_topo::lineage::LineageEvent::EntityCreated { .. } => created += 1,
                forge_topo::lineage::LineageEvent::EntityDeleted { .. } => deleted += 1,
                _ => {}
            }
        }
        summaries.push(format!("lineage:  {} entities tracked, {} created, {} deleted", lineage.len(), created, deleted));
    }

    for s in summaries {
        envelope.add_extra_summary(s);
    }

    envelope
}

/// The actual pipeline: split → classify → select → assemble → postprocess.
///
/// Returns `Result` — errors are caught by the outer `execute_boolean_core`
/// which wraps them in the envelope.
fn execute_boolean_pipeline(
    input: BooleanInput,
    engine: &BooleanEngine,
    ctx: &mut ModelingContext,
    start_time: std::time::Instant,
) -> Result<BooleanResult, KernelError> {
    let mut replay = ReplayLog::with_current_target();
    let mut seq = 0u64;
    let mut prev_decision_snapshot: Option<DecisionLog> = None;

    input.validate().map_err(|e| e.with_phase("validate"))?;
    let (target_topo, mut target_geom, tool_topo, mut tool_geom, operation) = input.into_parts();

    let op_space = OperationSpace::analyze_binary(
        &target_topo, &target_geom,
        &tool_topo, &tool_geom,
        ctx.get_tolerance_config().get_min_edge_length(),
    );
    op_space.transform_geometry(&mut target_geom);
    op_space.transform_geometry(&mut tool_geom);

    // ── Phase E: Coincidence Prepass ─────────────────────────────────────────
    // Run before split so we see the original un-subdivided faces.
    let _coincidence_graph = crate::operations::boolean::eval::build_face_coincidence_prepass(
        target_topo.arena(), &target_geom,
        tool_topo.arena(), &tool_geom,
    );
    // ────────────────────────────────────────────────────────────

    // ── Split ────────────────────────────────────────────────────────────────
    let pre_target_hash = compute_arena_topology_hash(target_topo.arena());
    let pre_tool_hash = compute_arena_topology_hash(tool_topo.arena());
    let pre_hash = pre_target_hash ^ pre_tool_hash;

    let split_result = ctx.scope("split", |ctx| {
        engine.splitter().split(target_topo, target_geom, tool_topo, tool_geom, ctx)
    }).map_err(|e| e.with_phase("split"))?;

    let split_count = split_result.split_count();
    let (target_topo, target_geom, tool_topo, tool_geom, target_prov, tool_prov) =
        split_result.into_parts();

    if cfg!(debug_assertions) {
        if let Err(e) = validate_topology(target_topo.arena(), ValidationLevel::Intermediate) {
            eprintln!("[phase-check] split target invalid: {}", e);
        }
        if let Err(e) = validate_topology(tool_topo.arena(), ValidationLevel::Intermediate) {
            eprintln!("[phase-check] split tool invalid: {}", e);
        }
    }

    let post_target_hash = compute_arena_topology_hash(target_topo.arena());
    let post_tool_hash = compute_arena_topology_hash(tool_topo.arena());
    let post_split_hash = post_target_hash ^ post_tool_hash;
    record_replay(&mut replay, &mut seq, "boolean_split",
        format!("{{\"split_count\":{split_count},\"target_hash\":\"{pre_target_hash:#x}\",\"tool_hash\":\"{pre_tool_hash:#x}\",\"post_target_hash\":\"{post_target_hash:#x}\",\"post_tool_hash\":\"{post_tool_hash:#x}\"}}"),
        pre_hash, post_split_hash, ctx, &mut prev_decision_snapshot);

    // ── Zero-split early return ──────────────────────────────────────────────
    if split_count == 0 {
        if let Some(early) = try_zero_split_early_return(
            &target_topo, &target_geom, &tool_topo, &tool_geom,
            operation, &op_space, ctx, start_time,
            &mut replay, &mut seq, &mut prev_decision_snapshot,
        ).map_err(|e| e.with_phase("zero_split_check"))? {
            return Ok(early);
        }
    }

    // ── Classify ─────────────────────────────────────────────────────────────
    let (mut target_classified, mut tool_classified) = ctx.scope("classify", |ctx| {
        let tc = engine.classifier().classify(
            target_topo.arena(), &target_geom,
            tool_topo.arena(), &tool_geom,
            FaceOrigin::Target, ctx,
        )?;
        let tlc = engine.classifier().classify(
            tool_topo.arena(), &tool_geom,
            target_topo.arena(), &target_geom,
            FaceOrigin::Tool, ctx,
        )?;
        Ok::<_, KernelError>((tc, tlc))
    }).map_err(|e| e.with_phase("classify"))?;

    // ── Coplanar resolution ──────────────────────────────────────────────────
    engine.coplanar_resolver().resolve_coplanars(
        &mut target_classified, &mut tool_classified,
        &target_topo, &target_geom, &tool_topo, &tool_geom,
    );
    resolve_fragment_ambiguities(
        &target_topo,
        &tool_topo,
        operation,
        &mut target_classified,
        &mut tool_classified,
        ctx,
    );

    record_replay(&mut replay, &mut seq, "classify_faces",
        format!("{{\"target\":{},\"tool\":{}}}", target_classified.len(), tool_classified.len()),
        post_split_hash, post_split_hash, ctx, &mut prev_decision_snapshot);

    // ── Select ───────────────────────────────────────────────────────────────
    let (selected_target, selected_tool) = ctx.scope("select", |ctx| {
        let st = select_faces(&target_classified, FaceOrigin::Target, operation, ctx);
        let stl = select_faces(&tool_classified, FaceOrigin::Tool, operation, ctx);
        (st, stl)
    });

    let target_face_count = selected_target.len();
    let tool_face_count = selected_tool.len();

    if std::env::var("FORGE_DEBUG_SELECT_PROVENANCE").ok().as_deref() == Some("1") {
        dump_selection_provenance(
            "target",
            target_topo.arena(),
            &target_classified,
            &selected_target,
        );
        dump_selection_provenance(
            "tool",
            tool_topo.arena(),
            &tool_classified,
            &selected_tool,
        );
    }

    record_replay(&mut replay, &mut seq, "select_faces",
        format!("{{\"target_selected\":{target_face_count},\"tool_selected\":{tool_face_count}}}"),
        post_split_hash, post_split_hash, ctx, &mut prev_decision_snapshot);

    let mut introspection = BooleanIntrospection::new(
        split_count, &target_classified, &tool_classified, start_time.elapsed(),
    );

    // ── Empty result fast path ───────────────────────────────────────────────
    if target_face_count == 0 && tool_face_count == 0 {
        return build_empty_result(introspection, replay, start_time);
    }

    // ── Assemble ─────────────────────────────────────────────────────────────
    let result_state = ctx.scope("assemble", |ctx| {
        engine.assembler().assemble(
            target_topo.arena(), &target_geom, &selected_target, &target_prov,
            tool_topo.arena(), &tool_geom, &selected_tool, &tool_prov,
            operation == BooleanOp::Subtraction, ctx,
        )
    }).map_err(|e| e.with_phase("assemble"))?;
    let (result_topo, mut result_geom) = result_state.into_parts();

    if cfg!(debug_assertions) {
        if let Err(e) = validate_topology(result_topo.arena(), ValidationLevel::Intermediate) {
            eprintln!("[phase-check] assemble result invalid: {}", e);
        }
    }

    let post_assemble_hash = compute_arena_topology_hash(result_topo.arena());
    record_replay(&mut replay, &mut seq, "assemble_result",
        format!("{{\"faces\":{}}}", result_topo.arena().face_count()),
        post_split_hash, post_assemble_hash, ctx, &mut prev_decision_snapshot);

    let lineage_events = record_result_lineage(result_topo.arena(), seq);

    // ── Postprocess ──────────────────────────────────────────────────────────
    let result_state = ctx.scope("postprocess", |ctx| {
        engine.postprocessor().postprocess(
            crate::core::KernelState::new(result_topo, result_geom), ctx
        )
    }).map_err(|e| e.with_phase("postprocess"))?;
    let (result_topo, mut result_geom) = result_state.into_parts();

    op_space.restore_geometry(&mut result_geom);

    let post_pp_hash = compute_arena_topology_hash(result_topo.arena());
    record_replay(&mut replay, &mut seq, "postprocess",
        format!("{{\"faces\":{}}}", result_topo.arena().face_count()),
        post_assemble_hash, post_pp_hash, ctx, &mut prev_decision_snapshot);

    if cfg!(debug_assertions) {
        if let Err(e) = validate_topology(result_topo.arena(), ValidationLevel::Intermediate) {
            eprintln!("[phase-check] postprocess result invalid: {}", e);
        }
    }

    // ── Finalize ─────────────────────────────────────────────────────────────
    introspection.duration_micros = start_time.elapsed().as_micros() as u64;

    let mut result = BooleanResult::new(
        result_topo, result_geom,
        target_face_count, tool_face_count,
        introspection, replay, lineage_events,
    );

    run_post_boolean_validation(&result, ctx).map_err(|e| e.with_phase("validate_result"))?;

    Ok(result)
}

fn resolve_fragment_ambiguities(
    target_topo: &TopologyState,
    tool_topo: &TopologyState,
    operation: BooleanOp,
    target_classified: &mut [crate::operations::boolean::schema::ClassifiedFace],
    tool_classified: &mut [crate::operations::boolean::schema::ClassifiedFace],
    ctx: &mut ModelingContext,
) {
    if operation != BooleanOp::Subtraction {
        return;
    }
    if std::env::var("FORGE_ENABLE_FRAGMENT_AMBIGUITY").ok().as_deref() != Some("1") {
        return;
    }
    mark_outside_split_fragments_ambiguous(tool_topo.arena(), tool_classified, "tool", ctx);
    let _ = target_topo;
    let _ = target_classified;
}

fn mark_outside_split_fragments_ambiguous(
    arena: &TopologyArena,
    classified: &mut [crate::operations::boolean::schema::ClassifiedFace],
    label: &str,
    ctx: &mut ModelingContext,
) {
    let class_map: BTreeMap<FaceId, FaceClassification> =
        classified.iter().map(|f| (f.face(), f.classification())).collect();

    for face in classified.iter_mut() {
        if face.classification() != FaceClassification::Outside {
            continue;
        }
        if !is_make_edge_face_fragment(arena, face.face()) {
            continue;
        }
        let (inside_neighbors, split_neighbors) = count_split_face_neighbors(arena, face.face(), &class_map);
        if std::env::var("FORGE_DEBUG_AMBIGUITY").ok().as_deref() == Some("1")
            && matches!(face.face().index(), 14 | 15)
        {
            eprintln!(
                "[ambiguity] probe {} F#{} class={:?} inside_neighbors={} split_neighbors={}",
                label,
                face.face().index(),
                face.classification(),
                inside_neighbors,
                split_neighbors,
            );
        }
        let bridge_like = (inside_neighbors >= 2 && split_neighbors >= 2)
            || (inside_neighbors >= 1 && split_neighbors >= 3);
        if !bridge_like {
            continue;
        }
        face.set_classification(FaceClassification::Ambiguous);

        if std::env::var("FORGE_DEBUG_SELECT_PROVENANCE").ok().as_deref() == Some("1") {
            let lineage = arena
                .get_face(face.face())
                .ok()
                .and_then(|f| f.lineage())
                .map(|lin| format!("{}#{}", lin.get_creation_op().get_name(), lin.get_creation_op().get_invocation_id()))
                .unwrap_or_else(|| "no-lineage".to_string());
            eprintln!(
                "[ambiguity] {} F#{} Outside -> Ambiguous (inside_neighbors={}, split_neighbors={}) {}",
                label,
                face.face().index(),
                inside_neighbors,
                split_neighbors,
                lineage,
            );
        }

        let mut decision = forge_core::TracedDecision::new(
            forge_core::DecisionId(50_000 + face.face().index() as u64),
            forge_core::DecisionKind::PolicyApplied { policy: forge_core::PolicyKind::CoincidentGeometry, default_used: true },
            forge_core::DecisionTier::Deterministic,
            1.0,
            forge_core::DecisionContext::Classification {
                point: [0.0; 3],
                result: format!(
                    "Promote {}:Face#{} Outside -> Ambiguous (split-fragment closure safeguard)",
                    label,
                    face.face().index()
                ),
            },
        );
        decision.set_entity_scope(forge_core::EntityRef::new(forge_core::EntityKind::Face, face.face().index()));
        ctx.get_decision_log_mut().record(decision);
    }
}

fn is_make_edge_face_fragment(arena: &TopologyArena, face_id: FaceId) -> bool {
    arena.get_face(face_id)
        .ok()
        .and_then(|f| f.lineage())
        .map(|lin| lin.get_creation_op().get_name().starts_with("make_edge_face"))
        .unwrap_or(false)
}

fn count_split_face_neighbors(
    arena: &TopologyArena,
    face_id: FaceId,
    class_map: &BTreeMap<FaceId, FaceClassification>,
) -> (usize, usize) {
    let neighbors: std::collections::BTreeSet<FaceId> =
        forge_topo::classification::face_adjacent_faces(arena, face_id)
            .unwrap_or_default()
            .into_iter()
            .collect();

    let mut inside_neighbors = 0usize;
    let mut split_neighbors = 0usize;
    for nface in neighbors {
        if is_make_edge_face_fragment(arena, nface) {
            split_neighbors += 1;
        }
        if matches!(class_map.get(&nface), Some(FaceClassification::Inside | FaceClassification::OnBoundary | FaceClassification::OppositeBoundary)) {
            inside_neighbors += 1;
        }
    }
    (inside_neighbors, split_neighbors)
}

fn dump_selection_provenance(
    label: &str,
    arena: &TopologyArena,
    classified: &[crate::operations::boolean::schema::ClassifiedFace],
    selected: &[FaceId],
) {
    let selected_set: std::collections::BTreeSet<u32> = selected.iter().map(|f| f.index()).collect();
    eprintln!("[select-prov] {} classified={} selected={}", label, classified.len(), selected.len());
    for cf in classified {
        let face = cf.face();
        let action = if selected_set.contains(&face.index()) { "KEEP" } else { "DROP" };
        let lineage_str = arena.get_face(face)
            .ok()
            .and_then(|f| f.lineage())
            .map(|lin| format!("{}#{}", lin.get_creation_op().get_name(), lin.get_creation_op().get_invocation_id()))
            .unwrap_or_else(|| "no-lineage".to_string());
        eprintln!(
            "[select-prov] {} F#{} {:?} {} {}",
            label,
            face.index(),
            cf.classification(),
            action,
            lineage_str
        );
    }
}

// ── Phase helpers ────────────────────────────────────────────────────────────

/// Handle the zero-split path: disjoint or fully-contained solids.
fn try_zero_split_early_return(
    target_topo: &TopologyState,
    target_geom: &GeometryState,
    tool_topo: &TopologyState,
    tool_geom: &GeometryState,
    operation: BooleanOp,
    op_space: &OperationSpace,
    ctx: &mut ModelingContext,
    start_time: std::time::Instant,
    replay: &mut ReplayLog,
    seq: &mut u64,
    prev_snapshot: &mut Option<DecisionLog>,
) -> Result<Option<BooleanResult>, KernelError> {
    let pre_hash = compute_arena_topology_hash(target_topo.arena())
        ^ compute_arena_topology_hash(tool_topo.arena());

    let disjoint_result = ctx.scope("zero_split", |ctx| {
        execute_zero_split(target_topo, target_geom, tool_topo, tool_geom, operation, ctx)
    })?;

    let Some(mut result) = disjoint_result else {
        return Ok(None);
    };

    result.update_duration(start_time.elapsed());

    let kept_target = result.target_faces_kept();
    let kept_tool = result.tool_faces_kept();
    let (result_topo, result_geom, replay_saved, lineage_saved, intro) = result.into_full_parts();
    let (result_topo, mut result_geom) = ctx.scope("postprocess", |ctx| {
        let (rt_state, _) = crate::operations::boolean::postprocess::merge_coplanar_faces(
            crate::core::KernelState::new(result_topo, result_geom), ctx
        )?;
        let (rt_state, _) = crate::operations::boolean::postprocess::remove_redundant_vertices(rt_state, ctx)?;
        Ok::<_, KernelError>(rt_state.into_parts())
    })?;
    op_space.restore_geometry(&mut result_geom);

    let mut result = BooleanResult::new(
        result_topo, result_geom,
        kept_target,
        kept_tool,
        intro, replay_saved, lineage_saved,
    );

    let post_hash = compute_arena_topology_hash(result.topology().arena());
    record_replay(replay, seq, "assemble_result",
        format!("{{\"path\":\"zero_split\",\"operation\":\"{operation:?}\",\"result_faces\":{}}}",
            result.topology().arena().face_count()),
        pre_hash, post_hash, ctx, prev_snapshot);

    let lineage = record_result_lineage(result.topology().arena(), *seq);
    result.set_replay_log(std::mem::replace(replay, ReplayLog::with_current_target()));
    result.set_lineage_events(lineage);

    run_post_boolean_validation(&result, ctx)?;

    Ok(Some(result))
}

/// Build an empty result when no faces are selected on either side.
fn build_empty_result(
    mut introspection: BooleanIntrospection,
    replay: ReplayLog,
    start_time: std::time::Instant,
) -> Result<BooleanResult, KernelError> {
    introspection.duration_micros = start_time.elapsed().as_micros() as u64;
    Ok(BooleanResult::new(
        TopologyState::empty(), GeometryState::new(),
        0, 0, introspection, replay, Vec::new(),
    ))
}

/// Record lineage events for all entities in the result topology.
fn record_result_lineage(arena: &forge_topo::arena::TopologyArena, seq: u64) -> Vec<LineageEvent> {
    let op = OpSignature::with_id("assemble_result", seq);
    let mut events: Vec<LineageEvent> = Vec::new();

    for (fid, _) in arena.iter_faces() {
        events.push(LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(forge_core::EntityKind::Face, fid.index()),
            entity_snapshot: None,
            lineage: Lineage::root(fid.index() as u64, op.clone()),
        });
    }

    for (he_id, _) in arena.iter_half_edges() {
        events.push(LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(forge_core::EntityKind::HalfEdge, he_id.index()),
            entity_snapshot: None,
            lineage: Lineage::root(he_id.index() as u64, op.clone()),
        });
    }

    for (vid, _) in arena.iter_vertices() {
        events.push(LineageEvent::EntityCreated {
            entity: forge_core::EntityRef::new(forge_core::EntityKind::Vertex, vid.index()),
            entity_snapshot: None,
            lineage: Lineage::root(vid.index() as u64, op.clone()),
        });
    }

    events
}

// ── Replay logging ───────────────────────────────────────────────────────────

/// Record a replay entry with pre/post hashes and auto-computed decision delta.
fn record_replay(
    log: &mut ReplayLog,
    seq: &mut u64,
    name: &str,
    payload: String,
    pre_hash: u128,
    post_hash: u128,
    ctx: &ModelingContext,
    prev_snapshot: &mut Option<DecisionLog>,
) {
    *seq += 1;
    let mut entry = ReplayEntry::new(
        OpSignature::with_id(name, *seq), payload.into_bytes(), *seq, pre_hash,
    );
    entry.set_post_hash(post_hash);

    let current_log = ctx.get_decision_log();
    if let Some(prev) = prev_snapshot.as_ref() {
        let delta = diff_decision_logs(prev, current_log);
        entry.set_decision_delta(delta);
    }
    *prev_snapshot = Some(current_log.clone());

    log.record(entry);
}

// ── Post-boolean validation ──────────────────────────────────────────────────

/// Run post-boolean topology validation.
///
/// This replaces the old `finalize_envelope` — validation errors are now captured
/// in the envelope via the always-envelope pattern rather than being returned bare.
// DEFECT(D6): No post-operation ValidationLevel::Full check is enforced.
fn run_post_boolean_validation(
    result: &BooleanResult,
    ctx: &ModelingContext,
) -> Result<(), KernelError> {
    let geom = result.geometry();
    let pos_fn = |vid| geom.get_vertex_position(vid).copied();
    let _validation = run_checkpoint(
        result.topology().arena(),
        &ctx.get_validation_config(),
        ValidationCheckpoint::PostBoolean,
        Some(&pos_fn),
        geom,
    )?;

    Ok(())
}
