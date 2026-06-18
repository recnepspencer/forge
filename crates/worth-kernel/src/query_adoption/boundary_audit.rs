#[cfg(test)]
use forge_query::facade::consumer_kit::hard_prohibition_boundary_audit_coverage;
use forge_query::facade::consumer_kit::{
    hard_prohibition_boundary_audit, ForgeQueryBoundaryAuditError, ForgeQueryBoundaryAuditReport,
    ForgeQueryBoundaryAuditSourceSet,
};

pub(super) fn kernel_query_hard_prohibition_boundary_audit(
) -> Result<ForgeQueryBoundaryAuditReport, ForgeQueryBoundaryAuditError> {
    hard_prohibition_boundary_audit()
        .covering_sources(kernel_query_boundary_sources())
        .evaluate()
}

#[cfg(test)]
fn query_hard_prohibition_registry_row_count() -> usize {
    hard_prohibition_boundary_audit_coverage().rows().len()
}

fn kernel_query_boundary_sources() -> ForgeQueryBoundaryAuditSourceSet {
    ForgeQueryBoundaryAuditSourceSet::new("worth-kernel")
        .source_file(
            "worth-kernel query adoption support pins",
            "crates/worth-kernel/src/query_adoption/support_pins.rs",
            include_str!("support_pins.rs"),
        )
        .source_file(
            "worth-kernel query adoption evidence reports",
            "crates/worth-kernel/src/query_adoption/evidence_reports.rs",
            include_str!("evidence_reports.rs"),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kernel_boundary_audit_uses_query_owned_hard_prohibition_registry() {
        let report = kernel_query_hard_prohibition_boundary_audit()
            .expect("Query hard-prohibition audit should evaluate");

        assert!(report.is_clean());
        assert_eq!(report.crate_name(), "worth-kernel");
        assert_eq!(report.source_labels().len(), 2);
        assert_eq!(
            report.coverage_rows().len(),
            query_hard_prohibition_registry_row_count()
        );
        assert!(!report
            .report_identity()
            .terminal_projection_for_reporting()
            .is_empty());
    }
}
