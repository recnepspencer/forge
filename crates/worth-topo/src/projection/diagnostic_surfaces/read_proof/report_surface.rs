use forge_query::facade::{ForgeQueryReadBuiltInOperator, ForgeQueryReadScopeClass};

use super::fallback::TopologyReadFallbackPosture;
use super::report::{
    TopologyReadAggregateReport, TopologyReadDebtRow, TopologyReadExecutionAggregateRow,
    TopologyReadExecutionEngine, TopologyReadFamilyAggregateRow, TopologyReadRequestFamily,
    TopologyReadRequestReport,
};
use crate::projection::runtime_boundary::read_lowering::TopologyReadRelationshipProofPosture;

impl TopologyReadExecutionEngine {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::QueryRuntimeCurrent => "query_runtime_current",
            Self::QueryRuntimeBranch => "query_runtime_branch",
            Self::QueryRuntimeHistorical => "query_runtime_historical",
            Self::QueryRuntimePreviewDerived => "query_runtime_preview_derived",
        }
    }
}

impl TopologyReadRequestFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HalfEdgeSharedVertexNeighborhood => "half_edge_shared_vertex_neighborhood",
            Self::HalfEdgeRadialNeighborhood => "half_edge_radial_neighborhood",
            Self::LoopCycleNeighborhood => "loop_cycle_neighborhood",
            Self::LocalRewireNeighborhood => "local_rewire_neighborhood",
        }
    }
}

impl TopologyReadRequestReport {
    pub fn request_family(&self) -> TopologyReadRequestFamily {
        self.request_family
    }

    pub fn execution_engine(&self) -> TopologyReadExecutionEngine {
        self.execution_engine
    }

    pub fn claimed_scope_class(&self) -> ForgeQueryReadScopeClass {
        self.request_family.claimed_scope_class()
    }

    pub fn executed_scope_class(&self) -> Option<ForgeQueryReadScopeClass> {
        self.executed_scope_class.clone()
    }

    pub fn executed_query_digest(&self) -> Option<&str> {
        self.executed_query_digest.as_deref()
    }

    pub fn executed_basis_digest(&self) -> Option<&str> {
        self.executed_basis_digest.as_deref()
    }

    pub fn executed_snapshot_token(&self) -> Option<&str> {
        self.executed_snapshot_token.as_deref()
    }

    pub fn executed_built_in_operator_coverage(&self) -> &[ForgeQueryReadBuiltInOperator] {
        self.executed_built_in_operator_coverage.as_slice()
    }

    pub fn fallback_posture(&self) -> TopologyReadFallbackPosture {
        self.fallback_posture
    }

    pub fn query_execution_count(&self) -> usize {
        self.query_execution_count
    }

    pub fn lowered_traversal_count(&self) -> usize {
        self.lowered_traversal_count
    }

    pub fn relationship_proof_posture(&self) -> TopologyReadRelationshipProofPosture {
        self.lowering_artifact.relationship_proof_posture()
    }

    pub fn relationship_proof_admission_count(&self) -> usize {
        self.relationship_proof_admission_count
    }

    pub fn canonical_query_digest(&self) -> &str {
        self.lowering_artifact.canonical_query_digest()
    }

    pub fn canonical_result_shape_digest(&self) -> &str {
        self.lowering_artifact.canonical_result_shape_digest()
    }

    pub fn root_entity(&self) -> &str {
        self.lowering_artifact.root_entity()
    }

    pub fn row_scan_fallback_count(&self) -> usize {
        self.row_scan_fallback_count
    }

    pub fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub fn repeated_rediscovery_denied_count(&self) -> usize {
        self.repeated_rediscovery_denied_count
    }
}

impl TopologyReadFamilyAggregateRow {
    pub fn request_family(&self) -> TopologyReadRequestFamily {
        self.request_family
    }

    pub fn request_count(&self) -> usize {
        self.request_count
    }

    pub fn query_execution_count(&self) -> usize {
        self.query_execution_count
    }

    pub fn lowered_traversal_count(&self) -> usize {
        self.lowered_traversal_count
    }

