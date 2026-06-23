use worth_ui::facade::{
    CapabilitySnapshot, WorthUiAdmittedCapabilityReloadBatch, WorthUiCapabilityChangedFacts,
};

fn main() {
    let _forged = WorthUiAdmittedCapabilityReloadBatch {
        candidate_snapshot: forged_snapshot(),
        family_rows: Vec::new(),
        changed_facts: forged_changed_facts(),
    };
}

fn forged_snapshot() -> CapabilitySnapshot {
    panic!("fixture should fail before runtime construction")
}

fn forged_changed_facts() -> WorthUiCapabilityChangedFacts {
    panic!("fixture should fail before runtime construction")
}
