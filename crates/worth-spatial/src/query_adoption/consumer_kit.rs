use forge_query::facade::consumer_kit::{
    EvidenceReportError, ForgeQueryBoundaryAuditError, ForgeQuerySupportPinningError,
};

use super::boundary_audit::spatial_query_hard_prohibition_boundary_audit;
use super::evidence_reports::spatial_query_adoption_evidence_report;
use super::support_projection::{
    current_spatial_support_snapshot, evaluate_current_spatial_support_pins,
    spatial_workload_support_pin_rows,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthSpatialQueryConsumerKitAdoptionStatus {
    support_pin_report_digest: String,
    support_requirement_count: usize,
    support_observed_row_count: usize,
    support_matched_required_count: usize,
    support_snapshot_row_count: usize,
    support_blocking_finding_count: usize,
    evidence_report_identity: String,
    evidence_digest_participation_identity: String,
    boundary_audit_report_identity: String,
    boundary_audit_source_count: usize,
    boundary_audit_coverage_row_count: usize,
    workload_support_pin_row_count: usize,
    hard_prohibition_audit_clean: bool,
}

impl WorthSpatialQueryConsumerKitAdoptionStatus {
    pub fn support_pin_report_digest(&self) -> &str {
        &self.support_pin_report_digest
    }

    pub const fn support_requirement_count(&self) -> usize {
        self.support_requirement_count
    }

    pub const fn support_observed_row_count(&self) -> usize {
        self.support_observed_row_count
    }

    pub const fn support_matched_required_count(&self) -> usize {
        self.support_matched_required_count
    }

    pub const fn support_snapshot_row_count(&self) -> usize {
        self.support_snapshot_row_count
    }

    pub const fn support_blocking_finding_count(&self) -> usize {
        self.support_blocking_finding_count
    }

    pub fn evidence_report_identity(&self) -> &str {
        &self.evidence_report_identity
    }

    pub fn evidence_digest_participation_identity(&self) -> &str {
        &self.evidence_digest_participation_identity
    }

    pub fn boundary_audit_report_identity(&self) -> &str {
        &self.boundary_audit_report_identity
    }

    pub const fn boundary_audit_source_count(&self) -> usize {
        self.boundary_audit_source_count
    }

    pub const fn boundary_audit_coverage_row_count(&self) -> usize {
        self.boundary_audit_coverage_row_count
    }

    pub const fn workload_support_pin_row_count(&self) -> usize {
        self.workload_support_pin_row_count
    }

    pub const fn hard_prohibition_audit_clean(&self) -> bool {
        self.hard_prohibition_audit_clean
    }
}

#[derive(Debug)]
pub enum WorthSpatialQueryConsumerKitAdoptionError {
    SupportPinning(ForgeQuerySupportPinningError),
    BoundaryAudit(ForgeQueryBoundaryAuditError),
    EvidenceReport(EvidenceReportError),
}

pub fn current_spatial_query_consumer_kit_adoption_status(
) -> Result<WorthSpatialQueryConsumerKitAdoptionStatus, WorthSpatialQueryConsumerKitAdoptionError> {
    let support_report = evaluate_current_spatial_support_pins()
        .map_err(WorthSpatialQueryConsumerKitAdoptionError::SupportPinning)?;
    support_report
        .assert_satisfied()
        .map_err(WorthSpatialQueryConsumerKitAdoptionError::SupportPinning)?;
    let boundary_report = spatial_query_hard_prohibition_boundary_audit()
        .map_err(WorthSpatialQueryConsumerKitAdoptionError::BoundaryAudit)?;
    boundary_report.assert_clean();
    let snapshot = current_spatial_support_snapshot();
    let workload_support_rows = spatial_workload_support_pin_rows(&snapshot, &support_report);
    let evidence_report = spatial_query_adoption_evidence_report(
        &support_report,
        &boundary_report,
        &workload_support_rows,
    )
    .map_err(WorthSpatialQueryConsumerKitAdoptionError::EvidenceReport)?;

    Ok(WorthSpatialQueryConsumerKitAdoptionStatus {
        support_pin_report_digest: support_report.report_digest().to_string(),
        support_requirement_count: support_report.requirement_count(),
        support_observed_row_count: support_report.observed_count(),
        support_matched_required_count: support_report.matched_required_count(),
        support_snapshot_row_count: support_report.snapshot_row_count(),
        support_blocking_finding_count: support_report.blocking_finding_count(),
        evidence_report_identity: evidence_report
            .report_identity()
            .terminal_projection_for_reporting()
            .to_string(),
        evidence_digest_participation_identity: evidence_report
            .digest_participation_identity()
            .terminal_projection_for_reporting()
            .to_string(),
        boundary_audit_report_identity: boundary_report
            .report_identity()
            .terminal_projection_for_reporting()
            .to_string(),
        boundary_audit_source_count: boundary_report.source_labels().len(),
        boundary_audit_coverage_row_count: boundary_report.coverage_rows().len(),
        workload_support_pin_row_count: workload_support_rows.len(),
        hard_prohibition_audit_clean: boundary_report.is_clean(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spatial_public_status_exposes_adoption_status_without_query_internals() {
        let status = current_spatial_query_consumer_kit_adoption_status()
            .expect("spatial Query consumer-kit adoption should be current");

        assert_eq!(status.support_requirement_count(), 3);
        assert_eq!(status.support_observed_row_count(), 1);
        assert_eq!(status.support_matched_required_count(), 3);
        assert_eq!(status.support_snapshot_row_count(), 22);
        assert_eq!(status.support_blocking_finding_count(), 0);
        assert_eq!(status.boundary_audit_source_count(), 2);
        assert!(status.boundary_audit_coverage_row_count() > 0);
        assert_eq!(status.workload_support_pin_row_count(), 7);
        assert!(status.hard_prohibition_audit_clean());
        assert!(!status.support_pin_report_digest().is_empty());
        assert!(!status.evidence_report_identity().is_empty());
        assert!(!status.evidence_digest_participation_identity().is_empty());
        assert!(!status.boundary_audit_report_identity().is_empty());
    }
}
