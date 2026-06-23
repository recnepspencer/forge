use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanDegenerateLoopOutcomeSet;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceRow;

fn main() {
    let degenerate_outcomes: PlanarBooleanDegenerateLoopOutcomeSet = todo!();
    let _ = WorkloadEvidenceRow::from_boolean_evidence_receipt(&degenerate_outcomes);
}
