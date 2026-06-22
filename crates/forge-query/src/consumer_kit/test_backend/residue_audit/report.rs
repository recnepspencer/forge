use crate::ForgeQueryEvidenceIdentity;

use crate::consumer_kit::consumer_residue::{
    ForgeQueryConsumerResidueFinding, ForgeQueryConsumerResidueReport,
};

use super::evidence::{
    derive_test_backend_residue_finding_identity, derive_test_backend_residue_report_identity,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryTestBackendResidueReport {
    consumer_name: String,
    audited_roots: Vec<String>,
    findings: Vec<ForgeQueryTestBackendResidueFinding>,
    finding_identities: Vec<ForgeQueryEvidenceIdentity>,
    report_identity: ForgeQueryEvidenceIdentity,
    scanned_file_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryTestBackendResidueFinding {
    source_path: String,
    residue_class: &'static str,
    matched_pattern: String,
}

impl ForgeQueryTestBackendResidueReport {
    pub fn consumer_name(&self) -> &str {
        &self.consumer_name
    }

    pub fn audited_roots(&self) -> &[String] {
        &self.audited_roots
    }

    pub fn findings(&self) -> &[ForgeQueryTestBackendResidueFinding] {
        &self.findings
    }

    pub fn finding_count(&self) -> usize {
        self.findings.len()
    }

    pub fn finding_identities(&self) -> &[ForgeQueryEvidenceIdentity] {
        &self.finding_identities
    }

    pub fn report_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.report_identity
    }

    pub fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub fn assert_clean(&self) {
        assert!(
            self.findings.is_empty(),
            "Query test backend residue audit found forbidden consumer residue: {:?}",
            self.findings
        );
    }

    pub(crate) fn from_consumer_residue_report(report: ForgeQueryConsumerResidueReport) -> Self {
        let findings = report
            .findings()
            .iter()
            .map(ForgeQueryTestBackendResidueFinding::from_consumer_residue_finding)
            .collect::<Vec<_>>();
        let finding_identities = findings
            .iter()
            .map(derive_test_backend_residue_finding_identity)
            .collect::<Vec<_>>();
        let report_identity = derive_test_backend_residue_report_identity(
            report.consumer_name(),
            report.audited_roots(),
            report.scanned_file_count(),
            &finding_identities,
        );
        Self {
            consumer_name: report.consumer_name().to_string(),
            audited_roots: report.audited_roots().to_vec(),
            findings,
            finding_identities,
            report_identity,
            scanned_file_count: report.scanned_file_count(),
        }
    }
}

impl ForgeQueryTestBackendResidueFinding {
    fn from_consumer_residue_finding(finding: &ForgeQueryConsumerResidueFinding) -> Self {
        Self {
            source_path: finding.source_path().to_string(),
            residue_class: finding.residue_class().as_str(),
            matched_pattern: finding.matched_pattern().to_string(),
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn residue_class(&self) -> &'static str {
        self.residue_class
    }

    pub fn matched_pattern(&self) -> &str {
        &self.matched_pattern
    }
}
