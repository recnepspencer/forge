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
    BooleanInput, BooleanOp,
};
use crate::operations::boolean::result::{BooleanResult, BooleanIntrospection};
use crate::operations::boolean::classify_schema::{FaceOrigin, FaceClassification, ClassifiedFace};
use crate::shared_ops::vertex::identity::VertexMatchKey;
use crate::operations::boolean::_deprecated::parametric::traits::BooleanEngine;
use crate::operations::boolean::_deprecated::parametric::engines::planar::{planar_engine, planar_engine_parametric};

use crate::operations::boolean::_deprecated::shared::select::select_faces;
use super::super::disjoint::execute_zero_split;
use crate::operations::boolean::_deprecated::shared::assemble::assemble_result;

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
        execute_boolean_core(input, ctx, planar_engine_parametric())
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
    execute_boolean_core(input, ctx, planar_engine_parametric())
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
            None,
        ),
        Err(_) => finalizer.collect_error(
            &mut envelope,
            TraceAdjunctSet::new(),
            TopologyHashBoundary {
                before: Some(topology_hash_before),
                after: None,
            },
            None,
        ),
    };
    if let Err(e) = finalize_result {
        return OperationResult::new(Err(KernelError::InternalError {
            message: format!("OperationFinalizer failed in execute_boolean_core: {:?}", e),
            context: None,
        }));
    }

    let summaries = Vec::<String>::new();

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
    let _coincidence_graph = crate::shared_ops::spatial::coincidence::build_face_coincidence_prepass(
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
    crate::shared_steps::replay::record_replay(&mut replay, &mut seq, "boolean_split",
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
    crate::operations::boolean::_deprecated::shared::ambiguity::resolve_fragment_ambiguities(
        &target_topo,
        &tool_topo,
        operation,
        &mut target_classified,
        &mut tool_classified,
        ctx,
    );

    crate::shared_steps::replay::record_replay(&mut replay, &mut seq, "classify_faces",
        format!("{{\"target\":{},\"tool\":{}}}", target_classified.len(), tool_classified.len()),
        post_split_hash, post_split_hash, ctx, &mut prev_decision_snapshot);

    // ── Select ───────────────────────────────────────────────────────────────
    let (selected_target, selected_tool): (Vec<FaceId>, Vec<FaceId>) = ctx.scope("select", |ctx| {
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

    crate::shared_steps::replay::record_replay(&mut replay, &mut seq, "select_faces",
        format!("{{\"target_selected\":{target_face_count},\"tool_selected\":{tool_face_count}}}"),
        post_split_hash, post_split_hash, ctx, &mut prev_decision_snapshot);

    let mut introspection = BooleanIntrospection::new(
        split_count, &target_classified, &tool_classified, start_time.elapsed(),
    );

    // ── Empty result fast path ───────────────────────────────────────────────
    if target_face_count == 0 && tool_face_count == 0 {
        return crate::operations::boolean::_deprecated::shared::empty::build_empty_result(introspection, replay, start_time);
    }

    // ── Assemble ─────────────────────────────────────────────────────────────
    let result_state = ctx.scope("assemble", |ctx| {
        engine.assembler().assemble(
            target_topo.arena(), &target_geom, &selected_target, &target_prov,
            tool_topo.arena(), &tool_geom, &selected_tool, &tool_prov,
            operation == BooleanOp::Subtraction, ctx,
        )
    }).map_err(|e| e.with_phase("assemble"))?;
    let (result_topo, result_geom, result_brep) = result_state.into_parts();

    if cfg!(debug_assertions) {
        if let Err(e) = validate_topology(result_topo.arena(), ValidationLevel::Intermediate) {
            eprintln!("[phase-check] assemble result invalid: {}", e);
        }
    }

    let post_assemble_hash = compute_arena_topology_hash(result_topo.arena());
    crate::shared_steps::replay::record_replay(&mut replay, &mut seq, "assemble_result",
        format!("{{\"faces\":{}}}", result_topo.arena().face_count()),
        post_split_hash, post_assemble_hash, ctx, &mut prev_decision_snapshot);

    let lineage_events = forge_topo::topology::history::bulk_stamp::record_result_lineage(result_topo.arena(), seq);

    // ── Postprocess ──────────────────────────────────────────────────────────
    let result_state = ctx.scope("postprocess", |ctx| {
        engine.postprocessor().postprocess(
            crate::core::KernelState::new(result_topo, result_geom, result_brep), ctx
        )
    }).map_err(|e| e.with_phase("postprocess"))?;
    let (result_topo, mut result_geom, result_brep) = result_state.into_parts();

    op_space.restore_geometry(&mut result_geom);

    let post_pp_hash = compute_arena_topology_hash(result_topo.arena());
    crate::shared_steps::replay::record_replay(&mut replay, &mut seq, "postprocess",
        format!("{{\"faces\":{}}}", result_topo.arena().face_count()),
        post_assemble_hash, post_pp_hash, ctx, &mut prev_decision_snapshot);

    if cfg!(debug_assertions) {
        if let Err(e) = validate_topology(result_topo.arena(), ValidationLevel::Intermediate) {
            eprintln!("[phase-check] postprocess result invalid: {}", e);
        }
    }

    // ── Finalize ─────────────────────────────────────────────────────────────
    introspection.duration_micros = start_time.elapsed().as_micros() as u64;

    let result = BooleanResult::new(
        result_topo, result_geom, result_brep,
        target_face_count, tool_face_count,
        introspection,
    );

    crate::shared_steps::validation::run_post_boolean_validation(&result, ctx).map_err(|e| e.with_phase("validate_result"))?;

    Ok(result)
}



fn dump_selection_provenance(
    label: &str,
    arena: &TopologyArena,
    classified: &[crate::operations::boolean::classify_schema::ClassifiedFace],
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
    let (result_topo, result_geom, result_brep, intro) = result.into_parts();
    let (result_topo, mut result_geom, result_brep) = ctx.scope("postprocess", |ctx| {
        let (rt_state, _) = crate::operations::boolean::_deprecated::parametric::postprocess::merge_coplanar_faces(
            crate::core::KernelState::new(result_topo, result_geom, result_brep), ctx
        )?;
        let (rt_state, _) = crate::operations::boolean::_deprecated::parametric::postprocess::remove_redundant_vertices(rt_state, ctx)?;
        Ok::<_, KernelError>(rt_state.into_parts())
    })?;
    op_space.restore_geometry(&mut result_geom);

    let result = BooleanResult::new(
        result_topo, result_geom, result_brep,
        kept_target,
        kept_tool,
        intro,
    );

    let post_hash = compute_arena_topology_hash(result.topology().arena());
    crate::shared_steps::replay::record_replay(replay, seq, "assemble_result",
        format!("{{\"path\":\"zero_split\",\"operation\":\"{operation:?}\",\"result_faces\":{}}}",
            result.topology().arena().face_count()),
        pre_hash, post_hash, ctx, prev_snapshot);

    crate::shared_steps::validation::run_post_boolean_validation(&result, ctx)?;

    Ok(Some(result))
}


