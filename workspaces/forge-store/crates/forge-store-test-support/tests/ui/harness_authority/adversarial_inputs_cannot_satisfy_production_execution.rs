use forge_store_physical_certification::layout_harness::runtime::S8RuntimeCoverageMatrix;
use forge_store_test_support::test_authority;

fn main() {
    let adversarial = test_authority::s8_layout_access::s8_layout_adversarial_inputs();
    let _ = S8RuntimeCoverageMatrix::default().record(adversarial);
}
