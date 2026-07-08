use forge_foundational::FoundationalMaterializedPerformanceReport;
use forge_store_layout_indexes::S8LayoutReadmissionWitness;

fn require_readmission(_: S8LayoutReadmissionWitness) {}

fn main() {
    let report: FoundationalMaterializedPerformanceReport<()> = todo!();
    require_readmission(report);
}
