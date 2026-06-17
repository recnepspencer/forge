use worth_spatial::facade::workload_operators::CoplanarOverlapWorkloadOperator;
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceRow;

fn main() {
    let rows: Vec<WorkloadEvidenceRow> = Vec::new();
    let _ = CoplanarOverlapWorkloadOperator::from_consumed_evidence(&rows);
}
