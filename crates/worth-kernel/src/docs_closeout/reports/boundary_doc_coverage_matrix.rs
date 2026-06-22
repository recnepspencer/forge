use std::collections::BTreeMap;
use std::path::Path;

use crate::docs_closeout::error::{WorthDocsCloseoutError, WorthDocsCloseoutErrorKind};
use crate::docs_closeout::model::invariant_evidence::WorthDocsInvariantEvidence;
use crate::docs_closeout::scan::report_digest::digest_lines;
use crate::docs_closeout::scan::workspace_scan::{
    expectation, scan_all_touched_crates, scan_all_touched_crates_for_root, WorthDocFile,
    WorthDocsCrateScan,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorthBoundaryDocCoverageStatus {
    Satisfied,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthBoundaryDocCoverageRow {
    boundary_id: String,
    crate_name: String,
    status: WorthBoundaryDocCoverageStatus,
    reason: String,
    row_digest: String,
    evidence: WorthDocsInvariantEvidence,
}

impl WorthBoundaryDocCoverageRow {
    pub fn boundary_id(&self) -> &str {
        &self.boundary_id
    }

    pub fn status(&self) -> WorthBoundaryDocCoverageStatus {
        self.status
    }

    pub fn crate_name(&self) -> &str {
        &self.crate_name
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
pub struct WorthBoundaryDocCoverageMatrix {
    rows: Vec<WorthBoundaryDocCoverageRow>,
    coverage_matrix_digest: String,
}

impl WorthBoundaryDocCoverageMatrix {
    pub fn rows(&self) -> &[WorthBoundaryDocCoverageRow] {
        &self.rows
    }

    pub fn coverage_matrix_digest(&self) -> &str {
        &self.coverage_matrix_digest
    }
}

pub fn current_worth_boundary_doc_coverage_matrix(
) -> Result<WorthBoundaryDocCoverageMatrix, WorthDocsCloseoutError> {
    let scans = scan_all_touched_crates()?;
    build_boundary_doc_coverage_matrix(scans)
}

pub fn worth_boundary_doc_coverage_matrix_for_root(
    workspace_root: &Path,
) -> Result<WorthBoundaryDocCoverageMatrix, WorthDocsCloseoutError> {
    let scans = scan_all_touched_crates_for_root(workspace_root)?;
    build_boundary_doc_coverage_matrix(scans)
}

fn build_boundary_doc_coverage_matrix(
    scans: Vec<WorthDocsCrateScan>,
) -> Result<WorthBoundaryDocCoverageMatrix, WorthDocsCloseoutError> {
    let mut rows = Vec::new();
    let mut ownership = BTreeMap::<String, usize>::new();

    for scan in &scans {
        for boundary in &scan.boundaries {
            *ownership
                .entry(boundary.metadata.doc_id.clone())
                .or_insert(0) += 1;
        }
    }

    for scan in scans {
        let expected = expectation(&scan.crate_name);
        for boundary_id in expected.expected_boundary_ids {
            let row = if let Some(boundary) = scan
                .boundaries
                .iter()
                .find(|boundary| boundary.metadata.doc_id == *boundary_id)
            {
                validate_boundary_row(&scan.crate_name, boundary_id, boundary, &ownership)
            } else {
                blocked_row(
                    &scan.crate_name,
                    boundary_id,
                    "boundary doc is missing".to_string(),
                    WorthDocsInvariantEvidence::default(),
                )
            };
            rows.push(row);
        }
    }

    if rows.is_empty() {
        return Err(WorthDocsCloseoutError::new(
            WorthDocsCloseoutErrorKind::TopologyDrift,
            None,
            "no boundary coverage rows were produced",
        ));
    }

    Ok(WorthBoundaryDocCoverageMatrix {
        coverage_matrix_digest: digest_lines(
            &rows
                .iter()
                .map(|row| {
                    format!(
                        "{}|{}|{:?}|{}|{}",
                        row.crate_name, row.boundary_id, row.status, row.reason, row.row_digest
                    )
                })
                .collect::<Vec<_>>(),
        ),
        rows,
    })
}

fn validate_boundary_row(
    crate_name: &str,
    boundary_id: &str,
    boundary: &WorthDocFile,
    ownership: &BTreeMap<String, usize>,
) -> WorthBoundaryDocCoverageRow {
    let required_headings = [
        "Boundary",
        "Allowed Upstream Inputs",
        "Required Downstream Outputs",
        "Stable Entry Points",
        "Forbidden Bypasses",
        "Binding Artifacts Or Receipts",
        "Related Docs",
    ];
    let mut evidence = WorthDocsInvariantEvidence::default();
    evidence.set_ownership_count(ownership.get(boundary_id).copied().unwrap_or_default());
    if boundary.relative_path != format!("boundaries/{boundary_id}.md") {
        evidence.set_actual_relative_path(boundary.relative_path.clone());
    }
    for heading in required_headings
        .iter()
        .filter(|heading| !boundary.headings.contains(**heading))
    {
        evidence.push_missing_heading(*heading);
    }
    if boundary.metadata.touches_query && !boundary.headings.contains("Query Usage") {
        evidence.push_missing_heading("Query Usage");
    }
    if let Some(reason) = blocked_reason(&evidence) {
        return blocked_row(crate_name, boundary_id, reason, evidence);
    }
    satisfied_row(
        crate_name,
        boundary_id,
        "boundary doc teaches the owning handoff explicitly".to_string(),
        evidence,
    )
}

fn blocked_reason(evidence: &WorthDocsInvariantEvidence) -> Option<String> {
    if evidence.ownership_count().unwrap_or_default() != 1 {
        return evidence
            .first_problem()
            .or_else(|| Some("boundary ownership is not unique".to_string()));
    }
    evidence.first_problem()
}

fn satisfied_row(
    crate_name: &str,
    boundary_id: &str,
    reason: String,
    evidence: WorthDocsInvariantEvidence,
) -> WorthBoundaryDocCoverageRow {
    build_row(
        crate_name,
        boundary_id,
        WorthBoundaryDocCoverageStatus::Satisfied,
        reason,
        evidence,
    )
}

fn blocked_row(
    crate_name: &str,
    boundary_id: &str,
    reason: String,
    evidence: WorthDocsInvariantEvidence,
) -> WorthBoundaryDocCoverageRow {
    build_row(
        crate_name,
        boundary_id,
        WorthBoundaryDocCoverageStatus::Blocked,
        reason,
        evidence,
    )
}

fn build_row(
    crate_name: &str,
    boundary_id: &str,
    status: WorthBoundaryDocCoverageStatus,
    reason: String,
    evidence: WorthDocsInvariantEvidence,
) -> WorthBoundaryDocCoverageRow {
    let row_digest = digest_lines(&[format!("{crate_name}|{boundary_id}|{status:?}|{reason}")]);
    WorthBoundaryDocCoverageRow {
        boundary_id: boundary_id.to_string(),
        crate_name: crate_name.to_string(),
        status,
        reason,
        row_digest,
        evidence,
    }
}
