use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapEdgeChainSet, PlanarBooleanSplitEdgeChainLedger,
    PlanarBooleanSplitEdgeChainLedgerReceipt, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitSourceEdgeCarrierSet,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::request::PlanarBooleanLoopReconstructionRequest;

pub struct PlanarBooleanLoopSourceProvenanceRecoveryInput<'a> {
    request: &'a PlanarBooleanLoopReconstructionRequest,
    split_ledger: &'a PlanarBooleanSplitEdgeChainLedger,
    split_ledger_receipt: &'a PlanarBooleanSplitEdgeChainLedgerReceipt,
    recovered_source_carriers: &'a PlanarBooleanSplitSourceEdgeCarrierSet,
    split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
}

impl<'a> PlanarBooleanLoopSourceProvenanceRecoveryInput<'a> {
    pub fn from_request_and_split_support(
        request: &'a PlanarBooleanLoopReconstructionRequest,
        split_ledger: &'a PlanarBooleanSplitEdgeChainLedger,
        split_ledger_receipt: &'a PlanarBooleanSplitEdgeChainLedgerReceipt,
        recovered_source_carriers: &'a PlanarBooleanSplitSourceEdgeCarrierSet,
        split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        overlap_chains: &'a PlanarBooleanOverlapEdgeChainSet,
    ) -> Self {
        Self {
            request,
            split_ledger,
            split_ledger_receipt,
            recovered_source_carriers,
            split_fragments,
            overlap_chains,
        }
    }

    pub(crate) fn request(&self) -> &'a PlanarBooleanLoopReconstructionRequest {
        self.request
    }

    pub(crate) fn split_ledger(&self) -> &'a PlanarBooleanSplitEdgeChainLedger {
        self.split_ledger
    }

    pub(crate) fn split_ledger_receipt(&self) -> &'a PlanarBooleanSplitEdgeChainLedgerReceipt {
        self.split_ledger_receipt
    }

    pub(crate) fn recovered_source_carriers(&self) -> &'a PlanarBooleanSplitSourceEdgeCarrierSet {
        self.recovered_source_carriers
    }

    pub(crate) fn split_fragments(&self) -> &'a PlanarBooleanSplitEdgeFragmentSet {
        self.split_fragments
    }

    pub(crate) fn overlap_chains(&self) -> &'a PlanarBooleanOverlapEdgeChainSet {
        self.overlap_chains
    }
}
