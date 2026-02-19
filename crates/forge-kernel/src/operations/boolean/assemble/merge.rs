//! Main boolean execution logic (split, classify, assemble).

use std::collections::HashMap;

use forge_core::{KernelError, OperationResult, OperationMetrics};
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};

use crate::analysis::proof_validation::checkpoint::{run_checkpoint, ValidationCheckpoint};

use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;
use crate::operations::boolean::schema::{BooleanInput, BooleanOp, BooleanResult, FaceOrigin};
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
    let start_time = std::time::Instant::now();

    input.validate()?;

    let (target_topo, target_geom, tool_topo, tool_geom, operation) = input.into_parts();

    let mut ctx = ModelingContext::default();
    ctx.enable_auto_persist();

    // ── Split phase ─────────────────────────────────────────────────
    let split_result = ctx.scope("split", |ctx| {
        split_all_faces(target_topo, target_geom, tool_topo, tool_geom, ctx)
    }).map_err(|e| {
        eprintln!("PHASE_FAIL: split - {e:?}");
        e
    })?;

    let split_count = split_result.split_count();
    let (target_topo, target_geom, tool_topo, tool_geom, target_prov, tool_prov) = split_result.into_parts();

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
            let mut envelope = wrap_boolean_result(result, start_time);
            envelope.set_decision_log(ctx.take_decision_log());
            return Ok(envelope);
        }
    }

    // ── Classify phase ──────────────────────────────────────────────
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

    // ── Select phase ────────────────────────────────────────────────
    let (selected_target, selected_tool) = ctx.scope("select", |ctx| {
        let st = select_faces(&target_classified, FaceOrigin::Target, operation, ctx);
        let stl = select_faces(&tool_classified, FaceOrigin::Tool, operation, ctx);
        (st, stl)
    });

    let target_face_count = selected_target.len();
    let tool_face_count = selected_tool.len();

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
        let result = BooleanResult::new(empty_topo, empty_geom, 0, 0, introspection);
        let mut envelope = wrap_boolean_result(result, start_time);
        envelope.set_decision_log(ctx.take_decision_log());
        return Ok(envelope);
    }

    // ── Assemble phase ──────────────────────────────────────────────
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

    // ── Post-process phase ──────────────────────────────────────────
    let (result_topo, result_geom) = ctx.scope("postprocess", |ctx| {
        let (mut rt, _merged_count) = crate::operations::boolean::postprocess::merge_coplanar_faces(result_topo, &result_geom, ctx)?;
        let (cleaned_topo, _removed_count) = crate::operations::boolean::postprocess::remove_redundant_vertices(rt, &result_geom, ctx)?;
        rt = cleaned_topo;
        Ok::<_, KernelError>((rt, result_geom))
    }).map_err(|e| {
        eprintln!("PHASE_FAIL: postprocess - {e:?}");
        e
    })?;

    introspection.duration_micros = start_time.elapsed().as_micros() as u64;

    let result = BooleanResult::new(
        result_topo,
        result_geom,
        target_face_count,
        tool_face_count,
        introspection,
    );

    let mut envelope = wrap_boolean_result(result, start_time);
    envelope.set_decision_log(ctx.take_decision_log());

    let validation_result = run_checkpoint(
        envelope.get_value().topology().arena(),
        ctx.get_validation_config(),
        ValidationCheckpoint::PostBoolean,
        None,
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
    target_prov: &HashMap<VertexId, VertexMatchKey>,
    tool_arena: &forge_topo::arena::TopologyArena,
    tool_geom: &GeometryStore,
    tool_faces: &[FaceId],
    tool_prov: &HashMap<VertexId, VertexMatchKey>,
    reverse_tool: bool,
    ctx: &mut ModelingContext,
) -> Result<(TopologyState, GeometryStore), KernelError> {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut result_geom = GeometryStore::new();
    
    let mut global_vertex_map: HashMap<VertexMatchKey, VertexId> = HashMap::new();
    let mut spatial_index = super::copy::SpatialVertexIndex::new();

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

    // Filter out any halfedges that cleanup may have deleted
    let active_he_ids: Vec<HalfEdgeId> = all_new_he_ids.iter()
        .filter(|id| draft.arena().get_half_edge(**id).is_ok())
        .copied()
        .collect();

    match stitch_twins(&mut draft, &active_he_ids, &result_geom, ctx) {
        Ok(()) => {}
        Err(e) => {
            let cleaned = cleanup_degenerate_topology(&mut draft, &result_geom)?;
            if cleaned > 0 {
                let remaining_he: Vec<HalfEdgeId> = draft.arena().iter_half_edges()
                    .map(|(id, _)| id)
                    .collect();
                stitch_twins(&mut draft, &remaining_he, &result_geom, ctx)?;
            } else {
                return Err(e);
            }
        }
    }

    let topo = draft.commit()?;
    Ok((topo, result_geom))
}
