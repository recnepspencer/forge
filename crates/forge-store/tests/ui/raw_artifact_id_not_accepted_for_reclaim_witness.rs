fn require_witness(_: forge_store::ReclaimEligibilityWitness) {}

fn main() {
    require_witness(String::from("layout-materialization:raw-id"));
}
