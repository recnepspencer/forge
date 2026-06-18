use std::fmt;

use super::finding::ForgeQueryBoundaryAuditFinding;
use super::report::ForgeQueryBoundaryAuditReport;
use crate::ForgeQueryEvidenceIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBoundaryAuditFailure {
    report: ForgeQueryBoundaryAuditReport,
}

impl ForgeQueryBoundaryAuditFailure {
    pub(crate) fn from_report(report: ForgeQueryBoundaryAuditReport) -> Self {
        Self { report }
    }

    pub fn report(&self) -> &ForgeQueryBoundaryAuditReport {
        &self.report
    }

    pub fn findings(&self) -> &[ForgeQueryBoundaryAuditFinding] {
        self.report.findings()
    }

    pub fn report_identity(&self) -> &ForgeQueryEvidenceIdentity {
        self.report.report_identity()
    }
}

impl fmt::Display for ForgeQueryBoundaryAuditFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hard prohibition boundary audit found {} prohibited seam usage finding(s)",
            self.findings().len()
        )
    }
}

impl std::error::Error for ForgeQueryBoundaryAuditFailure {}
