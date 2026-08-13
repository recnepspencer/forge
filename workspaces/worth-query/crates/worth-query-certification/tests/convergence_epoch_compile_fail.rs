#[test]
fn convergence_epoch_authority_boundaries_hold() {
    let cases = trybuild::TestCases::new();
    cases.pass("tests/ui/installed_domain/convergence_epoch/valid_public_typestate_progression.rs");
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/authority_construction_is_sealed.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/retained_candidate_cannot_authorize_publication.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/terminal_kind_cannot_be_relabelled.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/legacy_receipt_cannot_advance.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/started_iteration_cannot_split_or_recombine.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/lifecycle_counters_are_read_only.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yielded_iteration_cannot_expose_managed_run.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yielded_iteration_cannot_expose_lower_observations.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yielded_iteration_cannot_convert_to_managed_run.rs",
    );
    terminal_lower_object_cases(&cases);
    terminal_cleanup_lower_object_cases(&cases);
    yield_cleanup_cases(&cases);
    yield_recovery_cases(&cases);
    yield_denial_cases(&cases);
    readmission_denial_cases(&cases);
    readmission_recovery_cases(&cases);
    readmission_cleanup_cases(&cases);
    domain_evidence_cases(&cases);
}

fn readmission_denial_cases(cases: &trybuild::TestCases) {
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_denial_cannot_expose_lower_objects.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_denial_cannot_convert_to_lower_objects.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_denial_authority_cannot_be_constructed.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_denial_outcomes_must_be_resolved.rs",
    );
}

fn readmission_cleanup_cases(cases: &trybuild::TestCases) {
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_outcome_must_be_resolved.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_cleanup_authority_is_sealed.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_cleanup_pending_cannot_split.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_cleanup_outcomes_must_be_resolved.rs",
    );
}

fn domain_evidence_cases(cases: &trybuild::TestCases) {
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/domain_work_evidence_cannot_be_constructed.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/domain_evidence_raw_string_binders_are_absent.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/completed_domain_evidence_derivation_is_private.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/domain_evidence_binding_cannot_be_constructed.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/retained_candidate_evidence_cannot_be_constructed.rs",
    );
}

fn yield_denial_cases(cases: &trybuild::TestCases) {
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yield_denial_cannot_expose_lower_objects.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yield_denial_cannot_convert_to_lower_objects.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yield_denial_authority_cannot_be_constructed.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yield_denial_outcomes_must_be_resolved.rs",
    );
}

fn readmission_recovery_cases(cases: &trybuild::TestCases) {
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_recovery_cannot_expose_lower_objects.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_recovery_cannot_convert_to_lower_objects.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_recovery_authority_cannot_be_constructed.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/readmission_recovery_outcomes_must_be_resolved.rs",
    );
}

fn yield_cleanup_cases(cases: &trybuild::TestCases) {
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yield_cleanup_cannot_expose_lower_objects.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yield_cleanup_cannot_convert_to_lower_objects.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yield_cleanup_authority_cannot_be_constructed.rs",
    );
}

fn yield_recovery_cases(cases: &trybuild::TestCases) {
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yield_recovery_cannot_expose_managed_authority.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yield_recovery_cannot_convert_to_lower_authority.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yield_recovery_authority_is_sealed.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yield_recovery_authority_cannot_be_constructed.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/yield_recovery_outcomes_must_be_resolved.rs",
    );
}

fn terminal_lower_object_cases(cases: &trybuild::TestCases) {
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/terminal_cannot_expose_managed_run.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/terminal_cannot_convert_to_managed_run.rs",
    );
}

fn terminal_cleanup_lower_object_cases(cases: &trybuild::TestCases) {
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/terminal_cleanup_cannot_expose_lower_objects.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/terminal_cleanup_cannot_convert_to_lower_objects.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/terminal_cleanup_authority_cannot_be_constructed.rs",
    );
    cases.compile_fail(
        "tests/ui/installed_domain/convergence_epoch/terminal_cleanup_outcomes_must_be_resolved.rs",
    );
}
