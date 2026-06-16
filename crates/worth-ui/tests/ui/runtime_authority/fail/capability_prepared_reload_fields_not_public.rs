use worth_ui::facade::{WorthUiCapabilityPreparedReload, WorthUiCapabilityReloadEvidence};

fn main() {
    let _forged = WorthUiCapabilityPreparedReload {
        runtime_instance_witness: 1,
        evidence: forged_evidence(),
        candidate_snapshot: None,
    };
}

fn forged_evidence() -> WorthUiCapabilityReloadEvidence {
    panic!("app code must not mint runtime capability reload evidence")
}
