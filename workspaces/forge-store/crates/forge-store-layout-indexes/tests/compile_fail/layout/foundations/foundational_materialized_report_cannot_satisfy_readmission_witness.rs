use forge_store_layout_indexes::integrity::LayoutReadmissionWitness;

struct FoundationalMaterializedPerformanceReport<T>(T);

fn require_readmission(_: LayoutReadmissionWitness) {}

fn main() {
    let report = FoundationalMaterializedPerformanceReport(());
    require_readmission(report);
}
