//! Query-native  topology surfaces.
//!
//! These helpers are the topology-owned declaration and maintainer seam for
//! moving topology reads onto Forge Query's live/computed runtime. Topology
//! meaning stays in topology; wakeup, retained state, observation,
//! inspection, retained mutation evidence, and typed runtime posture move onto
//! Forge Query.

mod assembly;
mod derived;
mod diagnostics;
mod domain;
mod materialized;
mod naming;
#[cfg(test)]
mod query_rows;
#[cfg(test)]
mod row_lookup;
mod runtime;
mod support;

pub use assembly::{
    TopologyQueryAppliedIntent, TopologyQueryApplyError, TopologyQueryAssembly,
    TopologyQuerySnapshot,
};
pub use derived::{
    declare_topology_interpreted_surface, declare_topology_validation_surface,
    interpreted_topology_from_materialized_rows, topology_interpreted_computed_declaration,
    topology_validation_computed_declaration, validation_report_from_query_rows,
    TopologyInterpretedMaintainer, TopologyQuerySurfaceError, TopologyValidationMaintainer,
};
pub use diagnostics::{
    declare_topology_diagnostics_surface, declare_topology_equivalence_contract_surface,
    derived_read_diagnostics_from_query_rows, equivalence_contract_from_diagnostics_rows,
    topology_diagnostics_computed_declaration, topology_equivalence_contract_computed_declaration,
    TopologyDiagnosticsMaintainer, TopologyEquivalenceContractMaintainer,
    TopologyQueryMutationEvidence,
};
pub(crate) use domain::parity::{
    build_domain_query_view_parity_artifact, TopologyDomainQueryViewParityArtifact,
    TopologyDomainQueryViewRef,
};
pub use domain::{
    TopologyDomainQuery, TopologyDomainQueryAggregateReport, TopologyDomainQueryCloseoutReport,
    TopologyDomainQueryCloseoutRow, TopologyDomainQueryCloseoutStatus, TopologyDomainQueryDebtRow,
    TopologyDomainQueryError, TopologyDomainQueryErrorKind,
    TopologyDomainQueryExecutionAggregateRow, TopologyDomainQueryExecutionEngine,
    TopologyDomainQueryFallbackPosture, TopologyDomainQueryFamilyAggregateRow,
    TopologyDomainQueryLoweringPosture, TopologyDomainQueryParityAggregateReport,
    TopologyDomainQueryParityAggregateRow, TopologyDomainQueryParityKind,
    TopologyDomainQueryPhaseThreeBlocker, TopologyDomainQueryPhaseThreeBlockerRow,
    TopologyDomainQueryPhaseThreeBlockerStatus, TopologyDomainQueryProofReport,
    TopologyDomainQueryRelationshipProofPosture, TopologyDomainQueryRequestFamily,
    TopologyDomainQueryRequestReport, TopologyHalfEdgeRadialNeighborhoodView,
    TopologyHalfEdgeSharedVertexNeighborhoodView, TopologyLocalRewireNeighborhoodView,
    TopologyLoopCycleView, TopologyLoopNeighborEvidence, TopologyNoNPlusOneContract,
    TopologyNoNPlusOneContractRow, TopologyNoNPlusOneContractStatus,
};
pub(crate) use materialized::topology_relation_dependency_path;
pub use materialized::{
    declare_topology_entity_live_view, declare_topology_materialized_surface,
    declare_topology_relation_live_view, topology_entity_live_view_declaration,
    topology_materialized_computed_declaration, topology_relation_live_view_declaration,
    TopologyMaterializedMaintainer,
};
pub use naming::{
    declare_persistent_name_live_view, naming_attachment_report_from_query_rows,
    persistent_name_live_view_declaration,
};
#[cfg(test)]
pub(crate) use query_rows::{query_entity_id_from_row, query_relation_id_from_row};
#[cfg(test)]
pub(crate) use row_lookup::TopologyQueryRowLookup;
pub use runtime::{
    topology_runtime, TopologyQueryEditFamilySupportStatus, TopologyQueryEditLane,
    TopologyQueryEditLaneExecutionShape, TopologyQueryEditLaneSupportStatus,
    TopologyQueryReadFamilySupportStatus, TopologyRuntimeAdapters, TopologyRuntimeCloseout,
    TopologyRuntimeCloseoutFamily, TopologyRuntimeCloseoutRow, TopologyRuntimeCloseoutStatus,
    TopologyRuntimeEditFamilySupportRow, TopologyRuntimeEditLaneSupportRow, TopologyRuntimeFailure,
    TopologyRuntimePostureCapability, TopologyRuntimePostureRow, TopologyRuntimePostureStatus,
    TopologyRuntimeReadFamilySupportRow, TopologyRuntimeSupport,
};
pub(crate) use support::{
    parse_entity_identity, parse_entity_kind, parse_relation_identity, parse_relation_kind,
    query_entity_identity, required_text,
};

const QUERY_SURFACE_FAILURE_ROW_KEY: &str = "query_surface_error";

#[cfg(test)]
mod tests;
