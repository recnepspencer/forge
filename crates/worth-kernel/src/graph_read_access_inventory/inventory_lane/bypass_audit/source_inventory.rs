use std::{collections::BTreeSet, path::PathBuf};

use forge_query::facade::consumer_kit::{
    query_boundary_source_inventory, ForgeQueryBoundaryAuditSourceInventory,
};

use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::super::row::{WorthGraphReadAccessClassification, WorthGraphReadAccessInventoryRow};
use super::required_root_coverage::WorthGraphReadBypassRequiredRootCoverage;

pub(in crate::graph_read_access_inventory::inventory_lane) fn graph_read_bypass_source_inventory_from_rows(
    rows: &[WorthGraphReadAccessInventoryRow],
) -> Result<ForgeQueryBoundaryAuditSourceInventory, WorthGraphReadAccessInventoryError> {
    let workspace_root = workspace_root();
    let mut builder = query_boundary_source_inventory("worth-graph-read-access-inventory");
    let mut required_roots = BTreeSet::new();
    for row in rows {
        if row.classification() == WorthGraphReadAccessClassification::DeletionTarget {
            continue;
        }
        if required_roots.insert(row.source_path().to_string()) {
            builder = builder.required_root(workspace_root.join(row.source_path()));
        }
    }
    builder.include_rs_files().seal().map_err(|query_error| {
        error_with_message(
            WorthGraphReadAccessInventoryErrorKind::GraphReadBypassBoundaryAuditFailed,
            query_error.message(),
        )
    })
}

pub(in crate::graph_read_access_inventory::inventory_lane) fn required_root_coverage_from_inventory(
    inventory: &ForgeQueryBoundaryAuditSourceInventory,
) -> Vec<WorthGraphReadBypassRequiredRootCoverage> {
    inventory
        .required_roots()
        .iter()
        .map(|required_root| {
            let audited_source_labels = inventory
                .files()
                .iter()
                .filter(|file| file.source_path().starts_with(required_root.as_str()))
                .map(|file| file.label().to_string())
                .collect::<Vec<_>>();
            WorthGraphReadBypassRequiredRootCoverage::new(
                required_root.clone(),
                audited_source_labels.len(),
                audited_source_labels,
            )
        })
        .collect()
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn error_with_message(
    kind: WorthGraphReadAccessInventoryErrorKind,
    message: impl Into<String>,
) -> WorthGraphReadAccessInventoryError {
    WorthGraphReadAccessInventoryError::with_message(kind, message)
}
