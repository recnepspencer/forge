use crate::certification::error::TopologyCertificationError;

use super::super::support::{closed_row, ensure, source_text};
use super::super::TopologyQueryBoundaryCleanupArea;

pub(crate) fn certify_public_facade_row(
) -> Result<super::super::TopologyQueryBoundaryCleanupRow, TopologyCertificationError> {
    let lib = source_text("src/lib.rs")?;
    let facade = source_text("src/facade.rs")?;
    let query_domain = source_text("src/query_domain.rs")?;
    let compile_fail_contracts =
        source_text("src/certification/public_facade_contracts/compile_fail_contracts.rs")?;

    ensure(!lib.contains("pub mod projection;"))?;
    ensure(!facade.contains("from_query_rows"))?;
    ensure(!facade.contains("TopologyReadSessionState"))?;
    ensure(!facade.contains("TopologyConfiguredDomainReadSession"))?;
    ensure(!facade.contains("declare_topology_entity_live_view"))?;
    ensure(!facade.contains("declare_topology_materialized_surface"))?;
    ensure(!facade.contains("topology_materialized_computed_declaration"))?;
    ensure(!facade.contains("TopologyMaterializedMaintainer"))?;
    ensure(!facade.contains("TopologyInterpretedMaintainer"))?;
    ensure(!facade.contains("TopologyValidationMaintainer"))?;
    ensure(!facade.contains("TopologyDiagnosticsMaintainer"))?;
    ensure(!facade.contains("TopologyEquivalenceContractMaintainer"))?;
    ensure(!facade.contains("TopologyQuerySurfaceError"))?;
    ensure(!facade.contains("TopologyNamingAttachmentInput"))?;
    ensure(!facade.contains("naming_attachment_report_from_query_input"))?;
    ensure(!facade.contains("TopologyQueryMutationEvidence"))?;
    ensure(!facade.contains("build_derived_read_diagnostics"))?;
    ensure(!facade.contains("build_derived_invalidation_report"))?;
    ensure(!facade.contains("build_derived_rebuild_report"))?;
    ensure(!facade.contains("build_derived_fallback_report"))?;
    ensure(!facade.contains("build_milestone_one_bridge"))?;
    ensure(!facade.contains("milestone_one_bridge_mapping_registrations"))?;
    ensure(!facade.contains("milestone_one_bridge_aspect_registrations"))?;
    ensure(!query_domain.contains("declare_topology_entity_live_view"))?;
    ensure(!query_domain.contains("declare_topology_materialized_surface"))?;
    ensure(!query_domain.contains("TopologyMaterializedMaintainer"))?;
    ensure(!query_domain.contains("TopologyInterpretedMaintainer"))?;
    ensure(!query_domain.contains("TopologyValidationMaintainer"))?;
    ensure(!query_domain.contains("TopologyDiagnosticsMaintainer"))?;
    ensure(!query_domain.contains("TopologyEquivalenceContractMaintainer"))?;
    ensure(!query_domain.contains("TopologyQuerySurfaceError"))?;
    ensure(!query_domain.contains("TopologyNamingAttachmentInput"))?;
    ensure(!query_domain.contains("TopologyQueryMutationEvidence"))?;
    ensure(!query_domain.contains("build_derived_read_diagnostics"))?;
    ensure(!query_domain.contains("build_derived_invalidation_report"))?;
    ensure(!query_domain.contains("build_derived_rebuild_report"))?;
    ensure(!query_domain.contains("build_derived_fallback_report"))?;
    ensure(query_domain.contains("TopologyConfiguredDomainReadSession"))?;
    ensure(query_domain.contains("TopologyCurrentHeadReadHandleExt"))?;
    ensure(query_domain.contains("TopologySnapshotReadOnlyReadHandleExt"))?;
    ensure(compile_fail_contracts.contains("public_topology_reads_not_exported_from_facade.rs"))?;
    ensure(facade.contains("TopologyRuntimeSupport"))?;
    ensure(compile_fail_contracts.contains("public_query_row_helpers_not_exported.rs"))?;
    ensure(compile_fail_contracts.contains("public_derived_diagnostics_builders_not_exported.rs"))?;
    ensure(compile_fail_contracts.contains("public_query_row_materializer_not_exported.rs"))?;
    ensure(compile_fail_contracts.contains("public_projection_surface_entry_not_exported.rs"))?;
    ensure(compile_fail_contracts.contains("public_bridge_registration_entry_not_exported.rs"))?;

    closed_row(
        TopologyQueryBoundaryCleanupArea::PublicFacade,
        "the topology-facing public surface is limited to query-domain entry, handle-bound reads, runtime support, and report vocabulary; it no longer teaches raw projection declarations, maintainers, manual derived-diagnostics helper assembly, or bridge-registration wiring as competing topology entry workflows",
        Some("src/query_domain.rs"),
        [
            "src/lib.rs",
            "src/facade.rs",
            "src/query_domain.rs",
            "src/certification/public_facade_contracts/compile_fail_contracts.rs",
        ],
    )
}
