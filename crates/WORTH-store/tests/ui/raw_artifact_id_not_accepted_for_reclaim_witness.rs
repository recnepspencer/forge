fn require_witness(_: worth_store::ReclaimEligibilityWitness) {}

fn main() {
    require_witness(String::from("layout-materialization:raw-id"));
}
