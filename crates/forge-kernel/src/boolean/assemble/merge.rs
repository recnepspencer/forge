//! Main boolean execution logic (split, classify, assemble).

use std::collections::HashMap;

use forge_core::{KernelError, OperationResult, OperationMetrics};
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};

use crate::geometry_store::GeometryStore;
use crate::boolean::schema::{BooleanInput, BooleanOp, BooleanResult, FaceOrigin};
use crate::boolean::split::split_all_faces;
use crate::boolean::classify::classify_faces;
use crate::boolean::eval::VertexMatchKey;

use super::select::select_faces;
use super::disjoint::execute_zero_split;
use super::copy::{copy_faces, VertexDedup};
use super::stitch::stitch_twins;
use super::cleanup::cleanup_degenerate_topology;

/// Execute a Boolean operation on two solids.
pub fn execute_boolean(input: BooleanInput) -> Result<OperationResult<BooleanResult>, KernelError> {
    let start_time = std::time::Instant::now();
    let (target_topo, target_geom, tool_topo, tool_geom, operation) = input.into_parts();

    let mut split_result = split_all_faces(target_topo, target_geom, tool_topo, tool_geom)?;
    let split_count = split_result.split_count();
    let split_log = split_result.take_decision_log();
    let (target_topo, target_geom, tool_topo, tool_geom, target_prov, tool_prov) = split_result.into_parts();

    if split_count == 0 {
        let (disjoint_result, disjoint_log) = execute_zero_split(
            &target_topo, &target_geom,
            &tool_topo, &tool_geom,
            operation,
        )?;
        if let Some(mut result) = disjoint_result {
            result.update_duration(start_time.elapsed());
            let mut envelope = wrap_boolean_result(result, start_time);
            let mut log = split_log;
            log.merge(disjoint_log);
            envelope.set_decision_log(log);
            return Ok(envelope);
        }
    }

    let config = crate::core::ToleranceConfig::default();
    let mut decision_log = split_log;

    let (target_classified, target_classify_log) = classify_faces(
        target_topo.arena(),
        &target_geom,
        tool_topo.arena(),
        &tool_geom,
        FaceOrigin::Target,
        &config,
    )?;
    decision_log.merge(target_classify_log);

    let (tool_classified, tool_classify_log) = classify_faces(
        tool_topo.arena(),
        &tool_geom,
        target_topo.arena(),
        &target_geom,
        FaceOrigin::Tool,
        &config,
    )?;
    decision_log.merge(tool_classify_log);

    let (selected_target, target_select_log) = select_faces(&target_classified, FaceOrigin::Target, operation);
    let (selected_tool, tool_select_log) = select_faces(&tool_classified, FaceOrigin::Tool, operation);
    decision_log.merge(target_select_log);
    decision_log.merge(tool_select_log);

    eprintln!("=== CLASSIFY/SELECT ===");
    eprintln!("target_classified: {} faces", target_classified.len());
    for cf in &target_classified {
        eprintln!("  Target Face#{}: {:?}", cf.face(), cf.classification());
    }
    eprintln!("tool_classified: {} faces", tool_classified.len());
    for cf in &tool_classified {
        eprintln!("  Tool Face#{}: {:?}", cf.face(), cf.classification());
    }
    eprintln!("selected_target: {:?}", selected_target);
    eprintln!("selected_tool: {:?}", selected_tool);

    let target_face_count = selected_target.len();
    let tool_face_count = selected_tool.len();

    let mut introspection = crate::boolean::schema::BooleanIntrospection::new(
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
        envelope.set_decision_log(decision_log);
        return Ok(envelope);
    }

    let (result_topo, result_geom) = assemble_result(
        target_topo.arena(),
        &target_geom,
        &selected_target,
        &target_prov,
        tool_topo.arena(),
        &tool_geom,
        &selected_tool,
        &tool_prov,
        operation == BooleanOp::Subtraction,
    )?;

    let (mut result_topo, _merged_count) = crate::boolean::postprocess::merge_coplanar_faces(&result_topo, &result_geom)?;

    let (cleaned_topo, _removed_count) = crate::boolean::postprocess::remove_redundant_vertices(&result_topo, &result_geom)?;
    result_topo = cleaned_topo;

    introspection.duration_micros = start_time.elapsed().as_micros() as u64;

    let result = BooleanResult::new(
        result_topo,
        result_geom,
        target_face_count,
        tool_face_count,
        introspection,
    );

    let mut envelope = wrap_boolean_result(result, start_time);
    envelope.set_decision_log(decision_log);
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
) -> Result<(TopologyState, GeometryStore), KernelError> {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();
    let mut result_geom = GeometryStore::new();
    
    // Global map for cross-solid vertex gluing (keyed by quantized position)
    let mut global_vertex_map: HashMap<VertexMatchKey, VertexId> = HashMap::new();

    let mut all_new_he_ids: Vec<HalfEdgeId> = Vec::new();

    // Each solid gets its own LOCAL dedup (maps source VertexId → result VertexId).
    // The global_vertex_map handles cross-solid vertex identity via provenance.
    let mut target_dedup = VertexDedup::new();
    copy_faces(
        &mut draft, &mut result_geom, &mut target_dedup,
        &mut all_new_he_ids,
        &mut global_vertex_map,
        target_arena, target_geom, target_faces,
        false, // Never reverse Target
        Some(target_prov),
    )?;

    let mut tool_dedup = VertexDedup::new();
    copy_faces(
        &mut draft, &mut result_geom, &mut tool_dedup,
        &mut all_new_he_ids,
        &mut global_vertex_map,
        tool_arena, tool_geom, tool_faces,
        reverse_tool,
        Some(tool_prov),
    )?;

    cleanup_degenerate_topology(&mut draft, &result_geom)?;

    match stitch_twins(&mut draft, &all_new_he_ids) {
        Ok(()) => {}
        Err(e) => {
            let cleaned = cleanup_degenerate_topology(&mut draft, &result_geom)?;
            if cleaned > 0 {
                let remaining_he: Vec<HalfEdgeId> = draft.arena().iter_half_edges()
                    .map(|(id, _)| id)
                    .collect();
                stitch_twins(&mut draft, &remaining_he)?;
            } else {
                return Err(e);
            }
        }
    }

    let topo = draft.commit()?;
    Ok((topo, result_geom))
}
