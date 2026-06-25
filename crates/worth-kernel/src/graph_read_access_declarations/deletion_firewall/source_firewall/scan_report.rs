use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use super::super::errors::{
    WorthGraphReadDeclarationDeletionFirewallError,
    WorthGraphReadDeclarationDeletionFirewallErrorKind,
};
use super::super::stable_identity_digest::stable_digest;
use super::forbidden_pattern::FORBIDDEN_LOCAL_DECLARATION_PATTERNS;
use super::region_report::WorthGraphReadDeclarationSourceFirewallRegionReport;
use super::source_roots::{
    declaration_firewall_source_roots, should_scan_source_path, SourceFirewallRegion,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadDeclarationSourceFirewallReport {
    region_reports: Vec<WorthGraphReadDeclarationSourceFirewallRegionReport>,
    scanned_source_count: usize,
    violation_count: usize,
    violations: Vec<String>,
    report_digest: String,
}

impl WorthGraphReadDeclarationSourceFirewallReport {
    pub(crate) fn scan_workspace(
        workspace_root: &Path,
    ) -> Result<Self, WorthGraphReadDeclarationDeletionFirewallError> {
        let roots = declaration_firewall_source_roots(workspace_root);
        let mut region_counters = RegionCounters::from_roots(&roots);
        let mut violations = Vec::new();
        let mut scanned_source_count = 0;
        for (path, region) in firewall_sources(roots) {
            if !should_scan_source_path(&path) {
                continue;
            }
            scanned_source_count += 1;
            region_counters.count_scanned_source(region);
            let text = std::fs::read_to_string(&path).map_err(|_| {
                WorthGraphReadDeclarationDeletionFirewallError::new(
                    WorthGraphReadDeclarationDeletionFirewallErrorKind::SourceFirewallViolation,
                )
            })?;
            for pattern in FORBIDDEN_LOCAL_DECLARATION_PATTERNS {
                if pattern.applies_to(region) && text.contains(pattern.text()) {
                    region_counters.count_violation(region);
                    violations.push(format!("{} contains {}", path.display(), pattern.text()));
                }
            }
        }
        let region_reports = region_counters.into_region_reports();
        let violation_count = violations.len();
        if violation_count > 0 {
            return Err(WorthGraphReadDeclarationDeletionFirewallError::new(
                WorthGraphReadDeclarationDeletionFirewallErrorKind::SourceFirewallViolation,
            ));
        }
        let report_digest = stable_digest(&[
            "worth_graph_read_declaration_source_firewall_report_v1".to_string(),
            format!("scanned_region_count:{}", region_reports.len()),
            format!("scanned_source_count:{scanned_source_count}"),
            format!("violation_count:{violation_count}"),
            format!(
                "regions:{}",
                region_reports
                    .iter()
                    .map(|region| region.digest_part())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        ]);
        Ok(Self {
            region_reports,
            scanned_source_count,
            violation_count,
            violations,
            report_digest,
        })
    }

    pub const fn scanned_region_count(&self) -> usize {
        self.region_reports.len()
    }

    pub fn region_reports(&self) -> &[WorthGraphReadDeclarationSourceFirewallRegionReport] {
        &self.region_reports
    }

    pub const fn scanned_source_count(&self) -> usize {
        self.scanned_source_count
    }

    pub const fn violation_count(&self) -> usize {
        self.violation_count
    }

    pub fn violations(&self) -> &[String] {
        &self.violations
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

fn firewall_sources(
    source_roots: Vec<super::source_roots::SourceFirewallRoot>,
) -> Vec<(PathBuf, SourceFirewallRegion)> {
    source_roots
        .into_iter()
        .flat_map(|source_root| {
            rust_sources_under(source_root.root())
                .into_iter()
                .map(move |path| (path, source_root.region()))
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

struct RegionCounters {
    rows: BTreeMap<SourceFirewallRegion, MutableRegionCounter>,
}

impl RegionCounters {
    fn from_roots(source_roots: &[super::source_roots::SourceFirewallRoot]) -> Self {
        let mut rows = BTreeMap::new();
        for source_root in source_roots {
            rows.entry(source_root.region()).or_insert_with(|| {
                MutableRegionCounter::new(
                    source_root.region(),
                    audited_pattern_count(source_root.region()),
                )
            });
        }
        Self { rows }
    }

    fn count_scanned_source(&mut self, region: SourceFirewallRegion) {
        if let Some(counter) = self.rows.get_mut(&region) {
            counter.scanned_source_count += 1;
        }
    }

    fn count_violation(&mut self, region: SourceFirewallRegion) {
        if let Some(counter) = self.rows.get_mut(&region) {
            counter.violation_count += 1;
        }
    }

    fn into_region_reports(self) -> Vec<WorthGraphReadDeclarationSourceFirewallRegionReport> {
        self.rows
            .into_values()
            .map(|counter| {
                WorthGraphReadDeclarationSourceFirewallRegionReport::new(
                    counter.region,
                    counter.scanned_source_count,
                    counter.audited_pattern_count,
                    counter.violation_count,
                )
            })
            .collect()
    }
}

struct MutableRegionCounter {
    region: SourceFirewallRegion,
    scanned_source_count: usize,
    audited_pattern_count: usize,
    violation_count: usize,
}

impl MutableRegionCounter {
    const fn new(region: SourceFirewallRegion, audited_pattern_count: usize) -> Self {
        Self {
            region,
            scanned_source_count: 0,
            audited_pattern_count,
            violation_count: 0,
        }
    }
}

fn audited_pattern_count(region: SourceFirewallRegion) -> usize {
    FORBIDDEN_LOCAL_DECLARATION_PATTERNS
        .iter()
        .filter(|pattern| pattern.applies_to(region))
        .count()
}
