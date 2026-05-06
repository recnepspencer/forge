//! Query-native Worth topology surfaces.
//!
//! These helpers are the Worth-owned declaration and maintainer seam for
//! moving topology reads onto Forge Query's live/computed runtime. Topology
//! meaning stays in `worth-topo`; wakeup, retained state, observation,
//! inspection, retained mutation evidence, and typed runtime posture move onto
//! Forge Query.

mod assembly;
mod derived;
mod diagnostics;
mod domain;
mod materialized;
mod naming;
mod runtime;
mod snapshot_index;
mod snapshot_rows;
mod support;

pub use assembly::{
    WorthTopologyQueryAppliedIntent, WorthTopologyQueryApplyError, WorthTopologyQueryAssembly,
    WorthTopologyQuerySnapshot,
};
pub use derived::{
    declare_worth_topology_interpreted_surface, declare_worth_topology_validation_surface,
    interpreted_topology_from_materialized_rows, validation_report_from_query_rows,
    worth_topology_interpreted_computed_declaration,
    worth_topology_validation_computed_declaration, WorthTopologyInterpretedMaintainer,
    WorthTopologyQuerySurfaceError, WorthTopologyValidationMaintainer,
};
pub use diagnostics::{
    declare_worth_topology_diagnostics_surface,
    declare_worth_topology_equivalence_contract_surface, derived_read_diagnostics_from_query_rows,
    equivalence_contract_from_diagnostics_rows, worth_topology_diagnostics_computed_declaration,
    worth_topology_equivalence_contract_computed_declaration, WorthTopologyDiagnosticsMaintainer,
    WorthTopologyEquivalenceContractMaintainer, WorthTopologyQueryMutationEvidence,
};
pub(crate) use domain::WorthTopologyDomainQuery;
pub(crate) use materialized::topology_relation_dependency_path;
pub use materialized::{
    declare_worth_topology_entity_live_view, declare_worth_topology_materialized_surface,
    declare_worth_topology_relation_live_view, worth_topology_entity_live_view_declaration,
    worth_topology_materialized_computed_declaration,
    worth_topology_relation_live_view_declaration, WorthTopologyMaterializedMaintainer,
};
pub use naming::{
    declare_worth_persistent_name_live_view, naming_attachment_report_from_query_rows,
    worth_persistent_name_live_view_declaration,
};
pub use runtime::{
    worth_topology_runtime, WorthTopologyQueryEditFamilySupportStatus,
    WorthTopologyRuntimeAdapters, WorthTopologyRuntimeFailure, WorthTopologyRuntimeSupport,
};
pub(crate) use snapshot_index::WorthTopologyQuerySnapshotIndex;
#[cfg(test)]
pub(crate) use snapshot_rows::{query_entity_id_from_row, query_relation_id_from_row};
pub(crate) use support::{
    parse_entity_identity, parse_entity_kind, parse_relation_kind, required_text,
};

const QUERY_SURFACE_FAILURE_ROW_KEY: &str = "query_surface_error";

#[cfg(test)]
mod tests;
