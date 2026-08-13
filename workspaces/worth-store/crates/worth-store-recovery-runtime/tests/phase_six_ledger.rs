use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

const GUARANTEES: [&str; 14] = [
    "C8-P6-PLAN-01",
    "C8-P6-SCHEDULER-01",
    "C8-P6-PUBLICATION-01",
    "C8-P6-DURABILITY-01",
    "C8-P6-EFFECT-01",
    "C8-P6-FAILURE-01",
    "C8-P6-REOPEN-01",
    "C8-P6-HANDOFF-01",
    "C8-P6-COUNTERS-01",
    "C8-P6-QUIESCENCE-01",
    "C8-P6-PROGRESSION-01",
    "C8-P6-COMPILE-01",
    "C8-P6-API-01",
    "C8-P6-LEDGER-01",
];

const FINDINGS: [(&str, &str); 21] = [
    ("C8-P6-F01", "C8-P6-PLAN-01 C8-P6-SCHEDULER-01 C8-P6-PUBLICATION-01 C8-P6-DURABILITY-01 C8-P6-EFFECT-01 C8-P6-PROGRESSION-01 C8-P6-LEDGER-01"),
    ("C8-P6-F02", "C8-P6-SCHEDULER-01 C8-P6-EFFECT-01 C8-P6-REOPEN-01 C8-P6-PROGRESSION-01 C8-P6-LEDGER-01"),
    ("C8-P6-F03", "C8-P6-FAILURE-01 C8-P6-REOPEN-01 C8-P6-COUNTERS-01 C8-P6-LEDGER-01"),
    ("C8-P6-F04", "C8-P6-HANDOFF-01 C8-P6-COUNTERS-01 C8-P6-QUIESCENCE-01 C8-P6-PROGRESSION-01 C8-P6-COMPILE-01 C8-P6-LEDGER-01"),
    ("C8-P6-F05", "C8-P6-API-01 C8-P6-LEDGER-01"),
    ("C8-P6-F06", "C8-P6-FAILURE-01 C8-P6-HANDOFF-01 C8-P6-COUNTERS-01 C8-P6-COMPILE-01 C8-P6-API-01 C8-P6-LEDGER-01"),
    ("C8-P6-F07", "C8-P6-PROGRESSION-01 C8-P6-API-01 C8-P6-LEDGER-01"),
    ("C8-P6-F08", "C8-P6-PUBLICATION-01 C8-P6-FAILURE-01 C8-P6-COUNTERS-01 C8-P6-LEDGER-01"),
    ("C8-P6-F09", "C8-P6-API-01 C8-P6-LEDGER-01"),
    ("C8-P6-F10", "C8-P6-API-01 C8-P6-LEDGER-01"),
    ("C8-P6-F11", "C8-P6-PLAN-01 C8-P6-SCHEDULER-01 C8-P6-PUBLICATION-01 C8-P6-DURABILITY-01 C8-P6-EFFECT-01 C8-P6-FAILURE-01 C8-P6-REOPEN-01 C8-P6-HANDOFF-01 C8-P6-COUNTERS-01 C8-P6-QUIESCENCE-01 C8-P6-PROGRESSION-01 C8-P6-API-01 C8-P6-LEDGER-01"),
    ("C8-P6-F12", "C8-P6-PLAN-01 C8-P6-LEDGER-01"),
    ("C8-P6-F13", "C8-P6-PUBLICATION-01 C8-P6-FAILURE-01 C8-P6-COUNTERS-01 C8-P6-LEDGER-01"),
    ("C8-P6-F14", "C8-P6-SCHEDULER-01 C8-P6-FAILURE-01 C8-P6-REOPEN-01 C8-P6-COUNTERS-01 C8-P6-LEDGER-01"),
    ("C8-P6-F15", "C8-P6-LEDGER-01"),
    ("C8-P6-F16", "C8-P6-COUNTERS-01 C8-P6-PROGRESSION-01 C8-P6-LEDGER-01"),
    ("C8-P6-F17", "C8-P6-SCHEDULER-01 C8-P6-PUBLICATION-01 C8-P6-EFFECT-01 C8-P6-FAILURE-01 C8-P6-REOPEN-01 C8-P6-COUNTERS-01 C8-P6-PROGRESSION-01 C8-P6-LEDGER-01"),
    ("C8-P6-F18", "C8-P6-LEDGER-01"),
    ("C8-P6-F19", "C8-P6-SCHEDULER-01 C8-P6-API-01 C8-P6-LEDGER-01"),
    ("C8-P6-F20", "C8-P6-PLAN-01 C8-P6-PUBLICATION-01 C8-P6-EFFECT-01 C8-P6-COUNTERS-01 C8-P6-PROGRESSION-01 C8-P6-API-01 C8-P6-LEDGER-01"),
    ("C8-P6-F21", "C8-P6-PLAN-01 C8-P6-SCHEDULER-01 C8-P6-FAILURE-01 C8-P6-REOPEN-01 C8-P6-HANDOFF-01 C8-P6-COUNTERS-01 C8-P6-QUIESCENCE-01 C8-P6-PROGRESSION-01 C8-P6-COMPILE-01 C8-P6-API-01 C8-P6-LEDGER-01"),
];

