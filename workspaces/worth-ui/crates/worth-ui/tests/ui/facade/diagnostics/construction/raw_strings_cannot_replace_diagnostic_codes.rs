use worth_ui::facade::diagnostics::CapabilityDiagnosticCode;

fn requires_code(_code: CapabilityDiagnosticCode) {}

fn main() {
    requires_code("duplicate");
}
