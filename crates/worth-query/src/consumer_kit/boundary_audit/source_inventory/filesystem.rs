use std::path::Path;

use crate::consumer_kit::boundary_audit::error::{
    WorthQueryBoundaryAuditError, WorthQueryBoundaryAuditErrorKind,
};

use super::inventory::WorthQueryBoundaryAuditSourceInventoryFile;

pub(super) fn collect_rs_files(
    crate_name: &str,
    root: &Path,
    files: &mut Vec<WorthQueryBoundaryAuditSourceInventoryFile>,
) -> Result<(), WorthQueryBoundaryAuditError> {
    let entries = std::fs::read_dir(root).map_err(|error| {
        WorthQueryBoundaryAuditError::new(
            WorthQueryBoundaryAuditErrorKind::SourceInventoryReadFailed,
            format!(
                "failed to read source inventory root `{}`: {error}",
                root.display()
            ),
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|error| {
            WorthQueryBoundaryAuditError::new(
                WorthQueryBoundaryAuditErrorKind::SourceInventoryReadFailed,
                format!(
                    "failed to read source inventory entry under `{}`: {error}",
                    root.display()
                ),
            )
        })?;
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(crate_name, &path, files)?;
            continue;
        }
        if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
            continue;
        }
        files.push(discovered_inventory_file(crate_name, &path)?);
    }
    Ok(())
}

pub(super) fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn discovered_inventory_file(
    crate_name: &str,
    path: &Path,
) -> Result<WorthQueryBoundaryAuditSourceInventoryFile, WorthQueryBoundaryAuditError> {
    let source = std::fs::read_to_string(path).map_err(|error| {
        WorthQueryBoundaryAuditError::new(
            WorthQueryBoundaryAuditErrorKind::SourceInventoryReadFailed,
            format!(
                "failed to read boundary audit source `{}`: {error}",
                path.display()
            ),
        )
    })?;
    let source_path = normalize_path(path);
    Ok(WorthQueryBoundaryAuditSourceInventoryFile::discovered(
        inventory_label(crate_name, &source_path),
        source_path,
        source,
    ))
}

fn inventory_label(crate_name: &str, source_path: &str) -> String {
    let label_path = source_path
        .trim_end_matches(".rs")
        .replace(['/', '\\'], ".")
        .replace(':', ".");
    format!("{crate_name}.{label_path}")
}
