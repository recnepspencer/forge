use worth_ui::facade::{WorthUiAdmittedRuntimeChangeEvidence, WorthUiValidationReloadEvidence};

fn requires_admitted_runtime_change(_evidence: &WorthUiAdmittedRuntimeChangeEvidence) {}

fn validation_reload_evidence() -> WorthUiValidationReloadEvidence {
    panic!("fixture should not run")
}

fn main() {
    let validation_evidence = validation_reload_evidence();

    requires_admitted_runtime_change(&validation_evidence);
}
