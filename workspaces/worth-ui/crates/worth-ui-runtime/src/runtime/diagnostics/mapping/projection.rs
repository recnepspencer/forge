use crate::runtime::{
    WorthUiDiagnosticProjectionHook, WorthUiDiagnosticSource, WorthUiRuntimeDiagnostic,
    WorthUiRuntimeDiagnosticCode, WorthUiRuntimeDiagnosticFamily,
};

pub(crate) fn diagnostic_for_projection_hook(
    hook: &WorthUiDiagnosticProjectionHook,
) -> WorthUiRuntimeDiagnostic {
    WorthUiRuntimeDiagnostic::new(
        WorthUiRuntimeDiagnosticFamily::DiagnosticsProjection,
        WorthUiRuntimeDiagnosticCode::DiagnosticsProjectionAdmitted,
        WorthUiDiagnosticSource::ProjectionHook {
            hook_digest: hook.projection_digest(),
        },
        Some(hook.projection_digest()),
    )
}
