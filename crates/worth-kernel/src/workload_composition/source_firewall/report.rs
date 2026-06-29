use std::collections::BTreeSet;

use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::forbidden_surface::WorthTouchedGraphConflictForbiddenSurface;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictSourceFirewallViolation {
    region_label: String,
    source_path: String,
    surface_name: String,
    forbidden_surface: WorthTouchedGraphConflictForbiddenSurface,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictSourceFirewallRegionReport {
    region_label: String,
    root_identity: String,
    scanned_source_count: usize,
    covered_forbidden_surfaces: BTreeSet<WorthTouchedGraphConflictForbiddenSurface>,
    forbidden_surfaces: BTreeSet<WorthTouchedGraphConflictForbiddenSurface>,
    violation_count: usize,
    region_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphConflictSourceFirewallReport {
    region_reports: Vec<WorthTouchedGraphConflictSourceFirewallRegionReport>,
    violations: Vec<WorthTouchedGraphConflictSourceFirewallViolation>,
    report_digest: String,
}

impl WorthTouchedGraphConflictSourceFirewallViolation {
    pub(crate) fn new(
        region_label: impl Into<String>,
        source_path: impl Into<String>,
        surface_name: impl Into<String>,
        forbidden_surface: WorthTouchedGraphConflictForbiddenSurface,
    ) -> Self {
        Self {
            region_label: region_label.into(),
            source_path: source_path.into(),
            surface_name: surface_name.into(),
            forbidden_surface,
        }
    }

    pub fn region_label(&self) -> &str {
        &self.region_label
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub const fn forbidden_surface(&self) -> WorthTouchedGraphConflictForbiddenSurface {
        self.forbidden_surface
    }
}

impl WorthTouchedGraphConflictSourceFirewallRegionReport {
    pub(crate) fn new(
        region_label: impl Into<String>,
        root_identity: impl Into<String>,
        scanned_source_count: usize,
        covered_forbidden_surfaces: BTreeSet<WorthTouchedGraphConflictForbiddenSurface>,
        forbidden_surfaces: BTreeSet<WorthTouchedGraphConflictForbiddenSurface>,
        violation_count: usize,
    ) -> Self {
        let region_label = region_label.into();
        let root_identity = root_identity.into();
        let mut parts = vec![
            "worth-kernel:touched-graph-conflict-source-firewall-region:v1".to_string(),
            format!("region:{region_label}"),
            format!("root:{root_identity}"),
            format!("scanned:{scanned_source_count}"),
            format!("covered:{}", covered_forbidden_surfaces.len()),
            format!("violations:{violation_count}"),
        ];
        parts.extend(
            covered_forbidden_surfaces
                .iter()
                .map(|surface| format!("covered:{}", surface.as_str())),
        );
        parts.extend(
            forbidden_surfaces
                .iter()
                .map(|surface| format!("forbidden:{}", surface.as_str())),
        );
        let region_digest = truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts);
        Self {
            region_label,
            root_identity,
            scanned_source_count,
            covered_forbidden_surfaces,
            forbidden_surfaces,
            violation_count,
            region_digest,
        }
    }

    pub fn region_label(&self) -> &str {
        &self.region_label
    }

    pub fn root_identity(&self) -> &str {
        &self.root_identity
    }

    pub const fn scanned_source_count(&self) -> usize {
        self.scanned_source_count
    }

    pub fn covered_forbidden_surfaces(
        &self,
    ) -> &BTreeSet<WorthTouchedGraphConflictForbiddenSurface> {
        &self.covered_forbidden_surfaces
    }

    pub fn forbidden_surfaces(&self) -> &BTreeSet<WorthTouchedGraphConflictForbiddenSurface> {
        &self.forbidden_surfaces
    }

    pub const fn violation_count(&self) -> usize {
        self.violation_count
    }

    pub fn region_digest(&self) -> &str {
        &self.region_digest
    }
}

impl WorthTouchedGraphConflictSourceFirewallReport {
    pub(crate) fn new(
        region_reports: Vec<WorthTouchedGraphConflictSourceFirewallRegionReport>,
        violations: Vec<WorthTouchedGraphConflictSourceFirewallViolation>,
    ) -> Self {
        let mut parts = vec![
            "worth-kernel:touched-graph-conflict-source-firewall-report:v1".to_string(),
            format!("region-count:{}", region_reports.len()),
            format!("violation-count:{}", violations.len()),
        ];
        parts.extend(
            region_reports
                .iter()
                .map(|report| format!("region:{}", report.region_digest())),
        );
        parts.extend(violations.iter().map(|violation| {
            format!(
                "violation:{}:{}:{}:{}",
                violation.region_label(),
                violation.source_path(),
                violation.surface_name(),
                violation.forbidden_surface().as_str()
            )
        }));
        let report_digest = truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts);
        Self {
            region_reports,
            violations,
            report_digest,
        }
    }

    pub fn region_reports(&self) -> &[WorthTouchedGraphConflictSourceFirewallRegionReport] {
        &self.region_reports
    }

    pub fn violations(&self) -> &[WorthTouchedGraphConflictSourceFirewallViolation] {
        &self.violations
    }

    pub fn covered_forbidden_surfaces(
        &self,
    ) -> BTreeSet<WorthTouchedGraphConflictForbiddenSurface> {
        self.region_reports
            .iter()
            .flat_map(|report| report.covered_forbidden_surfaces().iter().copied())
            .collect()
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub const fn scanned_region_count(&self) -> usize {
        self.region_reports.len()
    }

    pub fn scanned_source_count(&self) -> usize {
        self.region_reports
            .iter()
            .map(WorthTouchedGraphConflictSourceFirewallRegionReport::scanned_source_count)
            .sum()
    }

    pub const fn violation_count(&self) -> usize {
        self.violations.len()
    }
}
