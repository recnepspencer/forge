//! Main boolean execution logic (split, classify, assemble).

use forge_core::KernelError;
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId};

use crate::geometry_store::GeometryStore;
use crate::boolean::schema::{BooleanInput, BooleanOp, BooleanResult, FaceOrigin};
use crate::boolean::split::{split_all_faces, SharedVertexMap};
use crate::boolean::classify::classify_faces;

use super::select::select_faces;
use super::disjoint::execute_zero_split;
use super::copy::{copy_faces, VertexDedup};
use super::stitch::stitch_twins;

/// Execute a Boolean operation on two solids.
///
/// Pipeline:
/// 1. Split faces of both solids along mutual intersections
/// 2. **Fast path**: if no splits, solids don't volumetrically intersect
///    → handle disjoint/touching/coplanar cases directly
/// 3. Classify faces of target relative to tool
/// 4. Classify faces of tool relative to target
/// 5. Select faces based on operation type
/// 6. Copy selected face topology into a new arena with remapped handles
pub fn execute_boolean(input: BooleanInput) -> Result<BooleanResult, KernelError> {
    let start_time = std::time::Instant::now();
    let (target_topo, target_geom, tool_topo, tool_geom, operation) = input.into_parts();

    let split_result = split_all_faces(target_topo, target_geom, tool_topo, tool_geom)?;
    let split_count = split_result.split_count();
    let (target_topo, target_geom, tool_topo, tool_geom, shared_vertices) = split_result.into_parts();

    if split_count == 0 {
        if let Some(mut result) = execute_zero_split(
            &target_topo, &target_geom,
            &tool_topo, &tool_geom,
            operation,
        )? {
            result.update_duration(start_time.elapsed());
            return Ok(result);
        }
    }

    let config = crate::core::ToleranceConfig::default();

    let target_classified = classify_faces(
        target_topo.arena(),
        &target_geom,
        tool_topo.arena(),
        &tool_geom,
        FaceOrigin::Target,
        &config,
    )?;

    let tool_classified = classify_faces(
        tool_topo.arena(),
        &tool_geom,
        target_topo.arena(),
        &target_geom,
        FaceOrigin::Tool,
        &config,
    )?;

    let selected_target = select_faces(&target_classified, FaceOrigin::Target, operation);
    let selected_tool = select_faces(&tool_classified, FaceOrigin::Tool, operation);

    let target_face_count = selected_target.len();
    let tool_face_count = selected_tool.len();

    let mut introspection = crate::boolean::schema::BooleanIntrospection::new(
        split_count,
        &target_classified,
        &tool_classified,
        start_time.elapsed(), // Intermediate duration, close enough
    );

    if target_face_count == 0 && tool_face_count == 0 {
        let empty_topo = TopologyState::empty();
        let empty_geom = GeometryStore::new();
        // Update total duration
        introspection.duration_micros = start_time.elapsed().as_micros() as u64;
        return Ok(BooleanResult::new(empty_topo, empty_geom, 0, 0, introspection));
    }

    let (result_topo, result_geom) = assemble_result(
        target_topo.arena(),
        &target_geom,
        &selected_target,
        tool_topo.arena(),
        &tool_geom,
        &selected_tool,
        &shared_vertices,
        operation == BooleanOp::Subtraction,
    )?;

    // Post-processing: Merge coplanar faces for canonical output
    let (mut result_topo, _merged_count) = crate::boolean::postprocess::merge_coplanar_faces(&result_topo, &result_geom)?;
    
    // Post-processing: Remove redundant (collinear) vertices
    let (cleaned_topo, _removed_count) = crate::boolean::postprocess::remove_redundant_vertices(&result_topo, &result_geom)?;
    result_topo = cleaned_topo;

    // Final duration update
    introspection.duration_micros = start_time.elapsed().as_micros() as u64;

    Ok(BooleanResult::new(
        result_topo,
        result_geom,
        target_face_count,
        tool_face_count,
        introspection,
    ))
}


/// Assemble the Boolean result from selected faces of both arenas.
///
/// Uses position-based vertex deduplication so vertices shared between
/// target and tool (at split boundaries) map to the same new VertexId.
/// After copying all faces, twins are stitched by matching directed
/// edges (origin→dest) with (dest→origin).
fn assemble_result(
    target_arena: &forge_topo::arena::TopologyArena,
    target_geom: &GeometryStore,
    target_faces: &[FaceId],
    tool_arena: &forge_topo::arena::TopologyArena,
    tool_geom: &GeometryStore,
    tool_faces: &[FaceId],
    _shared_vertices: &SharedVertexMap,
    reverse_tool: bool,
) -> Result<(TopologyState, GeometryStore), KernelError> {
    let state = TopologyState::empty();
    let mut draft = state.begin_mutation();
    let mut result_geom = GeometryStore::new();
    let mut vertex_dedup = VertexDedup::new();

    let mut all_new_he_ids: Vec<HalfEdgeId> = Vec::new();

    eprintln!("assembling result with {} target faces and {} tool faces", target_faces.len(), tool_faces.len());

    copy_faces(
        &mut draft, &mut result_geom, &mut vertex_dedup,
        &mut all_new_he_ids,
        target_arena, target_geom, target_faces,
        false, // Never reverse Target
    )?;

    copy_faces(
        &mut draft, &mut result_geom, &mut vertex_dedup,
        &mut all_new_he_ids,
        tool_arena, tool_geom, tool_faces,
        reverse_tool,
    )?;

    stitch_twins(&mut draft, &all_new_he_ids)?;

    let topo = draft.commit()?;
    Ok((topo, result_geom))
}
