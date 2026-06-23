use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanFragmentContinuationIndex;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceRow;

fn main() {
    let continuation_index: PlanarBooleanFragmentContinuationIndex = todo!();
    let _ = WorkloadEvidenceRow::from_boolean_evidence_receipt(&continuation_index);
}
