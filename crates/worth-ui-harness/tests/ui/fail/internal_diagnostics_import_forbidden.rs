use worth_ui::runtime::diagnostics_projection::WorthUiRuntimeDiagnosticsProjection;

fn main() {
    let _ = std::mem::size_of::<WorthUiRuntimeDiagnosticsProjection<'static>>();
}
