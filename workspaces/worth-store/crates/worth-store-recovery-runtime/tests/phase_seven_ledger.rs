#[path = "phase_seven_ledger/support.rs"]
mod support;

use std::collections::BTreeMap;

use support::{
    cells, ledger_rows, read, repository_root, source_closure_path, source_identity,
    validate_requirement_inventory, validate_shape, LedgerContract,
};

const GUARANTEES: [&str; 17] = [
    "C8-P7-PLAN-01",
    "C8-P7-AUTHORITY-01",
    "C8-P7-ELIGIBILITY-01",
    "C8-P7-FRESHNESS-01",
    "C8-P7-SCHEDULER-01",
    "C8-P7-SAFETY-01",
    "C8-P7-LIMITS-01",
    "C8-P7-EFFECT-01",
    "C8-P7-FAILURE-01",
    "C8-P7-CANCELLATION-01",
    "C8-P7-CRASH-01",
    "C8-P7-COUNTERS-01",
    "C8-P7-QUIESCENCE-01",
    "C8-P7-PROGRESSION-01",
    "C8-P7-COMPILE-01",
    "C8-P7-API-01",
    "C8-P7-LEDGER-01",
];

const FINDINGS: [(&str, &str); 45] = [
    ("C8-P7-F01", "C8-P7-PLAN-01 C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-SCHEDULER-01 C8-P7-SAFETY-01 C8-P7-LIMITS-01 C8-P7-EFFECT-01 C8-P7-FAILURE-01 C8-P7-CRASH-01 C8-P7-COUNTERS-01 C8-P7-QUIESCENCE-01 C8-P7-PROGRESSION-01 C8-P7-LEDGER-01"),
    ("C8-P7-F02", "C8-P7-PLAN-01 C8-P7-SAFETY-01 C8-P7-LEDGER-01"),
    ("C8-P7-F03", "C8-P7-AUTHORITY-01 C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-EFFECT-01 C8-P7-COMPILE-01 C8-P7-LEDGER-01"),
    ("C8-P7-F04", "C8-P7-FRESHNESS-01 C8-P7-SCHEDULER-01 C8-P7-LIMITS-01 C8-P7-EFFECT-01 C8-P7-FAILURE-01 C8-P7-COUNTERS-01 C8-P7-QUIESCENCE-01 C8-P7-LEDGER-01"),
    ("C8-P7-F05", "C8-P7-SCHEDULER-01 C8-P7-QUIESCENCE-01 C8-P7-PROGRESSION-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F06", "C8-P7-PLAN-01 C8-P7-SAFETY-01 C8-P7-FAILURE-01 C8-P7-CRASH-01 C8-P7-PROGRESSION-01 C8-P7-LEDGER-01"),
    ("C8-P7-F07", "C8-P7-PLAN-01 C8-P7-SAFETY-01 C8-P7-CRASH-01 C8-P7-COUNTERS-01 C8-P7-LEDGER-01"),
    ("C8-P7-F08", "C8-P7-PLAN-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F09", "C8-P7-AUTHORITY-01 C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-SAFETY-01 C8-P7-EFFECT-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F10", "C8-P7-PLAN-01 C8-P7-AUTHORITY-01 C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-SCHEDULER-01 C8-P7-SAFETY-01 C8-P7-LIMITS-01 C8-P7-EFFECT-01 C8-P7-FAILURE-01 C8-P7-CANCELLATION-01 C8-P7-CRASH-01 C8-P7-COUNTERS-01 C8-P7-QUIESCENCE-01 C8-P7-PROGRESSION-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F11", "C8-P7-SCHEDULER-01 C8-P7-FAILURE-01 C8-P7-CANCELLATION-01 C8-P7-COUNTERS-01 C8-P7-QUIESCENCE-01 C8-P7-PROGRESSION-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F12", "C8-P7-SAFETY-01 C8-P7-FAILURE-01 C8-P7-CRASH-01 C8-P7-COUNTERS-01 C8-P7-QUIESCENCE-01 C8-P7-LEDGER-01"),
    ("C8-P7-F13", "C8-P7-AUTHORITY-01 C8-P7-SAFETY-01 C8-P7-EFFECT-01 C8-P7-FAILURE-01 C8-P7-COUNTERS-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F14", "C8-P7-AUTHORITY-01 C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-SAFETY-01 C8-P7-EFFECT-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F15", "C8-P7-EFFECT-01 C8-P7-FAILURE-01 C8-P7-COUNTERS-01 C8-P7-LEDGER-01"),
    ("C8-P7-F16", "C8-P7-LEDGER-01"),
    ("C8-P7-F17", "C8-P7-FRESHNESS-01 C8-P7-SCHEDULER-01 C8-P7-FAILURE-01 C8-P7-COUNTERS-01 C8-P7-QUIESCENCE-01 C8-P7-LEDGER-01"),
    ("C8-P7-F18", "C8-P7-PLAN-01 C8-P7-AUTHORITY-01 C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-SAFETY-01 C8-P7-EFFECT-01 C8-P7-CANCELLATION-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F19", "C8-P7-AUTHORITY-01 C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-SAFETY-01 C8-P7-EFFECT-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F20", "C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F21", "C8-P7-PLAN-01 C8-P7-SAFETY-01 C8-P7-CRASH-01 C8-P7-COUNTERS-01 C8-P7-LEDGER-01"),
    ("C8-P7-F22", "C8-P7-LEDGER-01"),
    ("C8-P7-F23", "C8-P7-PLAN-01 C8-P7-SAFETY-01 C8-P7-LIMITS-01 C8-P7-COUNTERS-01 C8-P7-PROGRESSION-01 C8-P7-LEDGER-01"),
    ("C8-P7-F24", "C8-P7-AUTHORITY-01 C8-P7-SAFETY-01 C8-P7-EFFECT-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F25", "C8-P7-PLAN-01 C8-P7-SAFETY-01 C8-P7-CRASH-01 C8-P7-COUNTERS-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F26", "C8-P7-LEDGER-01"),
    ("C8-P7-F27", "C8-P7-AUTHORITY-01 C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-SAFETY-01 C8-P7-EFFECT-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F28", "C8-P7-PLAN-01 C8-P7-AUTHORITY-01 C8-P7-ELIGIBILITY-01 C8-P7-EFFECT-01 C8-P7-CANCELLATION-01 C8-P7-LEDGER-01"),
    ("C8-P7-F29", "C8-P7-AUTHORITY-01 C8-P7-ELIGIBILITY-01 C8-P7-CANCELLATION-01 C8-P7-PROGRESSION-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F30", "C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-FAILURE-01 C8-P7-COUNTERS-01 C8-P7-PROGRESSION-01 C8-P7-LEDGER-01"),
    ("C8-P7-F31", "C8-P7-PLAN-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F32", "C8-P7-FRESHNESS-01 C8-P7-SCHEDULER-01 C8-P7-FAILURE-01 C8-P7-COUNTERS-01 C8-P7-LEDGER-01"),
    ("C8-P7-F33", "C8-P7-AUTHORITY-01 C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-SAFETY-01 C8-P7-EFFECT-01 C8-P7-FAILURE-01 C8-P7-COUNTERS-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F34", "C8-P7-SCHEDULER-01 C8-P7-FAILURE-01 C8-P7-COUNTERS-01 C8-P7-LEDGER-01"),
    ("C8-P7-F35", "C8-P7-PLAN-01 C8-P7-COUNTERS-01 C8-P7-LEDGER-01"),
    ("C8-P7-F36", "C8-P7-EFFECT-01 C8-P7-FAILURE-01 C8-P7-COUNTERS-01 C8-P7-LEDGER-01"),
    ("C8-P7-F37", "C8-P7-AUTHORITY-01 C8-P7-SCHEDULER-01 C8-P7-EFFECT-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F38", "C8-P7-AUTHORITY-01 C8-P7-SCHEDULER-01 C8-P7-EFFECT-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F39", "C8-P7-FRESHNESS-01 C8-P7-SCHEDULER-01 C8-P7-FAILURE-01 C8-P7-COUNTERS-01 C8-P7-LEDGER-01"),
    ("C8-P7-F40", "C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-LIMITS-01 C8-P7-COUNTERS-01 C8-P7-PROGRESSION-01 C8-P7-LEDGER-01"),
    ("C8-P7-F41", "C8-P7-AUTHORITY-01 C8-P7-EFFECT-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F42", "C8-P7-COUNTERS-01 C8-P7-QUIESCENCE-01 C8-P7-PROGRESSION-01 C8-P7-LEDGER-01"),
    ("C8-P7-F43", "C8-P7-AUTHORITY-01 C8-P7-EFFECT-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F44", "C8-P7-COMPILE-01 C8-P7-LEDGER-01"),
    ("C8-P7-F45", "C8-P7-AUTHORITY-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
];

const REQUIRED_SOURCES: &[(&str, &str)] = &[
    (
        "C8-P7-AUTHORITY-01",
        "workspaces/worth-store/crates/worth-store-physical-backend/src/recovery_media/cleanup.rs",
    ),
    (
        "C8-P7-AUTHORITY-01",
        "workspaces/worth-store/crates/worth-store-physical-backend/src/recovery_media/admitted.rs",
    ),
    (
        "C8-P7-AUTHORITY-01",
        "workspaces/worth-store/crates/worth-store-physical-backend/src/recovery_media/cleanup/revalidation.rs",
    ),
    (
        "C8-P7-AUTHORITY-01",
        "workspaces/worth-store/crates/worth-store-wal/src/cleanup_admission.rs",
    ),
    (
        "C8-P7-AUTHORITY-01",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/recovery_freshness/cleanup/plan.rs",
    ),
    (
        "C8-P7-AUTHORITY-01",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/recovery_coordination/cleanup/close.rs",
    ),
    (
        "C8-P7-AUTHORITY-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/src/entry/admission.rs",
    ),
    (
        "C8-P7-AUTHORITY-01",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/recovery_freshness/registration.rs",
    ),
    (
        "C8-P7-AUTHORITY-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/ui/cleanup_removal_is_not_a_media_method.rs",
    ),
    (
        "C8-P7-SAFETY-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/src/orchestration/planning/selected_source_inventory.rs",
    ),
    (
        "C8-P7-SAFETY-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/phase_seven_cleanup/process/artifact_snapshot/selected_records.rs",
    ),
    (
        "C8-P7-EFFECT-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/phase_seven_faults/revalidation.rs",
    ),
    (
        "C8-P7-LIMITS-01",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/recovery_freshness/binding.rs",
    ),
    (
        "C8-P7-COUNTERS-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/phase_seven_cleanup/process/assertions.rs",
    ),
    (
        "C8-P7-COMPILE-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/ui/cleanup_removal_is_not_a_media_method.stderr",
    ),
    (
        "C8-P7-COMPILE-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/ui/cleanup_fresh_reopen_cannot_be_reused.stderr",
    ),
    (
        "C8-P7-COMPILE-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/ui/cleanup_close_cannot_be_forged.stderr",
    ),
    (
        "C8-P7-COMPILE-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/ui/cleanup_close_cannot_be_reused.stderr",
    ),
    (
        "C8-P7-PROGRESSION-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/ui/recovery_construction_requires_cleanup_close.stderr",
    ),
    (
        "C8-P7-PROGRESSION-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/src/orchestration/mod.rs",
    ),
    (
        "C8-P7-API-01",
        "workspaces/worth-store/tools/store-test-runner/src/fresh_process_recovery_boundary_gate/facade_inventory/runtime_phase_seven_surface_contract/store_cleanup_lifecycle.rs",
    ),
    (
        "C8-P7-API-01",
        "workspaces/worth-store/crates/worth-store/Cargo.toml",
    ),
    (
        "C8-P7-API-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/Cargo.toml",
    ),
    (
        "C8-P7-API-01",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/media_ownership/mod.rs",
    ),
    (
        "C8-P7-CRASH-01",
        "workspaces/worth-store/crates/worth-store-physical-backend/src/filesystem_media/fault_interposition.rs",
    ),
    (
        "C8-P7-FAILURE-01",
        "workspaces/worth-store/crates/worth-store/src/physical_runtime/recovery_coordination/owner/certification_faults.rs",
    ),
    (
        "C8-P7-API-01",
        "workspaces/worth-store/crates/worth-store-recovery-physics/src/source_precedence/checkpoint_base.rs",
    ),
    (
        "C8-P7-EFFECT-01",
        "workspaces/worth-store/crates/worth-store-recovery-runtime/tests/phase_seven_faults/revalidation/denial_matrix.rs",
    ),
    (
        "C8-P7-SCHEDULER-01",
        "workspaces/worth-store/tools/store-test-runner/src/raw_media_owner_gate.rs",
    ),
];

const CONTRACT: LedgerContract<'static> = LedgerContract {
    guarantees: &GUARANTEES,
    findings: &FINDINGS,
    requirement_inventory_sha256:
        "8c8eb013b685d5f4ba59565e6318ae87d03e476b326929097725abb60d94a481",
    guarantee_semantics_sha256: "25da990fe6dddb0b242d7540339a6c8eb17d70454be10d8a375fb48bf1197a20",
    finding_history_sha256: "ffc214cf28671d3d4676cfd90e390b4ac6318ab194efda0541753c1466d280c4",
    audit_history_sha256: "929d44d22cb8eb874465e63e58fd8bc90c60d147e43e8eb99df0fd8639df69d4",
    required_sources: REQUIRED_SOURCES,
};

