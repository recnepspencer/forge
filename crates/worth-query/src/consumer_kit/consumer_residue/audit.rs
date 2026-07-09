use std::path::{Path, PathBuf};

use crate::consumer_kit::boundary_audit::{
    WorthQueryBoundaryAuditError, WorthQueryBoundaryAuditErrorKind,
};

use super::detection::{class_filter_allows, scan_consumer_residue_source};
use super::evidence::{
    derive_consumer_residue_finding_identity, derive_consumer_residue_report_identity,
    derive_consumer_residue_source_inventory_identity,
};
use super::finding::WorthQueryConsumerResidueFinding;
use super::inventory::WorthQueryConsumerResidueSourceInventory;
use super::registry::WorthQueryConsumerResidueClass;
use super::report::{WorthQueryConsumerResidueReport, WorthQueryConsumerResidueReportCounters};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerResidueAudit {
    consumer_name: String,
    required_roots: Vec<PathBuf>,
    allowed_query_owned_roots: Vec<PathBuf>,
    class_filter: Option<Vec<WorthQueryConsumerResidueClass>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryConsumerResidueQueryOwnedRootAuthority {
    _private: (),
}

pub fn query_consumer_residue_audit(
    consumer_name: impl Into<String>,
) -> WorthQueryConsumerResidueAudit {
    WorthQueryConsumerResidueAudit {
        consumer_name: consumer_name.into(),
        required_roots: Vec::new(),
        allowed_query_owned_roots: Vec::new(),
        class_filter: None,
    }
}

#[cfg(test)]
pub(crate) fn query_owned_consumer_residue_root_authority(
) -> WorthQueryConsumerResidueQueryOwnedRootAuthority {
    WorthQueryConsumerResidueQueryOwnedRootAuthority { _private: () }
}

impl WorthQueryConsumerResidueAudit {
    pub fn required_root(mut self, root: impl Into<PathBuf>) -> Self {
        self.required_roots.push(root.into());
        self
    }

    pub fn required_query_owned_implementation_root(
        mut self,
        root: impl Into<PathBuf>,
        _authority: &WorthQueryConsumerResidueQueryOwnedRootAuthority,
    ) -> Self {
        let root = root.into();
        self.required_roots.push(root.clone());
        self.allowed_query_owned_roots.push(root.into());
        self
    }

    pub(crate) fn with_class_filter(
        mut self,
        classes: impl IntoIterator<Item = WorthQueryConsumerResidueClass>,
    ) -> Self {
        self.class_filter = Some(classes.into_iter().collect());
        self
    }

    pub fn evaluate(self) -> Result<WorthQueryConsumerResidueReport, WorthQueryBoundaryAuditError> {
        validate_consumer_residue_audit_request(&self)?;
        let scan = scan_consumer_residue_required_roots(&self)?;
        Ok(seal_consumer_residue_report(self.consumer_name, scan))
    }
}

struct WorthQueryConsumerResidueScan {
    audited_roots: Vec<String>,
    audited_source_paths: Vec<String>,
    findings: Vec<WorthQueryConsumerResidueFinding>,
    counters: WorthQueryConsumerResidueReportCounters,
}

fn validate_consumer_residue_audit_request(
    audit: &WorthQueryConsumerResidueAudit,
) -> Result<(), WorthQueryBoundaryAuditError> {
    if audit.consumer_name.trim().is_empty() {
        return Err(WorthQueryBoundaryAuditError::new(
            WorthQueryBoundaryAuditErrorKind::EmptyCrateName,
            "consumer residue audit consumer name must not be empty",
        ));
    }
    if audit.required_roots.is_empty() {
        return Err(WorthQueryBoundaryAuditError::new(
            WorthQueryBoundaryAuditErrorKind::MissingRequiredRoot,
            "consumer residue audit requires at least one source root",
        ));
    }
    Ok(())
}

fn scan_consumer_residue_required_roots(
    audit: &WorthQueryConsumerResidueAudit,
) -> Result<WorthQueryConsumerResidueScan, WorthQueryBoundaryAuditError> {
    let mut audited_roots = Vec::new();
    let mut audited_source_paths = Vec::new();
    let mut findings = Vec::new();
    let mut counters = WorthQueryConsumerResidueReportCounters::default();
    for root in &audit.required_roots {
        assert_consumer_residue_root_exists(root)?;
        audited_roots.push(normalize_path(root));
        scan_root(
            &audit.consumer_name,
            root,
            &audit.allowed_query_owned_roots,
            audit.class_filter.as_deref(),
            &mut audited_source_paths,
            &mut findings,
            &mut counters,
        )?;
    }
    audited_source_paths.sort();
    sort_consumer_residue_findings(&mut findings);
    Ok(WorthQueryConsumerResidueScan {
        audited_roots,
        audited_source_paths,
        findings,
        counters,
    })
}

fn assert_consumer_residue_root_exists(root: &Path) -> Result<(), WorthQueryBoundaryAuditError> {
    if root.exists() {
        return Ok(());
    }
    Err(WorthQueryBoundaryAuditError::new(
        WorthQueryBoundaryAuditErrorKind::MissingRequiredRoot,
        format!(
            "required consumer residue root `{}` does not exist",
            root.display()
        ),
    ))
}

fn sort_consumer_residue_findings(findings: &mut [WorthQueryConsumerResidueFinding]) {
    findings.sort_by(|left, right| {
        left.source_path()
            .cmp(right.source_path())
            .then(left.line().cmp(&right.line()))
            .then(left.column().cmp(&right.column()))
            .then(left.residue_class().cmp(&right.residue_class()))
    });
}

fn seal_consumer_residue_report(
    consumer_name: String,
    scan: WorthQueryConsumerResidueScan,
) -> WorthQueryConsumerResidueReport {
    let finding_identities = scan
        .findings
        .iter()
        .map(derive_consumer_residue_finding_identity)
        .collect::<Vec<_>>();
    let source_inventory = seal_consumer_residue_source_inventory(
        &consumer_name,
        &scan.audited_roots,
        scan.audited_source_paths,
        scan.counters.skipped_non_rust_file_count,
    );
    let report_identity = derive_consumer_residue_report_identity(
        &consumer_name,
        &scan.audited_roots,
        source_inventory.inventory_digest(),
        scan.counters.scanned_file_count,
        scan.counters.parsed_item_count,
        scan.counters.visited_node_count,
        &finding_identities,
    );
    WorthQueryConsumerResidueReport::sealed(
        consumer_name,
        scan.audited_roots,
        scan.findings,
        finding_identities,
        report_identity,
        source_inventory,
        scan.counters,
    )
}

fn seal_consumer_residue_source_inventory(
    consumer_name: &str,
    audited_roots: &[String],
    audited_source_paths: Vec<String>,
    skipped_non_rust_file_count: usize,
) -> WorthQueryConsumerResidueSourceInventory {
    let source_inventory_identity = derive_consumer_residue_source_inventory_identity(
        consumer_name,
        audited_roots,
        &audited_source_paths,
        skipped_non_rust_file_count,
    );
    WorthQueryConsumerResidueSourceInventory::sealed(
        audited_source_paths,
        skipped_non_rust_file_count,
        source_inventory_identity,
    )
}

fn scan_root(
    consumer_name: &str,
    root: &Path,
    allowed_roots: &[PathBuf],
    class_filter: Option<&[WorthQueryConsumerResidueClass]>,
    audited_source_paths: &mut Vec<String>,
    findings: &mut Vec<WorthQueryConsumerResidueFinding>,
    counters: &mut WorthQueryConsumerResidueReportCounters,
) -> Result<(), WorthQueryBoundaryAuditError> {
    let entries = std::fs::read_dir(root).map_err(|error| {
        WorthQueryBoundaryAuditError::new(
            WorthQueryBoundaryAuditErrorKind::SourceInventoryReadFailed,
            format!(
                "failed to read consumer residue root `{}`: {error}",
                root.display()
            ),
        )
    })?;

    for entry in entries {
        let entry = entry.map_err(|error| {
            WorthQueryBoundaryAuditError::new(
                WorthQueryBoundaryAuditErrorKind::SourceInventoryReadFailed,
                format!(
                    "failed to read consumer residue entry under `{}`: {error}",
                    root.display()
                ),
            )
        })?;
        scan_entry(
            consumer_name,
            &entry.path(),
            allowed_roots,
            class_filter,
            audited_source_paths,
            findings,
            counters,
        )?;
    }
    Ok(())
}

fn scan_entry(
    consumer_name: &str,
    path: &Path,
    allowed_roots: &[PathBuf],
    class_filter: Option<&[WorthQueryConsumerResidueClass]>,
    audited_source_paths: &mut Vec<String>,
    findings: &mut Vec<WorthQueryConsumerResidueFinding>,
    counters: &mut WorthQueryConsumerResidueReportCounters,
) -> Result<(), WorthQueryBoundaryAuditError> {
    if path.is_dir() {
        scan_root(
            consumer_name,
            path,
            allowed_roots,
            class_filter,
            audited_source_paths,
            findings,
            counters,
        )?;
        return Ok(());
    }
    if path.extension().and_then(|extension| extension.to_str()) != Some("rs") {
        counters.skipped_non_rust_file_count += 1;
        return Ok(());
    }
    audited_source_paths.push(normalize_path(path));
    counters.scanned_file_count += 1;
    scan_rust_source_file(
        consumer_name,
        path,
        allowed_roots,
        class_filter,
        findings,
        counters,
    )
}

fn scan_rust_source_file(
    consumer_name: &str,
    path: &Path,
    allowed_roots: &[PathBuf],
    class_filter: Option<&[WorthQueryConsumerResidueClass]>,
    findings: &mut Vec<WorthQueryConsumerResidueFinding>,
    counters: &mut WorthQueryConsumerResidueReportCounters,
) -> Result<(), WorthQueryBoundaryAuditError> {
    let source = std::fs::read_to_string(path).map_err(|error| {
        WorthQueryBoundaryAuditError::new(
            WorthQueryBoundaryAuditErrorKind::SourceInventoryReadFailed,
            format!(
                "failed to read consumer residue source `{}`: {error}",
                path.display()
            ),
        )
    })?;
    let source_path = normalize_path(path);
    let source_label = inventory_label(consumer_name, &source_path);
    let is_query_owned = allowed_roots.iter().any(|root| path.starts_with(root));
    let classification = scan_consumer_residue_source(
        &source_label,
        &source_path,
        &source,
        is_query_owned,
        class_filter,
    )?;
    counters.parsed_item_count += classification.parsed_item_count;
    counters.visited_node_count += classification.visited_node_count;
    findings.extend(
        classification
            .findings
            .into_iter()
            .filter(|finding| class_filter_allows(class_filter, finding.residue_class())),
    );
    Ok(())
}

fn inventory_label(consumer_name: &str, source_path: &str) -> String {
    let label_path = source_path
        .trim_end_matches(".rs")
        .replace(['/', '\\'], ".")
        .replace(':', ".");
    format!("{consumer_name}.{label_path}")
}

pub(crate) fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
