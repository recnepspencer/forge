use crate::WorthQueryEvidenceIdentity;

use super::error::WorthQueryGraphReadBypassResidueError;
use super::evidence::derive_graph_read_bypass_report_residue_certification_identity;
use super::finding::WorthQueryGraphReadBypassFinding;
use super::registry::WorthQueryGraphReadBypassClass;
use super::residue::WorthQueryGraphReadBypassResidueManifest;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassCounters {
    evaluated_source_count: usize,
    finding_count: usize,
    skipped_empty_source_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassReport {
    consumer_name: String,
    audited_source_labels: Vec<String>,
    source_inventory_identities: Vec<WorthQueryEvidenceIdentity>,
    findings: Vec<WorthQueryGraphReadBypassFinding>,
    finding_identities: Vec<WorthQueryEvidenceIdentity>,
    report_identity: WorthQueryEvidenceIdentity,
    counters: WorthQueryGraphReadBypassCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphReadBypassReportResidueCertification {
    report_identity: WorthQueryEvidenceIdentity,
    residue_manifest_digest: String,
    certified_finding_count: usize,
    certification_identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryGraphReadBypassCounters {
    pub(crate) fn new(
        evaluated_source_count: usize,
        finding_count: usize,
        skipped_empty_source_count: usize,
    ) -> Self {
        Self {
            evaluated_source_count,
            finding_count,
            skipped_empty_source_count,
        }
    }

    pub fn evaluated_source_count(&self) -> usize {
        self.evaluated_source_count
    }

    pub fn finding_count(&self) -> usize {
        self.finding_count
    }

    pub fn skipped_empty_source_count(&self) -> usize {
        self.skipped_empty_source_count
    }
}

impl WorthQueryGraphReadBypassReport {
    pub(crate) fn sealed(
        consumer_name: String,
        audited_source_labels: Vec<String>,
        source_inventory_identities: Vec<WorthQueryEvidenceIdentity>,
        findings: Vec<WorthQueryGraphReadBypassFinding>,
        finding_identities: Vec<WorthQueryEvidenceIdentity>,
        report_identity: WorthQueryEvidenceIdentity,
        counters: WorthQueryGraphReadBypassCounters,
    ) -> Self {
        Self {
            consumer_name,
            audited_source_labels,
            source_inventory_identities,
            findings,
            finding_identities,
            report_identity,
            counters,
        }
    }

    pub fn consumer_name(&self) -> &str {
        &self.consumer_name
    }

    pub fn audited_source_labels(&self) -> &[String] {
        &self.audited_source_labels
    }

    pub fn source_inventory_identities(&self) -> &[WorthQueryEvidenceIdentity] {
        &self.source_inventory_identities
    }

    pub fn findings(&self) -> &[WorthQueryGraphReadBypassFinding] {
        &self.findings
    }

    pub fn finding_identities(&self) -> &[WorthQueryEvidenceIdentity] {
        &self.finding_identities
    }

    pub fn report_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.report_identity
    }

    pub fn counters(&self) -> &WorthQueryGraphReadBypassCounters {
        &self.counters
    }

    pub fn is_clean(&self) -> bool {
        self.findings.is_empty()
    }

    pub fn finding_count_for_class(&self, class: WorthQueryGraphReadBypassClass) -> usize {
        self.findings
            .iter()
            .filter(|finding| finding.class() == class)
            .count()
    }

    pub fn certify_with_residue(
        &self,
        manifest: &WorthQueryGraphReadBypassResidueManifest,
    ) -> Result<
        WorthQueryGraphReadBypassReportResidueCertification,
        WorthQueryGraphReadBypassResidueError,
    > {
        for finding in &self.findings {
            let manifest_count = manifest.current_count_for_class(finding.class());
            let required_count = self.finding_count_for_class(finding.class());
            if manifest_count < required_count {
                return Err(WorthQueryGraphReadBypassResidueError::coverage_shortfall(
                    finding.class(),
                    manifest_count,
                    required_count,
                ));
            }
        }
        let certified_finding_count = self.findings.len();
        let certification_identity = derive_graph_read_bypass_report_residue_certification_identity(
            &self.report_identity,
            manifest.manifest_digest(),
            certified_finding_count,
        );
        Ok(WorthQueryGraphReadBypassReportResidueCertification {
            report_identity: self.report_identity.clone(),
            residue_manifest_digest: manifest.manifest_digest().to_string(),
            certified_finding_count,
            certification_identity,
        })
    }
}

impl WorthQueryGraphReadBypassReportResidueCertification {
    pub fn report_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.report_identity
    }

    pub fn residue_manifest_digest(&self) -> &str {
        &self.residue_manifest_digest
    }

    pub fn certified_finding_count(&self) -> usize {
        self.certified_finding_count
    }

    pub fn certification_identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.certification_identity
    }
}
