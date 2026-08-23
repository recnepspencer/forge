use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

#[path = "exact_source_map/required_sources.rs"]
mod required_sources;

use required_sources::EXACT_SOURCE_MAP;

pub(super) const SOURCE_MAP_SHA256: &str =
    "61ffe92adb998bd5b7b8455c03cc80d657c173120996e922a9a64661ca6a0068";

#[test]
fn exact_source_map_rejects_omission_and_substitution_mutants() {
    let root = repository_root();
    let closure =
        read(&root.join("_docs/worth-store/physical-reconstruction-c8-phase-8-source-closure.csv"));
    validate_source_map(&closure);

    let omitted = remove_exact_row(&closure, EXACT_SOURCE_MAP[0]);
    assert!(std::panic::catch_unwind(|| validate_source_map(&omitted)).is_err());
    let substituted = closure.replacen(EXACT_SOURCE_MAP[1].2, "substituted-runtime-owner", 1);
    assert!(std::panic::catch_unwind(|| validate_source_map(&substituted)).is_err());
    let extra = format!(
        "{closure}\nC8-P8-LEDGER-01,{},uncontracted-extra-row,{}",
        EXACT_SOURCE_MAP[0].1,
        "00".repeat(32)
    );
    assert!(std::panic::catch_unwind(|| validate_source_map(&extra)).is_err());
}

pub(super) fn validate_source_map(closure: &str) {
    let rows = closure
        .lines()
        .skip(1)
        .filter(|line| !line.is_empty())
        .map(|line| line.split(',').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let available = rows
        .iter()
        .filter(|row| row.len() == 4)
        .map(|row| (row[0], row[1], row[2]))
        .collect::<BTreeSet<_>>();
    for expected in EXACT_SOURCE_MAP {
        assert!(
            available.contains(&expected),
            "exact Phase 8 source map omitted or substituted {expected:?}"
        );
    }
    let canonical = available
        .iter()
        .map(|(guarantee, source, relation)| format!("{guarantee},{source},{relation}"))
        .collect::<Vec<_>>()
        .join("\n");
    let digest = Sha256::digest(canonical.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    assert_eq!(
        digest, SOURCE_MAP_SHA256,
        "exact Phase 8 source map gained, lost, or reassigned a causal row"
    );
}

fn remove_exact_row(closure: &str, expected: (&str, &str, &str)) -> String {
    let prefix = format!("{},{},{}", expected.0, expected.1, expected.2);
    closure
        .lines()
        .filter(|line| !line.starts_with(&prefix))
        .collect::<Vec<_>>()
        .join("\n")
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
