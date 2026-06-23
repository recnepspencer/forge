use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanWalkOutcomeSet;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceRow;

fn main() {
    let walk_outcomes: PlanarBooleanWalkOutcomeSet = todo!();
    let _ = WorkloadEvidenceRow::from_boolean_evidence_receipt(&walk_outcomes);
}
