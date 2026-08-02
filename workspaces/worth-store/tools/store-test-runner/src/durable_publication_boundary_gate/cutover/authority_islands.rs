use std::collections::BTreeSet;
use std::path::Path;

use crate::workspace_root;

const FORBIDDEN_PATHS: &[&str] = &[
    "crates/worth-store-physical-backend/src/durability_ordering/execution/file_runtime.rs",
    "crates/worth-store-recovery-physics/src/wal_durability/ack_precondition.rs",
    "crates/worth-store-recovery-physics/src/wal_durability/ack_receipt.rs",
];

const REQUIRED_RECOVERY_BASIS_PATHS: &[&str] = &[
    "crates/worth-store-recovery-physics/src/wal_recovery_basis/mod.rs",
    "crates/worth-store-recovery-physics/src/wal_recovery_basis/append_receipt.rs",
    "crates/worth-store-recovery-physics/src/wal_recovery_basis/crash_basis.rs",
    "crates/worth-store-recovery-physics/src/wal_recovery_basis/durability_observation.rs",
];

const FORBIDDEN_SOURCE_FRAGMENTS: &[&str] = &[
    "StoreDurability",
    "AcknowledgmentPrecondition",
    "DurableAckBasis",
    "DurableAckReceipt",
    "IllegalAcknowledgment",
];

#[test]
fn displaced_executor_and_false_acknowledgment_islands_are_absent() {
    inspect_cutover(&workspace_root()).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn authority_island_gate_rejects_path_vocabulary_and_destination_mutants() {
    for forbidden in FORBIDDEN_PATHS {
        inspect_path_inventory([*forbidden], REQUIRED_RECOVERY_BASIS_PATHS.iter().copied())
            .expect_err("a displaced authority path must fail the cutover gate");
    }

    for forbidden in FORBIDDEN_SOURCE_FRAGMENTS {
        inspect_source("crates/mutant/src/lib.rs", forbidden)
            .expect_err("displaced authority vocabulary must fail the cutover gate");
    }

    inspect_path_inventory(
        std::iter::empty::<&str>(),
        ["crates/worth-store-recovery-physics/src/wal_recovery_basis/mod.rs"],
    )
    .expect_err("an incomplete recovery-basis destination must fail the cutover gate");
}

fn inspect_cutover(root: &Path) -> Result<(), String> {
    inspect_path_inventory(
        FORBIDDEN_PATHS
            .iter()
            .copied()
            .filter(|path| root.join(path).exists()),
        REQUIRED_RECOVERY_BASIS_PATHS
            .iter()
            .copied()
            .filter(|path| root.join(path).exists()),
    )?;
    inspect_sources(root, &root.join("crates"))
}

fn inspect_path_inventory<'path>(
    forbidden_paths: impl IntoIterator<Item = &'path str>,
    required_paths: impl IntoIterator<Item = &'path str>,
) -> Result<(), String> {
    for path in forbidden_paths {
        return Err(format!("displaced authority path remains: {path}"));
    }
    let required = required_paths.into_iter().collect::<BTreeSet<_>>();
    let expected = REQUIRED_RECOVERY_BASIS_PATHS
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if required != expected {
        let missing = expected.difference(&required).copied().collect::<Vec<_>>();
        return Err(format!(
            "recovery WAL basis destination is incomplete; missing {missing:?}"
        ));
    }
    Ok(())
}

fn inspect_sources(root: &Path, source_root: &Path) -> Result<(), String> {
    let mut pending = vec![source_root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(&directory)
            .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?
        {
            let path = entry
                .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?
                .path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
                let source = std::fs::read_to_string(&path)
                    .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
                inspect_source(&relative_path(root, &path), &source)?;
            }
        }
    }
    Ok(())
}

fn inspect_source(path: &str, source: &str) -> Result<(), String> {
    for forbidden in FORBIDDEN_SOURCE_FRAGMENTS {
        if source.contains(forbidden) {
            return Err(format!(
                "{path} retains displaced authority vocabulary `{forbidden}`"
            ));
        }
    }
    Ok(())
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}
