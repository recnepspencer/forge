use forge_query::facade::consumer_kit::ForgeQueryGraphReadBypassAdoptionProof;

use super::required_root_coverage::WorthGraphReadBypassRequiredRootCoverage;
use super::residue_report_row::WorthGraphReadBypassResidueReportRow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadBypassAdoptionReport {
    covered_roots: Vec<String>,
    required_root_coverage: Vec<WorthGraphReadBypassRequiredRootCoverage>,
    audited_source_labels: Vec<String>,
    source_inventory_identity: String,
    source_inventory_count: usize,
    evaluated_source_count: usize,
    finding_count: usize,
    skipped_empty_source_count: usize,
    residue_rows: Vec<WorthGraphReadBypassResidueReportRow>,
    residue_manifest_digest: String,
    residue_certified_finding_count: usize,
    unclassified_finding_count: usize,
    adoption_manifest_digest: String,
}

impl WorthGraphReadBypassAdoptionReport {
    pub(in crate::graph_read_access_inventory::inventory_lane) fn from_query_adoption(
        adoption: ForgeQueryGraphReadBypassAdoptionProof,
        covered_roots: Vec<String>,
        required_root_coverage: Vec<WorthGraphReadBypassRequiredRootCoverage>,
        audited_source_labels: Vec<String>,
        source_inventory_identity: String,
        source_inventory_count: usize,
        evaluated_source_count: usize,
        finding_count: usize,
        skipped_empty_source_count: usize,
    ) -> Self {
        let residue_rows = adoption
            .residue_manifest()
            .rows()
            .iter()
            .map(WorthGraphReadBypassResidueReportRow::from_query_row)
            .collect();

        Self {
            covered_roots,
            required_root_coverage,
            audited_source_labels,
            source_inventory_identity,
            source_inventory_count,
            evaluated_source_count,
            finding_count,
            skipped_empty_source_count,
            residue_rows,
            residue_manifest_digest: adoption.residue_manifest().manifest_digest().to_string(),
            residue_certified_finding_count: adoption
                .residue_certification()
                .certified_finding_count(),
            unclassified_finding_count: adoption.unclassified_finding_count(),
            adoption_manifest_digest: adoption.manifest().manifest_digest().to_string(),
        }
    }

    pub fn covered_roots(&self) -> &[String] {
        &self.covered_roots
    }

    pub fn required_root_coverage(&self) -> &[WorthGraphReadBypassRequiredRootCoverage] {
        &self.required_root_coverage
    }

    pub fn audited_source_labels(&self) -> &[String] {
        &self.audited_source_labels
    }

    pub fn source_inventory_identity(&self) -> &str {
        &self.source_inventory_identity
    }

    pub const fn source_inventory_count(&self) -> usize {
        self.source_inventory_count
    }

    pub const fn evaluated_source_count(&self) -> usize {
        self.evaluated_source_count
    }

    pub const fn finding_count(&self) -> usize {
        self.finding_count
    }

    pub const fn skipped_empty_source_count(&self) -> usize {
        self.skipped_empty_source_count
    }

    pub fn residue_manifest_digest(&self) -> &str {
        &self.residue_manifest_digest
    }

    pub fn residue_rows(&self) -> &[WorthGraphReadBypassResidueReportRow] {
        &self.residue_rows
    }

    pub const fn residue_certified_finding_count(&self) -> usize {
        self.residue_certified_finding_count
    }

    pub const fn unclassified_finding_count(&self) -> usize {
        self.unclassified_finding_count
    }

    pub fn adoption_manifest_digest(&self) -> &str {
        &self.adoption_manifest_digest
    }
}