#[test]
fn phase_seven_ledger_is_exact_source_bound_and_audit_honest() {
    let root = repository_root();
    let ledger = read(&root.join(support::ledger_path()));
    let closure = read(&root.join(source_closure_path()));
    let specification =
        read(&root.join(
            "_docs/worth-store/physical-reconstruction-c8-fresh-process-recovery-and-reopen.md",
        ));
    validate_requirement_inventory(
        &specification,
        &GUARANTEES,
        CONTRACT.requirement_inventory_sha256,
    );
    let validated = validate_shape(&root, &ledger, &closure, CONTRACT);
    let identities = validated
        .ledger_rows
        .iter()
        .map(|row| {
            let values = cells(row);
            (values[0].clone(), values[5].clone())
        })
        .collect::<BTreeMap<_, _>>();
    let mismatches = GUARANTEES
        .iter()
        .filter_map(|guarantee| {
            let expected = source_identity(&root, &validated.closures[*guarantee]);
            (identities[*guarantee] != expected)
                .then(|| format!("{guarantee}={} expected {expected}", identities[*guarantee]))
        })
        .collect::<Vec<_>>();
    assert!(
        mismatches.is_empty(),
        "stale Phase 7 source identities: {mismatches:#?}"
    );
}

#[test]
fn ledger_and_history_mutants_fail_even_with_coordinated_rebinding() {
    let root = repository_root();
    let ledger = read(&root.join(support::ledger_path()));
    let closure = read(&root.join(source_closure_path()));
    let first = ledger_rows(&ledger)[0].clone();
    let demoted = ledger.replacen("| PROVED |", "| IMPLEMENTED |", 1);
    for mutant in [
        ledger.replacen(&first, "", 1),
        ledger.replacen(&first, &format!("{first}\n{first}"), 1),
        ledger.replacen("| C8-P7-F01 | Critical", "| C8-P7-F99 | Critical", 1),
        ledger.replacen("C8-P7-F06 C8-P7-F07 C8-P7-F08", "C8-P7-F06 C8-P7-F08", 1),
        ledger.replacen("A fully materialized no-op recovery", "A recovery", 1),
        ledger.replacen(
            "Every discovered or C8-created artifact receives one exact current retained deferred quarantined unsupported or safely removed disposition",
            "Every artifact receives a disposition",
            1,
        ),
        ledger.replacen("| DEFECTS | C8-P7-F09", "| CLEAN | C8-P7-F09", 1),
        demoted,
        format!(
            "{ledger}\n| external/reviewer | gpt-5.6-sol high | injected | CLEAN | none | none |"
        ),
        format!("{ledger}\n| malformed-audit-row | too-few-cells |"),
    ] {
        assert!(
            std::panic::catch_unwind(|| validate_shape(&root, &mutant, &closure, CONTRACT))
                .is_err()
        );
    }
}

