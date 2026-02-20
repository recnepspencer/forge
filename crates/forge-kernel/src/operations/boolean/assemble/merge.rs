//! Main boolean execution logic (split, classify, assemble).

use std::collections::BTreeMap;

use forge_core::{KernelError, OperationResult, OperationMetrics};
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::replay::{ReplayLog, ReplayEntry};
use forge_topo::lineage::{LineageEvent, EntityKind, Lineage, OpSignature};
use forge_topo::hashing::compute_arena_topology_hash;

use crate::analysis::proof_validation::checkpoint::{run_checkpoint, ValidationCheckpoint};
use crate::analysis::proof_validation::diagnose_pipeline::{diagnose_arena, PipelineStage};

use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;
use crate::operations::boolean::schema::{
    BooleanInput, BooleanOp, BooleanResult, FaceOrigin, FaceClassification, BooleanIntrospection,
};
use crate::operations::boolean::split::split_all_faces;
use crate::operations::boolean::classify::classify_faces;
use crate::operations::boolean::eval::{VertexMatchKey, compute_face_centroid};

use super::select::select_faces;
use super::disjoint::execute_zero_split;
use super::copy::{copy_faces, VertexDedup};
use super::stitch::stitch_twins;
use super::cleanup::cleanup_degenerate_topology;

/// Execute a Boolean operation on two solids.
pub fn execute_boolean(input: BooleanInput) -> Result<OperationResult<BooleanResult>, KernelError> {
    let mut ctx = ModelingContext::default();
    ctx.enable_auto_persist();
    execute_boolean_core(input, ctx)
}

/// Execute a Boolean operation with forced classification overrides.
///
/// This re-runs the full Boolean pipeline (split → classify → select →
/// assemble → postprocess) but forces specific face classifications
/// to different values. Used by counterfactual replay to produce
/// genuinely different topology from the same inputs.
///
/// Each override is `(DecisionId, FaceClassification)` where the
/// `DecisionId` matches the face index used in `classify_faces`.
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

