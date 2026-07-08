use forge_store_layout_indexes::S8LayoutReadmissionWitness;
use forge_store_test_support::StoreTerminalProjectionJsonFixture;

fn require_readmission(_: S8LayoutReadmissionWitness) {}

fn main() {
    let projection: StoreTerminalProjectionJsonFixture = todo!();
    require_readmission(projection);
}