#[test]
fn source_closure_mutants_fail_even_with_coordinated_rebinding() {
    let root = repository_root();
    let ledger = read(&root.join(support::ledger_path()));
    let closure = read(&root.join(source_closure_path()));
    let duplicate = format!("{closure}\n{}", closure.lines().nth(1).unwrap());
    let foreign = closure.replacen(
        "workspaces/worth-store/crates/worth-store-recovery-runtime/src/cleanup/plan.rs",
        "README.md",
        1,
    );
    for mutant in [duplicate, foreign] {
        assert!(
            std::panic::catch_unwind(|| validate_shape(&root, &ledger, &mutant, CONTRACT)).is_err()
        );
    }
    for required in [
        "C8-P7-AUTHORITY-01,workspaces/worth-store/crates/worth-store-physical-backend/src/recovery_media/cleanup.rs",
        "C8-P7-AUTHORITY-01,workspaces/worth-store/crates/worth-store/src/physical_runtime/recovery_coordination/cleanup/close.rs",
        "C8-P7-COMPILE-01,workspaces/worth-store/crates/worth-store-recovery-runtime/tests/ui/cleanup_close_cannot_be_forged.stderr",
        "C8-P7-FAILURE-01,workspaces/worth-store/crates/worth-store/src/physical_runtime/recovery_coordination/owner/certification_faults.rs",
        "C8-P7-API-01,workspaces/worth-store/crates/worth-store-recovery-physics/src/source_precedence/checkpoint_base.rs",
        "C8-P7-EFFECT-01,workspaces/worth-store/crates/worth-store-recovery-runtime/tests/phase_seven_faults/revalidation/denial_matrix.rs",
        "C8-P7-SCHEDULER-01,workspaces/worth-store/tools/store-test-runner/src/raw_media_owner_gate.rs",
    ] {
        let mandatory_omission = closure
            .lines()
            .filter(|line| !line.contains(required))
            .collect::<Vec<_>>()
            .join("\n");
        assert!(std::panic::catch_unwind(|| {
            validate_shape(&root, &ledger, &mandatory_omission, CONTRACT)
        })
        .is_err());
    }
}

