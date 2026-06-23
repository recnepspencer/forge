use worth_spatial::facade::planar_boolean_loop_reconstruction::PlanarBooleanLoopRoleOutcomeSet;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceRow;

fn main() {
    let role_outcomes: PlanarBooleanLoopRoleOutcomeSet = todo!();
    let _ = WorkloadEvidenceRow::from_boolean_evidence_receipt(&role_outcomes);
}
