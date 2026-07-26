#[test]
fn convergence_epoch_authority_boundaries_hold() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/ui/installed_domain/convergence_epoch/valid_public_typestate_progression.rs");
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/authority_construction_is_sealed.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/terminal_authority_cannot_escalate.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/terminal_kind_cannot_be_relabelled.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/legacy_receipt_cannot_advance.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_outcome_must_be_resolved.rs",
    );
}