#[test]
fn normative_requirement_mutants_fail_independently_of_source_rebinding() {
    let root = repository_root();
    let specification =
        read(&root.join(
            "_docs/worth-store/physical-reconstruction-c8-fresh-process-recovery-and-reopen.md",
        ));
    let added_requirement = specification.replace(
        "<!-- c8-phase7-requirements:end -->",
        "| C8-P7-OMITTED-99 | A newly normative requirement must have a ledger row. |\n<!-- c8-phase7-requirements:end -->",
    );
    assert!(std::panic::catch_unwind(|| {
        validate_requirement_inventory(
            &added_requirement,
            &GUARANTEES,
            CONTRACT.requirement_inventory_sha256,
        )
    })
    .is_err());
    let rewritten_requirement = specification.replacen(
        "Every discovered or C.8-created artifact has one exact cleanup disposition derived from the sealed reopened basis.",
        "Every artifact has a cleanup disposition.",
        1,
    );
    assert!(std::panic::catch_unwind(|| {
        validate_requirement_inventory(
            &rewritten_requirement,
            &GUARANTEES,
            CONTRACT.requirement_inventory_sha256,
        )
    })
    .is_err());
}

#[test]
fn inherited_phase_six_closure_is_proved_and_clean() {
    let root = repository_root();
    let phase_six =
        read(&root.join("_docs/worth-store/physical-reconstruction-c8-phase-6-closure-ledger.md"));
    let rows = phase_six
        .lines()
        .filter(|line| line.starts_with("| C8-P6-") && cells(line).len() == 9)
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), 14);
    assert!(rows.iter().all(|row| cells(row)[6] == "PROVED"));
    let audit = phase_six
        .lines()
        .filter(|line| line.starts_with("| /root/") && line.contains("gpt-5.6-sol high"))
        .last()
        .expect("Phase 6 independent audit row");
    assert_eq!(cells(audit)[3], "CLEAN");
}
