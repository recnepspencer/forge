use forge_query::facade::consumer_kit::{
    hard_prohibition_boundary_audit, hard_prohibition_registry, ForgeQueryBoundaryAuditReport,
    ForgeQueryBoundaryAuditSourceSet,
};

pub(crate) fn audit_evidence_lookup_query_hard_prohibitions_for_sources(
    sources: ForgeQueryBoundaryAuditSourceSet,
) -> Result<
    ForgeQueryBoundaryAuditReport,
    forge_query::facade::consumer_kit::ForgeQueryBoundaryAuditError,
> {
    let _registry = hard_prohibition_registry();
    hard_prohibition_boundary_audit()
        .covering_sources(sources)
        .evaluate()
}
