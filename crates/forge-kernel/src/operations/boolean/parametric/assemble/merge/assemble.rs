//! Result assembly and scale computation for the main boolean pipeline.
//!
//! DOMAIN: Copy selected faces into a fresh arena, stitch twins, and compute
//! characteristic scales for adaptive tolerances.

use std::collections::BTreeMap;

use forge_core::KernelError;
use forge_topo::state::TopologyState;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};

use crate::core::{ModelingContext, KernelState};
use crate::geometry_state::GeometryState;
use crate::shared_ops::vertex_identity::VertexMatchKey;

use crate::analysis::proof_validation::diagnose_pipeline::{diagnose_arena, PipelineStage};
use super::super::copy::{copy_faces, repair_vertex_identity, VertexDedup};
use super::super::stitch::stitch_twins;
use super::super::cleanup::cleanup_degenerate_topology;

/// Assemble the Boolean result from selected faces of both arenas.
pub(crate) fn assemble_result(
    target_arena: &forge_topo::arena::TopologyArena,
    target_geom: &GeometryState,
    target_faces: &[FaceId],
    target_prov: &BTreeMap<VertexId, VertexMatchKey>,
    tool_arena: &forge_topo::arena::TopologyArena,
    tool_geom: &GeometryState,
    tool_faces: &[FaceId],
    tool_prov: &BTreeMap<VertexId, VertexMatchKey>,
    reverse_tool: bool,
    ctx: &mut ModelingContext,
) -> Result<KernelState, KernelError> {
    let characteristic_scale = compute_characteristic_scale(
        target_arena, target_geom, tool_arena, tool_geom,
    );

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut result_geom = GeometryState::new();

    let mut global_vertex_map: BTreeMap<VertexMatchKey, VertexId> = BTreeMap::new();
    let weld_floor = ctx.get_gap_closure().get_max_gap() * 4.0;
    let weld_linear = (characteristic_scale.max(1e-15) * 1e-8).max(weld_floor);
    let mut spatial_index = super::super::copy::VertexWelder::with_linear_tolerance(weld_linear);

    let mut all_new_he_ids: Vec<HalfEdgeId> = Vec::new();

    let mut target_dedup = VertexDedup::new();
    copy_faces(
        &mut draft, &mut result_geom, &mut target_dedup,
        &mut all_new_he_ids,
        &mut global_vertex_map,
        &mut spatial_index,
        target_arena, target_geom, target_faces,
        false,
        "target",
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
        "tool",
        Some(tool_prov),
    )?;

    cleanup_degenerate_topology(&mut draft, &result_geom)?;

    repair_vertex_identity(
        &mut draft, &result_geom,
        spatial_index.weld_tolerance_sq().sqrt(),
        ctx,
    )?;

    let _post_copy_diag = diagnose_arena(draft.arena(), PipelineStage::PostCopy);

    let active_he_ids: Vec<HalfEdgeId> = all_new_he_ids.iter()
        .filter(|id| draft.arena().get_half_edge(**id).is_ok())
        .copied()
        .collect();

    let report = stitch_twins(&mut draft, &active_he_ids, &result_geom, spatial_index.weld_tolerance_sq(), ctx)?;
    if !report.is_fully_paired() {
        let cleaned = cleanup_degenerate_topology(&mut draft, &result_geom)?;
        if cleaned > 0 {
            let remaining_he: Vec<HalfEdgeId> = draft.arena().iter_half_edges()
                .map(|(id, _)| id)
                .collect();
            let retry = stitch_twins(&mut draft, &remaining_he, &result_geom, spatial_index.weld_tolerance_sq(), ctx)?;
            retry.require_fully_paired(&draft, &result_geom, ctx)?;
        } else {
            report.require_fully_paired(&draft, &result_geom, ctx)?;
        }
    }

    let topo = draft.commit()?;
    Ok(KernelState::new(topo, result_geom, crate::brep::state::BrepState::new()))
}

/// Compute the characteristic scale of two input solids for adaptive tolerances.
///
/// Returns the maximum bounding box diagonal of vertices across both arenas.
/// Floored at 1e-15 to prevent division-by-zero for degenerate geometry.
pub(super) fn compute_characteristic_scale(
    target_arena: &forge_topo::arena::TopologyArena,
    target_geom: &GeometryState,
    tool_arena: &forge_topo::arena::TopologyArena,
    tool_geom: &GeometryState,
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
