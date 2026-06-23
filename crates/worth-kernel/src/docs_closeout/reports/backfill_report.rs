use super::backfill_inventory::{WorthBackfillDocKind, WORTH_BACKFILL_SURFACE_EXPECTATIONS};
use std::path::Path;

use crate::docs_closeout::error::{WorthDocsCloseoutError, WorthDocsCloseoutErrorKind};
use crate::docs_closeout::model::invariant_evidence::WorthDocsInvariantEvidence;
use crate::docs_closeout::scan::report_digest::digest_lines;
use crate::docs_closeout::scan::workspace_scan::{
    scan_all_touched_crates, scan_all_touched_crates_for_root, WorthDocsCrateScan,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorthDocsBackfillStatus {
    Satisfied,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthDocsBackfillRow {
    surface_name: String,
    status: WorthDocsBackfillStatus,
    reason: String,
    row_digest: String,
    evidence: WorthDocsInvariantEvidence,
}

impl WorthDocsBackfillRow {
    pub fn surface_name(&self) -> &str {
        &self.surface_name
    }

    pub fn status(&self) -> WorthDocsBackfillStatus {
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
pub struct WorthDocsBackfillReport {
    rows: Vec<WorthDocsBackfillRow>,
    report_digest: String,
}

impl WorthDocsBackfillReport {
    pub fn rows(&self) -> &[WorthDocsBackfillRow] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

pub fn current_worth_docs_backfill_report(
) -> Result<WorthDocsBackfillReport, WorthDocsCloseoutError> {
    let scans = scan_all_touched_crates()?;
    build_backfill_report(scans)
}

pub fn worth_docs_backfill_report_for_root(
    workspace_root: &Path,
) -> Result<WorthDocsBackfillReport, WorthDocsCloseoutError> {
    let scans = scan_all_touched_crates_for_root(workspace_root)?;
    build_backfill_report(scans)
}

fn build_backfill_report(
    scans: Vec<WorthDocsCrateScan>,
) -> Result<WorthDocsBackfillReport, WorthDocsCloseoutError> {
    let rows = WORTH_BACKFILL_SURFACE_EXPECTATIONS
        .iter()
        .map(|surface| build_backfill_row(&scans, surface))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(WorthDocsBackfillReport {
        report_digest: digest_lines(
            &rows
                .iter()
                .map(|row| format!("{}|{:?}|{}", row.surface_name, row.status, row.reason))
                .collect::<Vec<_>>(),
        ),
        rows,
    })
}

fn build_backfill_row(
    scans: &[WorthDocsCrateScan],
    surface: &super::backfill_inventory::WorthBackfillSurfaceExpectation,
) -> Result<WorthDocsBackfillRow, WorthDocsCloseoutError> {
    let scan = scans
        .iter()
        .find(|scan| scan.crate_name == surface.crate_name)
        .ok_or_else(|| missing_scan(surface.crate_name))?;
    let doc = match surface.doc_kind {
        WorthBackfillDocKind::Feature => scan
            .features
            .iter()
            .find(|feature| feature.metadata.doc_id == surface.surface_id),
        WorthBackfillDocKind::Boundary => scan
            .boundaries
            .iter()
            .find(|boundary| boundary.metadata.doc_id == surface.surface_id),
    };

    let mut evidence = WorthDocsInvariantEvidence::default();
    let (status, reason) = if let Some(doc) = doc {
        if doc.relative_path != surface.relative_path {
            evidence.set_actual_relative_path(doc.relative_path.clone());
        }
        if !scan.readme.markdown.contains(surface.readme_link_path) {
            evidence.push_missing_readme_fragment(surface.readme_link_path);
        }
        if !doc.markdown.contains(surface.required_jump_link) {
            evidence.push_missing_markdown_fragment(surface.required_jump_link);
        }
        if !doc.headings.contains("Related Docs") {
            evidence.push_missing_heading("Related Docs");
        }
        if let Some(reason) = evidence.first_problem() {
            (WorthDocsBackfillStatus::Blocked, reason)
        } else {
            (
                WorthDocsBackfillStatus::Satisfied,
                "older public surface has one owning doc and README graph exposure".to_string(),
            )
        }
    } else {
        (
            WorthDocsBackfillStatus::Blocked,
            "owning doc is missing".to_string(),
        )
    };

    Ok(build_row(surface.surface_id, status, reason, evidence))
}

fn build_row(
    surface_name: &str,
    status: WorthDocsBackfillStatus,
    reason: String,
    evidence: WorthDocsInvariantEvidence,
) -> WorthDocsBackfillRow {
    WorthDocsBackfillRow {
        surface_name: surface_name.to_string(),
        row_digest: digest_lines(&[format!("{surface_name}|{status:?}|{reason}")]),
        reason,
        status,
        evidence,
    }
}

fn missing_scan(crate_name: &str) -> WorthDocsCloseoutError {
    WorthDocsCloseoutError::new(
        WorthDocsCloseoutErrorKind::TopologyDrift,
        None,
        format!("missing crate scan for `{crate_name}`"),
    )
}
