use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
};

use forge_query::facade::consumer_kit::{
    query_boundary_source_inventory, ForgeQueryBoundaryAuditSourceInventory,
};

use super::super::inventory_error::{
    WorthGraphReadAccessInventoryError, WorthGraphReadAccessInventoryErrorKind,
};
use super::super::row::{WorthGraphReadAccessClassification, WorthGraphReadAccessInventoryRow};
use super::required_root_coverage::WorthGraphReadBypassRequiredRootCoverage;

pub(crate) fn graph_read_bypass_source_inventory_from_rows(
    rows: &[WorthGraphReadAccessInventoryRow],
) -> Result<ForgeQueryBoundaryAuditSourceInventory, WorthGraphReadAccessInventoryError> {
    let workspace_root = workspace_root();
    let mut builder = query_boundary_source_inventory("worth-graph-read-access-inventory");
    let mut required_roots = Vec::new();
    let mut seen_source_paths = BTreeSet::new();
    for row in rows {
        if !is_live_audit_root(row.classification()) {
            continue;
        }
        if seen_source_paths.insert(row.source_path().to_string()) {
            required_roots.push(workspace_root.join(row.source_path()));
        }
    }
    for root in non_overlapping_roots(required_roots)? {
        builder = builder.required_root(root);
    }
    builder.include_rs_files().seal().map_err(|query_error| {
        error_with_message(
            WorthGraphReadAccessInventoryErrorKind::GraphReadBypassBoundaryAuditFailed,
            query_error.message(),
        )
    })
}

const fn is_live_audit_root(classification: WorthGraphReadAccessClassification) -> bool {
    !matches!(
        classification,
        WorthGraphReadAccessClassification::DeletionTarget
            | WorthGraphReadAccessClassification::CappedResidue
    )
}

fn non_overlapping_roots(
    roots: Vec<PathBuf>,
) -> Result<Vec<PathBuf>, WorthGraphReadAccessInventoryError> {
    let mut canonical_roots = Vec::new();
    for root in roots {
        let canonical = canonical_root(&root)?;
        if !canonical_roots.contains(&canonical) {
            canonical_roots.push(canonical);
        }
    }
    canonical_roots.sort();

    let mut pruned = Vec::new();
    for root in canonical_roots {
        if pruned
            .iter()
            .any(|existing: &PathBuf| root.starts_with(existing))
        {
            continue;
        }
        pruned.push(root);
    }
    Ok(pruned)
}

fn canonical_root(root: &Path) -> Result<PathBuf, WorthGraphReadAccessInventoryError> {
    root.canonicalize().map_err(|error| {
        error_with_message(
            WorthGraphReadAccessInventoryErrorKind::GraphReadBypassBoundaryAuditFailed,
            format!(
                "failed to canonicalize graph-read bypass source root `{}`: {error}",
                root.display()
            ),
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
