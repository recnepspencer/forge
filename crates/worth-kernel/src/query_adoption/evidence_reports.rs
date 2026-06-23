use forge_query::facade::consumer_kit::{
    EvidenceReport, EvidenceReportDeclaration, EvidenceReportError, EvidenceReportScope,
    ForgeQueryBoundaryAuditReport, ForgeQuerySupportPinReport,
};

pub(super) fn kernel_query_adoption_evidence_report(
    support_report: &ForgeQuerySupportPinReport,
    boundary_report: &ForgeQueryBoundaryAuditReport,
) -> Result<EvidenceReport, EvidenceReportError> {
    EvidenceReportDeclaration::new(
        EvidenceReportScope::new("worth-kernel.query-adoption.phase-three")?,
        "worth-kernel-query-consumer-kit-adoption",
    )?
    .shape_participating("crate", "worth-kernel")?
    .value_participating("support_pin_report_digest", support_report.report_digest())?
    .usize_participating(
        "support_requirement_count",
        support_report.requirement_count(),
    )?
    .value_participating(
        "boundary_audit_report_identity",
        boundary_report
            .report_identity()
            .terminal_projection_for_reporting(),
    )?
    .usize_participating(
        "boundary_audit_coverage_rows",
        boundary_report.coverage_rows().len(),
    )?
    .bool_participating("hard_prohibition_audit_clean", boundary_report.is_clean())?
    .diagnostic_value_nonparticipating(
        "authority_boundary",
        "kernel orchestration derives from lower-crate receipts and Query evidence",
    )?
    .seal()
}

#[cfg(test)]
mod tests {
    use forge_query::facade::consumer_kit::EvidenceReportFieldParticipation;

    use super::super::boundary_audit::kernel_query_hard_prohibition_boundary_audit;
    use super::super::support_pins::evaluate_current_kernel_support_pins;
    use super::*;

    #[test]
    fn kernel_evidence_report_derives_digest_participation_through_query_api() {
        let support_report = evaluate_current_kernel_support_pins().expect("support pin report");
        let boundary_report = kernel_query_hard_prohibition_boundary_audit().expect("audit report");
        let report = kernel_query_adoption_evidence_report(&support_report, &boundary_report)
            .expect("Query evidence report should seal");

        assert_eq!(
            report.field("crate").expect("crate field").participation(),
            EvidenceReportFieldParticipation::Participating
        );
        assert_eq!(report.indexed_field_count(), 7);
        assert!(!report
            .report_identity()
            .terminal_projection_for_reporting()
            .is_empty());
        assert!(!report
            .digest_participation_identity()
            .terminal_projection_for_reporting()
            .is_empty());
        assert_ne!(
            report.report_identity().terminal_projection_for_reporting(),
            support_report.report_digest()
        );
    }
}
