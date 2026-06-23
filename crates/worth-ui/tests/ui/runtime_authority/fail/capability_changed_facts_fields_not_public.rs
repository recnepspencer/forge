use worth_ui::facade::{WorthUiCapabilityChangedFacts, WorthUiChangedRuntimeFacts};

fn main() {
    let _forged = WorthUiCapabilityChangedFacts {
        changed_facts: forged_changed_facts(),
        active_snapshot_digest_before: 1,
        active_snapshot_digest_after: 2,
    };
}

fn forged_changed_facts() -> WorthUiChangedRuntimeFacts {
    panic!("fixture should fail before runtime construction")
}
