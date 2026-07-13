use forge_foundational::FoundationalPolicyAdmissionReceipt;
use forge_store_layout_indexes::LayoutStrategyRegistrySnapshot;

fn require_snapshot(_snapshot: LayoutStrategyRegistrySnapshot) {}

fn main() {
    let receipt: FoundationalPolicyAdmissionReceipt = todo!();
    require_snapshot(receipt);
}
