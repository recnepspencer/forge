use worth_foundational::FoundationalMaterializedPerformanceReport;
use worth_store_layout_indexes::S8ExecutionReadmissionWitness;

fn require_readmission(_: S8ExecutionReadmissionWitness) {}

fn main() {
    let report: FoundationalMaterializedPerformanceReport<()> = todo!();
    require_readmission(report);
}
