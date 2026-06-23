use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopReconstructionLedger;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceRow;

fn main() {
    let synthetic_ledger: PlanarBooleanLoopReconstructionLedger = todo!();
    let _ = WorkloadEvidenceRow::from_boolean_evidence_receipt(&synthetic_ledger);
}
