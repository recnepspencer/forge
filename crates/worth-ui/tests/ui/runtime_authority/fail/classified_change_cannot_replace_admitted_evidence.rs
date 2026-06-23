use worth_ui::facade::{WorthUiAdmittedRuntimeChangeEvidence, WorthUiClassifiedRuntimeChange};

fn requires_admitted_evidence(_evidence: WorthUiAdmittedRuntimeChangeEvidence) {}

fn main() {
    let classified: WorthUiClassifiedRuntimeChange = unreachable!();
    requires_admitted_evidence(classified);
}