    pub fn relationship_proof_admission_count(&self) -> usize {
        self.relationship_proof_admission_count
    }

    pub fn row_scan_fallback_count(&self) -> usize {
        self.row_scan_fallback_count
    }

    pub fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub fn repeated_rediscovery_denied_count(&self) -> usize {
        self.repeated_rediscovery_denied_count
    }
}

impl TopologyReadDebtRow {
    pub fn request_family(&self) -> TopologyReadRequestFamily {
        self.request_family
    }

    pub fn request_count(&self) -> usize {
        self.request_count
    }

    pub fn fallback_posture(&self) -> TopologyReadFallbackPosture {
        self.fallback_posture
    }

    pub fn relationship_proof_posture(&self) -> TopologyReadRelationshipProofPosture {
        self.relationship_proof_posture
    }
}

impl TopologyReadExecutionAggregateRow {
    pub fn request_family(&self) -> TopologyReadRequestFamily {
        self.request_family
    }

    pub fn claimed_scope_class(&self) -> ForgeQueryReadScopeClass {
        self.claimed_scope_class.clone()
    }

    pub fn executed_scope_class(&self) -> Option<ForgeQueryReadScopeClass> {
        self.executed_scope_class.clone()
    }

    pub fn execution_engine(&self) -> TopologyReadExecutionEngine {
        self.execution_engine
    }

    pub fn fallback_posture(&self) -> TopologyReadFallbackPosture {
        self.fallback_posture
    }

    pub fn relationship_proof_posture(&self) -> TopologyReadRelationshipProofPosture {
        self.relationship_proof_posture
    }

    pub fn request_count(&self) -> usize {
        self.request_count
    }

    pub fn query_execution_count(&self) -> usize {
        self.query_execution_count
    }

    pub fn lowered_traversal_count(&self) -> usize {
        self.lowered_traversal_count
    }

    pub fn relationship_proof_admission_count(&self) -> usize {
        self.relationship_proof_admission_count
    }

    pub fn row_scan_fallback_count(&self) -> usize {
        self.row_scan_fallback_count
    }

    pub fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub fn repeated_rediscovery_denied_count(&self) -> usize {
        self.repeated_rediscovery_denied_count
    }
}

impl TopologyReadAggregateReport {
    pub fn request_count(&self) -> usize {
        self.request_count
    }

    pub fn query_runtime_current_execution_count(&self) -> usize {
        self.query_runtime_current_execution_count
    }

    pub fn query_runtime_historical_execution_count(&self) -> usize {
        self.query_runtime_historical_execution_count
    }

    pub fn local_neighborhood_execution_count(&self) -> usize {
        self.local_neighborhood_execution_count
    }

    pub fn anchored_expansion_execution_count(&self) -> usize {
        self.anchored_expansion_execution_count
    }

    pub fn explicit_broad_search_execution_count(&self) -> usize {
        self.explicit_broad_search_execution_count
    }

    pub fn locality_claim_mismatch_count(&self) -> usize {
        self.locality_claim_mismatch_count
    }

    pub fn query_execution_count(&self) -> usize {
        self.query_execution_count
    }

    pub fn lowered_traversal_count(&self) -> usize {
        self.lowered_traversal_count
    }

    pub fn relationship_proof_admission_count(&self) -> usize {
        self.relationship_proof_admission_count
    }

    pub fn row_scan_fallback_count(&self) -> usize {
        self.row_scan_fallback_count
    }

    pub fn whole_view_fallback_count(&self) -> usize {
        self.whole_view_fallback_count
    }

    pub fn repeated_rediscovery_denied_count(&self) -> usize {
        self.repeated_rediscovery_denied_count
    }

    pub fn family_rows(&self) -> &[TopologyReadFamilyAggregateRow] {
        self.family_rows.as_slice()
    }

    pub fn debt_rows(&self) -> &[TopologyReadDebtRow] {
        self.debt_rows.as_slice()
    }

    pub fn execution_rows(&self) -> &[TopologyReadExecutionAggregateRow] {
        self.execution_rows.as_slice()
    }
}
