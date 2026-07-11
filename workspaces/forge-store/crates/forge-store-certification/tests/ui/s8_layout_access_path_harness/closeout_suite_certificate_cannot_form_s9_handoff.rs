use forge_store_certification::{
    certify_s8_layout_closeout_suite, project_s8_layout_handoff_grammar,
};
use forge_store_physical_certification::layout_harness::runtime::S8RuntimeCoverageMatrix;

fn main() {
    let matrix = S8RuntimeCoverageMatrix::default();
    let suite = certify_s8_layout_closeout_suite(&matrix).unwrap();
    // A closeout suite is courtroom evidence, not lower Store handoff grammar.
    let _ = project_s8_layout_handoff_grammar(suite);
}
