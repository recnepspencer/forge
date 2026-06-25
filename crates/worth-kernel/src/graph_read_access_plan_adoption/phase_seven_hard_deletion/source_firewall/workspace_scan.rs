use std::path::{Path, PathBuf};

use super::firewall_region_row::WorthGraphReadAccessHardDeletionSourceFirewallRegionRow;
use super::firewall_report::WorthGraphReadAccessHardDeletionSourceFirewallReport;
use super::firewall_violation::WorthGraphReadAccessHardDeletionSourceFirewallViolation;
use super::forbidden_pattern::HARD_DELETION_FORBIDDEN_PATTERNS;
use super::source_roots::{hard_deletion_source_roots, should_scan_source_path};
use super::source_text_scan::scan_source_text;

pub(crate) fn scan_workspace(
    workspace_root: &Path,
) -> Result<
    WorthGraphReadAccessHardDeletionSourceFirewallReport,
    WorthGraphReadAccessHardDeletionSourceFirewallViolation,
> {
    let roots = hard_deletion_source_roots(workspace_root);
    let mut region_rows = Vec::new();
    let mut scanned_source_count = 0;
    for root in roots {
        let region = root.region();
        let mut region_scanned_source_count = 0;
        for path in rust_sources_under(root.root()) {
            if !should_scan_source_path(&path) {
                continue;
            }
            scanned_source_count += 1;
            region_scanned_source_count += 1;
            let text = std::fs::read_to_string(&path).map_err(|_| {
                WorthGraphReadAccessHardDeletionSourceFirewallViolation::new(
                    &path.display().to_string(),
                    "source_read_failed",
                )
            })?;
            scan_source_text(&path.display().to_string(), region, &text)?;
        }
        region_rows.push(
            WorthGraphReadAccessHardDeletionSourceFirewallRegionRow::new(
                region,
                root.root_identity().to_string(),
                region_scanned_source_count,
            ),
        );
    }
    debug_assert_eq!(
        scanned_source_count,
        region_rows
            .iter()
            .map(|row| row.scanned_source_count())
            .sum::<usize>()
    );
    Ok(WorthGraphReadAccessHardDeletionSourceFirewallReport::new(
        region_rows,
        HARD_DELETION_FORBIDDEN_PATTERNS.len(),
        0,
    ))
}

fn rust_sources_under(root: &Path) -> Vec<PathBuf> {
    let mut sources = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        let Ok(entries) = std::fs::read_dir(&path) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                sources.push(path);
            }
        }
    }
    sources
}
