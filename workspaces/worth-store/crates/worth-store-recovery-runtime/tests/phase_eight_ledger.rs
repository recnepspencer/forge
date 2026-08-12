use std::collections::BTreeSet;
use std::path::PathBuf;

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

#[test]
fn phase_eight_ledger_matches_normative_inventory_and_sources_exist() {
    let root = repository_root();
    let spec =
        read(&root.join(
            "_docs/worth-store/physical-reconstruction-c8-fresh-process-recovery-and-reopen.md",
        ));
    let ledger =
        read(&root.join("_docs/worth-store/physical-reconstruction-c8-phase-8-closure-ledger.md"));
    let closure =
        read(&root.join("_docs/worth-store/physical-reconstruction-c8-phase-8-source-closure.csv"));
    let expected = GUARANTEES.iter().map(|value| (*value).to_owned()).collect();
    assert_eq!(marked_ids(&spec, "c8-phase8-requirements"), expected);
    let expected = GUARANTEES.iter().map(|value| (*value).to_owned()).collect();
    assert_eq!(marked_ids(&ledger, "c8-phase8-ledger"), expected);
    for line in closure.lines().skip(1).filter(|line| !line.is_empty()) {
        let columns = line.split(',').collect::<Vec<_>>();
        assert_eq!(columns.len(), 3, "invalid Phase 8 closure row {line}");
        assert!(GUARANTEES.contains(&columns[0]));
        assert!(
            root.join(columns[1]).is_file(),
            "missing source {}",
            columns[1]
        );
        assert!(!columns[2].is_empty());
    }
}

fn marked_ids(document: &str, marker: &str) -> BTreeSet<String> {
    let start = document.find(&format!("<!-- {marker}:start -->")).unwrap();
    let end = document.find(&format!("<!-- {marker}:end -->")).unwrap();
    document[start..end]
        .lines()
        .filter_map(|line| line.strip_prefix("| "))
        .filter_map(|line| line.split_once(" |"))
        .map(|(identity, _)| identity.to_owned())
        .filter(|identity| identity.starts_with("C8-P8-"))
        .collect()
}

fn read(path: &std::path::Path) -> String {
    std::fs::read_to_string(path).unwrap()
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .canonicalize()
        .unwrap()
}
