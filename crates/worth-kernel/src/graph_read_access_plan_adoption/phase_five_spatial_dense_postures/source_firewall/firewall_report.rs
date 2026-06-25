use std::path::{Path, PathBuf};

use super::super::stable_digest;
use super::firewall_violation::WorthGraphReadAccessSpatialDenseSourceFirewallViolation;
use super::forbidden_pattern::FORBIDDEN_PATTERNS;
use super::source_roots::{
    phase_five_source_roots, should_scan_source_path, WorthGraphReadAccessSpatialDenseSourceRegion,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessSpatialDenseSourceFirewallReport {
    scanned_region_count: usize,
    scanned_source_count: usize,
    forbidden_pattern_count: usize,
    violation_count: usize,
    report_digest: String,
}

pub(crate) fn scan_workspace(
    workspace_root: &Path,
) -> Result<
    WorthGraphReadAccessSpatialDenseSourceFirewallReport,
    WorthGraphReadAccessSpatialDenseSourceFirewallViolation,
> {
    let roots = phase_five_source_roots(workspace_root);
    let scanned_region_count = roots.len();
    let mut scanned_source_count = 0;
    for (path, region) in firewall_sources(roots) {
        if !should_scan_source_path(&path) {
            continue;
        }
        scanned_source_count += 1;
        let text = std::fs::read_to_string(&path).map_err(|_| {
            WorthGraphReadAccessSpatialDenseSourceFirewallViolation::new(
                &path.display().to_string(),
                "source_read_failed",
            )
        })?;
        scan_source_text(&path.display().to_string(), region, &text)?;
    }
    Ok(WorthGraphReadAccessSpatialDenseSourceFirewallReport::new(
        scanned_region_count,
        scanned_source_count,
        FORBIDDEN_PATTERNS.len(),
        0,
    ))
}

pub(crate) fn scan_source(
    source_path: &str,
    source_text: &str,
) -> Result<
    WorthGraphReadAccessSpatialDenseSourceFirewallReport,
    WorthGraphReadAccessSpatialDenseSourceFirewallViolation,
> {
    scan_source_text(
        source_path,
        WorthGraphReadAccessSpatialDenseSourceRegion::StandaloneTestInput,
        source_text,
    )?;
    Ok(WorthGraphReadAccessSpatialDenseSourceFirewallReport::new(
        1,
        1,
        FORBIDDEN_PATTERNS.len(),
        0,
    ))
}

fn scan_source_text(
    source_path: &str,
    region: WorthGraphReadAccessSpatialDenseSourceRegion,
    source_text: &str,
) -> Result<(), WorthGraphReadAccessSpatialDenseSourceFirewallViolation> {
    for pattern in FORBIDDEN_PATTERNS {
        if pattern.applies_to(region) && source_text.contains(pattern.needle) {
            return Err(
                WorthGraphReadAccessSpatialDenseSourceFirewallViolation::new(
                    source_path,
                    pattern.label,
                ),
            );
        }
    }
    Ok(())
}

fn firewall_sources(
    roots: Vec<super::source_roots::WorthGraphReadAccessSpatialDenseSourceRoot>,
) -> Vec<(PathBuf, WorthGraphReadAccessSpatialDenseSourceRegion)> {
    roots
        .into_iter()
        .flat_map(|root| {
            rust_sources_under(root.root())
                .into_iter()
                .map(move |path| (path, root.region()))
        })
        .collect()
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

impl WorthGraphReadAccessSpatialDenseSourceFirewallReport {
    fn new(
        scanned_region_count: usize,
        scanned_source_count: usize,
        forbidden_pattern_count: usize,
        violation_count: usize,
    ) -> Self {
        let report_digest = stable_digest(&[
            "worth_graph_read_access_spatial_dense_source_firewall_v1".to_string(),
            format!("regions:{scanned_region_count}"),
            format!("scanned:{scanned_source_count}"),
            format!("patterns:{forbidden_pattern_count}"),
            format!("violations:{violation_count}"),
        ]);
        Self {
            scanned_region_count,
            scanned_source_count,
            forbidden_pattern_count,
            violation_count,
            report_digest,
        }
    }

    pub const fn scanned_region_count(&self) -> usize {
        self.scanned_region_count
    }

    pub const fn scanned_source_count(&self) -> usize {
        self.scanned_source_count
    }

    pub const fn forbidden_pattern_count(&self) -> usize {
        self.forbidden_pattern_count
    }

    pub const fn violation_count(&self) -> usize {
        self.violation_count
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
