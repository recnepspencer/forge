mod import_reconciliation;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use super::documents::{
    read_repository_document, split_csv, workspace_relative, CUTOVER_INVENTORY,
};
use super::facade_inventory::disposition_contract::expected_current_disposition;
use crate::workspace_root;
use import_reconciliation::{assert_direct_consumer_contract, imported_physics_surfaces};

const HEADER: &str = "path,responsibility,destination_owner,disposition,last_consumer,deletion_phase,absence_gate,status";
const PHYSICS_ROOT: &str = "crates/worth-store-recovery-physics";
const SEMANTIC_CUTOVER_MODULES: &[&str] = &[
    "source_precedence",
    "candidate_evaluation",
    "checkpoint_cutover",
    "partial_publication",
    "redo_replay",
    "page_redo",
    "recovery_budget",
    "wal_recovery_basis",
    "entry",
    "memory_allocation",
    "staged_wal_application",
    "staged_wal_replay_source",
    "publication",
    "recovery_completion",
    "offline_verifier",
    "recovery_evidence",
    "backup_restore",
    "point_in_time_recovery",
    "rollback_recovery",
    "replica_bootstrap_source",
    "blob_replay",
    "btree_replay",
    "layout_readmission",
    "layout_projection",
    "corruption_readmission",
    "integrity_damage_map",
    "integrity_handoff",
    "integrity_input",
    "integrity_vetted_records",
    "recovery_blocking_integrity",
    "recovery_integrity_handoff_receipt",
    "security_metadata_admission",
    "security_scope_propagation",
];
const EXPLICIT_WORKSPACE_PATHS: &[&str] = &[
    "crates/worth-store/src/physical_runtime/durability/closeout/handoff.rs",
    "crates/worth-store/src/physical_runtime/durability/closeout/operation_fates/fact.rs",
    "crates/worth-store-offline-verifier/src/bin/physical_store_offline_observer.rs",
    "crates/worth-store-offline-verifier/src/bin/physical_store_offline_observer/bounded_residency_verification.rs",
    "crates/worth-store-offline-verifier/src/bin/physical_store_offline_observer/bounded_residency_verification/configuration.rs",
    "crates/worth-store-offline-verifier/src/bin/physical_store_offline_observer/bounded_residency_verification/expectation.rs",
    "crates/worth-store-offline-verifier/src/bin/physical_store_offline_observer/current_manifest.rs",
    "crates/worth-store-offline-verifier/src/bin/physical_store_offline_observer/hostile_physical_truth.rs",
];
const AUTHORITATIVE_DOCUMENTS: &[&str] = &[
    "_docs/worth-store/physical-reconstruction-c8-fresh-process-recovery-and-reopen.md",
    "_docs/worth-store/physical-foundation-reconstruction-roadmap.md",
    "_docs/worth-store/storage-foundation-s4.md",
];

#[test]
fn scoped_cutover_inventory_matches_current_source_and_consumer_closure() {
    let document = read_repository_document(CUTOVER_INVENTORY).expect("read C.8 cutover inventory");
    let rows = parse_inventory(&document).expect("parse C.8 cutover inventory");
    let actual = discover_scope().expect("discover C.8 cutover scope");
    let expected = rows
        .iter()
        .map(|row| row.path.clone())
        .collect::<BTreeSet<_>>();
    let omitted = actual.difference(&expected).collect::<Vec<_>>();
    let stale = expected.difference(&actual).collect::<Vec<_>>();
    assert!(
        omitted.is_empty() && stale.is_empty(),
        "C.8 cutover inventory omitted {omitted:?} or retained unscoped {stale:?}"
    );
}

#[test]
fn cutover_rows_have_exact_dispositions_and_non_generic_owners() {
    let document = read_repository_document(CUTOVER_INVENTORY).expect("read C.8 cutover inventory");
    let rows = parse_inventory(&document).expect("parse C.8 cutover inventory");
    let mut paths = BTreeMap::new();
    for row in rows {
        assert!(
            paths.insert(row.path.clone(), ()).is_none(),
            "duplicate C.8 cutover row for {}",
            row.path
        );
        assert!(matches!(
            row.disposition.as_str(),
            "preserve" | "narrow" | "replace" | "delete"
        ));
        assert_eq!(row.status, "inventory-open");
        assert!(!matches!(
            row.responsibility.as_str(),
            "recovery" | "physics" | "support" | "evidence" | "utility"
        ));
        if row.disposition == "delete" {
            assert_eq!(row.destination_owner, "none");
        } else {
            assert!(!matches!(
                row.destination_owner.as_str(),
                "none" | "recovery" | "physics" | "support" | "evidence" | "utility"
            ));
        }
        if matches!(row.disposition.as_str(), "replace" | "delete") {
            assert_ne!(row.deletion_phase, "preserve");
            assert_ne!(row.absence_gate, "none");
        }
        assert_ne!(row.last_consumer, "unknown");
        assert_direct_consumer_contract(&row);
        assert_semantic_disposition(&row);
    }
}

