//! Result assembly and scale computation for the main boolean pipeline.
//!
//! DOMAIN: Copy selected faces into a fresh arena, stitch twins, and compute
//! characteristic scales for adaptive tolerances.

use std::collections::BTreeMap;

use forge_core::KernelError;
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};

use crate::core::ModelingContext;
use crate::geometry_store::GeometryStore;
use crate::operations::boolean::eval::VertexMatchKey;

use crate::analysis::proof_validation::diagnose_pipeline::{diagnose_arena, PipelineStage};
use super::super::copy::{copy_faces, VertexDedup};
use super::super::stitch::stitch_twins;
use super::super::cleanup::cleanup_degenerate_topology;

/// Assemble the Boolean result from selected faces of both arenas.
pub(super) fn assemble_result(
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
    let mut spatial_index = super::super::copy::SpatialVertexIndex::new(characteristic_scale);

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
pub(super) fn compute_characteristic_scale(
    target_arena: &forge_topo::arena::TopologyArena,
    target_geom: &GeometryStore,
    tool_arena: &forge_topo::arena::TopologyArena,
    tool_geom: &GeometryStore,
) -> f64 {
    let mut min_pos = [f64::INFINITY; 3];
    let mut max_pos = [f64::NEG_INFINITY; 3];

    for (vid, _) in target_arena.iter_vertices() {
        if let Some(pos) = target_geom.get_vertex_position(vid) {
            min_pos = forge_math::linalg::component_min(min_pos, *pos);
            max_pos = forge_math::linalg::component_max(max_pos, *pos);
        }
    }
    for (vid, _) in tool_arena.iter_vertices() {
        if let Some(pos) = tool_geom.get_vertex_position(vid) {
            min_pos = forge_math::linalg::component_min(min_pos, *pos);
            max_pos = forge_math::linalg::component_max(max_pos, *pos);
        }
    }

    let diagonal = forge_math::linalg::norm(forge_math::linalg::sub(max_pos, min_pos));

    diagonal.max(1e-15)
}
