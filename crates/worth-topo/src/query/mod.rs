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
mod materialized;
mod naming;

pub use assembly::{
    worth_topology_query_workspace, WorthTopologyQueryAppliedIntent, WorthTopologyQueryApplyError,
    WorthTopologyQueryAssembly, WorthTopologyQueryImportError, WorthTopologyQuerySnapshot,
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
pub use materialized::{
    declare_worth_topology_entity_live_view, declare_worth_topology_materialized_surface,
    declare_worth_topology_relation_live_view, materialized_topology_from_query_rows,
    worth_topology_entity_live_view_declaration, worth_topology_materialized_computed_declaration,
    worth_topology_relation_live_view_declaration, WorthTopologyMaterializedMaintainer,
};
pub use naming::{
    declare_worth_persistent_name_live_view, naming_attachment_report_from_query_rows,
    worth_persistent_name_live_view_declaration,
};

const QUERY_SURFACE_FAILURE_ROW_KEY: &str = "query_surface_error";

#[cfg(test)]
mod tests;
