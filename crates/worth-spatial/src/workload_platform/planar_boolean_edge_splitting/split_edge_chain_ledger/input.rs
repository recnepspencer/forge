use super::declaration::PlanarBooleanSplitEdgeChainLedgerDeclaration;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet, PlanarBooleanOverlapEdgeChainSet,
    PlanarBooleanSplitChainValidationReceipt, PlanarBooleanSplitDecisionLogQueryResult,
    PlanarBooleanSplitEdgeFragmentSet, PlanarBooleanSplitPersistentNamingReceipt,
    PlanarBooleanSplitVertexIdentitySet,
};

pub struct PlanarBooleanSplitEdgeChainLedgerInput<'a> {
    declaration: PlanarBooleanSplitEdgeChainLedgerDeclaration,
    split_request: &'a PlanarBooleanEdgeSplitRequest,
    endpoint_boundary_schedules: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    interval_subdivision_schedules: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
    split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
    split_chain_validation: &'a PlanarBooleanSplitChainValidationReceipt,
    split_persistent_names: &'a PlanarBooleanSplitPersistentNamingReceipt,
    split_decision_log: &'a PlanarBooleanSplitDecisionLogQueryResult,
}

impl<'a> PlanarBooleanSplitEdgeChainLedgerInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_query_products(
        declaration: PlanarBooleanSplitEdgeChainLedgerDeclaration,
        split_request: &'a PlanarBooleanEdgeSplitRequest,
        endpoint_boundary_schedules: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
        interval_subdivision_schedules: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
        split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
        split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
        split_chain_validation: &'a PlanarBooleanSplitChainValidationReceipt,
        split_persistent_names: &'a PlanarBooleanSplitPersistentNamingReceipt,
        split_decision_log: &'a PlanarBooleanSplitDecisionLogQueryResult,
    ) -> Self {
        Self {
            declaration,
            split_request,
            endpoint_boundary_schedules,
            interval_subdivision_schedules,
            split_vertices,
            split_fragments,
            overlap_chains,
            split_chain_validation,
            split_persistent_names,
            split_decision_log,
        }
    }

    pub(crate) fn declaration(&self) -> &PlanarBooleanSplitEdgeChainLedgerDeclaration {
        &self.declaration
    }
    pub(crate) fn split_request(&self) -> &PlanarBooleanEdgeSplitRequest {
        self.split_request
    }
    pub(crate) fn endpoint_boundary_schedules(
        &self,
    ) -> &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet {
        self.endpoint_boundary_schedules
    }
    pub(crate) fn interval_subdivision_schedules(
        &self,
    ) -> &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
        self.interval_subdivision_schedules
    }
    pub(crate) fn split_vertices(&self) -> &PlanarBooleanSplitVertexIdentitySet {
        self.split_vertices
    }
    pub(crate) fn split_fragments(&self) -> &PlanarBooleanSplitEdgeFragmentSet {
        self.split_fragments
    }
    pub(crate) fn overlap_chains(&self) -> &PlanarBooleanOverlapEdgeChainSet {
        self.overlap_chains
    }
    pub(crate) fn split_chain_validation(&self) -> &PlanarBooleanSplitChainValidationReceipt {
        self.split_chain_validation
    }
    pub(crate) fn split_persistent_names(&self) -> &PlanarBooleanSplitPersistentNamingReceipt {
        self.split_persistent_names
    }
    pub(crate) fn split_decision_log(&self) -> &PlanarBooleanSplitDecisionLogQueryResult {
        self.split_decision_log
    }
}
