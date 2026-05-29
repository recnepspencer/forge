pub(crate) mod derived_surfaces;
pub(crate) mod diagnostic_surfaces;
mod domain_entry;
pub(crate) mod read_views;
pub(crate) mod runtime_boundary;
pub(crate) mod truth_surfaces;

pub(crate) use derived_surfaces::decode_single_computed_row;
pub use derived_surfaces::{
    declare_topology_interpreted_surface, declare_topology_validation_surface,
    interpreted_topology_from_materialized_rows, topology_interpreted_computed_declaration,
    topology_validation_computed_declaration, validation_report_from_query_rows,
    TopologyInterpretedMaintainer, TopologyQuerySurfaceError, TopologyValidationMaintainer,
};
pub use diagnostic_surfaces::query_diagnostics::{
    declare_topology_diagnostics_surface, declare_topology_equivalence_contract_surface,
    equivalence_contract_from_diagnostics_rows, topology_diagnostics_computed_declaration,
    topology_equivalence_contract_computed_declaration, TopologyDiagnosticsMaintainer,
    TopologyEquivalenceContractMaintainer, TopologyQueryMutationEvidence,
};
pub use domain_entry::{
    topology_current_head_authoritative_context, topology_query_domain,
    topology_query_domain_entry, topology_query_domain_entry_checked,
    topology_query_domain_proof_root, topology_snapshot_read_only_context,
    TopologyCurrentHeadAuthoritativeContext, TopologyCurrentHeadConfiguredDomainHandle,
    TopologyCurrentHeadConfiguredDomainHandleChecked, TopologyQueryDomain,
    TopologySnapshotReadOnlyConfiguredDomainHandle,
    TopologySnapshotReadOnlyConfiguredDomainHandleChecked, TopologySnapshotReadOnlyContext,
};
pub(crate) use read_views::domain::parity::{
    build_domain_query_view_parity_artifact, TopologyDomainQueryViewParityArtifact,
    TopologyDomainQueryViewRef,
};
pub use read_views::domain::{
    TopologyConfiguredDomainReadSession, TopologyCurrentHeadReadHandleExt,
    TopologyCurrentHeadReadSession, TopologyDomainQuery, TopologyDomainQueryAggregateReport,
    TopologyDomainQueryCloseoutReport, TopologyDomainQueryCloseoutRow,
    TopologyDomainQueryCloseoutStatus, TopologyDomainQueryDebtRow, TopologyDomainQueryError,
    TopologyDomainQueryErrorKind, TopologyDomainQueryExecutionAggregateRow,
    TopologyDomainQueryExecutionEngine, TopologyDomainQueryFallbackPosture,
    TopologyDomainQueryFamilyAggregateRow, TopologyDomainQueryLoweringPosture,
    TopologyDomainQueryParityAggregateReport, TopologyDomainQueryParityAggregateRow,
    TopologyDomainQueryParityKind, TopologyDomainQueryPhaseThreeBlocker,
    TopologyDomainQueryPhaseThreeBlockerRow, TopologyDomainQueryPhaseThreeBlockerStatus,
    TopologyDomainQueryProofReport, TopologyDomainQueryRelationshipProofPosture,
    TopologyDomainQueryRequestFamily, TopologyDomainQueryRequestReport,
    TopologyHalfEdgeRadialNeighborhoodView, TopologyHalfEdgeSharedVertexNeighborhoodView,
    TopologyLocalRewireNeighborhoodView, TopologyLoopCycleView, TopologyLoopNeighborEvidence,
    TopologyNoNPlusOneContract, TopologyNoNPlusOneContractRow, TopologyNoNPlusOneContractStatus,
    TopologySnapshotReadOnlyReadHandleExt, TopologySnapshotReadOnlyReadSession,
};
pub(crate) use runtime_boundary::query_support::{
    parse_entity_identity, parse_relation_identity, query_entity_identity, required_text,
};
#[cfg(test)]
pub(crate) use runtime_boundary::query_support::{
    query_entity_id_from_row, query_relation_id_from_row, TopologyQueryRowLookup,
};
pub use truth_surfaces::{
    declare_persistent_name_live_view, declare_topology_entity_live_view,
    declare_topology_materialized_surface, declare_topology_relation_live_view,
    naming_attachment_report_from_query_input, persistent_name_live_view_declaration,
    topology_entity_live_view_declaration, topology_materialized_computed_declaration,
    topology_relation_live_view_declaration, TopologyMaterializedMaintainer,
    TopologyNamingAttachmentInput,
};
