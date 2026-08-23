use std::collections::BTreeSet;

use super::semantic_contract::expected_destination;

#[test]
fn unknown_plausible_destination_has_no_compiled_contract() {
    assert!(expected_destination(
        "crates/worth-store-recovery-runtime/src/orchestration/planning/uncontracted.rs",
    )
    .is_none());
}

#[test]
fn same_stem_paths_across_domains_have_distinct_responsibilities() {
    let runtime =
        expected_destination("crates/worth-store-recovery-runtime/src/observation/report.rs")
            .unwrap();
    let observer = expected_destination(
        "crates/worth-store-offline-verifier/src/c8_recovery_observation/report.rs",
    )
    .unwrap();

    assert_ne!(runtime.responsibility, observer.responsibility);
    assert_ne!(runtime.owner, observer.owner);
}

#[test]
fn denial_paths_have_domain_specific_responsibilities() {
    let paths = [
        "crates/worth-store-recovery-physics/src/wal_prefix/denial.rs",
        "crates/worth-store-recovery-physics/src/redo_replay/denial.rs",
        "crates/worth-store-recovery-physics/src/operation_reconciliation/denial.rs",
        "crates/worth-store-recovery-physics/src/recovery_budget/denial.rs",
    ];
    let responsibilities = paths
        .into_iter()
        .map(|path| expected_destination(path).unwrap().responsibility)
        .collect::<BTreeSet<_>>();

    assert_eq!(responsibilities.len(), paths.len());
}

#[test]
fn same_stem_and_page_redo_phase_substitutions_are_rejected() {
    let physics_plan =
        expected_destination("crates/worth-store-recovery-physics/src/redo_replay/plan.rs")
            .unwrap();
    let cleanup_plan =
        expected_destination("crates/worth-store-recovery-runtime/src/cleanup/plan.rs").unwrap();
    assert_ne!(physics_plan.responsibility, cleanup_plan.responsibility);

    let eligibility =
        expected_destination("crates/worth-store-recovery-physics/src/page_redo/eligibility.rs")
            .unwrap();
    let staged =
        expected_destination("crates/worth-store-recovery-runtime/src/progression/staged.rs")
            .unwrap();
    assert_eq!(eligibility.phase, "phase-4");
    assert_eq!(staged.phase, "phase-5");
}
