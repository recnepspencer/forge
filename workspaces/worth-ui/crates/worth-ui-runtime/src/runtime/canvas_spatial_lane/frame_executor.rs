use crate::runtime::canvas_spatial_lane::frame_target::WorthUiCanvasSpatialFrameTargetKind;
use crate::runtime::{
    WorthUiCanvasSpatialCounters, WorthUiCanvasSpatialFrameDenial,
    WorthUiCanvasSpatialFrameDenialReason, WorthUiCanvasSpatialFrameReceipt,
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialLane, WorthUiCanvasSpatialPlan,
    WorthUiHandlePlanGeneration,
};

pub(crate) struct WorthUiCanvasSpatialFrameExecutor;

impl WorthUiCanvasSpatialFrameExecutor {
    pub(crate) fn execute(
        plan: &WorthUiCanvasSpatialPlan,
        target: WorthUiCanvasSpatialFrameTarget,
    ) -> Result<WorthUiCanvasSpatialFrameReceipt, WorthUiCanvasSpatialFrameDenial> {
        let mut counters = WorthUiCanvasSpatialCounters::default();
        counters.merge_plan_counters(plan.counters());

        match target.kind() {
            WorthUiCanvasSpatialFrameTargetKind::Viewport(viewport_plan) => {
                counters.record_viewport_transform();
                execute_indexed_target(
                    plan,
                    target,
                    viewport_plan.lane_handle().plan_index(),
                    viewport_plan.lane_handle().plan_generation(),
                    WorthUiCanvasSpatialLane::ViewportTransform,
                    counters,
                )
            }
            WorthUiCanvasSpatialFrameTargetKind::Draw(handle) => execute_indexed_target(
                plan,
                target,
                handle.plan_index(),
                handle.plan_generation(),
                WorthUiCanvasSpatialLane::Draw,
                counters,
            ),
            WorthUiCanvasSpatialFrameTargetKind::HitTest(hit_test_plan) => {
                counters.record_spatial_hit_test();
                execute_indexed_target(
                    plan,
                    target,
                    hit_test_plan.lane_handle().plan_index(),
                    hit_test_plan.lane_handle().plan_generation(),
                    WorthUiCanvasSpatialLane::HitTest,
                    counters,
                )
            }
            WorthUiCanvasSpatialFrameTargetKind::Overlay(overlay_plan) => {
                counters.record_overlay_plan();
                execute_indexed_target(
                    plan,
                    target,
                    overlay_plan.lane_handle().plan_index(),
                    overlay_plan.lane_handle().plan_generation(),
                    WorthUiCanvasSpatialLane::Overlay,
                    counters,
                )
            }
            WorthUiCanvasSpatialFrameTargetKind::ToolState(handle) => {
                counters.record_tool_state_attachment();
                execute_indexed_target(
                    plan,
                    target,
                    handle.plan_index(),
                    handle.plan_generation(),
                    WorthUiCanvasSpatialLane::ToolState,
                    counters,
                )
            }
            #[cfg(test)]
            WorthUiCanvasSpatialFrameTargetKind::DomainGeometryTruthOwner(handle) => {
                counters.record_domain_geometry_truth_read();
                Err(WorthUiCanvasSpatialFrameDenial::new(
                    WorthUiCanvasSpatialFrameDenialReason::DomainGeometryTruthOwnership,
                    Some(handle.plan_index()),
                    counters,
                ))
            }
            #[cfg(test)]
            WorthUiCanvasSpatialFrameTargetKind::RendererInternalOwner(handle) => {
                counters.record_renderer_internal_read();
                Err(WorthUiCanvasSpatialFrameDenial::new(
                    WorthUiCanvasSpatialFrameDenialReason::RendererInternalOwnership,
                    Some(handle.plan_index()),
                    counters,
                ))
            }
            #[cfg(test)]
            WorthUiCanvasSpatialFrameTargetKind::DomainGeometryHitTest(handle) => {
                counters.record_domain_geometry_truth_read();
                Err(WorthUiCanvasSpatialFrameDenial::new(
                    WorthUiCanvasSpatialFrameDenialReason::DomainGeometryTruthRead,
                    Some(handle.plan_index()),
                    counters,
                ))
            }
            #[cfg(test)]
            WorthUiCanvasSpatialFrameTargetKind::Component(handle) => {
                counters.record_denial();
                Err(WorthUiCanvasSpatialFrameDenial::new(
                    WorthUiCanvasSpatialFrameDenialReason::NonCanvasSpatialClaim,
                    Some(handle.plan_index()),
                    counters,
                ))
            }
        }
    }
}

fn execute_indexed_target(
    plan: &WorthUiCanvasSpatialPlan,
    target: WorthUiCanvasSpatialFrameTarget,
    plan_index: u32,
    plan_generation: WorthUiHandlePlanGeneration,
    lane: WorthUiCanvasSpatialLane,
    mut counters: WorthUiCanvasSpatialCounters,
) -> Result<WorthUiCanvasSpatialFrameReceipt, WorthUiCanvasSpatialFrameDenial> {
    let Some(row) = plan.row_for_plan_index(plan_index) else {
        counters.record_denial();
        return Err(WorthUiCanvasSpatialFrameDenial::new(
            WorthUiCanvasSpatialFrameDenialReason::TargetNotInCanvasSpatialPlan,
            Some(plan_index),
            counters,
        ));
    };

    if row.lane_handle().plan_generation() != plan_generation {
        counters.record_certification_failure();
        return Err(WorthUiCanvasSpatialFrameDenial::new(
            WorthUiCanvasSpatialFrameDenialReason::TargetGenerationMismatch,
            Some(plan_index),
            counters,
        ));
    }

    if matches!(lane, WorthUiCanvasSpatialLane::Draw) {
        counters.record_draw_pass();
    }

    Ok(WorthUiCanvasSpatialFrameReceipt::new(
        super::WorthUiCanvasSpatialFrameReceiptInput {
            target,
            lane,
            touched_plan_indexes: vec![row.plan_index()],
            touched_runtime_handles: vec![row.runtime_handle()],
            command_plan_indexes: plan.command_plan_indexes().to_vec(),
            diagnostics_plan_indexes: plan.diagnostics_plan_indexes().to_vec(),
            selection_state_slot_handles: plan.selection_state_slot_handles().to_vec(),
            counters,
            certification: plan.certification(),
        },
    ))
}
