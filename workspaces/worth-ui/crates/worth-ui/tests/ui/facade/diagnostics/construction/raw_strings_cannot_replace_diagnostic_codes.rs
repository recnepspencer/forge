use worth_ui::facade::CapabilityDiagnosticCode;

fn requires_code(_code: CapabilityDiagnosticCode) {}

fn main() {
    requires_code("duplicate");
}
