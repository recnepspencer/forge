use worth_ui::facade::WorthUiDiagnosticProjectionHook;

fn main() {
    let hook = WorthUiDiagnosticProjectionHook::projection("workspace.diagnostics.panel");
    let _ = hook.active_artifact_digest();
}
