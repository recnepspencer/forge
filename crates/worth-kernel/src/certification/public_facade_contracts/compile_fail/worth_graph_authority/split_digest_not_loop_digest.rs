use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitOperationalTruthDigest;
use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionLedgerReceipt;

fn requires_loop_ledger_receipt(_: PlanarBooleanLoopReconstructionLedgerReceipt) {}

fn promote_split_digest(split_digest: PlanarBooleanSplitOperationalTruthDigest) {
    requires_loop_ledger_receipt(split_digest);
}

fn main() {}
