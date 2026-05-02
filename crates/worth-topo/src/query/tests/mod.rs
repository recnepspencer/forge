use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryInspection, ForgeQueryRuntimeStateKind, ForgeQueryWorkspace,
};
use serde_json::json;
use worth_schema::facade::{WorthTopologyEntityKind, WorthTopologyRelationKind};

use super::{
    equivalence_contract_from_diagnostics_rows, interpreted_topology_from_materialized_rows,
    materialized::materialized_topology_from_query_rows, validation_report_from_query_rows,
    WorthTopologyQueryMutationEvidence,
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
