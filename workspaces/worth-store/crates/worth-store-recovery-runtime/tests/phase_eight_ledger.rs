use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

const GUARANTEES: [&str; 10] = [
    "C8-P8-RUNTIME-REPORT-01",
    "C8-P8-OBSERVER-01",
    "C8-P8-PROTOCOL-01",
    "C8-P8-API-01",
    "C8-P8-CUTOVER-01",
    "C8-P8-PHYSICS-01",
    "C8-P8-DEPENDENCY-01",
    "C8-P8-DOCUMENTATION-01",
    "C8-P8-RETIREMENT-01",
    "C8-P8-LEDGER-01",
];

const FINDINGS: [&str; 4] = [
    "C8-P8-F01|High|C8-P8-RUNTIME-REPORT-01 C8-P8-OBSERVER-01 C8-P8-PROTOCOL-01 C8-P8-DOCUMENTATION-01|No shipped runtime or independent observer report protocol existed|Added distinct version-one envelopes bounded observer walk CLI output and operator guide|typed protocol and exact limit twins pass",
    "C8-P8-F02|High|C8-P8-CUTOVER-01 C8-P8-PHYSICS-01 C8-P8-RETIREMENT-01|Attempting whole-module deletion before migrating consumers broke layout and certification crates|Restored the module and made consumer migration precede absence claims|warnings-denied destination crates compile while retirement remains ACTIVE",
    "C8-P8-F03|High|C8-P8-RUNTIME-REPORT-01 C8-P8-OBSERVER-01 C8-P8-PROTOCOL-01 C8-P8-LEDGER-01|Codec round trips did not prove that the production recovery command and the separately owned offline observer emitted evidence across a real process boundary|Added a writer process the shipped recovery executable and a distinct offline-verifier observer process then decoded both reports in the parent and cross-fed each payload to the other family decoder|warnings-denied Phase 8 process proof observes a real recovered Store and both wrong-family crossings are rejected",
    "C8-P8-F04|High|C8-P8-OBSERVER-01 C8-P8-PROTOCOL-01 C8-P8-DOCUMENTATION-01 C8-P8-LEDGER-01|Adding traversal counters silently redefined the promised version-one observer payload digest domain and compatibility window as version two|Kept progressive traversal counters in observation evidence while restoring the exact version-one report fields digest domain and one-version compatibility window|literal v1 wire and artifact-set oracles four-axis limit twins and the shipped CLI process pass warnings denied",
];

#[test]
fn phase_eight_ledger_is_a_requirement_source_and_history_bijection() {
    let root = repository_root();
    let spec =
        read(&root.join(
            "_docs/worth-store/physical-reconstruction-c8-fresh-process-recovery-and-reopen.md",
        ));
    let ledger =
        read(&root.join("_docs/worth-store/physical-reconstruction-c8-phase-8-closure-ledger.md"));
    let closure =
        read(&root.join("_docs/worth-store/physical-reconstruction-c8-phase-8-source-closure.csv"));

    let expected = guarantee_set();
    let specification = marked_requirements(&spec, "c8-phase8-requirements");
    let ledger_rows = marked_ledger(&ledger, "c8-phase8-ledger");
    assert_eq!(
        specification.keys().cloned().collect::<BTreeSet<_>>(),
        expected
    );
    assert_eq!(
        ledger_rows.keys().cloned().collect::<BTreeSet<_>>(),
        expected
    );
    for (identity, requirement) in specification {
        let row = ledger_rows.get(&identity).expect("ledger guarantee");
        assert_eq!(
            row.requirement, requirement,
            "rewritten {identity} requirement"
        );
        assert!(matches!(
            row.status.as_str(),
            "ACTIVE" | "IMPLEMENTED" | "PROVED"
        ));
        assert!(!row.evidence_owner.is_empty());
        assert!(!row.causal_proof.is_empty());
    }

    validate_source_closure(&root, &closure, &expected);
    assert_eq!(finding_rows(&ledger), FINDINGS.map(str::to_owned).into());
    validate_audit_posture(
        &ledger,
        ledger_rows.values().all(|row| row.status == "PROVED"),
    );
}

struct LedgerRow {
    requirement: String,
    evidence_owner: String,
    causal_proof: String,
    status: String,
}

