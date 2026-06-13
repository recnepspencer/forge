use worth_ui::facade::WorthUiRuntimeDiagnosticCode;

fn takes_code(_: WorthUiRuntimeDiagnosticCode) {}

fn main() {
    takes_code("reload.failure_preserved");
}