/// Shared Boolean execution core. All public entry points delegate here.
fn execute_boolean_core(
    input: BooleanInput,
    mut ctx: ModelingContext,
) -> Result<OperationResult<BooleanResult>, KernelError> {
    let start_time = std::time::Instant::now();
    let mut replay_log = ReplayLog::with_current_target();
    let mut invocation_counter = 0u64;
    let mut lineage_events: Vec<LineageEvent> = Vec::new();

    input.validate()?;

    let (target_topo, mut target_geom, tool_topo, mut tool_geom, operation) = input.into_parts();

    // ── Local Coordinate Transform (P2.4) ───────────────────────────
    let mut all_points = Vec::new();
    for (v_id, _) in target_topo.arena().iter_vertices() {
        if let Some(pos) = target_geom.get_vertex_position(v_id) {
            all_points.push(*pos);
        }
    }
    for (v_id, _) in tool_topo.arena().iter_vertices() {
        if let Some(pos) = tool_geom.get_vertex_position(v_id) {
            all_points.push(*pos);
        }
    }

    let analysis = forge_geom::spatial::local_space::ScaleAnalysis::compute(&all_points, 1e-6);
    let needs_transform = analysis.get_needs_local_transform();
    let local_space = if needs_transform {
        forge_geom::spatial::local_space::LocalCoordinateSpace::from_points(&all_points)
    } else {
        forge_geom::spatial::local_space::LocalCoordinateSpace::identity()
    };
    
    if needs_transform {
        eprintln!("[SCALE_TRANSFORM] Applying local space (exact Rational). Condition={:.1e}, Scale={:.1e}", analysis.get_condition_number(), local_space.get_scale());
        target_geom.transform(&local_space);
        tool_geom.transform(&local_space);
    }

    let pre_split_hash = compute_arena_topology_hash(target_topo.arena())
        ^ compute_arena_topology_hash(tool_topo.arena());

    // ── Split phase ─────────────────────────────────────────────────
    let split_result = ctx.scope("split", |ctx| {
        split_all_faces(target_topo, target_geom, tool_topo, tool_geom, ctx)
    }).map_err(|e| {
        eprintln!("PHASE_FAIL: split - {e:?}");
        e
    })?;

    let split_count = split_result.split_count();
    let (target_topo, target_geom, tool_topo, tool_geom, target_prov, tool_prov) = split_result.into_parts();

    let post_split_hash = compute_arena_topology_hash(target_topo.arena())
        ^ compute_arena_topology_hash(tool_topo.arena());
    invocation_counter += 1;
    let mut split_entry = ReplayEntry::new(
        OpSignature::with_id("boolean_split", invocation_counter),
        format!("{{\"split_count\":{}}}", split_count),
        invocation_counter,
        pre_split_hash,
    );
    split_entry.set_post_hash(post_split_hash);
    replay_log.record(split_entry);

    // ── Zero-split fast path ────────────────────────────────────────
    eprintln!("[DIAGNOSTIC] split_count={}", split_count);
    if split_count == 0 {
        let disjoint_result = ctx.scope("zero_split", |ctx| {
            execute_zero_split(
                &target_topo, &target_geom,
                &tool_topo, &tool_geom,
                operation,
                ctx,
            )
        })?;
        eprintln!("[DIAGNOSTIC] zero_split returned: {}", disjoint_result.is_some());
        if let Some(mut result) = disjoint_result {
            result.update_duration(start_time.elapsed());
            let (topo, mut geom) = result.into_parts();
            if needs_transform {
                geom.inverse_transform(&local_space);
            }
            let mut envelope = wrap_boolean_result(BooleanResult::new(
                topo,
                geom,
                target_topo.arena().face_count(),
                tool_topo.arena().face_count(),
                BooleanIntrospection::default(), // Approximation, but it's zero-split
                ReplayLog::new(),
                Vec::new(),
            ), start_time);
            envelope.set_decision_log(ctx.take_decision_log());
            return Ok(envelope);
        }
    }

    // ── Classify phase ──────────────────────────────────────────────
    let pre_classify_hash = post_split_hash;
    let (target_classified, tool_classified) = ctx.scope("classify", |ctx| {
        let tc = classify_faces(
            target_topo.arena(),
            &target_geom,
            tool_topo.arena(),
            &tool_geom,
            FaceOrigin::Target,
            ctx,
        )?;
        let tlc = classify_faces(
            tool_topo.arena(),
            &tool_geom,
            target_topo.arena(),
            &target_geom,
            FaceOrigin::Tool,
            ctx,
        )?;
        Ok::<_, KernelError>((tc, tlc))
    }).map_err(|e| {
        eprintln!("PHASE_FAIL: classify - {e:?}");
        e
    })?;

    invocation_counter += 1;
    let mut classify_entry = ReplayEntry::new(
        OpSignature::with_id("classify_faces", invocation_counter),
        format!("{{\"target\":{},\"tool\":{}}}", target_classified.len(), tool_classified.len()),
        invocation_counter,
        pre_classify_hash,
    );
    classify_entry.set_post_hash(pre_classify_hash);
    replay_log.record(classify_entry);

    // ── Select phase ────────────────────────────────────────────────
    let (selected_target, selected_tool) = ctx.scope("select", |ctx| {
        let st = select_faces(&target_classified, FaceOrigin::Target, operation, ctx);
        let stl = select_faces(&tool_classified, FaceOrigin::Tool, operation, ctx);
        (st, stl)
    });

    let target_face_count = selected_target.len();
    let tool_face_count = selected_tool.len();

    invocation_counter += 1;
    let mut select_entry = ReplayEntry::new(
        OpSignature::with_id("select_faces", invocation_counter),
        format!("{{\"target_selected\":{},\"tool_selected\":{}}}", target_face_count, tool_face_count),
        invocation_counter,
        pre_classify_hash,
    );
    select_entry.set_post_hash(pre_classify_hash);
    replay_log.record(select_entry);

    eprintln!("[DIAGNOSTIC] classify/select: target_classified={} tool_classified={} target_selected={} tool_selected={} op={:?}",
        target_classified.len(), tool_classified.len(), target_face_count, tool_face_count, operation);
    for cf in &target_classified {
        let centroid = compute_face_centroid(target_topo.arena(), &target_geom, cf.face()).unwrap_or([0.0; 3]);
        let kept = selected_target.contains(&cf.face());
        eprintln!("  Target:Face#{} {:?} {} centroid=[{:.4},{:.4},{:.4}]",
            cf.face().index(), cf.classification(), if kept {"KEEP"} else {"DROP"}, centroid[0], centroid[1], centroid[2]);
    }
    for cf in &tool_classified {
        let centroid = compute_face_centroid(tool_topo.arena(), &tool_geom, cf.face()).unwrap_or([0.0; 3]);
        let kept = selected_tool.contains(&cf.face());
        eprintln!("  Tool:Face#{} {:?} {} centroid=[{:.4},{:.4},{:.4}]",
            cf.face().index(), cf.classification(), if kept {"KEEP"} else {"DROP"}, centroid[0], centroid[1], centroid[2]);
    }

    let mut introspection = crate::operations::boolean::schema::BooleanIntrospection::new(
        split_count,
        &target_classified,
        &tool_classified,
        start_time.elapsed(),
    );

    if target_face_count == 0 && tool_face_count == 0 {
        let empty_topo = TopologyState::empty();
        let empty_geom = GeometryStore::new();
        introspection.duration_micros = start_time.elapsed().as_micros() as u64;
        let result = BooleanResult::new(empty_topo, empty_geom, 0, 0, introspection, replay_log, lineage_events);
        let mut envelope = wrap_boolean_result(result, start_time);
        envelope.set_decision_log(ctx.take_decision_log());
        return Ok(envelope);
    }

    // ── Assemble phase ──────────────────────────────────────────────
    let pre_assemble_hash = pre_classify_hash;
    let (result_topo, result_geom) = ctx.scope("assemble", |ctx| {
        assemble_result(
            target_topo.arena(),
            &target_geom,
            &selected_target,
            &target_prov,
            tool_topo.arena(),
            &tool_geom,
            &selected_tool,
            &tool_prov,
            operation == BooleanOp::Subtraction,
            ctx,
        )
    }).map_err(|e| {
        eprintln!("PHASE_FAIL: assemble - {e:?}");
        e
    })?;

    let post_assemble_hash = compute_arena_topology_hash(result_topo.arena());
    invocation_counter += 1;
    let mut assemble_entry = ReplayEntry::new(
        OpSignature::with_id("assemble_result", invocation_counter),
        format!("{{\"faces\":{}}}", result_topo.arena().face_count()),
        invocation_counter,
        pre_assemble_hash,
    );
    assemble_entry.set_post_hash(post_assemble_hash);
    replay_log.record(assemble_entry);

    for (fid, _) in result_topo.arena().iter_faces() {
        lineage_events.push(LineageEvent::EntityCreated {
            entity_kind: EntityKind::Face,
            lineage: Lineage::root(fid.index() as u64, OpSignature::with_id("assemble_result", invocation_counter)),
        });
    }

    // ── Post-process phase ──────────────────────────────────────────
    let pre_postprocess_hash = post_assemble_hash;
    let (result_topo, mut result_geom) = ctx.scope("postprocess", |ctx| {
        let (mut rt, _merged_count) = crate::operations::boolean::postprocess::merge_coplanar_faces(result_topo, &result_geom, ctx)?;
        let (cleaned_topo, _removed_count) = crate::operations::boolean::postprocess::remove_redundant_vertices(rt, &result_geom, ctx)?;
        rt = cleaned_topo;
        Ok::<_, KernelError>((rt, result_geom))
    }).map_err(|e| {
        eprintln!("PHASE_FAIL: postprocess - {e:?}");
        e
    })?;

    if needs_transform {
        result_geom.inverse_transform(&local_space);
    }

    let post_postprocess_hash = compute_arena_topology_hash(result_topo.arena());
    invocation_counter += 1;
    let mut postprocess_entry = ReplayEntry::new(
        OpSignature::with_id("postprocess", invocation_counter),
        format!("{{\"faces\":{}}}", result_topo.arena().face_count()),
        invocation_counter,
        pre_postprocess_hash,
    );
    postprocess_entry.set_post_hash(post_postprocess_hash);
    replay_log.record(postprocess_entry);

    introspection.duration_micros = start_time.elapsed().as_micros() as u64;

    let result = BooleanResult::new(
        result_topo,
        result_geom,
        target_face_count,
        tool_face_count,
        introspection,
        replay_log,
        lineage_events,
    );

    let mut envelope = wrap_boolean_result(result, start_time);
    envelope.set_decision_log(ctx.take_decision_log());

    let result_geom_ref = envelope.get_value().geometry();
    let pos_fn = |vid| result_geom_ref.get_vertex_position(vid).copied();
    let validation_result = run_checkpoint(
        envelope.get_value().topology().arena(),
        ctx.get_validation_config(),
        ValidationCheckpoint::PostBoolean,
        Some(&pos_fn),
        1e-10,
        1e-12,
    )?;
    envelope.add_validation_result(format!("{:?}", validation_result));

    Ok(envelope)
}

