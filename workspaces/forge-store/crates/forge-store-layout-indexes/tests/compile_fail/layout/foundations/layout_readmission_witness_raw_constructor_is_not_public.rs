use forge_store_layout_indexes::declarations::layout_declarations;
use forge_store_layout_indexes::integrity::LayoutReadmissionWitness;

fn main() {
    let family = layout_declarations().seed_family().family();
    let _ = LayoutReadmissionWitness::terminal_import(family);
}
