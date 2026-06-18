use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanEdgeSplitRequest;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceRow;

fn main() {
    let split_request: PlanarBooleanEdgeSplitRequest = todo!();
    let _ = WorkloadEvidenceRow::from_boolean_evidence_receipt(&split_request);
}
