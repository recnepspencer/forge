use super::semantic_contract::{expected_phase, expected_responsibility};

#[test]
fn same_stem_and_page_redo_phase_substitutions_are_rejected() {
    assert_ne!(
        expected_responsibility("crates/worth-store-recovery-physics/src/redo_replay/plan.rs"),
        expected_responsibility("crates/worth-store-recovery-runtime/src/cleanup/plan.rs")
    );
    assert_ne!(
        expected_responsibility(
            "crates/worth-store-recovery-physics/src/source_precedence/admission.rs",
        ),
        expected_responsibility("crates/worth-store-recovery-runtime/src/entry/admission.rs")
    );
    assert_eq!(
        expected_phase("crates/worth-store-recovery-physics/src/page_redo/eligibility.rs"),
        "phase-4"
    );
    assert_eq!(
        expected_phase("crates/worth-store-recovery-physics/src/page_redo/transition.rs"),
        "phase-5"
    );
}
