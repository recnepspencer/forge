use worth_store_layout_indexes::integrity::LayoutReadmissionWitness;
use worth_store_test_support::StoreTerminalProjectionJsonFixture;

fn require_readmission(_: LayoutReadmissionWitness) {}

fn main() {
    let projection: StoreTerminalProjectionJsonFixture = todo!();
    require_readmission(projection);
}
