use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

use super::workspace_source::{read, workspace_relative};
use crate::workspace_root;

mod consumer_family;
mod direct_pool_reference;
mod legacy_module_closure;
mod replacement_owner;

use consumer_family::discover_families;

const REMOVAL_LEDGER: &str = "_docs/worth-store/physical-reconstruction-c6-removal-ledger.csv";

const EXCLUDED_POLICY_SOURCES: &[&str] = &[
    "tools/store-test-runner/src/c5_1_sealing_gate.rs",
    "tools/store-test-runner/src/c5_1_sealing_gate/",
    "tools/store-test-runner/src/physical_residency_boundary_gate/",
];

#[test]
fn every_temporary_and_legacy_consumer_has_a_removal_disposition() {
    let discovered = discover_consumers().expect("discover temporary and legacy consumers");
    let ledger = removal_ledger().expect("parse C.6 removal ledger");
    compare_inventory(&discovered, &ledger).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn every_removal_row_has_a_future_owner_and_mechanical_absence_gate() {
    for row in removal_ledger().expect("parse C.6 removal ledger").values() {
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
        match &row.status {
            RemovalStatus::InventoryOpen => {}
            RemovalStatus::Deleted(phase) => assert_eq!(
                phase, &row.deletion_phase,
                "{} deletion status disagrees with assigned phase",
                row.path
            ),
        }
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
fn inventory_gate_rejects_unclassified_consumers() {
    let mut discovered = BTreeMap::new();
    discovered.insert(
        "crates/worth-store/src/physical_runtime/foreign.rs".to_owned(),
        BTreeSet::from(["c6-identifier".to_owned()]),
    );
    let denial = compare_inventory(&discovered, &BTreeMap::new())
        .expect_err("an unclassified consumer must be denied");
    assert!(denial.contains("unclassified consumers"));

    let families = discover_families(
        "crates/example/Cargo.toml",
        r#"legacy = { features = ["legacy-s2-models"] }"#,
    );
    assert!(families.contains("legacy-s2-feature"));

    let indirect_alias = discover_families(
        "crates/example/Cargo.toml",
        r#"
[features]
bridge = ["worth-store-buffer-pool/legacy-s2-models"]
certification = ["bridge"]
"#,
    );
    assert!(
        indirect_alias.contains("legacy-s2-feature"),
        "the manifest edge is legacy even when an aggregate alias gates independent modules"
    );

    let certification = discover_families(
        "crates/worth-store-certification/src/new_consumer.rs",
        "use worth_store_buffer_pool::PhysicalResidencyCounters;",
    );
    assert!(
        certification.contains("direct-pool-consumer"),
        "certification is a consumer of Store truth, not a canonical pool owner"
    );
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

fn discover_consumers() -> Result<BTreeMap<String, BTreeSet<String>>, String> {
    let workspace = workspace_root();
    let sources = inventory_sources(&workspace)?;
    let mut discovered = BTreeMap::new();
    for path in &sources {
        let relative = workspace_relative(&path);
        if EXCLUDED_POLICY_SOURCES
            .iter()
            .any(|excluded| relative == *excluded || relative.starts_with(excluded))
        {
            continue;
        }
        let source = read(&path)?;
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
        discovered.entry(relative).or_insert(families);
    }
    Ok(discovered)
}

fn removal_ledger() -> Result<BTreeMap<String, RemovalRow>, String> {
    let document = read(&repository_root().join(REMOVAL_LEDGER))?;
    let mut rows = BTreeMap::new();
    for (index, line) in document.lines().enumerate() {
        if index == 0 || line.trim().is_empty() {
            continue;
        }
        let columns = line.split(',').map(str::trim).collect::<Vec<_>>();
        if columns.len() != 6 {
            return Err(format!(
                "removal ledger row {} has {} columns, expected 6",
                index + 1,
                columns.len()
            ));
        }
        let path = columns[0].to_owned();
        let families = columns[1]
            .split(';')
            .map(str::to_owned)
            .collect::<BTreeSet<_>>();
        let row = RemovalRow {
            path: path.clone(),
            families,
            deletion_phase: columns[2].to_owned(),
            replacement_owner: columns[3].to_owned(),
            absence_gate: columns[4].to_owned(),
            status: RemovalStatus::parse(columns[5])?,
        };
        if rows.insert(path.clone(), row).is_some() {
            return Err(format!("duplicate removal ledger row for {path}"));
        }
    }
    Ok(rows)
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
        return Err(format!(
            "physical residency removal ledger mismatch; unclassified consumers: {unclassified:?}; stale open rows: {stale_open:?}; rediscovered deleted rows: {rediscovered_deleted:?}"
        ));
    }
    for (path, families) in discovered {
        let row = ledger.get(path).expect("path sets are equal");
        if !matches!(row.status, RemovalStatus::InventoryOpen) {
            continue;
        }
        if &row.families != families {
            return Err(format!(
                "physical residency removal ledger family mismatch for {path}; discovered {families:?}, recorded {:?}",
                row.families
            ));
        }
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
            } else if path.extension().and_then(|value| value.to_str()) == Some("rs")
                || path.file_name().and_then(|value| value.to_str()) == Some("Cargo.toml")
            {
                sources.push(path);
            }
        }
    }
    sources.sort();
    Ok(sources)
}

fn repository_root() -> PathBuf {
    workspace_root()
        .parent()
        .and_then(Path::parent)
        .expect("worth-store workspace must live under workspaces")
        .to_path_buf()
}

struct RemovalRow {
    path: String,
    families: BTreeSet<String>,
    deletion_phase: String,
    replacement_owner: String,
    absence_gate: String,
    status: RemovalStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RemovalStatus {
    InventoryOpen,
    Deleted(String),
}

impl RemovalStatus {
    fn parse(value: &str) -> Result<Self, String> {
        if value == "inventory-open" {
            return Ok(Self::InventoryOpen);
        }
        if let Some(phase) = value.strip_prefix("deleted-") {
            if matches!(
                phase,
                "phase-3" | "phase-5" | "phase-6" | "phase-7" | "phase-8"
            ) {
                return Ok(Self::Deleted(phase.to_owned()));
            }
        }
        Err(format!("invalid removal status {value}"))
    }
}

fn removal_row(deletion_phase: &str, status: RemovalStatus) -> RemovalRow {
    RemovalRow {
        path: "controlled-mutant.rs".to_owned(),
        families: BTreeSet::from(["c6-identifier".to_owned()]),
        deletion_phase: deletion_phase.to_owned(),
        replacement_owner: "controlled replacement".to_owned(),
        absence_gate: "source-and-metadata-absence".to_owned(),
        status,
    }
}
