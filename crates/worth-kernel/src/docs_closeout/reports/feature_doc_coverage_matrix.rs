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
pub enum WorthFeatureDocCoverageStatus {
    Satisfied,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthFeatureDocCoverageRow {
    feature_id: String,
    crate_name: String,
    status: WorthFeatureDocCoverageStatus,
    reason: String,
    row_digest: String,
    evidence: WorthDocsInvariantEvidence,
}

impl WorthFeatureDocCoverageRow {
    pub fn feature_id(&self) -> &str {
        &self.feature_id
    }

    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }

    pub fn status(&self) -> WorthFeatureDocCoverageStatus {
        self.status
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn evidence(&self) -> &WorthDocsInvariantEvidence {
        &self.evidence
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthFeatureDocCoverageMatrix {
    rows: Vec<WorthFeatureDocCoverageRow>,
    coverage_matrix_digest: String,
}

impl WorthFeatureDocCoverageMatrix {
    pub fn rows(&self) -> &[WorthFeatureDocCoverageRow] {
        &self.rows
    }

    pub fn coverage_matrix_digest(&self) -> &str {
        &self.coverage_matrix_digest
    }
}

pub fn current_worth_feature_doc_coverage_matrix(
) -> Result<WorthFeatureDocCoverageMatrix, WorthDocsCloseoutError> {
    let scans = scan_all_touched_crates()?;
    build_feature_doc_coverage_matrix(scans)
}

pub fn worth_feature_doc_coverage_matrix_for_root(
    workspace_root: &Path,
) -> Result<WorthFeatureDocCoverageMatrix, WorthDocsCloseoutError> {
    let scans = scan_all_touched_crates_for_root(workspace_root)?;
    build_feature_doc_coverage_matrix(scans)
}

fn build_feature_doc_coverage_matrix(
    scans: Vec<WorthDocsCrateScan>,
) -> Result<WorthFeatureDocCoverageMatrix, WorthDocsCloseoutError> {
    let mut rows = Vec::new();
    let mut ownership = BTreeMap::<String, usize>::new();

    for scan in &scans {
        for feature in &scan.features {
            *ownership
                .entry(feature.metadata.doc_id.clone())
                .or_insert(0) += 1;
        }
    }

    for scan in scans {
        let expected = expectation(&scan.crate_name);
        for feature_id in expected.expected_feature_ids {
            let row = if let Some(feature) = scan
                .features
                .iter()
                .find(|feature| feature.metadata.doc_id == *feature_id)
            {
                validate_feature_row(&scan.crate_name, feature_id, feature, &ownership)?
            } else {
                blocked_row(
                    &scan.crate_name,
                    feature_id,
                    "feature doc is missing".to_string(),
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
            "no feature coverage rows were produced",
        ));
    }

    Ok(WorthFeatureDocCoverageMatrix {
        coverage_matrix_digest: digest_lines(
            &rows
                .iter()
                .map(|row| {
                    format!(
                        "{}|{}|{:?}|{}|{}",
                        row.crate_name, row.feature_id, row.status, row.reason, row.row_digest
                    )
                })
                .collect::<Vec<_>>(),
        ),
        rows,
    })
}

fn validate_feature_row(
    crate_name: &str,
    feature_id: &str,
    feature: &WorthDocFile,
    ownership: &BTreeMap<String, usize>,
) -> Result<WorthFeatureDocCoverageRow, WorthDocsCloseoutError> {
    let required_headings = [
        "What This Feature Is",
        "Why You Use It",
        "Stable Entry Points",
        "Common Path",
        "Advanced Path",
        "Inspection And Debugging",
        "Anti-Patterns",
        "Current Limits",
        "Related Docs",
    ];
    let mut evidence = WorthDocsInvariantEvidence::default();
    let ownership_count = ownership.get(feature_id).copied().unwrap_or_default();
    evidence.set_ownership_count(ownership_count);
    if feature.relative_path != format!("features/{feature_id}.md") {
        evidence.set_actual_relative_path(feature.relative_path.clone());
    }
    for heading in required_headings
        .iter()
        .filter(|heading| !feature.headings.contains(**heading))
    {
        evidence.push_missing_heading(*heading);
    }
    if feature.metadata.query_integration_required
        && !feature.headings.contains("Query Integration")
    {
        evidence.push_missing_heading("Query Integration");
    }
    if feature.metadata.query_proof_required {
        for term in ["evidence-report", "hard-prohibition", "support pin"] {
            if !feature.markdown.contains(term) {
                evidence.push_missing_markdown_fragment(term);
            }
        }
    }
    Ok(if let Some(reason) = blocked_reason(&evidence) {
        blocked_row(crate_name, feature_id, reason, evidence)
    } else {
        satisfied_row(
            crate_name,
            feature_id,
            "feature doc owns one shipped surface with explicit workflow headings".to_string(),
            evidence,
        )
    })
}

fn blocked_reason(evidence: &WorthDocsInvariantEvidence) -> Option<String> {
    if evidence.ownership_count().unwrap_or_default() != 1 {
        return evidence
            .first_problem()
            .or_else(|| Some("feature ownership is not unique".to_string()));
    }
    evidence.first_problem()
}

fn satisfied_row(
    crate_name: &str,
    feature_id: &str,
    reason: String,
    evidence: WorthDocsInvariantEvidence,
) -> WorthFeatureDocCoverageRow {
    build_row(
        crate_name,
        feature_id,
        WorthFeatureDocCoverageStatus::Satisfied,
        reason,
        evidence,
    )
}

fn blocked_row(
    crate_name: &str,
    feature_id: &str,
    reason: String,
    evidence: WorthDocsInvariantEvidence,
) -> WorthFeatureDocCoverageRow {
    build_row(
        crate_name,
        feature_id,
        WorthFeatureDocCoverageStatus::Blocked,
        reason,
        evidence,
    )
}

fn build_row(
    crate_name: &str,
    feature_id: &str,
    status: WorthFeatureDocCoverageStatus,
    reason: String,
    evidence: WorthDocsInvariantEvidence,
) -> WorthFeatureDocCoverageRow {
    let row_digest = digest_lines(&[format!("{crate_name}|{feature_id}|{status:?}|{reason}")]);
    WorthFeatureDocCoverageRow {
        feature_id: feature_id.to_string(),
        crate_name: crate_name.to_string(),
        status,
        reason,
        row_digest,
        evidence,
    }
}