fn marked_requirements(document: &str, marker: &str) -> BTreeMap<String, String> {
    marked_table(document, marker)
        .filter_map(|columns| {
            (columns.len() == 2 && columns[0].starts_with("C8-P8-"))
                .then(|| (columns[0].to_owned(), columns[1].to_owned()))
        })
        .collect()
}

fn marked_ledger(document: &str, marker: &str) -> BTreeMap<String, LedgerRow> {
    marked_table(document, marker)
        .filter_map(|columns| {
            (columns.len() == 7 && columns[0].starts_with("C8-P8-")).then(|| {
                (
                    columns[0].to_owned(),
                    LedgerRow {
                        requirement: columns[2].to_owned(),
                        evidence_owner: columns[3].to_owned(),
                        causal_proof: columns[4].to_owned(),
                        status: columns[5].to_owned(),
                    },
                )
            })
        })
        .collect()
}

fn marked_table<'a>(document: &'a str, marker: &str) -> impl Iterator<Item = Vec<&'a str>> {
    let start = document.find(&format!("<!-- {marker}:start -->")).unwrap();
    let end = document.find(&format!("<!-- {marker}:end -->")).unwrap();
    document[start..end].lines().filter_map(|line| {
        line.strip_prefix("| ").map(|row| {
            row.trim_end_matches(" |")
                .split(" | ")
                .map(str::trim)
                .collect()
        })
    })
}

fn validate_source_closure(root: &Path, closure: &str, expected: &BTreeSet<String>) {
    let mut covered = BTreeSet::new();
    let mut unique = BTreeSet::new();
    assert_eq!(closure.lines().next(), Some("guarantee,source,relation"));
    for line in closure.lines().skip(1).filter(|line| !line.is_empty()) {
        let columns = line.split(',').collect::<Vec<_>>();
        assert_eq!(columns.len(), 3, "invalid Phase 8 closure row {line}");
        assert!(expected.contains(columns[0]), "foreign guarantee {line}");
        assert!(!columns[2].is_empty(), "missing relation {line}");
        assert!(unique.insert(line), "duplicate closure row {line}");
        let source = Path::new(columns[1]);
        assert!(!source.is_absolute() && !source.components().any(|part| part.as_os_str() == ".."));
        assert!(root.join(source).is_file(), "missing source {}", columns[1]);
        covered.insert(columns[0].to_owned());
    }
    assert_eq!(
        &covered, expected,
        "every guarantee needs a causal source family"
    );
}

fn finding_rows(document: &str) -> BTreeSet<String> {
    section_rows(
        document,
        "## Phase 8 finding history",
        "## Independent audit history",
    )
    .filter(|columns| {
        columns
            .first()
            .is_some_and(|value| value.starts_with("C8-P8-F"))
    })
    .map(|columns| columns.join("|"))
    .collect()
}

fn validate_audit_posture(document: &str, all_proved: bool) {
    let rows = section_rows(document, "## Independent audit history", "\u{0}")
        .filter(|columns| columns.first().is_some_and(|value| *value != "Reviewer"))
        .collect::<Vec<_>>();
    assert!(!rows.is_empty(), "audit history cannot disappear");
    if all_proved {
        assert!(
            rows.iter().any(|row| {
                row.len() == 6
                    && row[0] != "pending"
                    && row[1].contains("gpt-5.6")
                    && row[3] == "CLEAN"
                    && row[5].contains("frozen")
            }),
            "PROVED requires one attributable independent CLEAN audit"
        );
    }
}

fn section_rows<'a>(
    document: &'a str,
    start_heading: &str,
    end_heading: &str,
) -> impl Iterator<Item = Vec<&'a str>> {
    let start = document.find(start_heading).unwrap();
    let end = document[start + start_heading.len()..]
        .find(end_heading)
        .map_or(document.len(), |offset| {
            start + start_heading.len() + offset
        });
    document[start..end].lines().filter_map(|line| {
        line.strip_prefix("| ").map(|row| {
            row.trim_end_matches(" |")
                .split(" | ")
                .map(str::trim)
                .collect()
        })
    })
}

fn guarantee_set() -> BTreeSet<String> {
    GUARANTEES.iter().map(|value| (*value).to_owned()).collect()
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .canonicalize()
        .unwrap()
}
