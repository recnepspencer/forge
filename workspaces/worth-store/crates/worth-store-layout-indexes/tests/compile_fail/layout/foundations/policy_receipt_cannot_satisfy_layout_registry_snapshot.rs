use worth_foundational::FoundationalPolicyAdmissionReceipt;
use worth_store_layout_indexes::LayoutStrategyRegistrySnapshot;

fn require_snapshot(_snapshot: LayoutStrategyRegistrySnapshot) {}

fn main() {
    let receipt: FoundationalPolicyAdmissionReceipt = todo!();
    require_snapshot(receipt);
}