const FINDING_HISTORY_SHA256: &str =
    "96cedf5f948cd69bbc0d63c2eb1b9331659438acb23c3fec27d1ee6a08f7181a";

#[test]
fn phase_six_ledger_is_exact_and_source_bound() {
    let root = repository_root();
    let ledger = read(&root.join(ledger_path()));
    let closure = read(&root.join(source_closure_path()));
    let closures = parse_closures(&root, &closure);
    let rows = ledger_rows(&ledger);
    assert_eq!(rows.len(), GUARANTEES.len());
    validate_findings(&ledger);
    let ids = rows
        .iter()
        .map(|row| {
            let cells = cells(row);
            assert_eq!(cells[1], "6");
            assert!(!cells[2].is_empty() && !cells[3].is_empty() && !cells[4].is_empty());
            assert!(matches!(cells[6].as_str(), "IMPLEMENTED" | "PROVED"));
            (cells[0].clone(), cells[5].clone())
        })
        .collect::<BTreeMap<_, _>>();
    let mismatches = GUARANTEES
        .iter()
        .filter_map(|guarantee| {
            let expected = source_identity(&root, &closures[*guarantee]);
            (ids[*guarantee] != expected)
                .then(|| format!("{guarantee}={} expected {expected}", ids[*guarantee]))
        })
        .collect::<Vec<_>>();
    assert!(
        mismatches.is_empty(),
        "stale Phase 6 source identities: {mismatches:#?}"
    );
    let audit = rows_between(&ledger, "## Independent audit history", "__end__", "| ");
    assert!(
        !audit.is_empty(),
        "retain the complete Phase 6 audit history"
    );
    assert!(audit.iter().all(|row| cells(row)[1] == "gpt-5.6-sol high"));
    let audit_cells = cells(audit.last().unwrap());
    if rows.iter().all(|row| cells(row)[6] == "PROVED") {
        assert_eq!(audit_cells[3], "CLEAN");
    } else {
        assert!(matches!(audit_cells[3].as_str(), "PENDING" | "DEFECTS"));
    }
}

#[test]
fn ledger_omission_duplicate_status_and_source_mutants_fail() {
    let root = repository_root();
    let ledger = read(&root.join(ledger_path()));
    let closure = read(&root.join(source_closure_path()));
    let first = ledger_rows(&ledger)[0].clone();
    for mutant in [
        ledger.replacen(&first, "", 1),
        ledger.replacen(&first, &format!("{first}\n{first}"), 1),
        ledger.replacen("| PROVED |", "| ACTIVE |", 1),
        ledger.replacen("| C8-P6-F01 | Critical", "| C8-P6-F99 | Critical", 1),
        ledger.replacen("C8-P6-F13 C8-P6-F14 C8-P6-F16", "C8-P6-F13 C8-P6-F16", 1),
        ledger.replacen(
            "PublicationIndeterminate and escaped denial terminals",
            "Publication terminals",
            1,
        ),
    ] {
        assert!(std::panic::catch_unwind(|| validate_shape(&root, &mutant, &closure)).is_err());
    }
    let foreign = closure.replacen(
        "workspaces/worth-store/crates/worth-store-recovery-runtime/src/progression/planned/basis.rs",
        "README.md",
        1,
    );
    assert!(std::panic::catch_unwind(|| validate_shape(&root, &ledger, &foreign)).is_err());
}

fn validate_shape(root: &Path, ledger: &str, closure: &str) {
    let closures = parse_closures(root, closure);
    let rows = ledger_rows(ledger);
    assert_eq!(rows.len(), GUARANTEES.len());
    assert_eq!(
        rows.iter()
            .map(|row| cells(row)[0].clone())
            .collect::<BTreeSet<_>>(),
        GUARANTEES.iter().map(|value| (*value).to_owned()).collect()
    );
    assert!(rows
        .iter()
        .all(|row| matches!(cells(row)[6].as_str(), "IMPLEMENTED" | "PROVED")));
    assert_eq!(closures.len(), GUARANTEES.len());
    validate_findings(ledger);
}

