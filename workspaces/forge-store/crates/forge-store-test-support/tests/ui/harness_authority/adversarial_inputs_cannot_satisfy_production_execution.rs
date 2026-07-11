use forge_store_physical_certification::layout_harness::runtime::LayoutRuntimeCoverageMatrix;
use forge_store_test_support::test_authority;

fn main() {
    let adversarial = test_authority::s8_layout_projection::s8_layout_adversarial_inputs();
    let _ = LayoutRuntimeCoverageMatrix::default().record(adversarial);
}
