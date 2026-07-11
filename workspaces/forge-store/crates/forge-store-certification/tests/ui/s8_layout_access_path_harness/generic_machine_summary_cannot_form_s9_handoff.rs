use forge_store_certification::project_s8_layout_handoff_grammar;
use forge_store_layout_indexes::layout_certification::S9_REQUIRED_LAYOUT_MACHINES;

fn main() {
    // A copied machine-name array is not an admitted lower Store handoff.
    let _ = project_s8_layout_handoff_grammar(S9_REQUIRED_LAYOUT_MACHINES);
}
