use super::finding::WorthQueryEvidenceReportAdoptionFinding;
use super::source_set::WorthQueryEvidenceReportAdoptionResidueClassification;
use crate::WorthQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryEvidenceReportAdoptionResidueRow {
    source_label: String,
    source_path: Option<String>,
    symbol: String,
    classification: WorthQueryEvidenceReportAdoptionResidueClassification,
    usage_count: usize,
    row_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryEvidenceReportAdoptionResidueRow {
    pub(crate) fn sealed(
        source_label: String,
        source_path: Option<String>,
        symbol: String,
        classification: WorthQueryEvidenceReportAdoptionResidueClassification,
        usage_count: usize,
        row_identity: WorthQueryEvidenceIdentity,
    ) -> Self {
        Self {
            source_label,
            source_path,
            symbol,
            classification,
            usage_count,
            row_identity,
        }
    }

    pub fn source_label(&self) -> &str {
        &self.source_label
    }

    pub fn source_path(&self) -> Option<&str> {
        self.source_path.as_deref()
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn classification(&self) -> WorthQueryEvidenceReportAdoptionResidueClassification {
        self.classification
    }

    pub fn usage_count(&self) -> usize {
        self.usage_count
    }

    pub fn row_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.row_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryEvidenceReportAdoptionReport {
    crate_name: String,
    source_labels: Vec<String>,
    residue_rows: Vec<WorthQueryEvidenceReportAdoptionResidueRow>,
    findings: Vec<WorthQueryEvidenceReportAdoptionFinding>,
    finding_identities: Vec<WorthQueryEvidenceIdentity>,
    report_identity: WorthQueryEvidenceIdentity,
    parsed_item_count: usize,
    visited_site_count: usize,
}

impl WorthQueryEvidenceReportAdoptionReport {
    pub(crate) fn sealed(
        crate_name: String,
        source_labels: Vec<String>,
        residue_rows: Vec<WorthQueryEvidenceReportAdoptionResidueRow>,
        findings: Vec<WorthQueryEvidenceReportAdoptionFinding>,
        finding_identities: Vec<WorthQueryEvidenceIdentity>,
        report_identity: WorthQueryEvidenceIdentity,
        parsed_item_count: usize,
        visited_site_count: usize,
    ) -> Self {
        Self {
            crate_name,
            source_labels,
            residue_rows,
            findings,
            finding_identities,
            report_identity,
            parsed_item_count,
            visited_site_count,
        }
    }

    pub fn crate_name(&self) -> &str {
        &self.crate_name
    }

    pub fn source_labels(&self) -> &[String] {
        &self.source_labels
    }

    pub fn residue_rows(&self) -> &[WorthQueryEvidenceReportAdoptionResidueRow] {
        &self.residue_rows
    }

    pub fn findings(&self) -> &[WorthQueryEvidenceReportAdoptionFinding] {
        &self.findings
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

    pub fn visited_site_count(&self) -> usize {
        self.visited_site_count
    }

    pub fn assert_clean(&self) {
        assert!(
            self.findings.is_empty(),
            "evidence report adoption audit found {} violation(s)",
            self.findings.len()
        );
    }
}
