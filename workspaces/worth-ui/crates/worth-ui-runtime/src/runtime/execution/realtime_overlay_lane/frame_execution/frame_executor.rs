use crate::runtime::execution::handle_allocation::resolve_handle_row;
use crate::runtime::execution::realtime_overlay_lane::frame_target::WorthUiRealtimeFrameTargetKind;
use crate::runtime::{
    WorthUiHandleResolutionOutcome, WorthUiHudPlan, WorthUiPlanNodeInputFamily,
    WorthUiRealtimeFrameDenial, WorthUiRealtimeFrameDenialReason, WorthUiRealtimeFrameReceipt,
    WorthUiRealtimeFrameTarget, WorthUiRealtimeLaneCounters, WorthUiRealtimeOverlayLane,
};

pub(crate) struct WorthUiRealtimeFrameExecutor;

impl WorthUiRealtimeFrameExecutor {
    pub(crate) fn execute(
        plan: &WorthUiHudPlan,
        target: WorthUiRealtimeFrameTarget,
    ) -> Result<WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameDenial> {
        let counters = WorthUiRealtimeLaneCounters::default();
        match target.kind() {
            WorthUiRealtimeFrameTargetKind::RendererSurface(handle) => execute_renderer_surface(
                plan,
                target,
                handle.plan_index(),
                handle.locator(),
                counters,
            ),
        }
    }
}

fn execute_renderer_surface(
    plan: &WorthUiHudPlan,
    target: WorthUiRealtimeFrameTarget,
    plan_index: u32,
    target_locator: crate::runtime::WorthUiRuntimeHandleLocator,
    mut counters: WorthUiRealtimeLaneCounters,
) -> Result<WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameDenial> {
    let (row, resolution_evidence) = resolve_handle_row(
        plan.handle_receipt().arena_identity(),
        WorthUiPlanNodeInputFamily::RealtimeOverlay,
        target_locator,
        |index| plan.row_for_plan_index(index),
        |row| row.runtime_handle(),
    )
    .map_err(|evidence| {
        let reason = realtime_resolution_denial(evidence.outcome());
        if reason == WorthUiRealtimeFrameDenialReason::TargetNotInHudPlan {
            counters.record_denial();
        } else {
            counters.record_certification_failure();
        }
        WorthUiRealtimeFrameDenial::new(reason, Some(plan_index), counters)
            .with_resolution_evidence(evidence)
    })?;

    counters.record_frame_synchronized_pass();
    counters.record_renderer_surface_handoff();
    let surface = row.renderer_surface_admission();
    counters.record_targeted_overlay_rows(surface.overlay_row_limit());
    Ok(WorthUiRealtimeFrameReceipt::new(
        super::WorthUiRealtimeFrameReceiptInput {
            target,
            lane: WorthUiRealtimeOverlayLane::HudOverlay,
            renderer_surface_admission: surface,
            touched_plan_index: row.plan_index(),
            touched_runtime_handle: row.runtime_handle(),
            touched_overlay_row_count: surface.overlay_row_limit(),
            counters,
            certification: plan.certification(row),
            resolution_evidence,
            work_scope: crate::runtime::WorthUiFrameWorkScope::new(
                u64::from(surface.overlay_row_limit()),
                counters.targeted_overlay_row_count() as u64,
            ),
        },
    ))
}

fn realtime_resolution_denial(
    outcome: WorthUiHandleResolutionOutcome,
) -> WorthUiRealtimeFrameDenialReason {
    match outcome {
        WorthUiHandleResolutionOutcome::TargetMissing => {
            WorthUiRealtimeFrameDenialReason::TargetNotInHudPlan
        }
        WorthUiHandleResolutionOutcome::ForeignSessionArena => {
            WorthUiRealtimeFrameDenialReason::TargetArenaMismatch
        }
        WorthUiHandleResolutionOutcome::StaleSlotGeneration => {
            WorthUiRealtimeFrameDenialReason::TargetSlotGenerationMismatch
        }
        WorthUiHandleResolutionOutcome::WrongFamily => {
            WorthUiRealtimeFrameDenialReason::TargetFamilyMismatch
        }
        WorthUiHandleResolutionOutcome::Resolved => {
            unreachable!("resolved handle evidence is not a denial")
        }
    }
}
