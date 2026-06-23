use worth_ui::facade::{WorthUiAdmittedRuntimeChangeEvidence, WorthUiCapabilityReloadEvidence};

fn requires_admitted_runtime_change(_evidence: &WorthUiAdmittedRuntimeChangeEvidence) {}

fn capability_reload_evidence() -> WorthUiCapabilityReloadEvidence {
    panic!("fixture should not run")
}

fn main() {
    let capability_evidence = capability_reload_evidence();

    requires_admitted_runtime_change(&capability_evidence);
}
