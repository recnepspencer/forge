use worth_spatial::facade::planar_boolean_edge_splitting::{
    PlanarBooleanOverlapEdgeChainSet, PlanarBooleanSplitEdgeChainLedger,
    PlanarBooleanSplitEdgeChainLedgerReceipt, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitSourceEdgeCarrierSet,
};
use worth_spatial::facade::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopReconstructionRequest, PlanarBooleanLoopSourceProvenanceRecoveryInput,
};

fn bogus<T>() -> T {
    panic!("compile-fail fixture")
}

fn main() {
    let _ = PlanarBooleanLoopSourceProvenanceRecoveryInput {
        request: bogus::<&PlanarBooleanLoopReconstructionRequest>(),
        split_ledger: bogus::<&PlanarBooleanSplitEdgeChainLedger>(),
        split_ledger_receipt: bogus::<&PlanarBooleanSplitEdgeChainLedgerReceipt>(),
        recovered_source_carriers: bogus::<&PlanarBooleanSplitSourceEdgeCarrierSet>(),
        split_fragments: bogus::<&PlanarBooleanSplitEdgeFragmentSet>(),
        overlap_chains: bogus::<&PlanarBooleanOverlapEdgeChainSet>(),
    };
}
