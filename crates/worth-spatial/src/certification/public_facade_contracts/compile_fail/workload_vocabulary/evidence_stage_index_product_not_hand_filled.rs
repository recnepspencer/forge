use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStageIndexProduct;

fn main() {
    let _ = WorkloadEvidenceStageIndexProduct {
        rows: Vec::new(),
        stage_offsets: [None; 26],
    };
}