/// Wrap a `BooleanResult` in an `OperationResult` envelope with timing.
fn wrap_boolean_result(
    result: BooleanResult,
    start_time: std::time::Instant,
) -> OperationResult<BooleanResult> {
    let state_hash_after = forge_topo::hashing::compute_arena_topology_hash(result.topology().arena());

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
    envelope.set_state_hash_after(state_hash_after);
    envelope
}


/// Assemble the Boolean result from selected faces of both arenas.
fn assemble_result(
    target_arena: &forge_topo::arena::TopologyArena,
    target_geom: &GeometryStore,
    target_faces: &[FaceId],
    target_prov: &BTreeMap<VertexId, VertexMatchKey>,
    tool_arena: &forge_topo::arena::TopologyArena,
    tool_geom: &GeometryStore,
    tool_faces: &[FaceId],
    tool_prov: &BTreeMap<VertexId, VertexMatchKey>,
    reverse_tool: bool,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, GeometryStore), KernelError> {
    let characteristic_scale = compute_characteristic_scale(
        target_arena, target_geom, tool_arena, tool_geom,
    );

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut result_geom = GeometryStore::new();
    
    let mut global_vertex_map: BTreeMap<VertexMatchKey, VertexId> = BTreeMap::new();
    let mut spatial_index = super::copy::SpatialVertexIndex::new(characteristic_scale);

    let mut all_new_he_ids: Vec<HalfEdgeId> = Vec::new();

    let mut target_dedup = VertexDedup::new();
    copy_faces(
        &mut draft, &mut result_geom, &mut target_dedup,
        &mut all_new_he_ids,
        &mut global_vertex_map,
        &mut spatial_index,
        target_arena, target_geom, target_faces,
        false,
        Some(target_prov),
    )?;

    let mut tool_dedup = VertexDedup::new();
    copy_faces(
        &mut draft, &mut result_geom, &mut tool_dedup,
        &mut all_new_he_ids,
        &mut global_vertex_map,
        &mut spatial_index,
        tool_arena, tool_geom, tool_faces,
        reverse_tool,
        Some(tool_prov),
    )?;

    cleanup_degenerate_topology(&mut draft, &result_geom)?;

    let post_copy_diag = diagnose_arena(draft.arena(), PipelineStage::PostCopy);
    eprintln!("[PIPELINE_DIAG] {post_copy_diag}");

    // Filter out any halfedges that cleanup may have deleted
    let active_he_ids: Vec<HalfEdgeId> = all_new_he_ids.iter()
        .filter(|id| draft.arena().get_half_edge(**id).is_ok())
        .copied()
        .collect();

    match stitch_twins(&mut draft, &active_he_ids, &result_geom, spatial_index.weld_tolerance_sq(), ctx) {
        Ok(()) => {}
        Err(e) => {
            let stitch_diag = diagnose_arena(draft.arena(), PipelineStage::PostStitch);
            eprintln!("[PIPELINE_DIAG] {stitch_diag}");

            let cleaned = cleanup_degenerate_topology(&mut draft, &result_geom)?;
            if cleaned > 0 {
                let remaining_he: Vec<HalfEdgeId> = draft.arena().iter_half_edges()
                    .map(|(id, _)| id)
                    .collect();
                stitch_twins(&mut draft, &remaining_he, &result_geom, spatial_index.weld_tolerance_sq(), ctx)?;
            } else {
                return Err(e);
            }
        }
    }

    let topo = draft.commit()?;
    Ok((topo, result_geom))
}

