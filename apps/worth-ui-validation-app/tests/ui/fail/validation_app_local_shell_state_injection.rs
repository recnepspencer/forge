use worth_ui_harness::facade::{HarnessEvidenceBasis, HarnessEvidenceBundle};

fn main() {
    let mut evidence = HarnessEvidenceBundle::empty();
    let injected_basis = injected_runtime_basis();
    evidence.observe_runtime_launch(injected_basis);
}

fn injected_runtime_basis() -> HarnessEvidenceBasis {
    panic!("compile-fail fixture")
}
