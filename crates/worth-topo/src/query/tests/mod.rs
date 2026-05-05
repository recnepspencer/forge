use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryInspection, ForgeQueryRuntimeStateKind,
};
use serde_json::json;
use worth_schema::facade::{WorthTopologyEntityKind, WorthTopologyRelationKind};

use super::{
    equivalence_contract_from_diagnostics_rows, interpreted_topology_from_materialized_rows,
    validation_report_from_query_rows, WorthTopologyQueryMutationEvidence,
};
use crate::facade::{
    DerivedTopologyValidationReport, InterpretedTopologyView, MaterializedTopologyView,
    WorthDerivedReadDiagnostics,
};
use crate::materialization::WorthTopologyMaterializer;
use worth_schema::facade::WorthMutationOrigin;

const MATERIALIZED_TOPOLOGY_SURFACE: &str = "worth.topology.materialized";
const INTERPRETED_TOPOLOGY_SURFACE: &str = "worth.topology.interpreted";
const VALIDATION_TOPOLOGY_SURFACE: &str = "worth.topology.validation";
const DIAGNOSTICS_TOPOLOGY_SURFACE: &str = "worth.topology.diagnostics";
const EQUIVALENCE_TOPOLOGY_SURFACE: &str = "worth.topology.equivalence_contract";

mod derived_chain;
mod domain_query;
mod domain_query_lowering;
mod domain_query_parity;
mod failures;
mod materialization;
mod snapshot_index;
mod support;