/// Compute the characteristic scale of two input solids for adaptive tolerances.
///
/// Returns the maximum bounding box diagonal of vertices across both arenas.
/// Floored at 1e-15 to prevent division-by-zero for degenerate geometry.
fn compute_characteristic_scale(
    target_arena: &forge_topo::arena::TopologyArena,
    target_geom: &GeometryStore,
    tool_arena: &forge_topo::arena::TopologyArena,
    tool_geom: &GeometryStore,
) -> f64 {
    let mut min_pos = [f64::INFINITY; 3];
    let mut max_pos = [f64::NEG_INFINITY; 3];

    for (vid, _) in target_arena.iter_vertices() {
        if let Some(pos) = target_geom.get_vertex_position(vid) {
            for i in 0..3 {
                min_pos[i] = min_pos[i].min(pos[i]);
                max_pos[i] = max_pos[i].max(pos[i]);
            }
        }
    }
    for (vid, _) in tool_arena.iter_vertices() {
        if let Some(pos) = tool_geom.get_vertex_position(vid) {
            for i in 0..3 {
                min_pos[i] = min_pos[i].min(pos[i]);
                max_pos[i] = max_pos[i].max(pos[i]);
            }
        }
    }

    let dx = max_pos[0] - min_pos[0];
    let dy = max_pos[1] - min_pos[1];
    let dz = max_pos[2] - min_pos[2];
    let diagonal = (dx * dx + dy * dy + dz * dz).sqrt();

    diagonal.max(1e-15)
}
