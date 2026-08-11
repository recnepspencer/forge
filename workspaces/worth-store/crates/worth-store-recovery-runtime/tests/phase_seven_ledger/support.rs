use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

#[derive(Clone, Copy)]
pub(super) struct LedgerContract<'a> {
    pub(super) guarantees: &'a [&'a str],
    pub(super) findings: &'a [(&'a str, &'a str)],
    pub(super) finding_history_sha256: &'a str,
}

pub(super) struct ValidatedLedger {
    pub(super) ledger_rows: Vec<String>,
    pub(super) closures: BTreeMap<String, BTreeSet<String>>,
}

pub(super) fn validate_shape(
    root: &Path,
    ledger: &str,
    closure: &str,
    contract: LedgerContract<'_>,
) -> ValidatedLedger {
    let rows = ledger_rows(ledger);
    assert_eq!(rows.len(), contract.guarantees.len());
    assert_eq!(
        rows.iter()
            .map(|row| cells(row)[0].clone())
            .collect::<BTreeSet<_>>(),
        contract
            .guarantees
            .iter()
            .map(|value| (*value).to_owned())
            .collect()
    );
    for row in &rows {
        let values = cells(row);
        assert_eq!(values.len(), 9);
        assert_eq!(values[1], "7");
        assert!(values[2..5].iter().all(|value| !value.is_empty()));
        assert_eq!(values[5].len(), 64);
        assert!(values[5].bytes().all(|byte| byte.is_ascii_hexdigit()));
        assert!(matches!(values[6].as_str(), "IMPLEMENTED" | "PROVED"));
    }
    validate_findings(ledger, contract);
    validate_audit(ledger, &rows);
    let closures = parse_closures(root, closure, contract.guarantees);
    ValidatedLedger {
        ledger_rows: rows,
        closures,
    }
}

fn validate_findings(ledger: &str, contract: LedgerContract<'_>) {
    let rows = rows_between(
        ledger,
        "## Phase 7 finding history",
        "## Independent audit history",
        "| C8-P7-F",
    );
    assert_eq!(rows.len(), contract.findings.len());
    assert_eq!(
        format!("{:x}", Sha256::digest(rows.join("\n").as_bytes())),
        contract.finding_history_sha256,
        "Phase 7 finding history content drifted"
    );
    let actual = rows
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
        actual,
        contract
            .findings
            .iter()
            .map(|(id, guarantees)| ((*id).to_owned(), (*guarantees).to_owned()))
            .collect()
    );
    for row in ledger_rows(ledger) {
        let values = cells(&row);
        let expected = contract
            .findings
            .iter()
            .filter(|(_, guarantees)| guarantees.split_whitespace().any(|id| id == values[0]))
            .map(|(finding, _)| *finding)
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(values[7], expected, "finding relation for {}", values[0]);
    }
}

fn validate_audit(ledger: &str, guarantee_rows: &[String]) {
    let audit = rows_between(
        ledger,
        "## Independent audit history",
        "__end__",
        "| /root/",
    );
    assert!(!audit.is_empty(), "retain complete Phase 7 audit history");
    assert!(audit.iter().all(|row| cells(row)[1] == "gpt-5.6-sol high"));
    let last = cells(audit.last().unwrap());
    if guarantee_rows.iter().all(|row| cells(row)[6] == "PROVED") {
        assert_eq!(last[3], "CLEAN");
    } else {
        assert!(matches!(last[3].as_str(), "PENDING" | "DEFECTS"));
    }
}

fn parse_closures(
    root: &Path,
    document: &str,
    guarantees: &[&str],
) -> BTreeMap<String, BTreeSet<String>> {
    let mut lines = document.lines();
    assert_eq!(lines.next(), Some("guarantee,path"));
    let mut closures = BTreeMap::<String, BTreeSet<String>>::new();
    for line in lines.filter(|line| !line.trim().is_empty()) {
        let (guarantee, path) = line.split_once(',').expect("two-column closure row");
        assert!(guarantees.contains(&guarantee));
        assert!(root.join(path).is_file(), "missing causal source {path}");
        assert!(closures
            .entry(guarantee.into())
            .or_default()
            .insert(path.into()));
    }
    assert_eq!(
        closures.keys().map(String::as_str).collect::<BTreeSet<_>>(),
        guarantees.iter().copied().collect()
    );
    closures
}

pub(super) fn source_identity(root: &Path, paths: &BTreeSet<String>) -> String {
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
            if line.starts_with("| C8-P7-") {
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

pub(super) fn ledger_rows(ledger: &str) -> Vec<String> {
    rows_between(
        ledger,
        "<!-- c8-phase7-ledger:start -->",
        "<!-- c8-phase7-ledger:end -->",
        "| C8-P7-",
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
        .filter(|line| line.starts_with(prefix))
        .map(str::to_owned)
        .collect()
}

pub(super) fn cells(row: &str) -> Vec<String> {
    row.trim()
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_owned())
        .collect()
}

pub(super) fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

pub(super) fn ledger_path() -> &'static str {
    "_docs/worth-store/physical-reconstruction-c8-phase-7-closure-ledger.md"
}

pub(super) fn source_closure_path() -> &'static str {
    "_docs/worth-store/physical-reconstruction-c8-phase-7-source-closure.csv"
}

pub(super) fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .canonicalize()
        .unwrap()
}
