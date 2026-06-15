use worth_ui_harness::facade::HarnessEvidenceBundle;

fn main() {
    let mut evidence = HarnessEvidenceBundle::empty();
    evidence.observe_visible_frame();
}
