use super::finding::WorthQuerySupportPinFinding;
use crate::consumer_kit::support_pinning::error::WorthQuerySupportPinningError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySupportPinReport {
    consumer_name: String,
    contract_digest: String,
    observed_schema_identity: String,
    observed_source_matrix_digest: String,
    observed_snapshot_digest: String,
    requirement_count: usize,
    observed_count: usize,
    matched_required_count: usize,
    snapshot_row_count: usize,
    finding_count: usize,
    blocking_finding_count: usize,
    findings: Vec<WorthQuerySupportPinFinding>,
    report_digest: String,
}

impl WorthQuerySupportPinReport {
    pub(crate) fn new(
        consumer_name: String,
        contract_digest: String,
        observed_schema_identity: String,
        observed_source_matrix_digest: String,
        observed_snapshot_digest: String,
        requirement_count: usize,
        observed_count: usize,
        matched_required_count: usize,
        snapshot_row_count: usize,
        findings: Vec<WorthQuerySupportPinFinding>,
        report_digest: String,
    ) -> Self {
        let finding_count = findings.len();
        let blocking_finding_count = findings.iter().filter(|finding| finding.blocking()).count();
        Self {
            consumer_name,
            contract_digest,
            observed_schema_identity,
            observed_source_matrix_digest,
            observed_snapshot_digest,
            requirement_count,
            observed_count,
            matched_required_count,
            snapshot_row_count,
            finding_count,
            blocking_finding_count,
            findings,
            report_digest,
        }
    }

    pub fn assert_satisfied(&self) -> Result<(), WorthQuerySupportPinningError> {
        if self.blocking_finding_count == 0 {
            Ok(())
        } else {
            let blocking_findings = self
                .findings
                .iter()
                .filter(|finding| finding.blocking())
                .cloned()
                .collect::<Vec<_>>();
            Err(WorthQuerySupportPinningError::with_blocking_findings(
                self.consumer_name.clone(),
                self.report_digest.clone(),
                blocking_findings,
            ))
        }
    }

    pub fn satisfied(&self) -> bool {
        self.blocking_finding_count == 0
    }

    pub fn consumer_name(&self) -> &str {
        &self.consumer_name
    }

    pub fn contract_digest(&self) -> &str {
        &self.contract_digest
    }

    pub fn observed_schema_identity(&self) -> &str {
        &self.observed_schema_identity
    }

    pub fn observed_source_matrix_digest(&self) -> &str {
        &self.observed_source_matrix_digest
    }

    pub fn observed_snapshot_digest(&self) -> &str {
        &self.observed_snapshot_digest
    }

    pub fn requirement_count(&self) -> usize {
        self.requirement_count
    }

    pub fn observed_count(&self) -> usize {
        self.observed_count
    }

    pub fn matched_required_count(&self) -> usize {
        self.matched_required_count
    }

    pub fn snapshot_row_count(&self) -> usize {
        self.snapshot_row_count
    }

    pub fn finding_count(&self) -> usize {
        self.finding_count
    }

    pub fn blocking_finding_count(&self) -> usize {
        self.blocking_finding_count
    }

    pub fn findings(&self) -> &[WorthQuerySupportPinFinding] {
        &self.findings
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}
