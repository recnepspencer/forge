use worth_ui::facade::{
    WorthUiDiagnosticsProjection, WorthUiDiagnosticsProjectionCounters,
    WorthUiDiagnosticsProjectionDenial, WorthUiDiagnosticsProjectionDenialReason,
    WorthUiDiagnosticsProjectionHook, WorthUiDiagnosticsProjectionHookEffect,
    WorthUiDiagnosticsProjectionRequest, WorthUiDiagnosticsSurfaceBinding, WorthUiFrameCostRow,
    WorthUiFrameCostSurface, WorthUiFrameCostSurfaceKind, WorthUiPlanInspectionSurface,
    WorthUiQueryStatusRow, WorthUiQueryStatusSurface, WorthUiReloadStatusSurface,
    WorthUiRuntimeDiagnosticsProjection,
};

fn main() {
    let binding = WorthUiDiagnosticsSurfaceBinding::new("workspace.diagnostics.panel");
    let hook = WorthUiDiagnosticsProjectionHook::surface(binding.surface_id());

    assert_ne!(binding.surface_digest(), 0);
    assert_eq!(
        hook.effect(),
        &WorthUiDiagnosticsProjectionHookEffect::PresentationOnly
    );

    let _: Option<WorthUiDiagnosticsProjection> = None;
    let _: Option<WorthUiDiagnosticsProjectionCounters> = None;
    let _: Option<WorthUiDiagnosticsProjectionDenial> = None;
    let _: Option<WorthUiDiagnosticsProjectionRequest<'static>> = None;
    let _: Option<WorthUiRuntimeDiagnosticsProjection<'static>> = None;
    let _: Option<WorthUiReloadStatusSurface> = None;
    let _: Option<WorthUiPlanInspectionSurface> = None;
    let _: Option<WorthUiFrameCostSurface> = None;
    let _: Option<WorthUiFrameCostRow> = None;
    let _: Option<WorthUiQueryStatusSurface> = None;
    let _: Option<WorthUiQueryStatusRow> = None;

    let _ = WorthUiFrameCostSurfaceKind::FoundationalCounter;
    let _ = WorthUiDiagnosticsProjectionDenialReason::FreeformQueryStatusRow;
}
