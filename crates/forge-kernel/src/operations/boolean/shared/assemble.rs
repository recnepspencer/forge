//! Result assembly and scale computation for the main boolean pipeline.
//!
//! DOMAIN: Copy selected faces into a fresh arena, stitch twins, and compute
//! characteristic scales for adaptive tolerances.

use std::collections::BTreeMap;

use forge_core::KernelError;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::state::TopologyState;

use crate::core::{KernelState, ModelingContext};
use crate::geometry_state::GeometryState;
use crate::shared_ops::vertex_identity::VertexMatchKey;

use crate::shared_ops::copy::{copy_faces, VertexDedup};
use crate::shared_steps::vertex_repair::repair_vertex_identity;
use crate::shared_steps::stitch::stitch_twins;
use crate::analysis::proof_validation::diagnose_pipeline::{diagnose_arena, PipelineStage};

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
    let target_positions = target_arena
        .iter_vertices()
        .filter_map(|(vid, _)| target_geom.get_vertex_position(vid));
    let tool_positions = tool_arena
        .iter_vertices()
        .filter_map(|(vid, _)| tool_geom.get_vertex_position(vid));

    let characteristic_scale = forge_geom::spatial::bounds::compute_characteristic_scale(
        target_positions.chain(tool_positions),
    );

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();
    let mut result_geom = GeometryState::new();

    let mut global_vertex_map: BTreeMap<VertexMatchKey, VertexId> = BTreeMap::new();
    let weld_floor = ctx.get_gap_closure().get_max_gap() * 4.0;
    let weld_linear = (characteristic_scale.max(1e-15) * 1e-8).max(weld_floor);
    let mut spatial_index = crate::shared_ops::copy::VertexWelder::with_linear_tolerance(weld_linear);

    let mut all_new_he_ids: Vec<HalfEdgeId> = Vec::new();

    let mut target_dedup = VertexDedup::new();
    copy_faces(
        &mut draft,
        &mut result_geom,
        &mut target_dedup,
        &mut all_new_he_ids,
        &mut global_vertex_map,
        &mut spatial_index,
        target_arena,
        target_geom,
        target_faces,
        false,
        "target",
        Some(target_prov),
    )?;

    let mut tool_dedup = VertexDedup::new();
    copy_faces(
        &mut draft,
        &mut result_geom,
        &mut tool_dedup,
        &mut all_new_he_ids,
        &mut global_vertex_map,
        &mut spatial_index,
        tool_arena,
        tool_geom,
        tool_faces,
        reverse_tool,
        "tool",
        Some(tool_prov),
    )?;

    forge_topo::algorithms::simplify::cleanup_degenerate_topology(&mut draft)?;

    repair_vertex_identity(
        &mut draft,
        &result_geom,
        spatial_index.weld_tolerance_sq().sqrt(),
        ctx,
    )?;

    let _post_copy_diag = diagnose_arena(draft.arena(), PipelineStage::PostCopy);

    let active_he_ids: Vec<HalfEdgeId> = all_new_he_ids
        .iter()
        .filter(|id| draft.arena().get_half_edge(**id).is_ok())
        .copied()
        .collect();

    let report = stitch_twins(
        &mut draft,
        &active_he_ids,
        &result_geom,
        spatial_index.weld_tolerance_sq(),
        ctx,
    )?;
    if !report.is_fully_paired() {
        let cleaned = forge_topo::algorithms::simplify::cleanup_degenerate_topology(&mut draft)?;
        if cleaned > 0 {
            let remaining_he: Vec<HalfEdgeId> =
                draft.arena().iter_half_edges().map(|(id, _)| id).collect();
            let retry = stitch_twins(
                &mut draft,
                &remaining_he,
                &result_geom,
                spatial_index.weld_tolerance_sq(),
                ctx,
            )?;
            retry.require_fully_paired(&draft, &result_geom, ctx)?;
        } else {
            report.require_fully_paired(&draft, &result_geom, ctx)?;
        }
    }

    let topo = draft.commit()?;
    Ok(KernelState::new(
        topo,
        result_geom,
        crate::brep::state::BrepState::new(),
    ))
}