fn validate_findings(ledger: &str) {
    let finding_rows = rows_between(
        ledger,
        "## Phase 6 finding history",
        "## Independent audit history",
        "| C8-P6-F",
    );
    assert_eq!(finding_rows.len(), FINDINGS.len());
    let finding_history = finding_rows.join("\n");
    assert_eq!(
        format!("{:x}", Sha256::digest(finding_history.as_bytes())),
        FINDING_HISTORY_SHA256,
        "Phase 6 finding history content drifted"
    );
    let findings = finding_rows
        .iter()
        .map(|row| {
            let values = cells(row);
            assert_eq!(values.len(), 6);
            assert!(matches!(values[1].as_str(), "Critical" | "High" | "Medium"));
            assert!(values[3..].iter().all(|value| !value.is_empty()));
            (values[0].clone(), values[2].clone())
        })
        .collect::<BTreeMap<_, _>>();
    assert_eq!(
        findings,
        FINDINGS
            .into_iter()
            .map(|(finding, guarantees)| (finding.to_owned(), guarantees.to_owned()))
            .collect()
    );
    for row in ledger_rows(ledger) {
        let values = cells(&row);
        let expected = FINDINGS
            .iter()
            .filter(|(_, guarantees)| guarantees.split_whitespace().any(|id| id == values[0]))
            .map(|(finding, _)| *finding)
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(values[7], expected, "finding relation for {}", values[0]);
    }
}

fn parse_closures(root: &Path, document: &str) -> BTreeMap<String, BTreeSet<String>> {
    let mut lines = document.lines();
    assert_eq!(lines.next(), Some("guarantee,path"));
    let mut closures = BTreeMap::<String, BTreeSet<String>>::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let (guarantee, path) = line.split_once(',').expect("two-column closure row");
        assert!(GUARANTEES.contains(&guarantee));
        assert!(root.join(path).is_file(), "missing causal source {path}");
        assert!(closures
            .entry(guarantee.into())
            .or_default()
            .insert(path.into()));
    }
    assert_eq!(
        closures.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        GUARANTEES.into_iter().collect()
    );
    closures
}

fn source_identity(root: &Path, paths: &BTreeSet<String>) -> String {
    let mut digest = Sha256::new();
    for path in paths {
        digest.update((path.len() as u64).to_le_bytes());
        digest.update(path.as_bytes());
        let bytes = std::fs::read(root.join(path)).expect("read causal source");
        let bytes = if path == ledger_path() {
            normalize_ledger_identities(&String::from_utf8(bytes).expect("utf8 ledger"))
                .into_bytes()
        } else {
            bytes
        };
        digest.update((bytes.len() as u64).to_le_bytes());
        digest.update(bytes);
    }
    format!("{:x}", digest.finalize())
}

fn normalize_ledger_identities(ledger: &str) -> String {
    ledger
        .lines()
        .map(|line| {
            if line.starts_with("| C8-P6-") {
                let mut values = cells(line);
                values[5] = "<source-identity>".into();
                format!("| {} |", values.join(" | "))
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn ledger_rows(ledger: &str) -> Vec<String> {
    rows_between(
        ledger,
        "<!-- c8-phase6-ledger:start -->",
        "<!-- c8-phase6-ledger:end -->",
        "| C8-P6-",
    )
}

fn rows_between(document: &str, start: &str, end: &str, prefix: &str) -> Vec<String> {
    let after = document.split_once(start).expect("start marker").1;
    let body = if end == "__end__" {
        after
    } else {
        after.split_once(end).expect("end marker").0
    };
    body.lines()
        .filter(|line| line.starts_with(prefix) && !line.starts_with("| ---"))
        .filter(|line| cells(line).first().is_some_and(|cell| *cell != "Reviewer"))
        .map(str::to_owned)
        .collect()
}

fn cells(row: &str) -> Vec<String> {
    row.trim()
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_owned())
        .collect()
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn ledger_path() -> &'static str {
    "_docs/worth-store/physical-reconstruction-c8-phase-6-closure-ledger.md"
}
fn source_closure_path() -> &'static str {
    "_docs/worth-store/physical-reconstruction-c8-phase-6-source-closure.csv"
}
fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .canonicalize()
        .unwrap()
}
