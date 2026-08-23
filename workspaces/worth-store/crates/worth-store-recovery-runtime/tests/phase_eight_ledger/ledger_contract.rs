use std::collections::{BTreeMap, BTreeSet};

pub(super) const GUARANTEES: [&str; 10] = [
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

const LEDGER_CONTRACT: [(&str, &str, &str); 10] = [
    (
        "C8-P8-RUNTIME-REPORT-01",
        "recovery runtime observation owner",
        "round-trip terminal-outcome process comparison and explicit mutated-observer disagreement test",
    ),
    (
        "C8-P8-OBSERVER-01",
        "offline verifier observation owner",
        "exact-at and one-over directory-entry directory artifact and byte tests plus shipped CLI process and parent comparison",
    ),
    (
        "C8-P8-PROTOCOL-01",
        "runtime and observer protocol owners",
        "wrong-family future-version malformed and digest twins",
    ),
    (
        "C8-P8-API-01",
        "Phase 1 facade inventory owner",
        "production-derived exact facade equality",
    ),
    (
        "C8-P8-CUTOVER-01",
        "cutover inventory owner",
        "consumer import and absence gates plus the persisted checkpoint crash/reopen matrix and paused S10 quarantine closure",
    ),
    (
        "C8-P8-PHYSICS-01",
        "recovery physics facade owner",
        "dependency and source inventory review plus retired-family absence proof",
    ),
    (
        "C8-P8-DEPENDENCY-01",
        "Cargo graph owner",
        "warnings-denied feature and dependency gates plus formal binding-completeness proof",
    ),
    (
        "C8-P8-DOCUMENTATION-01",
        "documentation owner",
        "command examples and README contract review",
    ),
    (
        "C8-P8-RETIREMENT-01",
        "scoped deletion owner",
        "exact absence inventory plus killed-writer reopen evidence and S10 quarantine/source-map proof",
    ),
    (
        "C8-P8-LEDGER-01",
        "Phase 8 ledger owner",
        "requirement bijection and source existence test",
    ),
];

#[derive(Debug, Clone)]
pub(super) struct LedgerRow {
    pub(super) requirement: String,
    pub(super) evidence_owner: String,
    pub(super) causal_proof: String,
    pub(super) status: String,
    pub(super) deferred: String,
}

pub(super) fn validate_bijection(
    specification: &BTreeMap<String, String>,
    ledger_rows: &BTreeMap<String, LedgerRow>,
) {
    let expected = guarantee_set();
    assert_eq!(specification.len(), expected.len());
    assert_eq!(ledger_rows.len(), expected.len());
    assert_eq!(
        specification.keys().cloned().collect::<BTreeSet<_>>(),
        expected
    );
    assert_eq!(
        ledger_rows.keys().cloned().collect::<BTreeSet<_>>(),
        expected
    );
    for (identity, requirement) in specification {
        let row = ledger_rows.get(identity).expect("ledger guarantee");
        let (_, owner, proof) = LEDGER_CONTRACT
            .iter()
            .find(|(candidate, _, _)| candidate == identity)
            .expect("ledger contract guarantee");
        assert_eq!(
            row.requirement, *requirement,
            "rewritten {identity} requirement"
        );
        assert_eq!(row.evidence_owner, *owner, "rewritten {identity} owner");
        assert_eq!(row.causal_proof, *proof, "rewritten {identity} proof");
        assert_eq!(row.status, "CLOSED", "open {identity} ledger row");
        assert_eq!(row.deferred, "—", "deferred {identity} ledger row");
    }
}

pub(super) fn marked_requirements(document: &str, marker: &str) -> BTreeMap<String, String> {
    marked_table(document, marker)
        .filter_map(|columns| {
            (columns.len() == 2 && columns[0].starts_with("C8-P8-"))
                .then(|| (columns[0].to_owned(), columns[1].to_owned()))
        })
        .collect()
}

pub(super) fn marked_ledger(document: &str, marker: &str) -> BTreeMap<String, LedgerRow> {
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
                        deferred: columns[6].to_owned(),
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

pub(super) fn guarantee_set() -> BTreeSet<String> {
    GUARANTEES.iter().map(|value| (*value).to_owned()).collect()
}
