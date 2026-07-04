use crate::compiled_product_reuse_decision::TopologyDerivedReuseDecisionPosture;
use crate::derived_topology::compiled_product_consumer_cutover::DerivedEquivalenceContractReport;
use crate::derived_topology::materialized_graph::MaterializationFallbackClass;
use crate::validation::DerivedTopologyValidationReport;
use schema::facade::platform::authority::{
    touched_graph_conflict::{
        BatchAdmissionPlannerRouteWitnessKind, ConflictIndependencePlannerRouteWitnessKind,
    },
    DerivedInvalidationTarget,
};

use super::source::TopologyDerivedDiagnosticProjectionSource;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DerivedInvalidationTargetRow {
    pub target: DerivedInvalidationTarget,
    pub bridge_scope: String,
    pub declaration_ids: Vec<String>,
    pub triggered: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DerivedInvalidationReport {
    pub touched_aspect_count: usize,
    pub topology_touched: bool,
    pub naming_touched: bool,
    pub triggered_target_count: usize,
    pub rows: Vec<DerivedInvalidationTargetRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DerivedRebuildReport {
    pub whole_view_rebuild: bool,
    pub topology_entity_count: usize,
    pub topology_relation_count: usize,
    pub interpreted_wire_count: usize,
    pub interpreted_shell_count: usize,
    pub boundary_interpretation_count: usize,
    pub radial_interpretation_count: usize,
    pub validation_row_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DerivedFallbackReport {
    pub whole_view_materialization: bool,
    pub materialization_fallback_class: Option<MaterializationFallbackClass>,
    pub precision_fallback_count: usize,
    pub precision_budget_fallback_count: usize,
    pub explicit_fallback_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DerivedValidationExecutionReport {
    pub source: String,
    pub execution_count: usize,
    pub registered_rule_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DerivedReadDiagnostics {
    pub(crate) diagnostic_projection_source: TopologyDerivedDiagnosticProjectionSource,
    pub compiled_product_reuse_route_packet_identity: Option<String>,
    pub topology_reuse_posture: Option<TopologyDerivedReuseDecisionPosture>,
    pub spatial_reuse_posture: Option<String>,
    pub spatial_reuse_decision_identity_digest: Option<String>,
    pub spatial_rebuild_denial_identity_digest: Option<String>,
    pub batch_admission_route_packet_identity: Option<String>,
    pub batch_admission_denial_witness_identity: Option<String>,
    pub batch_admission_denial_witness_kind: Option<BatchAdmissionPlannerRouteWitnessKind>,
    pub conflict_independence_route_packet_identity: Option<String>,
    pub conflict_independence_denial_witness_identity: Option<String>,
    pub conflict_independence_denial_witness_kind:
        Option<ConflictIndependencePlannerRouteWitnessKind>,
    pub invalidation_report: DerivedInvalidationReport,
    pub rebuild_report: DerivedRebuildReport,
    pub fallback_report: DerivedFallbackReport,
    pub validation_report: DerivedTopologyValidationReport,
    pub validation_execution_report: DerivedValidationExecutionReport,
    pub equivalence_contract_report: DerivedEquivalenceContractReport,
}
