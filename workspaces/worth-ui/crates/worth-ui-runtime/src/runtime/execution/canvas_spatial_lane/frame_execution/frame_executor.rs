use crate::runtime::execution::canvas_spatial_lane::frame_target::WorthUiCanvasSpatialFrameTargetKind;
use crate::runtime::execution::handle_allocation::resolve_handle_row;
use crate::runtime::{
    WorthUiCanvasSpatialCounters, WorthUiCanvasSpatialFrameDenial,
    WorthUiCanvasSpatialFrameDenialReason, WorthUiCanvasSpatialFrameReceipt,
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialLane, WorthUiCanvasSpatialPlan,
    WorthUiHandleResolutionOutcome, WorthUiPlanNodeInputFamily,
};

pub(crate) struct WorthUiCanvasSpatialFrameExecutor;

impl WorthUiCanvasSpatialFrameExecutor {
    pub(crate) fn execute(
        plan: &WorthUiCanvasSpatialPlan,
        target: WorthUiCanvasSpatialFrameTarget,
    ) -> Result<WorthUiCanvasSpatialFrameReceipt, WorthUiCanvasSpatialFrameDenial> {
        let mut counters = WorthUiCanvasSpatialCounters::default();
        counters.merge_plan_counters(plan.counters());
        let (locator, lane) = match target.kind() {
            WorthUiCanvasSpatialFrameTargetKind::Viewport(value) => (
                value.lane_handle().locator(),
                WorthUiCanvasSpatialLane::ViewportTransform,
            ),
            WorthUiCanvasSpatialFrameTargetKind::Draw(handle) => {
                (handle.locator(), WorthUiCanvasSpatialLane::Draw)
            }
            WorthUiCanvasSpatialFrameTargetKind::HitTest(value) => (
                value.lane_handle().locator(),
                WorthUiCanvasSpatialLane::HitTest,
            ),
            WorthUiCanvasSpatialFrameTargetKind::Overlay(handle) => {
                (handle.locator(), WorthUiCanvasSpatialLane::Overlay)
            }
            WorthUiCanvasSpatialFrameTargetKind::ToolState(handle) => {
                (handle.locator(), WorthUiCanvasSpatialLane::ToolState)
            }
        };
        let plan_index = locator.plan_index();
        let (row, resolution_evidence) = resolve_handle_row(
            plan.handle_receipt().arena_identity(),
            WorthUiPlanNodeInputFamily::CanvasSpatial,
            locator,
            |index| plan.row_for_plan_index(index),
            |row| row.runtime_handle(),
        )
        .map_err(|evidence| {
            let reason = canvas_resolution_denial(evidence.outcome());
            counters.record_certification_failure();
            WorthUiCanvasSpatialFrameDenial::new(reason, Some(plan_index), counters)
                .with_resolution_evidence(evidence)
        })?;
        match lane {
            WorthUiCanvasSpatialLane::ViewportTransform => counters.record_viewport_transform(),
            WorthUiCanvasSpatialLane::Draw => counters.record_draw_pass(),
            WorthUiCanvasSpatialLane::HitTest => counters.record_spatial_hit_test(),
            WorthUiCanvasSpatialLane::Overlay => counters.record_overlay_plan(),
            WorthUiCanvasSpatialLane::ToolState => counters.record_tool_state_attachment(),
        }
        let (visible, hit_regions, overlay_rows, tool_rows) = match lane {
            WorthUiCanvasSpatialLane::Draw => (row.visible_primitive_limit(), 0, 0, 0),
            WorthUiCanvasSpatialLane::HitTest => (0, 1, 0, 0),
            WorthUiCanvasSpatialLane::Overlay => (0, 0, row.overlay_row_limit(), 0),
            WorthUiCanvasSpatialLane::ToolState => (0, 0, 0, row.tool_state_row_limit()),
            WorthUiCanvasSpatialLane::ViewportTransform => (0, 0, 0, 0),
        };
        let requested_breadth = u64::from(visible)
            + u64::from(hit_regions)
            + u64::from(overlay_rows)
            + u64::from(tool_rows)
            + u64::from(matches!(lane, WorthUiCanvasSpatialLane::ViewportTransform));
        Ok(WorthUiCanvasSpatialFrameReceipt::new(
            super::WorthUiCanvasSpatialFrameReceiptInput {
                target,
                lane,
                touched_plan_index: row.plan_index(),
                touched_runtime_handle: row.runtime_handle(),
                visible_primitive_count: visible,
                queried_hit_test_region_count: hit_regions,
                touched_overlay_row_count: overlay_rows,
                touched_tool_state_row_count: tool_rows,
                counters,
                certification: plan.certification(),
                resolution_evidence,
                work_scope: crate::runtime::WorthUiFrameWorkScope::new(
                    requested_breadth,
                    requested_breadth,
                ),
            },
        ))
    }
}

fn canvas_resolution_denial(
    outcome: WorthUiHandleResolutionOutcome,
) -> WorthUiCanvasSpatialFrameDenialReason {
    match outcome {
        WorthUiHandleResolutionOutcome::TargetMissing => {
            WorthUiCanvasSpatialFrameDenialReason::TargetNotInCanvasSpatialPlan
        }
        WorthUiHandleResolutionOutcome::ForeignSessionArena => {
            WorthUiCanvasSpatialFrameDenialReason::TargetArenaMismatch
        }
        WorthUiHandleResolutionOutcome::StaleSlotGeneration => {
            WorthUiCanvasSpatialFrameDenialReason::TargetSlotGenerationMismatch
        }
        WorthUiHandleResolutionOutcome::WrongFamily => {
            WorthUiCanvasSpatialFrameDenialReason::TargetFamilyMismatch
        }
        WorthUiHandleResolutionOutcome::Resolved => {
            unreachable!("resolved handle evidence is not a denial")
        }
    }
}
