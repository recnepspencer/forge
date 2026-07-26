use std::collections::BTreeSet;
use std::path::Path;

use crate::topology::WorkspaceSourceInventory;

use super::ledger;

const CERTIFICATION_ROOT: &str = "crates/worth-ui-certification/src";

pub(super) fn audit(
    inventory: &WorkspaceSourceInventory,
    rows: &[toml::Value],
) -> Result<(), String> {
    let actual = certification_source_paths(inventory);
    let mut classified = BTreeSet::new();
    for row in rows {
        let paths = audit_surface_row(inventory, row, &actual)?;
        for path in &paths {
            if !classified.insert(path.clone()) {
                return Err(format!(
                    "certification source `{path}` has multiple audiences"
                ));
            }
        }
    }
    if actual != classified {
        return Err(format!(
            "certification audience inventory is incomplete: unclassified={:?}",
            actual.difference(&classified).collect::<Vec<_>>()
        ));
    }
    Ok(())
}

fn certification_source_paths(inventory: &WorkspaceSourceInventory) -> BTreeSet<String> {
    inventory
        .rust_files_under(CERTIFICATION_ROOT)
        .filter_map(|source| {
            source
                .relative_path()
                .strip_prefix(CERTIFICATION_ROOT)
                .ok()
                .map(normalize)
        })
        .collect()
}

fn audit_surface_row(
    inventory: &WorkspaceSourceInventory,
    row: &toml::Value,
    actual: &BTreeSet<String>,
) -> Result<Vec<String>, String> {
    let id = ledger::text(row, "id")?;
    validate_row_fields(row)?;
    if ledger::text(row, "audience")? != "certification-support" {
        return Err(format!("`{id}` should remain certification-only"));
    }
    let paths = actual
        .iter()
        .filter(|path| row_matches(row, path))
        .cloned()
        .collect::<Vec<_>>();
    if paths.len() != ledger::integer(row, "expected_files")? as usize {
        return Err(format!(
            "`{id}` certification file count changed: {}",
            paths.len()
        ));
    }
    let observed = content_fingerprint(inventory, &paths);
    let expected = ledger::text(row, "content_fingerprint")?;
    if observed != expected {
        return Err(format!(
            "`{id}` certification public surface changed: {observed} != {expected}"
        ));
    }
    Ok(paths)
}

fn validate_row_fields(row: &toml::Value) -> Result<(), String> {
    for field in [
        "path",
        "match",
        "content_fingerprint",
        "audience",
        "responsibility",
        "disposition",
        "forbidden_shortcut",
    ] {
        ledger::text(row, field)?;
    }
    Ok(())
}

fn row_matches(row: &toml::Value, path: &str) -> bool {
    let owner_path = ledger::text(row, "path").unwrap_or("");
    match ledger::text(row, "match").unwrap_or("") {
        "exact" => path == owner_path,
        "directory" => {
            path == format!("{owner_path}/mod.rs") || path.starts_with(&format!("{owner_path}/"))
        }
        _ => false,
    }
}

fn content_fingerprint(inventory: &WorkspaceSourceInventory, paths: &[String]) -> String {
    let mut bytes = Vec::new();
    for path in paths {
        bytes.extend_from_slice(path.as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(
            inventory
                .text(Path::new(CERTIFICATION_ROOT).join(path))
                .as_bytes(),
        );
        bytes.push(0);
    }
    ledger::fingerprint(bytes)
}

fn normalize(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
