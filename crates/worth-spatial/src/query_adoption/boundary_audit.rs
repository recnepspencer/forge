#[cfg(test)]
use forge_query::facade::consumer_kit::hard_prohibition_boundary_audit_coverage;
use forge_query::facade::consumer_kit::{
    hard_prohibition_boundary_audit, ForgeQueryBoundaryAuditError, ForgeQueryBoundaryAuditReport,
    ForgeQueryBoundaryAuditSourceSet,
};

pub(super) fn spatial_query_hard_prohibition_boundary_audit(
) -> Result<ForgeQueryBoundaryAuditReport, ForgeQueryBoundaryAuditError> {
    hard_prohibition_boundary_audit()
        .covering_sources(spatial_query_boundary_sources())
        .evaluate()
}

#[cfg(test)]
fn query_hard_prohibition_registry_row_count() -> usize {
    hard_prohibition_boundary_audit_coverage().rows().len()
}

fn spatial_query_boundary_sources() -> ForgeQueryBoundaryAuditSourceSet {
    ForgeQueryBoundaryAuditSourceSet::new("worth-spatial")
        .source_file(
            "worth-spatial query adoption support projection",
            "crates/worth-spatial/src/query_adoption/support_projection.rs",
            include_str!("support_projection.rs"),
        )
        .source_file(
            "worth-spatial query adoption evidence reports",
            "crates/worth-spatial/src/query_adoption/evidence_reports.rs",
            include_str!("evidence_reports.rs"),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spatial_boundary_audit_uses_query_owned_hard_prohibition_registry() {
        let report = spatial_query_hard_prohibition_boundary_audit()
            .expect("Query hard-prohibition audit should evaluate");

        assert!(report.is_clean());
        assert_eq!(report.crate_name(), "worth-spatial");
        assert_eq!(report.source_labels().len(), 2);
        assert_eq!(
            report.coverage_rows().len(),
            query_hard_prohibition_registry_row_count()
        );
    }
}
