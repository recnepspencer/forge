use worth_ui::facade::diagnostics::WorthUiRuntimeDiagnosticCode;

fn takes_code(_: WorthUiRuntimeDiagnosticCode) {}

fn main() {
    takes_code("reload.failure_preserved");
}
