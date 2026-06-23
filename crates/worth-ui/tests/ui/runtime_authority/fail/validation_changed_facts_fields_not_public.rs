use worth_ui::facade::{WorthUiChangedRuntimeFacts, WorthUiValidationChangedFacts};

fn forged_changed_facts() -> WorthUiChangedRuntimeFacts {
    panic!("fixture should not run")
}

fn main() {
    let _forged = WorthUiValidationChangedFacts {
        changed_facts: forged_changed_facts(),
        active_artifact_digest_before: 1,
        active_artifact_digest_after: 2,
        active_plan_digest_before: 3,
        active_plan_digest_after: 4,
    };
}
