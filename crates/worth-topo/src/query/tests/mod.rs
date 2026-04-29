use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryInspection, ForgeQueryRuntimeStateKind, ForgeQueryWorkspace,
};
use serde_json::{json, Value};
use worth_schema::facade::{WorthTopologyEntityKind, WorthTopologyRelationKind};

use super::{
    declare_worth_topology_entity_live_view, declare_worth_topology_interpreted_surface,
    declare_worth_topology_materialized_surface, declare_worth_topology_relation_live_view,
    declare_worth_topology_validation_surface, equivalence_contract_from_diagnostics_rows,
    interpreted_topology_from_materialized_rows, materialized::parse_relation_kind,
    materialized::topology_relation_dependency_path, materialized_topology_from_query_rows,
    validation_report_from_query_rows, worth_topology_query_workspace, WorthTopologyQueryAssembly,
    WorthTopologyQueryMutationEvidence, QUERY_SURFACE_FAILURE_ROW_KEY,
};
use crate::facade::{
    DerivedTopologyValidationReport, InterpretedTopologyView, MaterializedTopologyView,
    WorthDerivedReadDiagnostics,
};
use worth_schema::facade::WorthMutationOrigin;

const MATERIALIZED_TOPOLOGY_SURFACE: &str = "worth.topology.materialized";
const INTERPRETED_TOPOLOGY_SURFACE: &str = "worth.topology.interpreted";
const VALIDATION_TOPOLOGY_SURFACE: &str = "worth.topology.validation";
const DIAGNOSTICS_TOPOLOGY_SURFACE: &str = "worth.topology.diagnostics";
const EQUIVALENCE_TOPOLOGY_SURFACE: &str = "worth.topology.equivalence_contract";

mod derived_chain;
mod failures;
mod materialization;
mod support;
