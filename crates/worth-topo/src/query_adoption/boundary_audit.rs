#[cfg(test)]
use forge_query::facade::consumer_kit::hard_prohibition_boundary_audit_coverage;
use forge_query::facade::consumer_kit::{
    hard_prohibition_boundary_audit, ForgeQueryBoundaryAuditError, ForgeQueryBoundaryAuditReport,
    ForgeQueryBoundaryAuditSourceSet,
};

pub(super) fn topology_query_hard_prohibition_boundary_audit(
) -> Result<ForgeQueryBoundaryAuditReport, ForgeQueryBoundaryAuditError> {
    hard_prohibition_boundary_audit()
        .covering_sources(topology_query_boundary_sources())
        .evaluate()
}

#[cfg(test)]
fn query_hard_prohibition_registry_row_count() -> usize {
    hard_prohibition_boundary_audit_coverage().rows().len()
}

fn topology_query_boundary_sources() -> ForgeQueryBoundaryAuditSourceSet {
    ForgeQueryBoundaryAuditSourceSet::new("worth-topo")
        .source_file(
            "worth-topo query adoption runtime support",
            "crates/worth-topo/src/query_adoption/runtime_support.rs",
            include_str!("runtime_support.rs"),
        )
        .source_file(
            "worth-topo query adoption evidence reports",
            "crates/worth-topo/src/query_adoption/evidence_reports.rs",
            include_str!("evidence_reports.rs"),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn topology_boundary_audit_uses_query_owned_hard_prohibition_registry() {
        let report = topology_query_hard_prohibition_boundary_audit()
            .expect("Query hard-prohibition audit should evaluate");

        assert!(report.is_clean());
        assert_eq!(report.crate_name(), "worth-topo");
        assert_eq!(report.source_labels().len(), 2);
        assert_eq!(
            report.coverage_rows().len(),
            query_hard_prohibition_registry_row_count()
        );
    }
}
