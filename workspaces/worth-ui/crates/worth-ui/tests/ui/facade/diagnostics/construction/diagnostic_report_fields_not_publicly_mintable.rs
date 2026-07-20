use worth_ui::facade::{
    diagnostics::{CapabilityRegistrationReport, CapabilitySnapshot},
};

fn main() {
    let _report = CapabilityRegistrationReport {
        accepted_snapshot: impossible_snapshot(),
        registration_diagnostics: Vec::new(),
    };
}

fn impossible_snapshot() -> CapabilitySnapshot {
    loop {}
}
