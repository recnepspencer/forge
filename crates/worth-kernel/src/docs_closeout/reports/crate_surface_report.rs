use std::path::Path;

use crate::docs_closeout::error::{WorthDocsCloseoutError, WorthDocsCloseoutErrorKind};
use crate::docs_closeout::model::invariant_evidence::WorthDocsInvariantEvidence;
use crate::docs_closeout::scan::report_digest::digest_lines;
use crate::docs_closeout::scan::workspace_scan::{
    expectation, scan_all_touched_crates, scan_all_touched_crates_for_root, WorthDocsCrateScan,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorthCrateDocsSurfaceStatus {
    Satisfied,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthCrateDocsSurfaceRow {
    crate_name: String,
    status: WorthCrateDocsSurfaceStatus,
    reason: String,
    row_digest: String,
    evidence: WorthDocsInvariantEvidence,
}

impl WorthCrateDocsSurfaceRow {
    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }

    pub fn status(&self) -> WorthCrateDocsSurfaceStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }

    pub fn evidence(&self) -> &WorthDocsInvariantEvidence {
        &self.evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthCrateDocsSurfaceReport {
    rows: Vec<WorthCrateDocsSurfaceRow>,
    report_digest: String,
}

impl WorthCrateDocsSurfaceReport {
    pub fn rows(&self) -> &[WorthCrateDocsSurfaceRow] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn current_worth_crate_docs_surface_report(
) -> Result<WorthCrateDocsSurfaceReport, WorthDocsCloseoutError> {
    let scans = scan_all_touched_crates()?;
    build_crate_docs_surface_report(scans)
}

pub fn worth_crate_docs_surface_report_for_root(
    workspace_root: &Path,
) -> Result<WorthCrateDocsSurfaceReport, WorthDocsCloseoutError> {
    let scans = scan_all_touched_crates_for_root(workspace_root)?;
    build_crate_docs_surface_report(scans)
}

fn build_crate_docs_surface_report(
    scans: Vec<WorthDocsCrateScan>,
) -> Result<WorthCrateDocsSurfaceReport, WorthDocsCloseoutError> {
    let mut rows = Vec::new();
    for scan in scans {
        let expected = expectation(&scan.crate_name);
        let readme = &scan.readme;
        let mut categories = vec!["foundations", "boundaries"];
        if !expected.expected_feature_ids.is_empty() {
            categories.push("features");
        }
        let mut evidence = WorthDocsInvariantEvidence::default();
        if readme.metadata.crate_name != scan.crate_name {
            evidence.push_missing_metadata_entry(format!("crate={}", scan.crate_name));
        }
        if readme.metadata.doc_style.as_deref() != Some(expected.doc_style) {
            evidence.push_missing_metadata_entry(format!("doc_style={}", expected.doc_style));
        }
        for category in categories {
            if !readme.metadata.categories.contains(category) {
                evidence.push_missing_metadata_entry(format!("category:{category}"));
            }
            if !scan.docs_dir.join(category).exists() {
                evidence.push_missing_directory(category);
            }
            if !readme.markdown.contains(&format!("./{category}/")) {
                evidence.push_missing_readme_fragment(format!("./{category}/"));
            }
        }
        if scan.foundations.is_empty() {
            evidence.push_missing_directory("foundations-anchor");
        }
        for neighbor in expected.neighbors {
            if !readme.metadata.neighbor_crates.contains(*neighbor) {
                evidence.push_missing_metadata_entry(format!("neighbor:{neighbor}"));
            }
            if !readme.markdown.contains(neighbor) {
                evidence.push_missing_readme_fragment(*neighbor);
            }
        }
        for heading in ["Reading Style", "Map", "Neighboring Crates"] {
            if !readme.headings.contains(heading) {
                evidence.push_missing_heading(heading);
            }
        }
        let status = if let Some(reason) = evidence.first_problem() {
            blocked_row(&scan.crate_name, reason, evidence)
        } else {
            satisfied_row(
                &scan.crate_name,
                "crate README, categories, and reader graph are machine-checkable".to_string(),
                evidence,
            )
        };
        rows.push(status);
    }

    if rows.is_empty() {
        return Err(WorthDocsCloseoutError::new(
            WorthDocsCloseoutErrorKind::TopologyDrift,
            None,
            "no touched crate docs rows were produced",
        ));
    }

    Ok(WorthCrateDocsSurfaceReport {
        report_digest: digest_lines(
            &rows
                .iter()
                .map(|row| {
                    format!(
                        "{}|{:?}|{}|{}",
                        row.crate_name, row.status, row.reason, row.row_digest
                    )
                })
                .collect::<Vec<_>>(),
        ),
        rows,
    })
}

fn satisfied_row(
    crate_name: &str,
    reason: String,
    evidence: WorthDocsInvariantEvidence,
) -> WorthCrateDocsSurfaceRow {
    build_row(
        crate_name,
        WorthCrateDocsSurfaceStatus::Satisfied,
        reason,
        evidence,
    )
}

fn blocked_row(
    crate_name: &str,
    reason: String,
    evidence: WorthDocsInvariantEvidence,
) -> WorthCrateDocsSurfaceRow {
    build_row(
        crate_name,
        WorthCrateDocsSurfaceStatus::Blocked,
        reason,
        evidence,
    )
}

fn build_row(
    crate_name: &str,
    status: WorthCrateDocsSurfaceStatus,
    reason: String,
    evidence: WorthDocsInvariantEvidence,
) -> WorthCrateDocsSurfaceRow {
    let row_digest = digest_lines(&[format!("{crate_name}|{status:?}|{reason}")]);
    WorthCrateDocsSurfaceRow {
        crate_name: crate_name.to_string(),
        status,
        reason,
        row_digest,
        evidence,
    }
}
