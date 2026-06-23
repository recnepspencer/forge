use forge_query::facade::consumer_kit::{
    EvidenceReport, EvidenceReportDeclaration, EvidenceReportError, EvidenceReportScope,
    ForgeQueryBoundaryAuditReport, ForgeQuerySupportPinReport,
};

use super::support_projection::WorthSpatialWorkloadSupportPinRow;

pub(super) fn spatial_query_adoption_evidence_report(
    support_report: &ForgeQuerySupportPinReport,
    boundary_report: &ForgeQueryBoundaryAuditReport,
    workload_support_rows: &[WorthSpatialWorkloadSupportPinRow],
) -> Result<EvidenceReport, EvidenceReportError> {
    let workload_support_rows_identity =
        spatial_workload_support_pin_rows_identity(workload_support_rows);

    EvidenceReportDeclaration::new(
        EvidenceReportScope::new("worth-spatial.query-adoption.phase-six")?,
        "worth-spatial-query-consumer-kit-adoption",
    )?
    .shape_participating("crate", "worth-spatial")?
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
    .usize_participating(
        "spatial_workload_support_pin_rows",
        workload_support_rows.len(),
    )?
    .value_participating(
        "spatial_workload_support_pin_rows_identity",
        &workload_support_rows_identity,
    )?
    .bool_participating("hard_prohibition_audit_clean", boundary_report.is_clean())?
    .diagnostic_value_nonparticipating(
        "authority_boundary",
        "spatial evidence and witness truth remain spatial-owned Query consumers",
    )?
    .seal()
}

fn spatial_workload_support_pin_rows_identity(
    workload_support_rows: &[WorthSpatialWorkloadSupportPinRow],
) -> String {
    workload_support_rows
        .iter()
        .map(|row| {
            format!(
                "{}:{}:{}:{}:{}",
                row.workload_family().as_str(),
                row.query_runtime_family().as_str(),
                row.query_support_surface(),
                row.query_snapshot_row_digest(),
                row.support_pin_report_digest(),
            )
        })
        .collect::<Vec<_>>()
        .join("|")
}

#[cfg(test)]
mod tests {
    use forge_query::facade::consumer_kit::EvidenceReportFieldParticipation;

    use super::super::boundary_audit::spatial_query_hard_prohibition_boundary_audit;
    use super::super::support_projection::{
        current_spatial_support_snapshot, evaluate_current_spatial_support_pins,
        spatial_workload_support_pin_rows,
    };
    use super::*;

    #[test]
    fn spatial_evidence_report_derives_digest_participation_through_query_api() {
        let support_report = evaluate_current_spatial_support_pins().expect("support pin report");
        let boundary_report =
            spatial_query_hard_prohibition_boundary_audit().expect("audit report");
        let snapshot = current_spatial_support_snapshot();
        let workload_rows = spatial_workload_support_pin_rows(&snapshot, &support_report);
        let report = spatial_query_adoption_evidence_report(
            &support_report,
            &boundary_report,
            &workload_rows,
        )
        .expect("Query evidence report should seal");

        assert_eq!(
            report
                .field("support_pin_report_digest")
                .expect("support digest field")
                .participation(),
            EvidenceReportFieldParticipation::Participating
        );
        assert_eq!(
            report
                .field("spatial_workload_support_pin_rows_identity")
                .expect("workload support row identity")
                .participation(),
            EvidenceReportFieldParticipation::Participating
        );
        assert_eq!(report.indexed_field_count(), 9);
        assert!(!report
            .digest_participation_identity()
            .terminal_projection_for_reporting()
            .is_empty());
    }
}
