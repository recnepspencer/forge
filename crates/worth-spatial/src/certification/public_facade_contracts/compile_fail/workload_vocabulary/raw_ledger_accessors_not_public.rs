use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

fn main() {
    let ledger = WorkloadEvidenceLedger::from_rows(vec![WorkloadEvidenceRow::new(
        WorkloadEvidenceStage::Topology,
        "topology",
    )])
    .unwrap();

    let _ = ledger.rows();
    let _ = ledger.row_for_stage(WorkloadEvidenceStage::Topology);
    let _ = ledger.evidence_for_stage(WorkloadEvidenceStage::Topology);
}
