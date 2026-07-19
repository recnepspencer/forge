use super::failure::WorthQueryBoundaryAuditFailure;
use super::finding::WorthQueryBoundaryAuditFinding;
use super::registry_coverage::WorthQueryBoundaryAuditCoverageRow;
use crate::WorthQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBoundaryAuditReport {
    crate_name: String,
    source_labels: Vec<String>,
    source_paths: Vec<Option<String>>,
    coverage_rows: Vec<WorthQueryBoundaryAuditCoverageRow>,
    findings: Vec<WorthQueryBoundaryAuditFinding>,
    coverage_identity: WorthQueryEvidenceIdentity,
    finding_identities: Vec<WorthQueryEvidenceIdentity>,
    report_identity: WorthQueryEvidenceIdentity,
    parsed_item_count: usize,
    visited_call_count: usize,
}

impl WorthQueryBoundaryAuditReport {
    pub(crate) fn sealed(
        crate_name: String,
        source_labels: Vec<String>,
        source_paths: Vec<Option<String>>,
        coverage_rows: Vec<WorthQueryBoundaryAuditCoverageRow>,
        findings: Vec<WorthQueryBoundaryAuditFinding>,
        coverage_identity: WorthQueryEvidenceIdentity,
        finding_identities: Vec<WorthQueryEvidenceIdentity>,
        report_identity: WorthQueryEvidenceIdentity,
        parsed_item_count: usize,
        visited_call_count: usize,
    ) -> Self {
        Self {
            crate_name,
            source_labels,
            source_paths,
            coverage_rows,
            findings,
            coverage_identity,
            finding_identities,
            report_identity,
            parsed_item_count,
            visited_call_count,
        }
    }

    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }

    pub fn source_labels(&self) -> &[String] {
        &self.source_labels
    }

    pub fn source_paths(&self) -> &[Option<String>] {
        &self.source_paths
    }

    pub fn coverage_rows(&self) -> &[WorthQueryBoundaryAuditCoverageRow] {
        &self.coverage_rows
    }

    pub fn findings(&self) -> &[WorthQueryBoundaryAuditFinding] {
        &self.findings
    }

    pub fn coverage_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.coverage_identity
    }

    pub fn finding_identities(&self) -> &[WorthQueryEvidenceIdentity] {
        &self.finding_identities
    }

    pub fn report_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.report_identity
    }

    pub fn parsed_item_count(&self) -> usize {
        self.parsed_item_count
    }

    pub fn visited_call_count(&self) -> usize {
        self.visited_call_count
    }

    pub fn is_clean(&self) -> bool {
        self.findings.is_empty()
    }

    pub fn try_assert_clean(&self) -> Result<Self, WorthQueryBoundaryAuditFailure> {
        self.is_clean()
            .then(|| self.clone())
            .ok_or_else(|| WorthQueryBoundaryAuditFailure::from_report(self.clone()))
    }

    pub fn assert_clean(&self) {
        self.try_assert_clean()
            .expect("hard prohibition boundary audit found prohibited seam usage");
    }
}
