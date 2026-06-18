use crate::ForgeQueryEvidenceIdentity;

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
    matched_pattern: &'static str,
}

impl ForgeQueryTestBackendResidueReport {
    pub(super) fn sealed(
        consumer_name: String,
        audited_roots: Vec<String>,
        findings: Vec<ForgeQueryTestBackendResidueFinding>,
        finding_identities: Vec<ForgeQueryEvidenceIdentity>,
        report_identity: ForgeQueryEvidenceIdentity,
        scanned_file_count: usize,
    ) -> Self {
        Self {
            consumer_name,
            audited_roots,
            findings,
            finding_identities,
            report_identity,
            scanned_file_count,
        }
    }

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
}

impl ForgeQueryTestBackendResidueFinding {
    pub(super) fn discovered(
        source_path: String,
        residue_class: &'static str,
        matched_pattern: &'static str,
    ) -> Self {
        Self {
            source_path,
            residue_class,
            matched_pattern,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub fn residue_class(&self) -> &'static str {
        self.residue_class
    }

    pub fn matched_pattern(&self) -> &'static str {
        self.matched_pattern
    }
}
