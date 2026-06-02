use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryInspection, ForgeQueryRuntimeStateKind,
};
use schema::facade::platform::entities::TopologyEntityKind;
use schema::facade::platform::relations::TopologyRelationKind;
use serde_json::json;

use crate::facade::{
    DerivedReadDiagnostics, DerivedTopologyValidationReport, InterpretedTopologyView,
    MaterializedTopologyView,
};
use crate::projection::runtime_boundary::declared_query_surfaces::derived_surfaces::{
    interpreted_topology_from_materialized_rows, validation_report_from_query_rows,
};
use crate::projection::runtime_boundary::declared_query_surfaces::equivalence_contract_from_diagnostics_rows;
use crate::projection::runtime_boundary::declared_query_surfaces::query_diagnostics::TopologyQueryMutationEvidence;

const MATERIALIZED_TOPOLOGY_SURFACE: &str = ".topology.materialized";
const INTERPRETED_TOPOLOGY_SURFACE: &str = ".topology.interpreted";
const VALIDATION_TOPOLOGY_SURFACE: &str = ".topology.validation";
const DIAGNOSTICS_TOPOLOGY_SURFACE: &str = ".topology.diagnostics";
const EQUIVALENCE_TOPOLOGY_SURFACE: &str = ".topology.equivalence_contract";

mod derived_chain;
mod failures;
mod materialization;
mod row_lookup;
mod topology_reads;
