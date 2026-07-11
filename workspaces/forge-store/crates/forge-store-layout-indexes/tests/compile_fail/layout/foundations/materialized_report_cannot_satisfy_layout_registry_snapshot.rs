use forge_foundational::{
    FoundationalMaterializedPerformanceReport, FoundationalPolicyAdmissionReceipt,
};
use forge_store_layout_indexes::S8LayoutStrategyRegistrySnapshot;

fn require_snapshot(_snapshot: S8LayoutStrategyRegistrySnapshot) {}

fn main() {
    let report: FoundationalMaterializedPerformanceReport<FoundationalPolicyAdmissionReceipt> =
        todo!();
    require_snapshot(report);
}
