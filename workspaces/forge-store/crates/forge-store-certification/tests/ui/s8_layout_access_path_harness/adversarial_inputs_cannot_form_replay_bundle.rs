use forge_store_certification::assemble_s8_layout_replay_bundle;
use forge_store_test_support::test_authority::s8_layout_projection::s8_layout_adversarial_inputs;

fn main() {
    let adversarial = s8_layout_adversarial_inputs();
    let _ = assemble_s8_layout_replay_bundle(adversarial);
}
