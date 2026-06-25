use std::collections::BTreeSet;

use crate::graph_read_access_declarations::WorthGraphReadDeclarationDeletionLedgerRow;
use crate::graph_read_access_inventory::WorthGraphReadDeletionLedgerItem;

pub(crate) fn old_graph_read_source_path() -> &'static str {
    "crates/worth-kernel/src/query_adoption/graph_read_access"
}

pub(crate) fn deletion_item_fingerprints(
    deletion_items: &[WorthGraphReadDeletionLedgerItem],
) -> BTreeSet<(String, String, String, String, Option<String>)> {
    deletion_items
        .iter()
        .map(|item| {
            let identity = item.inventory_row_identity();
            (
                identity.source_path().to_string(),
                format!("{:?}", identity.owner()),
                identity.current_caller().to_string(),
                item.deletion_trigger().to_string(),
                item.blocker().map(str::to_string),
            )
        })
        .collect()
}

pub(crate) fn deletion_ledger_row_fingerprints(
    rows: &[WorthGraphReadDeclarationDeletionLedgerRow],
) -> BTreeSet<(String, String, String, String, Option<String>)> {
    rows.iter()
        .map(|row| {
            (
                row.source_path().to_string(),
                row.owner().to_string(),
                row.current_caller().to_string(),
                row.deletion_trigger().to_string(),
                row.blocker().map(str::to_string),
            )
        })
        .collect()
}

pub(crate) fn deletion_ledger_row_digests(
    rows: &[WorthGraphReadDeclarationDeletionLedgerRow],
) -> BTreeSet<(String, String)> {
    rows.iter()
        .map(|row| (row.source_path().to_string(), row.row_digest().to_string()))
        .collect()
}

pub(crate) fn deletion_item_source_paths(
    deletion_items: &[WorthGraphReadDeletionLedgerItem],
) -> BTreeSet<&str> {
    deletion_items
        .iter()
        .map(|item| item.inventory_row_identity().source_path())
        .collect()
}
