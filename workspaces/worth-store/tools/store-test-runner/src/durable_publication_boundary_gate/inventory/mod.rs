mod removal_ledger;
mod source_discovery;

use removal_ledger::{
    parse_removal_ledger, Disposition, RemovalRow, RemovalStatus, REMOVAL_LEDGER,
};
use source_discovery::{discover_tracked_consumers, TRACKED_FAMILIES};

use std::collections::{BTreeMap, BTreeSet};

use super::read_repository_document;
use crate::workspace_root;

#[test]
fn every_tracked_consumer_has_one_current_disposition() {
    let discovered = discover_tracked_consumers().expect("discover C.7 consumers");
    let ledger = current_ledger().expect("parse C.7 removal ledger");
    reconcile(&discovered, &ledger).unwrap_or_else(|denial| panic!("{denial}"));
}

#[test]
fn every_ledger_row_names_real_responsibility_owner_fate_and_absence_proof() {
    let ledger = current_ledger().expect("parse C.7 removal ledger");
    let family_ids = TRACKED_FAMILIES
        .iter()
        .map(|family| family.id)
        .collect::<BTreeSet<_>>();
    for row in ledger.values() {
        assert_semantic_classification(row, &family_ids);
        assert_path_fate(row);
    }
}

fn assert_semantic_classification(row: &RemovalRow, family_ids: &BTreeSet<&str>) {
    assert!(
        !row.responsibility.is_empty()
            && !row.responsibility.starts_with("tracked-")
            && row.responsibility != "durability-vocabulary-consumer",
        "{} has a generic responsibility",
        row.path
    );
    assert!(
        !row.destination_owner.is_empty() && row.destination_owner != "existing-domain-owner",
        "{} has a generic destination owner",
        row.path
    );
    assert_eq!(row.last_consumer, row.path);
    assert_eq!(
        row.absence_gate, "tracked-family-source-and-metadata-reconciliation",
        "{} lacks the C.7 mechanical absence proof",
        row.path
    );
    assert!(row
        .families
        .iter()
        .all(|family| family_ids.contains(family.as_str())));
    assert_eq!(row.families, row.match_counts.keys().cloned().collect());
}

fn assert_path_fate(row: &RemovalRow) {
    let path = workspace_root().join(&row.path);
    match row.status {
        RemovalStatus::InventoryOpen => assert!(path.is_file(), "{} is open but absent", row.path),
        RemovalStatus::Deleted(phase) => {
            assert!(!path.exists(), "{} is deleted but present", row.path);
            assert_eq!(phase, row.deletion_phase);
        }
    }
    if row.disposition == Disposition::Preserve {
        assert_eq!(
            row.deletion_phase,
            removal_ledger::DeletionPhase::Preserve,
            "{} is preserved but names a deletion phase",
            row.path
        );
    }
    if row.disposition == Disposition::Delete {
        assert!(matches!(row.status, RemovalStatus::Deleted(_)));
    }
}

fn current_ledger() -> Result<BTreeMap<String, RemovalRow>, String> {
    parse_removal_ledger(&read_repository_document(REMOVAL_LEDGER)?)
}

fn reconcile(
    discovered: &BTreeMap<String, BTreeMap<String, usize>>,
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
        .difference(&open_paths)
        .filter(|path| !deleted_paths.contains(*path))
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
    let family_mismatches = discovered
        .iter()
        .filter_map(|(path, match_counts)| {
            ledger.get(path).and_then(|row| {
                (&row.match_counts != match_counts).then(|| {
                    format!(
                        "{path}: discovered {match_counts:?} but ledger has {:?}",
                        row.match_counts
                    )
                })
            })
        })
        .collect::<Vec<_>>();
    if !unclassified.is_empty()
        || !stale_open.is_empty()
        || !rediscovered_deleted.is_empty()
        || !family_mismatches.is_empty()
    {
        return Err(format!(
            "C.7 removal ledger mismatch; unclassified={unclassified:?}; stale_open={stale_open:?}; rediscovered_deleted={rediscovered_deleted:?}; family_mismatches={family_mismatches:?}"
        ));
    }
    Ok(())
}

#[test]
fn reconciliation_rejects_omission_stale_row_and_family_drift() {
    let discovered = BTreeMap::from([(
        "crates/live.rs".to_owned(),
        BTreeMap::from([("page-lsn".to_owned(), 1)]),
    )]);
    let row = controlled_row(BTreeMap::from([("page-lsn".to_owned(), 1)]));
    assert!(reconcile(&discovered, &BTreeMap::new()).is_err());
    assert!(reconcile(
        &BTreeMap::new(),
        &BTreeMap::from([("crates/live.rs".to_owned(), row)])
    )
    .is_err());
    let drifted = controlled_row(BTreeMap::from([("wal-commit".to_owned(), 1)]));
    assert!(reconcile(
        &discovered,
        &BTreeMap::from([("crates/live.rs".to_owned(), drifted)])
    )
    .is_err());
    let count_drifted = controlled_row(BTreeMap::from([("page-lsn".to_owned(), 2)]));
    assert!(reconcile(
        &discovered,
        &BTreeMap::from([("crates/live.rs".to_owned(), count_drifted)])
    )
    .is_err());
}

fn controlled_row(match_counts: BTreeMap<String, usize>) -> RemovalRow {
    RemovalRow {
        path: "crates/live.rs".to_owned(),
        families: match_counts.keys().cloned().collect(),
        match_counts,
        responsibility: "controlled-page-ordering".to_owned(),
        destination_owner: "controlled-recovery-owner".to_owned(),
        disposition: Disposition::Preserve,
        last_consumer: "crates/live.rs".to_owned(),
        deletion_phase: removal_ledger::DeletionPhase::Preserve,
        absence_gate: "tracked-family-source-and-metadata-reconciliation".to_owned(),
        status: RemovalStatus::InventoryOpen,
    }
}
