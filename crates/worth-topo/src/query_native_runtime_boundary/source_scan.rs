use std::fs;
use std::path::{Path, PathBuf};

use super::inventory_row::WorthTopologyQueryNativeRuntimeBoundaryInventoryRow;
use super::stale_symbol::WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol;

pub(crate) fn current_stale_symbol_rows(
) -> Result<Vec<WorthTopologyQueryNativeRuntimeBoundaryInventoryRow>, String> {
    let source_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut rows = Vec::new();
    scan_source_tree(&source_root, &source_root, &mut rows)?;
    rows.sort_by(|left, right| {
        left.source_path()
            .cmp(right.source_path())
            .then(left.stale_symbol().cmp(&right.stale_symbol()))
    });
    Ok(rows)
}

#[cfg(test)]
pub(crate) fn stale_symbol_rows_from_source_pairs(
    sources: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
) -> Vec<WorthTopologyQueryNativeRuntimeBoundaryInventoryRow> {
    let mut rows = Vec::new();
    for (source_path, source) in sources {
        collect_rows_from_source(source_path.into(), &source.into(), &mut rows);
    }
    rows.sort_by(|left, right| {
        left.source_path()
            .cmp(right.source_path())
            .then(left.stale_symbol().cmp(&right.stale_symbol()))
    });
    rows
}

fn scan_source_tree(
    root: &Path,
    current: &Path,
    rows: &mut Vec<WorthTopologyQueryNativeRuntimeBoundaryInventoryRow>,
) -> Result<(), String> {
    for entry in fs::read_dir(current).map_err(|error| {
        format!(
            "failed to read worth-topo source directory `{}`: {error}",
            current.display()
        )
    })? {
        let entry = entry.map_err(|error| {
            format!(
                "failed to read worth-topo source directory entry `{}`: {error}",
                current.display()
            )
        })?;
        let path = entry.path();
        if path.is_dir() {
            if is_self_inventory_lane(root, &path) {
                continue;
            }
            scan_source_tree(root, &path, rows)?;
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("rs") {
            if is_test_source(root, &path) {
                continue;
            }
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read `{}`: {error}", path.display()))?;
            let relative_path = path
                .strip_prefix(root)
                .map_err(|error| {
                    format!(
                        "failed to normalize `{}` under `{}`: {error}",
                        path.display(),
                        root.display()
                    )
                })?
                .to_string_lossy()
                .replace('\\', "/");
            collect_rows_from_source(relative_path, &source, rows);
        }
    }
    Ok(())
}

fn collect_rows_from_source(
    source_path: String,
    source: &str,
    rows: &mut Vec<WorthTopologyQueryNativeRuntimeBoundaryInventoryRow>,
) {
    for (pattern, stale_symbol) in WorthTopologyQueryNativeRuntimeBoundaryStaleSymbol::patterns() {
        let observed_count = source.matches(pattern).count();
        if observed_count > 0 {
            rows.push(WorthTopologyQueryNativeRuntimeBoundaryInventoryRow::new(
                source_path.clone(),
                *stale_symbol,
                observed_count,
            ));
        }
    }
}

fn is_self_inventory_lane(root: &Path, path: &Path) -> bool {
    path.strip_prefix(root)
        .ok()
        .map(|relative| {
            relative
                .to_string_lossy()
                .replace('\\', "/")
                .starts_with("query_native_runtime_boundary")
        })
        .unwrap_or(false)
}

fn is_test_source(root: &Path, path: &Path) -> bool {
    path.strip_prefix(root)
        .ok()
        .map(|relative| {
            let normalized = relative.to_string_lossy().replace('\\', "/");
            normalized.ends_with("/tests.rs")
                || normalized == "tests.rs"
                || normalized.contains("/tests/")
        })
        .unwrap_or(false)
}
