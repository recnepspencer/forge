use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceRow, WorkloadEvidenceStageIndexProduct,
};

fn requires_stage_index(_: WorkloadEvidenceStageIndexProduct) {}

fn promote_raw_rows(rows: Vec<WorkloadEvidenceRow>) {
    requires_stage_index(rows);
}

fn main() {}
