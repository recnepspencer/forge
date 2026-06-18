use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet, PlanarBooleanOverlapEdgeChainSet,
    PlanarBooleanSplitChainValidationReceipt, PlanarBooleanSplitDecisionLogQueryResult,
    PlanarBooleanSplitEdgeChainLedgerQueryResult, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitOperationalTruthDigest, PlanarBooleanSplitPersistentNamingReceipt,
    PlanarBooleanSplitVertexIdentitySet,
};

pub struct PlanarBooleanEdgeSplitCloseout<'a> {
    closeout_identity: String,
    request: &'a PlanarBooleanEdgeSplitRequest,
    endpoint_boundary: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    interval_subdivision: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    vertices: &'a PlanarBooleanSplitVertexIdentitySet,
    fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
    validation: &'a PlanarBooleanSplitChainValidationReceipt,
    naming: &'a PlanarBooleanSplitPersistentNamingReceipt,
    decision_log: &'a PlanarBooleanSplitDecisionLogQueryResult,
    ledger: &'a PlanarBooleanSplitEdgeChainLedgerQueryResult,
    operational_truth: PlanarBooleanSplitOperationalTruthDigest,
}

impl<'a> PlanarBooleanEdgeSplitCloseout<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn from_query_products(
        request: &'a PlanarBooleanEdgeSplitRequest,
        endpoint_boundary: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
        interval_subdivision: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
        vertices: &'a PlanarBooleanSplitVertexIdentitySet,
        fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
        validation: &'a PlanarBooleanSplitChainValidationReceipt,
        naming: &'a PlanarBooleanSplitPersistentNamingReceipt,
        decision_log: &'a PlanarBooleanSplitDecisionLogQueryResult,
        ledger: &'a PlanarBooleanSplitEdgeChainLedgerQueryResult,
    ) -> Self {
        let operational_truth = PlanarBooleanSplitOperationalTruthDigest::from_split_products(
            fragments, validation, naming,
        );
        let closeout_identity = closeout_identity(
            request,
            endpoint_boundary,
            interval_subdivision,
            vertices,
            fragments,
            overlap_chains,
            validation,
            naming,
            decision_log,
            ledger,
            &operational_truth,
        );
        Self {
            closeout_identity,
            request,
            endpoint_boundary,
            interval_subdivision,
            vertices,
            fragments,
            overlap_chains,
            validation,
            naming,
            decision_log,
            ledger,
            operational_truth,
        }
    }

    pub fn closeout_identity(&self) -> &str {
        &self.closeout_identity
    }
    pub fn request(&self) -> &'a PlanarBooleanEdgeSplitRequest {
        self.request
    }
    pub fn endpoint_boundary(&self) -> &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet {
        self.endpoint_boundary
    }
    pub fn interval_subdivision(
        &self,
    ) -> &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
        self.interval_subdivision
    }
    pub fn vertices(&self) -> &'a PlanarBooleanSplitVertexIdentitySet {
        self.vertices
    }
    pub fn fragments(&self) -> &'a PlanarBooleanSplitEdgeFragmentSet {
        self.fragments
    }
    pub fn overlap_chains(&self) -> &'a PlanarBooleanOverlapEdgeChainSet {
        self.overlap_chains
    }
    pub fn validation(&self) -> &'a PlanarBooleanSplitChainValidationReceipt {
        self.validation
    }
    pub fn naming(&self) -> &'a PlanarBooleanSplitPersistentNamingReceipt {
        self.naming
    }
    pub fn decision_log(&self) -> &'a PlanarBooleanSplitDecisionLogQueryResult {
        self.decision_log
    }
    pub fn ledger(&self) -> &'a PlanarBooleanSplitEdgeChainLedgerQueryResult {
        self.ledger
    }
    pub fn operational_truth(&self) -> &PlanarBooleanSplitOperationalTruthDigest {
        &self.operational_truth
    }
}

#[allow(clippy::too_many_arguments)]
fn closeout_identity(
    request: &PlanarBooleanEdgeSplitRequest,
    endpoint_boundary: &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    interval_subdivision: &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    vertices: &PlanarBooleanSplitVertexIdentitySet,
    fragments: &PlanarBooleanSplitEdgeFragmentSet,
    overlap_chains: &PlanarBooleanOverlapEdgeChainSet,
    validation: &PlanarBooleanSplitChainValidationReceipt,
    naming: &PlanarBooleanSplitPersistentNamingReceipt,
    decision_log: &PlanarBooleanSplitDecisionLogQueryResult,
    ledger: &PlanarBooleanSplitEdgeChainLedgerQueryResult,
    operational_truth: &PlanarBooleanSplitOperationalTruthDigest,
) -> String {
    format!(
        "edge-split-closeout:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
        request.split_request_identity(),
        endpoint_boundary.schedule_set_identity(),
        interval_subdivision.schedule_set_identity(),
        vertices.split_vertex_identity_set_identity(),
        fragments.fragment_set_identity(),
        overlap_chains.chain_set_identity(),
        validation.receipt_identity(),
        naming.receipt_identity(),
        decision_log.receipt().receipt_identity(),
        ledger.receipt().receipt_identity(),
        operational_truth.digest_identity()
    )
}
