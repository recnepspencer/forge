use worth_ui::facade::{
    WorthUiHighFrequencyFramePolicy, WorthUiHighFrequencyFramePolicyDenial,
    WorthUiHighFrequencyFramePolicyDenialReason, WorthUiHudNode, WorthUiHudPlan,
    WorthUiHudPlanDenial, WorthUiHudPlanDenialReason, WorthUiRealtimeCertification,
    WorthUiRealtimeFrameDenial, WorthUiRealtimeFrameDenialReason, WorthUiRealtimeFramePriority,
    WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameTarget, WorthUiRealtimeLaneCounters,
    WorthUiRealtimeOverlayHook, WorthUiRealtimeOverlayLane, WorthUiRendererSurfaceAdmission,
    WorthUiRendererSurfaceHandle,
};

fn accepts_realtime_types(
    _policy: Option<WorthUiHighFrequencyFramePolicy>,
    _policy_denial: Option<WorthUiHighFrequencyFramePolicyDenial>,
    _policy_reason: Option<WorthUiHighFrequencyFramePolicyDenialReason>,
    _node: Option<WorthUiHudNode>,
    _plan: Option<WorthUiHudPlan>,
    _plan_denial: Option<WorthUiHudPlanDenial>,
    _plan_reason: Option<WorthUiHudPlanDenialReason>,
    _certification: Option<WorthUiRealtimeCertification>,
    _frame_denial: Option<WorthUiRealtimeFrameDenial>,
    _frame_reason: Option<WorthUiRealtimeFrameDenialReason>,
    _priority: Option<WorthUiRealtimeFramePriority>,
    _receipt: Option<WorthUiRealtimeFrameReceipt>,
    _target: Option<WorthUiRealtimeFrameTarget>,
    _counters: Option<WorthUiRealtimeLaneCounters>,
    _hook: Option<WorthUiRealtimeOverlayHook>,
    _lane: Option<WorthUiRealtimeOverlayLane>,
    _admission: Option<WorthUiRendererSurfaceAdmission>,
    _handle: Option<WorthUiRendererSurfaceHandle>,
) {
}

fn main() {
    let policy = WorthUiHighFrequencyFramePolicy::frame_budgeted(
        16,
        WorthUiRealtimeFramePriority::HudOverlay,
    )
    .expect("policy compiles");
    let _ = policy.frame_budget_millis();
    let _ = WorthUiRealtimeOverlayLane::HudOverlay.canonical_tag();
    let _ = WorthUiHudPlanDenialReason::MissingRealtimeOverlayHook;
    let _ = WorthUiRealtimeFrameDenialReason::OrdinaryWidgetFallback;
    let _ = WorthUiHighFrequencyFramePolicyDenialReason::ZeroFrameBudgetMillis;
    accepts_realtime_types(
        Some(policy),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(WorthUiRealtimeFramePriority::CriticalOverlay),
        None,
        None,
        None,
        None,
        Some(WorthUiRealtimeOverlayLane::RendererSurfaceHandoff),
        None,
        None,
    );
}
