use crate::runtime::realtime_overlay_lane::frame_target::WorthUiRealtimeFrameTargetKind;
use crate::runtime::{
    WorthUiHandlePlanGeneration, WorthUiHudPlan, WorthUiRealtimeFrameDenial,
    WorthUiRealtimeFrameDenialReason, WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameTarget,
    WorthUiRealtimeLaneCounters, WorthUiRealtimeOverlayLane,
};

pub(crate) struct WorthUiRealtimeFrameExecutor;

impl WorthUiRealtimeFrameExecutor {
    pub(crate) fn execute(
        plan: &WorthUiHudPlan,
        target: WorthUiRealtimeFrameTarget,
    ) -> Result<WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameDenial> {
        let mut counters = WorthUiRealtimeLaneCounters::default();
        counters.merge_plan_counters(plan.counters());

        match target.kind() {
            WorthUiRealtimeFrameTargetKind::RendererSurface(handle) => execute_renderer_surface(
                plan,
                target,
                handle.plan_index(),
                handle.plan_generation(),
                counters,
            ),
            #[cfg(test)]
            WorthUiRealtimeFrameTargetKind::OrdinaryWidgetFallback(handle) => {
                counters.record_denial();
                Err(WorthUiRealtimeFrameDenial::new(
                    WorthUiRealtimeFrameDenialReason::OrdinaryWidgetFallback,
                    Some(handle.plan_index()),
                    counters,
                ))
            }
            #[cfg(test)]
            WorthUiRealtimeFrameTargetKind::HiddenOrdinaryLayoutPass(handle) => {
                counters.record_ordinary_layout_pass();
                counters.record_certification_failure();
                Err(WorthUiRealtimeFrameDenial::new(
                    WorthUiRealtimeFrameDenialReason::HiddenOrdinaryLayoutPass,
                    Some(handle.plan_index()),
                    counters,
                ))
            }
            #[cfg(test)]
            WorthUiRealtimeFrameTargetKind::ForbiddenWorkSuppression(handle) => {
                counters.record_forbidden_work();
                counters.record_certification_failure();
                Err(WorthUiRealtimeFrameDenial::new(
                    WorthUiRealtimeFrameDenialReason::ForbiddenWorkCounterSuppression,
                    Some(handle.plan_index()),
                    counters,
                ))
            }
        }
    }
}

fn execute_renderer_surface(
    plan: &WorthUiHudPlan,
    target: WorthUiRealtimeFrameTarget,
    plan_index: u32,
    plan_generation: WorthUiHandlePlanGeneration,
    mut counters: WorthUiRealtimeLaneCounters,
) -> Result<WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameDenial> {
    let Some(row) = plan.row_for_plan_index(plan_index) else {
        counters.record_denial();
        return Err(WorthUiRealtimeFrameDenial::new(
            WorthUiRealtimeFrameDenialReason::TargetNotInHudPlan,
            Some(plan_index),
            counters,
        ));
    };

    if row.runtime_handle().plan_generation() != plan_generation {
        counters.record_certification_failure();
        return Err(WorthUiRealtimeFrameDenial::new(
            WorthUiRealtimeFrameDenialReason::TargetGenerationMismatch,
            Some(plan_index),
            counters,
        ));
    }

    counters.record_frame_synchronized_pass();
    counters.record_renderer_surface_handoff();
    Ok(WorthUiRealtimeFrameReceipt::new(
        super::WorthUiRealtimeFrameReceiptInput {
            target,
            lane: WorthUiRealtimeOverlayLane::HudOverlay,
            renderer_surface_admission: row.renderer_surface_admission(),
            touched_plan_indexes: vec![row.plan_index()],
            touched_runtime_handles: vec![row.runtime_handle()],
            command_plan_indexes: plan.command_plan_indexes().to_vec(),
            accessibility_plan_indexes: plan.accessibility_plan_indexes().to_vec(),
            diagnostics_plan_indexes: plan.diagnostics_plan_indexes().to_vec(),
            counters,
            certification: plan.certification(),
        },
    ))
}
