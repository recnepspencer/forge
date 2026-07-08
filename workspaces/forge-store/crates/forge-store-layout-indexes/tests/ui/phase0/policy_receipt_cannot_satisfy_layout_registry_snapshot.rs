use forge_foundational::FoundationalPolicyAdmissionReceipt;
use forge_store_layout_indexes::S8LayoutStrategyRegistrySnapshot;

fn require_snapshot(_snapshot: S8LayoutStrategyRegistrySnapshot) {}

fn main() {
    let receipt: FoundationalPolicyAdmissionReceipt = todo!();
    require_snapshot(receipt);
}
