use forge_query::facade::consumer_kit::{
    EvidenceReport, EvidenceReportDeclaration, EvidenceReportScope,
};

use super::error::{EvidenceLookupQueryConsumerKitError, EvidenceLookupQueryConsumerKitErrorKind};

pub(crate) struct EvidenceLookupQueryConsumerKitEvidence {
    report: EvidenceReport,
}

impl EvidenceLookupQueryConsumerKitEvidence {
    pub(crate) fn declare(
        matrix_digest: &str,
        support_snapshot_digest: &str,
        support_pin_report_digest: &str,
        boundary_audit_coverage_identity: &str,
        boundary_audit_identity: &str,
        residue_report_identity: &str,
    ) -> Result<Self, EvidenceLookupQueryConsumerKitError> {
        let scope = EvidenceReportScope::new("worth-spatial.evidence-lookup-query-consumer-kit")
            .map_err(|error| {
                EvidenceLookupQueryConsumerKitError::new(
                    EvidenceLookupQueryConsumerKitErrorKind::EvidenceReport,
                    format!("{error:?}"),
                )
            })?;
        let report = EvidenceReportDeclaration::new(scope, "phase-ten-closeout")
            .and_then(|report| report.value_participating("matrix_digest", matrix_digest))
            .and_then(|report| {
                report.value_participating("support_snapshot_digest", support_snapshot_digest)
            })
            .and_then(|report| {
                report.value_participating("support_pin_report_digest", support_pin_report_digest)
            })
            .and_then(|report| {
                report.value_participating(
                    "boundary_audit_coverage_identity",
                    boundary_audit_coverage_identity,
                )
            })
            .and_then(|report| {
                report.value_participating("boundary_audit_identity", boundary_audit_identity)
            })
            .and_then(|report| {
                report.value_participating("residue_report_identity", residue_report_identity)
            })
            .and_then(EvidenceReportDeclaration::seal)
            .map_err(|error| {
                EvidenceLookupQueryConsumerKitError::new(
                    EvidenceLookupQueryConsumerKitErrorKind::EvidenceReport,
                    format!("{error:?}"),
                )
            })?;
        Ok(Self { report })
    }

    pub(crate) fn report_identity(&self) -> &str {
        self.report
            .report_identity()
            .terminal_projection_for_reporting()
    }

    pub(crate) fn digest_participation_identity(&self) -> &str {
        self.report
            .digest_participation_identity()
            .terminal_projection_for_reporting()
    }
}