fn assert_semantic_disposition(row: &CutoverRow) {
    let Some(relative) = row
        .path
        .strip_prefix("crates/worth-store-recovery-physics/src/")
    else {
        return;
    };
    if relative == "lib.rs" || relative == "recovery_physics_compile_fail_proofs.md" {
        return;
    }
    let module = relative
        .split('/')
        .next()
        .unwrap_or(relative)
        .strip_suffix(".rs")
        .unwrap_or_else(|| relative.split('/').next().unwrap_or(relative));
    if module == "wal_topology" {
        assert_eq!(row.disposition, "replace");
        assert_eq!(row.destination_owner, "worth-store-wal/recovery-read");
        return;
    }
    if !SEMANTIC_CUTOVER_MODULES.contains(&module) {
        return;
    }
    let surface = relative
        .rsplit('/')
        .next()
        .unwrap_or(relative)
        .strip_suffix(".rs")
        .unwrap_or(relative);
    let (disposition, owner, _) = expected_current_disposition(module, surface);
    assert_eq!(
        row.disposition, disposition,
        "wrong disposition for {}",
        row.path
    );
    assert_eq!(row.destination_owner, owner, "wrong owner for {}", row.path);
}

#[test]
fn controlled_inventory_defects_are_rejected() {
    let document = read_repository_document(CUTOVER_INVENTORY).expect("read C.8 cutover inventory");
    let rows = parse_inventory(&document).expect("parse C.8 cutover inventory");
    let missing = rows
        .iter()
        .skip(1)
        .map(|row| row.path.clone())
        .collect::<BTreeSet<_>>();
    assert_ne!(
        missing,
        discover_scope().expect("discover controlled scope")
    );
    let mut duplicate = document.clone();
    duplicate.push_str(document.lines().nth(1).expect("inventory row"));
    let parsed = parse_inventory(&duplicate).expect("duplicate row remains syntactically valid");
    let unique = parsed.iter().map(|row| &row.path).collect::<BTreeSet<_>>();
    assert_ne!(unique.len(), parsed.len());
}

fn discover_scope() -> Result<BTreeSet<String>, String> {
    let workspace = workspace_root();
    let mut scope = BTreeSet::new();
    let physics = workspace.join(PHYSICS_ROOT);
    for path in collect_files(&physics)? {
        scope.insert(workspace_relative(&path)?);
    }

    for path in collect_files(&workspace.join("crates"))?
        .into_iter()
        .chain(collect_files(&workspace.join("tools"))?)
    {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("");
        let is_candidate = path.extension().and_then(|value| value.to_str()) == Some("rs")
            || file_name == "Cargo.toml";
        if !is_candidate {
            continue;
        }
        let referenced = if path.extension().and_then(|value| value.to_str()) == Some("rs") {
            !imported_physics_surfaces(&workspace_relative(&path)?)?.is_empty()
        } else {
            std::fs::read_to_string(&path)
                .map_err(|error| format!("cannot read {}: {error}", path.display()))?
                .lines()
                .any(|line| {
                    !line.trim_start().starts_with('#')
                        && line.contains("worth-store-recovery-physics")
                })
        };
        if referenced {
            scope.insert(workspace_relative(&path)?);
        }
    }

    scope.extend(
        EXPLICIT_WORKSPACE_PATHS
            .iter()
            .map(|path| (*path).to_owned()),
    );
    scope.extend(
        AUTHORITATIVE_DOCUMENTS
            .iter()
            .map(|path| (*path).to_owned()),
    );
    Ok(scope)
}

fn collect_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(path) = pending.pop() {
        for entry in std::fs::read_dir(&path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))?
        {
            let entry = entry.map_err(|error| format!("cannot inspect entry: {error}"))?;
            let path = entry.path();
            if path.file_name().and_then(|name| name.to_str()) == Some("target") {
                continue;
            }
            if path.is_dir() {
                pending.push(path);
            } else if path.is_file() {
                files.push(path);
            }
        }
    }
    Ok(files)
}

fn parse_inventory(document: &str) -> Result<Vec<CutoverRow>, String> {
    let mut lines = document.lines();
    if lines.next() != Some(HEADER) {
        return Err("C.8 cutover inventory has an invalid schema".into());
    }
    lines
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| {
            let columns = split_csv(line, 8)
                .map_err(|error| format!("C.8 cutover row {}: {error}", index + 2))?;
            Ok(CutoverRow {
                path: columns[0].to_owned(),
                responsibility: columns[1].to_owned(),
                destination_owner: columns[2].to_owned(),
                disposition: columns[3].to_owned(),
                last_consumer: columns[4].to_owned(),
                deletion_phase: columns[5].to_owned(),
                absence_gate: columns[6].to_owned(),
                status: columns[7].to_owned(),
            })
        })
        .collect()
}

struct CutoverRow {
    path: String,
    responsibility: String,
    destination_owner: String,
    disposition: String,
    last_consumer: String,
    deletion_phase: String,
    absence_gate: String,
    status: String,
}
