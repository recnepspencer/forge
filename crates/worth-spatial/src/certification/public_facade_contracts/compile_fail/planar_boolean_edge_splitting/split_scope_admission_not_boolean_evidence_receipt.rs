use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitScopeAdmission;
use worth_spatial::facade::workload_vocabulary::BooleanEvidenceReceipt;

fn main() {
    let admission = scope_admission();
    require_boolean_evidence_receipt(&admission);
}

fn require_boolean_evidence_receipt(_: &impl BooleanEvidenceReceipt) {}

fn scope_admission() -> PlanarBooleanEdgeSplitScopeAdmission {
    loop {}
}
