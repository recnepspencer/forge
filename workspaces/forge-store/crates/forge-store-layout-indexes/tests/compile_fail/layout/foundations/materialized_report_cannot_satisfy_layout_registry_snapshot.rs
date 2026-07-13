use forge_foundational::{
    FoundationalMaterializedPerformanceReport, FoundationalPolicyAdmissionReceipt,
};
use forge_store_layout_indexes::LayoutStrategyRegistrySnapshot;

fn require_snapshot(_snapshot: LayoutStrategyRegistrySnapshot) {}

fn main() {
    let report: FoundationalMaterializedPerformanceReport<FoundationalPolicyAdmissionReceipt> =
        todo!();
    require_snapshot(report);
}
