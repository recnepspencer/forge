use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use super::workspace_source::{read, workspace_relative};
use crate::workspace_root;

#[cfg(test)]
mod classifier_tests;
mod consumer_family;
mod direct_pool_reference;
mod ledger_document;
mod legacy_module_closure;
mod replacement_owner;

use consumer_family::{discover_families, is_legacy_s2_subject_family};
use ledger_document::{parse_removal_ledger, RemovalDisposition, RemovalRow, RemovalStatus};

const REMOVAL_LEDGER: &str = "_docs/worth-store/physical-reconstruction-c6-removal-ledger.csv";

const EXCLUDED_POLICY_SOURCES: &[&str] = &[
    "tools/store-test-runner/src/c5_1_sealing_gate.rs",
    "tools/store-test-runner/src/c5_1_sealing_gate/",
    "tools/store-test-runner/src/physical_residency_boundary_gate/",
    "tools/store-test-runner/src/mutation_campaign/catalog/physical_reconstruction_c6.rs",
];

#[test]
fn every_temporary_and_legacy_consumer_has_a_removal_disposition() {
    let discovered = discover_consumers().expect("discover temporary and legacy consumers");
    let ledger = removal_ledger().expect("parse C.6 removal ledger");
    compare_inventory(&discovered, &ledger).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn every_removal_row_has_a_future_owner_and_mechanical_absence_gate() {
    let ledger = removal_ledger().expect("parse C.6 removal ledger");
    for row in ledger.values() {
        assert!(
            matches!(
                row.deletion_phase.as_str(),
                "phase-3" | "phase-5" | "phase-6" | "phase-7" | "phase-8"
            ),
            "{} has invalid deletion phase {}",
            row.path,
            row.deletion_phase
        );
        assert!(
            !row.replacement_owner.is_empty(),
            "{} has no replacement owner",
            row.path
        );
        assert_eq!(
            row.absence_gate, "source-and-metadata-absence",
            "{} lacks a mechanical absence gate",
            row.path
        );
        assert!(
            !row.disposition_basis.is_empty(),
            "{} has no disposition basis",
            row.path
        );
        assert_disposition_path_fate(row).unwrap_or_else(|denial| panic!("{denial}"));
        match row.disposition {
            RemovalDisposition::Preserve => {
                assert!(
                    matches!(row.status, RemovalStatus::InventoryOpen),
                    "{} cannot be both preserved and recorded as deleted",
                    row.path
                );
            }
            RemovalDisposition::Narrow | RemovalDisposition::Delete => {}
        }
        match &row.status {
            RemovalStatus::InventoryOpen => {}
            RemovalStatus::Deleted(phase) => assert_eq!(
                phase, &row.deletion_phase,
                "{} deletion status disagrees with assigned phase",
                row.path
            ),
        }
    }
    replacement_owner::assert_all_rows_have_present_replacement(&ledger)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

fn assert_disposition_path_fate(row: &RemovalRow) -> Result<(), String> {
    let exists = workspace_root().join(&row.path).exists();
    match (row.disposition, exists) {
        (RemovalDisposition::Preserve, true)
        | (RemovalDisposition::Narrow, true)
        | (RemovalDisposition::Delete, false) => Ok(()),
        (RemovalDisposition::Preserve, false) => Err(format!(
            "{} is marked preserve but the path is absent",
            row.path
        )),
        (RemovalDisposition::Narrow, false) => Err(format!(
            "{} is marked narrow but the retained path is absent",
            row.path
        )),
        (RemovalDisposition::Delete, true) => Err(format!(
            "{} is marked delete but the path is present",
            row.path
        )),
    }
}

#[test]
fn every_completed_phase_six_row_has_a_present_source_owner() {
    assert_completed_rows_have_present_replacement("phase-6");
}

#[test]
fn every_completed_phase_five_row_has_a_present_replacement_owner() {
    assert_completed_rows_have_present_replacement("phase-5");
}

#[test]
fn every_completed_phase_seven_row_has_a_present_replacement_owner() {
    assert_completed_rows_have_present_replacement("phase-7");
}

#[test]
fn every_completed_phase_eight_row_has_a_present_replacement_owner() {
    assert_completed_rows_have_present_replacement("phase-8");
}

fn assert_completed_rows_have_present_replacement(phase: &str) {
    let ledger = removal_ledger().expect("parse C.6 removal ledger");
    replacement_owner::assert_completed_rows_have_present_replacement(phase, &ledger)
        .unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn inventory_gate_rejects_stale_open_and_rediscovered_deleted_rows() {
    let open = removal_row("phase-3", RemovalStatus::InventoryOpen);
    let denial = compare_inventory(
        &BTreeMap::new(),
        &BTreeMap::from([("open.rs".to_owned(), open)]),
    )
    .expect_err("a missing open row must be denied");
    assert!(denial.contains("stale open rows"));

    let deleted = removal_row("phase-3", RemovalStatus::Deleted("phase-3".to_owned()));
    let denial = compare_inventory(
        &BTreeMap::from([(
            "deleted.rs".to_owned(),
            BTreeSet::from(["c6-identifier".to_owned()]),
        )]),
        &BTreeMap::from([("deleted.rs".to_owned(), deleted)]),
    )
    .expect_err("a rediscovered deleted row must be denied");
    assert!(denial.contains("rediscovered deleted rows"));
}

#[test]
fn disposition_path_fate_enforces_preserve_narrow_and_delete() {
    let mut row = removal_row("phase-8", RemovalStatus::Deleted("phase-8".to_owned()));
    row.path = "Cargo.toml".to_owned();
    assert!(assert_disposition_path_fate(&row).is_err());

    row.disposition = RemovalDisposition::Narrow;
    assert!(assert_disposition_path_fate(&row).is_ok());
    row.disposition = RemovalDisposition::Preserve;
    assert!(assert_disposition_path_fate(&row).is_ok());

    row.path = "controlled-absent-removal-ledger-path.rs".to_owned();
    assert!(assert_disposition_path_fate(&row).is_err());
    row.disposition = RemovalDisposition::Narrow;
    assert!(assert_disposition_path_fate(&row).is_err());
    row.disposition = RemovalDisposition::Delete;
    assert!(assert_disposition_path_fate(&row).is_ok());
}

#[test]
fn inventory_discovery_unions_leaf_and_module_closure_families() {
    let mut discovered = BTreeMap::from([(
        "crates/example/src/legacy_leaf.rs".to_owned(),
        BTreeSet::from(["legacy-frame-table".to_owned()]),
    )]);

    merge_discovered_families(
        &mut discovered,
        "crates/example/src/legacy_leaf.rs".to_owned(),
        BTreeSet::from(["legacy-s2-module-closure".to_owned()]),
    );

    assert_eq!(
        discovered["crates/example/src/legacy_leaf.rs"],
        BTreeSet::from([
            "legacy-frame-table".to_owned(),
            "legacy-s2-module-closure".to_owned(),
        ])
    );
}

#[test]
fn inventory_gate_reports_every_family_mismatch_in_one_denial() {
    let discovered = BTreeMap::from([
        (
            "crates/example/src/first.rs".to_owned(),
            BTreeSet::from(["legacy-frame-table".to_owned()]),
        ),
        (
            "crates/example/src/second.rs".to_owned(),
            BTreeSet::from(["legacy-record-view".to_owned()]),
        ),
    ]);
    let mut first = removal_row("phase-8", RemovalStatus::InventoryOpen);
    first.path = "crates/example/src/first.rs".to_owned();
    let mut second = removal_row("phase-8", RemovalStatus::InventoryOpen);
    second.path = "crates/example/src/second.rs".to_owned();
    let ledger = BTreeMap::from([
        ("crates/example/src/first.rs".to_owned(), first),
        ("crates/example/src/second.rs".to_owned(), second),
    ]);

    let denial = compare_inventory(&discovered, &ledger)
        .expect_err("every mismatched row must be reported together");

    assert!(denial.contains("crates/example/src/first.rs"));
    assert!(denial.contains("crates/example/src/second.rs"));
}

fn discover_consumers() -> Result<BTreeMap<String, BTreeSet<String>>, String> {
    let workspace = workspace_root();
    let sources = inventory_sources(&workspace)?;
    let mut discovered = BTreeMap::new();
    for path in &sources {
        let relative = workspace_relative(path);
        if EXCLUDED_POLICY_SOURCES
            .iter()
            .any(|excluded| relative == *excluded || relative.starts_with(excluded))
        {
            continue;
        }
        let source = read(path)?;
        let families = discover_families(&relative, &source);
        if !families.is_empty() {
            discovered.insert(relative, families);
        }
    }
    for (path, families) in legacy_module_closure::discover(&workspace, &sources)? {
        let relative = workspace_relative(&path);
        if EXCLUDED_POLICY_SOURCES
            .iter()
            .any(|excluded| relative == *excluded || relative.starts_with(excluded))
        {
            continue;
        }
        merge_discovered_families(&mut discovered, relative, families);
    }
    Ok(discovered)
}

fn merge_discovered_families(
    discovered: &mut BTreeMap<String, BTreeSet<String>>,
    path: String,
    families: BTreeSet<String>,
) {
    discovered.entry(path).or_default().extend(families);
}

fn removal_ledger() -> Result<BTreeMap<String, RemovalRow>, String> {
    let document = read(&repository_root().join(REMOVAL_LEDGER))?;
    parse_removal_ledger(&document)
}

fn compare_inventory(
    discovered: &BTreeMap<String, BTreeSet<String>>,
    ledger: &BTreeMap<String, RemovalRow>,
) -> Result<(), String> {
    let discovered_paths = discovered.keys().collect::<BTreeSet<_>>();
    let open_paths = ledger
        .iter()
        .filter_map(|(path, row)| {
            matches!(row.status, RemovalStatus::InventoryOpen).then_some(path)
        })
        .collect::<BTreeSet<_>>();
    let deleted_paths = ledger
        .iter()
        .filter_map(|(path, row)| matches!(row.status, RemovalStatus::Deleted(_)).then_some(path))
        .collect::<BTreeSet<_>>();
    let unclassified = discovered_paths
        .iter()
        .filter(|path| !ledger.contains_key(path.as_str()))
        .map(|path| path.as_str())
        .collect::<Vec<_>>();
    let stale_open = open_paths
        .difference(&discovered_paths)
        .map(|path| path.as_str())
        .collect::<Vec<_>>();
    let rediscovered_deleted = deleted_paths
        .intersection(&discovered_paths)
        .map(|path| path.as_str())
        .collect::<Vec<_>>();
    if !unclassified.is_empty() || !stale_open.is_empty() || !rediscovered_deleted.is_empty() {
        let legacy_subject = rediscovered_deleted.iter().any(|path| {
            discovered.get(*path).is_some_and(|families| {
                families
                    .iter()
                    .any(|family| is_legacy_s2_subject_family(family))
            })
        });
        let predicate = if legacy_subject {
            "MUTANT_PREDICATE:legacy-s2-subject-reintroduced; "
        } else {
            ""
        };
        return Err(format!(
            "{predicate}physical residency removal ledger mismatch; unclassified consumers: {unclassified:?}; stale open rows: {stale_open:?}; rediscovered deleted rows: {rediscovered_deleted:?}"
        ));
    }
    let family_mismatches = discovered
        .iter()
        .filter_map(|(path, families)| {
            let row = ledger.get(path).expect("path sets are equal");
            (matches!(row.status, RemovalStatus::InventoryOpen) && &row.families != families).then(
                || {
                    format!(
                        "{path}: discovered {families:?}, recorded {:?}",
                        row.families
                    )
                },
            )
        })
        .collect::<Vec<_>>();
    if !family_mismatches.is_empty() {
        return Err(format!(
            "physical residency removal ledger family mismatches: {}",
            family_mismatches.join("; ")
        ));
    }
    Ok(())
}

fn inventory_sources(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut pending = vec![root.to_path_buf()];
    let mut sources = Vec::new();
    while let Some(directory) = pending.pop() {
        let entries = std::fs::read_dir(&directory)
            .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?;
        for entry in entries {
            let path = entry
                .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?
                .path();
            if path.is_dir() {
                if !matches!(
                    path.file_name().and_then(|value| value.to_str()),
                    Some("target" | ".git")
                ) {
                    pending.push(path);
                }
            } else if is_inventory_source(&path) {
                sources.push(path);
            }
        }
    }
    sources.sort();
    Ok(sources)
}

fn is_inventory_source(path: &Path) -> bool {
    let extension = path.extension().and_then(|value| value.to_str());
    extension == Some("rs")
        || path.file_name().and_then(|value| value.to_str()) == Some("Cargo.toml")
        || extension == Some("md")
            && path
                .components()
                .any(|component| matches!(component.as_os_str().to_str(), Some("src" | "tests")))
}

fn repository_root() -> PathBuf {
    workspace_root()
        .parent()
        .and_then(Path::parent)
        .expect("worth-store workspace must live under workspaces")
        .to_path_buf()
}

fn removal_row(deletion_phase: &str, status: RemovalStatus) -> RemovalRow {
    RemovalRow {
        path: "controlled-mutant.rs".to_owned(),
        families: BTreeSet::from(["c6-identifier".to_owned()]),
        deletion_phase: deletion_phase.to_owned(),
        replacement_owner: "controlled replacement".to_owned(),
        absence_gate: "source-and-metadata-absence".to_owned(),
        status,
        disposition: RemovalDisposition::Delete,
        disposition_basis: "controlled test disposition".to_owned(),
    }
}
