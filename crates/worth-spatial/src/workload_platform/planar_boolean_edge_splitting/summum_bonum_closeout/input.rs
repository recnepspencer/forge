use crate::workload_platform::planar_boolean_events::PlanarBooleanSegmentCandidateIndexProduct;

use super::super::{
    PlanarBooleanDownstreamSplitConsumption, PlanarBooleanEdgeSplitReplayParityReceipt,
    PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    PlanarBooleanLoopReconstructionSplitConsumption, PlanarBooleanOverlapEdgeChainSet,
    PlanarBooleanSplitDecisionLogReceipt, PlanarBooleanSplitEdgeChainLedgerReceipt,
    PlanarBooleanSplitEdgeFragmentSet, PlanarBooleanSplitPersistentNamingReceipt,
    PlanarBooleanSplitVertexIdentitySet,
};

#[derive(Clone, Copy)]
pub struct PlanarBooleanEdgeSplitSummumBonumCloseoutInput<'a> {
    candidate_index: &'a PlanarBooleanSegmentCandidateIndexProduct,
    endpoint_boundary: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    interval_subdivision: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    vertices: &'a PlanarBooleanSplitVertexIdentitySet,
    fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
    persistent_naming: &'a PlanarBooleanSplitPersistentNamingReceipt,
    decision_log: &'a PlanarBooleanSplitDecisionLogReceipt,
    split_ledger: &'a PlanarBooleanSplitEdgeChainLedgerReceipt,
    replay_parity: &'a PlanarBooleanEdgeSplitReplayParityReceipt,
    downstream_consumption: &'a PlanarBooleanDownstreamSplitConsumption,
    loop_reconstruction_consumption: &'a PlanarBooleanLoopReconstructionSplitConsumption,
}

impl<'a> PlanarBooleanEdgeSplitSummumBonumCloseoutInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        candidate_index: &'a PlanarBooleanSegmentCandidateIndexProduct,
        endpoint_boundary: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
        interval_subdivision: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
        vertices: &'a PlanarBooleanSplitVertexIdentitySet,
        fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
        persistent_naming: &'a PlanarBooleanSplitPersistentNamingReceipt,
        decision_log: &'a PlanarBooleanSplitDecisionLogReceipt,
        split_ledger: &'a PlanarBooleanSplitEdgeChainLedgerReceipt,
        replay_parity: &'a PlanarBooleanEdgeSplitReplayParityReceipt,
        downstream_consumption: &'a PlanarBooleanDownstreamSplitConsumption,
        loop_reconstruction_consumption: &'a PlanarBooleanLoopReconstructionSplitConsumption,
    ) -> Self {
        Self {
            candidate_index,
            endpoint_boundary,
            interval_subdivision,
            vertices,
            fragments,
            overlap_chains,
            persistent_naming,
            decision_log,
            split_ledger,
            replay_parity,
            downstream_consumption,
            loop_reconstruction_consumption,
        }
    }

    pub(crate) fn candidate_index(&self) -> &'a PlanarBooleanSegmentCandidateIndexProduct {
        self.candidate_index
    }
    pub(crate) fn endpoint_boundary(
        &self,
    ) -> &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet {
        self.endpoint_boundary
    }
    pub(crate) fn interval_subdivision(
        &self,
    ) -> &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
        self.interval_subdivision
    }
    pub(crate) fn vertices(&self) -> &'a PlanarBooleanSplitVertexIdentitySet {
        self.vertices
    }
    pub(crate) fn fragments(&self) -> &'a PlanarBooleanSplitEdgeFragmentSet {
        self.fragments
    }
    pub(crate) fn overlap_chains(&self) -> &'a PlanarBooleanOverlapEdgeChainSet {
        self.overlap_chains
    }
    pub(crate) fn persistent_naming(&self) -> &'a PlanarBooleanSplitPersistentNamingReceipt {
        self.persistent_naming
    }
    pub(crate) fn decision_log(&self) -> &'a PlanarBooleanSplitDecisionLogReceipt {
        self.decision_log
    }
    pub(crate) fn split_ledger(&self) -> &'a PlanarBooleanSplitEdgeChainLedgerReceipt {
        self.split_ledger
    }
    pub(crate) fn replay_parity(&self) -> &'a PlanarBooleanEdgeSplitReplayParityReceipt {
        self.replay_parity
    }
    pub(crate) fn downstream_consumption(&self) -> &'a PlanarBooleanDownstreamSplitConsumption {
        self.downstream_consumption
    }
    pub(crate) fn loop_reconstruction_consumption(
        &self,
    ) -> &'a PlanarBooleanLoopReconstructionSplitConsumption {
        self.loop_reconstruction_consumption
    }
}
