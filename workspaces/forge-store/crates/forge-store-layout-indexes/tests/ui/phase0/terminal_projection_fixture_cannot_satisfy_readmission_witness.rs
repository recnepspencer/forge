use forge_store_layout_indexes::S8ExecutionReadmissionWitness;
use forge_store_test_support::StoreTerminalProjectionJsonFixture;

fn require_readmission(_: S8ExecutionReadmissionWitness) {}

fn main() {
    let projection: StoreTerminalProjectionJsonFixture = todo!();
    require_readmission(projection);
}
