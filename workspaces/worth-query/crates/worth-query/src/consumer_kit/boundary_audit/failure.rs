use std::fmt;

use super::finding::WorthQueryBoundaryAuditFinding;
use super::report::WorthQueryBoundaryAuditReport;
use crate::WorthQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBoundaryAuditFailure {
    report: WorthQueryBoundaryAuditReport,
}

impl WorthQueryBoundaryAuditFailure {
    pub(crate) fn from_report(report: WorthQueryBoundaryAuditReport) -> Self {
        Self { report }
    }

    pub fn report(&self) -> &WorthQueryBoundaryAuditReport {
        &self.report
    }

    pub fn findings(&self) -> &[WorthQueryBoundaryAuditFinding] {
        self.report.findings()
    }

    pub fn report_identity(&self) -> &WorthQueryEvidenceIdentity {
        self.report.report_identity()
    }
}

impl fmt::Display for WorthQueryBoundaryAuditFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hard prohibition boundary audit found {} prohibited seam usage finding(s)",
            self.findings().len()
        )
    }
}

impl std::error::Error for WorthQueryBoundaryAuditFailure {}
