#[path = "phase_seven_ledger/support.rs"]
mod support;

use std::collections::BTreeMap;

use support::{
    cells, ledger_rows, read, repository_root, source_closure_path, source_identity,
    validate_shape, LedgerContract,
};

const GUARANTEES: [&str; 15] = [
    "C8-P7-PLAN-01",
    "C8-P7-ELIGIBILITY-01",
    "C8-P7-FRESHNESS-01",
    "C8-P7-SCHEDULER-01",
    "C8-P7-SAFETY-01",
    "C8-P7-LIMITS-01",
    "C8-P7-EFFECT-01",
    "C8-P7-FAILURE-01",
    "C8-P7-CRASH-01",
    "C8-P7-COUNTERS-01",
    "C8-P7-QUIESCENCE-01",
    "C8-P7-PROGRESSION-01",
    "C8-P7-COMPILE-01",
    "C8-P7-API-01",
    "C8-P7-LEDGER-01",
];

const FINDINGS: [(&str, &str); 8] = [
    ("C8-P7-F01", "C8-P7-PLAN-01 C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-SCHEDULER-01 C8-P7-SAFETY-01 C8-P7-LIMITS-01 C8-P7-EFFECT-01 C8-P7-FAILURE-01 C8-P7-CRASH-01 C8-P7-COUNTERS-01 C8-P7-QUIESCENCE-01 C8-P7-PROGRESSION-01 C8-P7-LEDGER-01"),
    ("C8-P7-F02", "C8-P7-PLAN-01 C8-P7-SAFETY-01 C8-P7-LEDGER-01"),
    ("C8-P7-F03", "C8-P7-ELIGIBILITY-01 C8-P7-FRESHNESS-01 C8-P7-EFFECT-01 C8-P7-COMPILE-01 C8-P7-LEDGER-01"),
    ("C8-P7-F04", "C8-P7-FRESHNESS-01 C8-P7-SCHEDULER-01 C8-P7-LIMITS-01 C8-P7-EFFECT-01 C8-P7-FAILURE-01 C8-P7-COUNTERS-01 C8-P7-QUIESCENCE-01 C8-P7-LEDGER-01"),
    ("C8-P7-F05", "C8-P7-SCHEDULER-01 C8-P7-QUIESCENCE-01 C8-P7-PROGRESSION-01 C8-P7-API-01 C8-P7-LEDGER-01"),
    ("C8-P7-F06", "C8-P7-PLAN-01 C8-P7-SAFETY-01 C8-P7-FAILURE-01 C8-P7-CRASH-01 C8-P7-PROGRESSION-01 C8-P7-LEDGER-01"),
    ("C8-P7-F07", "C8-P7-PLAN-01 C8-P7-SAFETY-01 C8-P7-CRASH-01 C8-P7-COUNTERS-01 C8-P7-LEDGER-01"),
    ("C8-P7-F08", "C8-P7-PLAN-01 C8-P7-COMPILE-01 C8-P7-API-01 C8-P7-LEDGER-01"),
];

const CONTRACT: LedgerContract<'static> = LedgerContract {
    guarantees: &GUARANTEES,
    findings: &FINDINGS,
    finding_history_sha256: "4f7f3d2f165a3f55103c2291e94e3f5f6556627e1efbb9afcb97fa6ba7710680",
};

#[test]
fn phase_seven_ledger_is_exact_source_bound_and_audit_honest() {
    let root = repository_root();
    let ledger = read(&root.join(support::ledger_path()));
    let closure = read(&root.join(source_closure_path()));
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
    let promoted = ledger.replace("| IMPLEMENTED |", "| PROVED |");
    for mutant in [
        ledger.replacen(&first, "", 1),
        ledger.replacen(&first, &format!("{first}\n{first}"), 1),
        ledger.replacen("| C8-P7-F01 | Critical", "| C8-P7-F99 | Critical", 1),
        ledger.replacen("C8-P7-F06 C8-P7-F07 C8-P7-F08", "C8-P7-F06 C8-P7-F08", 1),
        ledger.replacen("A fully materialized no-op recovery", "A recovery", 1),
        ledger.replacen("| PENDING | none |", "| CLEAN | none |", 1),
        promoted,
    ] {
        assert!(
            std::panic::catch_unwind(|| validate_shape(&root, &mutant, &closure, CONTRACT))
                .is_err()
        );
    }
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
