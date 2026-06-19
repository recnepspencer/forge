use worth_spatial::facade::planar_boolean_edge_splitting::PlanarBooleanSplitEdgeFragmentSet;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceRow;

fn main() {
    let fragments: PlanarBooleanSplitEdgeFragmentSet = todo!();
    let _ = WorkloadEvidenceRow::from_boolean_evidence_receipt(&fragments);
}
