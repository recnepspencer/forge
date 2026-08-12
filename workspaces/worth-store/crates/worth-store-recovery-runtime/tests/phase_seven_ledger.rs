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

const FINDINGS: [(&str, &str); 23] = [
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
];

const CONTRACT: LedgerContract<'static> = LedgerContract {
    guarantees: &GUARANTEES,
    findings: &FINDINGS,
    finding_history_sha256: "cded9d4da29bcd642702066f18cdd7a69987f7163b8ab134318646a4def1ab5a",
    audit_history_sha256: "6aabec5c8287ca6d48676d8722e2756c84a13f69a460b31c2c3f84d46191e985",
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
    validate_requirement_inventory(&specification, &GUARANTEES);
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
        ledger.replacen("| DEFECTS | C8-P7-F09", "| CLEAN | C8-P7-F09", 1),
        promoted,
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
    let specification =
        read(&root.join(
            "_docs/worth-store/physical-reconstruction-c8-fresh-process-recovery-and-reopen.md",
        ));
    let added_requirement = specification.replace(
        "<!-- c8-phase7-requirements:end -->",
        "| C8-P7-OMITTED-99 | A newly normative requirement must have a ledger row. |\n<!-- c8-phase7-requirements:end -->",
    );
    assert!(std::panic::catch_unwind(|| {
        validate_requirement_inventory(&added_requirement, &GUARANTEES)
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
